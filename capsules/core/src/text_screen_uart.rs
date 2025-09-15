// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

//! UART over a generic TextScreen (kernel::hil::text_screen).
//!
//! TX: forwards `transmit_buffer()` to `screen.print(..)` and completes via
//! `TextScreenClient::write_complete()`. RX: producers call `inject_byte()`;
//! bytes are buffered and completes on `\n` or when full.

// ONLY FOR NOW, WILL BE REMOVED LATER
#[allow(dead_code)]
use core::cell::{Cell, RefCell};
use core::cmp;

use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::text_screen::{TextScreen as HilTextScreen, TextScreenClient};
use kernel::hil::uart::{
    Configure, Error as UartError, Parameters, Receive, ReceiveClient, Transmit, TransmitClient,
};
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::ErrorCode;

const RX_FIFO_SIZE: usize = 64;

// UART interface over a generic text screen
pub struct TextConsoleUart<'a, S: HilTextScreen<'a>> {
    // where we output
    screen: &'a S,

    // UART clients
    tx_client: OptionalCell<&'a dyn TransmitClient>,
    rx_client: OptionalCell<&'a dyn ReceiveClient>,

    // TX state: in-flight write (owned by screen until write_complete())
    tx_busy: Cell<bool>,
    // we shouldn't need a deferred call for TX because TextScreen

    // RX state
    rx_buf: TakeCell<'static, [u8]>,
    rx_wanted: Cell<usize>,
    rx_len: Cell<usize>,

    // Type-ahead FIFO (drop-oldest on overflow
    rx_fifo: RefCell<[u8; RX_FIFO_SIZE]>,
    // ONLY FOR NOW, WILL BE REMOVED LATER
    #[allow(dead_code)]
    rx_head: Cell<usize>,
    rx_tail: Cell<usize>,
    rx_count: Cell<usize>,

    // deferred call to pump RX outside producer context
    dcall: DeferredCall,

    // Odd take
    // ProcessConsole typically treats submit as \n.
    // Users may send \r (Enter on some keyboards) or \r\n
    skip_next_lf: Cell<bool>, // if we just wrote '\n' because we saw '\r'
}

// ONLY FOR NOW, WILL BE REMOVED LATER
#[allow(dead_code)]
impl<'a, S: HilTextScreen<'a>> TextConsoleUart<'a, S> {
    pub fn new(screen: &'a S) -> Self {
        Self {
            screen,
            tx_client: OptionalCell::empty(),
            rx_client: OptionalCell::empty(),

            tx_busy: Cell::new(false),

            rx_buf: TakeCell::empty(),
            rx_wanted: Cell::new(0),
            rx_len: Cell::new(0),

            rx_fifo: RefCell::new([0; RX_FIFO_SIZE]),
            rx_head: Cell::new(0),
            rx_tail: Cell::new(0),
            rx_count: Cell::new(0),

            dcall: DeferredCall::new(),

            skip_next_lf: Cell::new(false),
        }
    }

    /// Must be called by the board after construction to receive screen callbacks
    /// (Call: screen.set_client(Some(capsule)) )
    pub fn set_as_screen_client(&'static self) {
        self.screen.set_client(Some(self));
    }

    /// Feed one input byte (from keyboard glue). Safe from deferred context
    pub fn inject_byte(&self, b: u8) {
        self.fifo_push(b);
        self.dcall.set();
    }

    // RX helpers
    fn fifo_push(&self, b: u8) {
        if self.rx_count.get() == RX_FIFO_SIZE {
            // drop-oldest
            self.rx_tail.set((self.rx_tail.get() + 1) % RX_FIFO_SIZE);
            // count stays at full
        } else {
            self.rx_count.set(self.rx_count.get() + 1);
        }
        let h = self.rx_head.get();
        self.rx_fifo.borrow_mut()[h] = b;
        self.rx_head.set((h + 1) % RX_FIFO_SIZE);
    }

    fn fifo_pop(&self) -> Option<u8> {
        if self.rx_count.get() == 0 {
            return None;
        }
        let t = self.rx_tail.get();
        let b = self.rx_fifo.borrow()[t];
        self.rx_tail.set((t + 1) % RX_FIFO_SIZE);
        self.rx_count.set(self.rx_count.get() - 1);
        Some(b)
    }

    fn pump_rx(&self) {
        let mut should_complete = false;
        let mut new_len = self.rx_len.get();

        self.rx_buf.map(|buf| {
            let wanted = self.rx_wanted.get();
            let mut len = self.rx_len.get();

            while len < wanted {
                // Pull next byte from FIFO
                let mut b = match self.fifo_pop() {
                    Some(b) => b,
                    None => break,
                };

                // CR/LF normalization
                // If previous byte was a '\r' we turned into '\n', skip a following real '\n'
                if b == b'\n' && self.skip_next_lf.replace(false) {
                    continue;
                }
                // Map lone '\r' to '\n' and remember to drop a following '\n' (CRLF)
                if b == b'\r' {
                    b = b'\n';
                    self.skip_next_lf.set(true);
                }

                buf[len] = b;
                len += 1;

                if b == b'\n' {
                    // include newline and complete early
                    break;
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

                // Move the 'static buffer into the callback
                self.rx_client.map(move |client| {
                    client.received_buffer(buf, len, Ok(()), UartError::None);
                });
            }
        }
    }
}

// Deferred Call Client
impl<'a, S: HilTextScreen<'a>> DeferredCallClient for TextConsoleUart<'a, S> {
    fn handle_deferred_call(&self) {
        self.pump_rx();
    }

    fn register(&'static self) {
        self.dcall.register(self);
    }
}

// UART Transmit

impl<'a, S: HilTextScreen<'a>> Transmit<'a> for TextConsoleUart<'a, S> {
    fn set_transmit_client(&self, client: &'a dyn TransmitClient) {
        self.tx_client.set(client);
    }

    fn transmit_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if self.tx_busy.get() {
            return Err((ErrorCode::BUSY, buffer));
        }

        let write_len = cmp::min(len, buffer.len());
        self.tx_busy.set(true);
        match self.screen.print(buffer, write_len) {
            Ok(()) => Ok(()),
            Err((e, buf)) => {
                self.tx_busy.set(false);
                Err((e, buf))
            }
        }
    }

    fn transmit_word(&self, _word: u32) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn transmit_abort(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }
}

// UART Receive

impl<'a, S: HilTextScreen<'a>> Receive<'a> for TextConsoleUart<'a, S> {
    fn set_receive_client(&self, client: &'a dyn ReceiveClient) {
        self.rx_client.set(client);
    }

    fn receive_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if self.rx_buf.is_some() {
            return Err((ErrorCode::BUSY, buffer));
        }
        let wanted = cmp::min(len, buffer.len());
        if wanted == 0 {
            return Err((ErrorCode::SIZE, buffer));
        }

        self.rx_len.set(0);
        self.rx_wanted.set(wanted);
        self.rx_buf.replace(buffer);

        // Try to consume immediately from FIFO
        self.dcall.set();
        Ok(())
    }

    fn receive_word(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn receive_abort(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }
}

// UART over TextScreen: Screen Client
impl<'a, S: HilTextScreen<'a>> TextScreenClient for TextConsoleUart<'a, S> {
    fn command_complete(&self, _r: Result<(), ErrorCode>) {
        // We don't currently expose cursor/display commands via UART
    }

    fn write_complete(&self, buffer: &'static mut [u8], len: usize, r: Result<(), ErrorCode>) {
        self.tx_busy.set(false);
        self.tx_client.map(|client| {
            client.transmitted_buffer(buffer, len, r);
        });
    }
}

impl<'a, S: HilTextScreen<'a>> Configure for TextConsoleUart<'a, S> {
    fn configure(&self, _params: Parameters) -> Result<(), ErrorCode> {
        Ok(())
    }
}
