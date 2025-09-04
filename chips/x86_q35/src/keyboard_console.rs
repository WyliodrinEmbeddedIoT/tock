// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

//! Keyboard/Console glue capsule
//!
//! Bridges ASCII keystrokes from the PS/2 keyboard device
//! into the VGA-backed
//! UART-like driver (VGAText) so ProcessConsole can receive input

use crate::keyboard::AsciiClient;
use crate::vga_uart_driver::VgaText;

// Forward bytes into VGAText's RX path
pub struct KeyboardConsole<'a> {
    vga: &'a VgaText<'a>,
}

impl<'a> KeyboardConsole<'a> {
    pub const fn new(vga: &'a VgaText<'a>) -> Self {
        Self { vga }
    }
}

impl AsciiClient for KeyboardConsole<'_> {
    fn put_byte(&self, b: u8) {
        // Newline completion is handled inside VgaText
        self.vga.inject_input_byte(b);
    }
}
