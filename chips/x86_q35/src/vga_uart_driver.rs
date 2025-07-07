//! `VgaUart` — a **synchronous, write‑only** façade that lets capsules
//! expecting a `hil::uart::Uart` talk to the VGA text buffer instead of a
//! serial port.  Lives solely in the x86 board crate; no core‑kernel changes.
//!
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

#![cfg(feature = "vga_text_80x25")]

use core::cell::Cell;
use core::cmp;
use core::fmt::Write as FmtWrite;

use kernel::hil::uart::{Configure, Parameters, Receive, ReceiveClient, Transmit, TransmitClient};
use kernel::ErrorCode;
use kernel::hil::uart;
use crate::vga::VgaText;

/// Global VGA writer allocated in `chips/x86_q35/vga.rs`.
extern "Rust" {
    static mut VGA_TEXT: VgaText;
}

/// UART‑compatible wrapper around the VGA text writer.
pub struct VgaUart {
    writer_ptr: *mut VgaText,
    tx_client: Cell<Option<&'static dyn TransmitClient>>,  // no lifetime param
    rx_client: Cell<Option<&'static dyn ReceiveClient>>,   // no lifetime param
}

impl VgaUart {
    /// # Safety
    /// `writer_ptr` must point to the **static** `VGA_TEXT` for the kernel's
    /// lifetime.
    pub const unsafe fn new(writer_ptr: *mut VgaText) -> Self {
        Self {
            writer_ptr,
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
        self.tx_client.set(Some(unsafe { core::mem::transmute::<&'a dyn TransmitClient, &'static dyn TransmitClient>(client) }));
    }

    fn transmit_buffer(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        // Write synchronously.
        unsafe {
            let writer = &mut *self.writer_ptr;
            let write_len = cmp::min(len, buffer.len());
            for &byte in &buffer[..write_len] {
                let _ = writer.write_char(byte as char);
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
        self.rx_client.set(Some(unsafe { core::mem::transmute::<&'a dyn ReceiveClient, &'static dyn ReceiveClient>(client) }));
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
