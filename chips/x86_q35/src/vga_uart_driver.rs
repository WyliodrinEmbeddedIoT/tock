// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

//! VgaUart` — a **synchronous, write-only (TX)** backend
//!
//! Implements `hil:uart::Transmit` and `hil::uart::Receive`
//! can read keyboard input injected by the keyboard-console capsule.
//!
//! RX policy
//! - Bytes are injected via `inject_input_byte(b)` (called by the capsule)
//! - If a receive is active, we copy into the caller/s buffer and complete
//! on newline or when full
//! - If no receive is active, bytes go to a small `type-Ahead FIFO1` (drops oldest when full)
//! - Work is done in a deferred call (IRQ is kept small)
//!
//! Note for TX:
//! `transmit_buffer` reports the consumed length : length(min(requested, buf.len())).

use crate::vga::Vga;
use core::{
    cell::{Cell, RefCell},
    cmp,
};
use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::uart::{
    Configure, Error as UartError, Parameters, Receive, ReceiveClient, Transmit, TransmitClient,
};
use kernel::utilities::cells::TakeCell;
use kernel::ErrorCode;
use tock_cells::optional_cell::OptionalCell;

const RX_FIFO_SIZE: usize = 64;

/// UART-compatible wrapper around the VGA text writer.
pub struct VgaText<'a> {
    // TX side
    vga_buffer: Vga,
    tx_client: OptionalCell<&'a dyn TransmitClient>,
    deferred_call: DeferredCall,
    pending_buf: TakeCell<'static, [u8]>,
    // number of actual bytes consumed (not the requested length)
    pending_len_consumed: Cell<usize>,
    // RX side
    rx_client: OptionalCell<&'a dyn ReceiveClient>,
    rx_buf: TakeCell<'static, [u8]>,
    rx_wanted: Cell<usize>,
    rx_len: Cell<usize>,
    // small FIFO (drop oldest on overflow)
    rx_fifo: RefCell<[u8; RX_FIFO_SIZE]>,
    rx_head: Cell<usize>,
    rx_tail: Cell<usize>,
    rx_count: Cell<usize>,
}

impl VgaText<'_> {
    pub fn new() -> Self {
        Self {
            vga_buffer: Vga::new(),
            tx_client: OptionalCell::empty(),
            rx_client: OptionalCell::empty(),
            deferred_call: DeferredCall::new(),
            pending_buf: TakeCell::empty(),
            pending_len_consumed: Cell::new(0),
            rx_buf: TakeCell::empty(),
            rx_wanted: Cell::new(0),
            rx_len: Cell::new(0),
            rx_fifo: RefCell::new([0; RX_FIFO_SIZE]),
            rx_head: Cell::new(0),
            rx_tail: Cell::new(0),
            rx_count: Cell::new(0),
        }
    }

    // TX helpers
    fn fire_tx_callback(&self, buf: &'static mut [u8], len_consumed: usize) {
        self.tx_client.map(|client| {
            client.transmitted_buffer(buf, len_consumed, Ok(()));
        });
    }

    // RX helpers
    fn fifo_push(&self, b: u8) {
        // drop-oldest on overflow
        if self.rx_count.get() == RX_FIFO_SIZE {
            // overwrite oldest: advance tail, decrement count stays at full
            self.rx_tail.set((self.rx_tail.get() + 1) % RX_FIFO_SIZE);
            self.rx_count.set(self.rx_count.get()) //stays full
        } else {
            self.rx_count.set(self.rx_count.get() + 1);
        }
        let head = self.rx_head.get();
        self.rx_fifo.borrow_mut()[head] = b;
        self.rx_head.set((head + 1) % RX_FIFO_SIZE);
    }
    fn fifo_pop(&self) -> Option<u8> {
        if self.rx_count.get() == 0 {
            return None;
        }
        let tail = self.rx_tail.get();
        let b = self.rx_fifo.borrow()[tail];
        self.rx_tail.set((tail + 1) % RX_FIFO_SIZE);
        self.rx_count.set(self.rx_count.get() - 1);
        Some(b)
    }

    fn pump_rx(&self) {
        // Move bytes from FIFO into an active RX buffer
        // Complete when newline is copied or wanted bytes threshold reached
        let mut should_complete = false;

        // we need the current length after the borrow ends
        let mut new_len = self.rx_len.get();

        self.rx_buf.map(|buf| {
            let wanted = self.rx_wanted.get();
            let mut len = self.rx_len.get();

            while len < wanted {
                match self.fifo_pop() {
                    Some(b) => {
                        buf[len] = b;
                        len += 1;
                        if b == b'\n' {
                            // include the newline in the completion
                            break;
                        }
                    }
                    None => break, // Nothing more available
                }
            }
            new_len = len;
            let last_is_newline = len > 0 && buf[len - 1] == b'\n';
            if len == wanted || last_is_newline {
                should_complete = true;
            }
        });
        self.rx_len.set(new_len);

        if should_complete {
            if let Some(buf) = self.rx_buf.take() {
                let len = self.rx_len.get();
                self.rx_len.set(0);
                self.rx_wanted.set(0);
                self.rx_client.map(|client| {
                    client.received_buffer(buf, len, Ok(()), UartError::None);
                });
            }
        }
    }
    /// Called by the keyboard_console capsule to feed a keystroke
    /// Safe to call from deferred context only
    pub fn inject_input_byte(&self, b: u8) {
        self.fifo_push(b);
        // Defer delivery into the active RX buffer
        self.deferred_call.set();
    }
}

// DeferredCallClient implementation
impl DeferredCallClient for VgaText<'_> {
    fn handle_deferred_call(&self) {
        if let Some(buf) = self.pending_buf.take() {
            let len = self.pending_len_consumed.get();
            self.fire_tx_callback(buf, len);
        }
        self.pump_rx();
    }

    fn register(&'static self) {
        self.deferred_call.register(self);
    }
}

// Transmit for Vga
impl<'a> Transmit<'a> for VgaText<'a> {
    fn set_transmit_client(&self, client: &'a dyn TransmitClient) {
        self.tx_client.set(client);
    }

    fn transmit_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        let write_len = cmp::min(len, buffer.len());
        for &byte in &buffer[..write_len] {
            self.vga_buffer.write_byte(byte);
        }
        self.pending_buf.replace(buffer);
        self.pending_len_consumed.set(write_len);
        self.deferred_call.set();
        Ok(())
    }

    fn transmit_word(&self, _word: u32) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn transmit_abort(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }
}

// Receive for Vga
impl<'a> Receive<'a> for VgaText<'a> {
    fn set_receive_client(&self, client: &'a dyn ReceiveClient) {
        self.rx_client.set(client);
    }

    fn receive_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        // Only one receive allowed
        let busy = {
            let mut present = false;
            self.rx_buf.map(|_| {
                present = true;
            });
            present
        };
        if busy {
            return Err((ErrorCode::BUSY, buffer));
        }
        let wanted = cmp::min(len, buffer.len());
        if wanted == 0 {
            return Err((ErrorCode::SIZE, buffer));
        }
        self.rx_len.set(0);
        self.rx_wanted.set(wanted);
        self.rx_buf.replace(buffer);

        // Try to place immediately from FIFO
        self.deferred_call.set();
        Ok(())
    }

    fn receive_word(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn receive_abort(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }
}

// Configure for Vga
impl Configure for VgaText<'_> {
    fn configure(&self, _params: Parameters) -> Result<(), ErrorCode> {
        Ok(())
    }
}
