// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

//! UART over a generic TextScreen (kernel::hil::text_screen).
//!
//! TX: forwards `transmit_buffer()` to `screen.print(..)` and completes via
//! `TextScreenClient::write_complete()`. RX: producers call `inject_byte()`;
//! bytes are buffered and completes on `\n` or when full.

use core::cell::{Cell, RefCell};
use core::cmp;
use kernel::hil::keyboard::KeyboardClient as HilKeyboardClient;

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

    kbd_shift_l: Cell<bool>,
    kbd_shift_r: Cell<bool>,
    kbd_caps: Cell<bool>,

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

impl<'a, S: HilTextScreen<'a>> TextConsoleUart<'a, S> {
    pub fn new(screen: &'a S) -> Self {
        Self {
            screen,
            kbd_shift_l: Cell::new(false),
            kbd_shift_r: Cell::new(false),
            kbd_caps: Cell::new(false),
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

    #[inline(always)]
    fn shift_active(&self) -> bool {
        self.kbd_shift_l.get() || self.kbd_shift_r.get()
    }

    // ADD
    fn map_keycode_to_ascii(&self, code: u16, pressed: bool) -> Option<u8> {
        if !pressed {
            return None;
        }

        // Update and ignore modifier keys here
        match code {
            42 => {
                // LEFTSHIFT
                self.kbd_shift_l.set(true);
                return None;
            }
            54 => {
                // RIGHTSHIFT
                self.kbd_shift_r.set(true);
                return None;
            }
            58 => {
                // CAPSLOCK toggles on press
                self.kbd_caps.set(!self.kbd_caps.get());
                return None;
            }
            _ => {}
        }

        // Basics
        match code {
            28 => return Some(b'\n'), // ENTER
            14 => return Some(0x08),  // BACKSPACE
            15 => return Some(b'\t'), // TAB
            57 => return Some(b' '),  // SPACE
            _ => {}
        }

        // Digits row 1..0
        if (2..=11).contains(&code) {
            let normal = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0'];
            let shifted = [b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')'];
            let idx = (code - 2) as usize;
            return Some(if self.shift_active() {
                shifted[idx]
            } else {
                normal[idx]
            });
        }

        // Letters (Linux keycode set)
        let letter = match code {
            30 => b'a',
            48 => b'b',
            46 => b'c',
            32 => b'd',
            18 => b'e',
            33 => b'f',
            34 => b'g',
            35 => b'h',
            23 => b'i',
            36 => b'j',
            37 => b'k',
            38 => b'l',
            50 => b'm',
            49 => b'n',
            24 => b'o',
            25 => b'p',
            16 => b'q',
            19 => b'r',
            31 => b's',
            20 => b't',
            22 => b'u',
            47 => b'v',
            17 => b'w',
            45 => b'x',
            21 => b'y',
            44 => b'z',
            _ => 0,
        };
        if letter != 0 {
            let upper = self.shift_active() ^ self.kbd_caps.get();
            return Some(if upper {
                letter.to_ascii_uppercase()
            } else {
                letter
            });
        }

        // (Optional) extend with punctuation if needed later.

        None
    }
    pub fn inject_key_event(&self, code: u16, pressed: bool) {
        // Track release of shift
        if !pressed {
            match code {
                42 => {
                    self.kbd_shift_l.set(false);
                    return;
                } // LShift up
                54 => {
                    self.kbd_shift_r.set(false);
                    return;
                } // RShift up
                _ => {}
            }
        }

        if let Some(b) = self.map_keycode_to_ascii(code, pressed) {
            self.inject_byte(b);
        }
    }

    /// Must be called by the board after construction to receive screen callbacks
    /// (Call: screen.set_client(Some(capsule)) )
    pub fn set_as_screen_client(&'static self) {
        self.screen.set_client(Some(self));
    }

    /// Feed one input byte. Safe from deferred context
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

impl<'a, S: HilTextScreen<'a>> HilKeyboardClient for TextConsoleUart<'a, S> {
    fn keys_pressed(&self, events: &[(u16, bool)], _r: Result<(), ErrorCode>) {
        for &(code, pressed) in events {
            self.inject_key_event(code, pressed);
        }
    }
}
