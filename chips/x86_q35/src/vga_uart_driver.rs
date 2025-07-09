// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2024.

//! VgaUart` — a **synchronous, write-only** façade that lets capsules
//! use a `hil::uart::Uart`-style interface while we actually write to
//! the VGA text buffer, not a serial port.
//!
//! The type lives only in te x86 Q35 board crate; the core kernel
//! remains untouched

//! Key design points
//!
//! • Implements the *minimum subset* of `Transmit` required by `MuxUart`.
//!   All writes copy bytes to the global `VGA_TEXT` then immediately invoke
//!   the transmit callback (split‑phase contract).  No DMA, so synchronous is
//!   safe.
//! • **Receive / abort / re‑configure** operations just return
//!   `ErrorCode::NOSUPPORT` — VGA is output‑only.
//! • No explicit `impl Uart` needed; the blanket `impl` in the HIL activates
//!   automatically once we satisfy `Configure + Transmit + Receive`.
//!
//! This file is compiled **only** when the `vga_text_80x25` feature is on, so
//! normal serial‑only builds stay unaffected.

use core::fmt::Write as FmtWrite;
use core::{cell::Cell, cmp};
use spin::Mutex;

use crate::vga::VgaText;
use kernel::hil::uart;
use kernel::hil::uart::{Configure, Parameters, Receive, ReceiveClient, Transmit, TransmitClient};
use kernel::ErrorCode;

/// UART‑compatible wrapper around the VGA text writer.
pub struct VgaUart {
    writer: &'static Mutex<VgaText>,
    tx_client: Cell<Option<&'static dyn TransmitClient>>, // no lifetime param
    rx_client: Cell<Option<&'static dyn ReceiveClient>>,  // no lifetime param
}

impl VgaUart {
    /// # Safety
    /// `writer_ptr` must point to the **static** `VGA_TEXT` for the kernel's
    /// lifetime.
    pub const unsafe fn new(writer: &'static Mutex<VgaText>) -> Self {
        Self {
            writer,
            tx_client: Cell::new(None),
            rx_client: Cell::new(None),
        }
    }

    fn fire_tx_callback(&self, buf: &'static mut [u8], len: usize) {
        if let Some(client) = self.tx_client.get() {
            client.transmitted_buffer(buf, len, Ok(()));
        }
    }
}

// Transmit (sync)
impl<'a> Transmit<'a> for VgaUart {
    fn set_transmit_client(&self, client: &'a dyn TransmitClient) {
        // Extend lifetime to 'static — safe: client lives for entire kernel run.
        self.tx_client.set(Some(unsafe {
            core::mem::transmute::<&'a dyn TransmitClient, &'static dyn TransmitClient>(client)
        }));
    }

    fn transmit_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        let write_len = cmp::min(len, buffer.len());
        {
            let mut vga = self.writer.lock();
            for &byte in &buffer[..write_len] {
                let _ = vga.write_char(byte as char);
            }
        }
        self.fire_tx_callback(buffer, len);
        Ok(())
    }

    fn transmit_word(&self, _word: u32) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn transmit_abort(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }
}

// Receive, all blueprints
impl<'a> Receive<'a> for VgaUart {
    fn set_receive_client(&self, client: &'a dyn ReceiveClient) {
        self.rx_client.set(Some(unsafe {
            core::mem::transmute::<&'a dyn ReceiveClient, &'static dyn ReceiveClient>(client)
        }));
    }

    fn receive_buffer(
        &self,
        buffer: &'static mut [u8],
        _len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if let Some(client) = self.rx_client.get() {
            // zero bytes received, success
            client.received_buffer(buffer, 0, Ok(()), uart::Error::None);
        }
        Ok(())
    }

    fn receive_word(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn receive_abort(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }
}

// Configure, no operation
impl Configure for VgaUart {
    fn configure(&self, _params: Parameters) -> Result<(), ErrorCode> {
        Ok(())
    }
}
