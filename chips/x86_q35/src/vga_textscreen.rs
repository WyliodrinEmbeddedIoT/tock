// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

//! VgaTextScreen - chip-side adapter that implements the kernel `TextScreen` HIL
//! by wrapping the existing VGA text-mode writer
//!
//! - `print()` writes bytes to VGA immediately, the completes via a deferred call
//! - Other commands are completed synchronously
//!
//! This file intentionally contains **no** UART concepts. It is purely a TextScreen

use crate::vga::{Vga, TEXT_BUFFER_HEIGHT, TEXT_BUFFER_WIDTH};
use core::cell::Cell;
use kernel::deferred_call::{DeferredCall, DeferredCallClient};
use kernel::hil::text_screen::{TextScreen as HilTextScreen, TextScreenClient};
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::ErrorCode;

pub struct VgaTextScreen<'a> {
    vga: Vga,
    client: OptionalCell<&'a dyn TextScreenClient>,

    // deferred completion for print()
    dcall: DeferredCall,
    tx_buf: TakeCell<'static, [u8]>,
    tx_len: Cell<usize>,
    busy: Cell<bool>,
}

impl VgaTextScreen<'_> {
    #[allow(dead_code)]
    // WILL BE REMOVED IN THE FUTURE
    pub fn new() -> Self {
        Self {
            vga: Vga::new(),
            client: OptionalCell::empty(),
            dcall: DeferredCall::new(),
            tx_buf: TakeCell::empty(),
            tx_len: Cell::new(0),
            busy: Cell::new(false),
        }
    }

    #[allow(dead_code)]
    // WILL BE REMOVED IN THE FUTURE
    /// (Optional) helpers the board may want to expose later.
    pub fn clear_screen(&self) {
        self.vga.clear();
    }

    // Keep the board wiring simple

    #[allow(dead_code)]
    // WILL BE REMOVED IN THE FUTURE
    pub fn register_deferred_call(&'static self) {
        self.dcall.register(self);
    }
}

impl<'a> HilTextScreen<'a> for VgaTextScreen<'a> {
    fn set_client(&self, client: Option<&'a dyn TextScreenClient>) {
        match client {
            Some(c) => self.client.set(c),
            None => {
                let _ = self.client.take();
            }
        }
    }
    fn get_size(&self) -> (usize, usize) {
        (TEXT_BUFFER_WIDTH, TEXT_BUFFER_HEIGHT)
    }

    fn print(
        &self,
        buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if self.busy.get() {
            return Err((ErrorCode::BUSY, buffer));
        }

        // Consume as many as asked or available
        let write_len = core::cmp::min(len, buffer.len());

        // Render immediately (simple byte writer handles \n/\r/tab inside Vga)
        for &b in &buffer[..write_len] {
            self.vga.write_byte(b);
        }

        // Schedule completion split-phase
        self.tx_len.set(write_len);
        self.tx_buf.replace(buffer);
        self.busy.set(true);
        self.dcall.set();
        Ok(())
    }

    fn set_cursor(&self, _x: usize, _y: usize) -> Result<(), ErrorCode> {
        // Minimal first pass: acknowledge immediately.
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }

    fn hide_cursor(&self) -> Result<(), ErrorCode> {
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }

    fn show_cursor(&self) -> Result<(), ErrorCode> {
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }

    fn blink_cursor_on(&self) -> Result<(), ErrorCode> {
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }

    fn blink_cursor_off(&self) -> Result<(), ErrorCode> {
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }

    fn display_on(&self) -> Result<(), ErrorCode> {
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }

    fn display_off(&self) -> Result<(), ErrorCode> {
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }

    fn clear(&self) -> Result<(), ErrorCode> {
        self.vga.clear();
        self.client.map(|c| c.command_complete(Ok(())));
        Ok(())
    }
}

impl DeferredCallClient for VgaTextScreen<'_> {
    fn handle_deferred_call(&self) {
        if let Some(buf) = self.tx_buf.take() {
            let len = self.tx_len.get();
            self.busy.set(false);
            self.client.map(|c| c.write_complete(buf, len, Ok(())));
        }
    }

    fn register(&'static self) {
        self.dcall.register(self);
    }
}
