// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

use core::cell::{Cell, RefCell};
use core::marker::PhantomData;
use kernel::debug;
use kernel::utilities::registers::register_bitfields;
use tock_registers::LocalRegisterCopy;
use x86::registers::io;

/// Run `f` with interrupts disabled, then restore the previous IF state.
///
/// `pop_scan_code()` executes in thread context while scan-codes are pushed
/// from IRQ 1.  Masking interrupts around the head/tail/count mutations keeps
/// the ring buffer coherent even if a keystroke arrives in the middle of the
/// read-modify-write sequence.
#[cfg(not(test))]
#[inline(always)]
fn with<R, F: FnOnce() -> R>(f: F) -> R {
    // Read EFLAGS to see whether IF was set.
    let mut flags: usize;
    unsafe {
        core::arch::asm!(
        "pushf",
        "pop {flags}",
        flags = lateout(reg) flags,
        options(nomem, preserves_flags)
        );
    }
    let were_enabled = flags & (1 << 9) != 0; // bit 9 = IF

    // Disable interrupts if they were enabled.
    if were_enabled {
        unsafe {
            core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
        }
    }

    // Critical section
    let ret = f();

    // Restore IF to its previous state.
    if were_enabled {
        unsafe {
            core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
        }
    }
    ret
}

// stub version: compiled when `cargo test` runs on the host
// will be removed once tests are gone before prod.
#[cfg(test)]
#[inline(always)]
fn with<R, F: FnOnce() -> R>(f: F) -> R {
    // Host tests don’t need interrupt masking and can’t execute CLI/STI.
    f()
}

/// PS/2 controller ports
const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PIC1_DATA_PORT: u16 = 0x21;

/// Timeout limit for spin loops
const BUFFER_SIZE: usize = 32;

/// Depth of the scan-code ring buffer
const TIMEOUT_LIMIT: usize = 1_000_000;

// Define the two status‐register bits
register_bitfields![u8,
    pub STATUS [
        OUTPUT_FULL OFFSET(0) NUMBITS(1), // data ready
        INPUT_FULL  OFFSET(1) NUMBITS(1), // input buffer full
    ]
];

/// Note: There is no hardware interrupt when the input buffer empties, so we must poll bit 1.
/// See OSDev documentation:
/// https://wiki.osdev.org/I8042_PS/2_Controller#Status_Register
#[inline(always)]
fn wait_input_ready() {
    let mut s = LocalRegisterCopy::<u8, STATUS::Register>::new(0);
    let mut n = 0;
    while {
        s.set(unsafe { io::inb(PS2_STATUS_PORT) });
        s.is_set(STATUS::INPUT_FULL)
    } {
        if {
            n += 1;
            n
        } >= TIMEOUT_LIMIT
        {
            break;
        }
    }
}

/// Data-ready events trigger IRQ1, handled asynchronously in `handle_interrupt()`.
/// See OSDev documentation:
/// https://wiki.osdev.org/I8042_PS/2_Controller#Status_Register
#[inline(always)]
fn wait_output_ready() {
    let mut s = LocalRegisterCopy::<u8, STATUS::Register>::new(0);
    let mut n = 0;
    while {
        s.set(unsafe { io::inb(PS2_STATUS_PORT) });
        !s.is_set(STATUS::OUTPUT_FULL)
    } {
        if {
            n += 1;
            n
        } >= TIMEOUT_LIMIT
        {
            break;
        }
    }
}

/// Read one byte from the data port (0x60).
#[inline(always)]
fn read_data() -> u8 {
    wait_output_ready();
    unsafe { io::inb(PS2_DATA_PORT) }
}

/// Send a command byte to the controller (port 0x64).
#[inline(always)]
fn write_command(c: u8) {
    wait_input_ready();
    unsafe { io::outb(PS2_STATUS_PORT, c) }
}
/// Write a data byte to the data port (0x60).
#[inline(always)]
fn write_data(d: u8) {
    wait_input_ready();
    unsafe { io::outb(PS2_DATA_PORT, d) }
}

/// Send a byte to the keyboard and wait for ACK (`0xFA`).
/// If the device replies RESEND (`0xFE`) we retry **three times**.
const MAX_RETIRES: usize = 3;
fn send_with_ack(byte: u8) -> bool {
    for _ in 0..=MAX_RETIRES {
        write_data(byte);
        let resp = read_data();
        match resp {
            0xFA => return true, // ACK
            0xFE => continue,    // RESEND -> try again
            _ => return false,   // error
        }
    }
    false
}

/// PS/2 controller driver (the “8042” peripheral)
pub struct Ps2Controller {
    buffer: RefCell<[u8; BUFFER_SIZE]>,
    head: Cell<usize>,
    tail: Cell<usize>,
    count: Cell<usize>, // new field to track number of valid entries
    _p: PhantomData<()>,
}

impl Ps2Controller {
    pub const fn new() -> Self {
        Self {
            buffer: RefCell::new([0; BUFFER_SIZE]),
            head: Cell::new(0),
            tail: Cell::new(0),
            count: Cell::new(0), // ← initialize count
            _p: PhantomData,
        }
    }
    pub fn init(&self) {
        unsafe {
            /* disable both ports */
            write_command(0xAD);
            write_command(0xA7);

            /* flush any stale byte */
            while io::inb(PS2_STATUS_PORT) & 1 != 0 {
                let _ = io::inb(PS2_DATA_PORT);
            }

            /* controller self-test */
            write_command(0xAA);
            let _ = {
                wait_output_ready();
                read_data()
            } == 0x55;

            // IRQ1 fire test
            write_command(0x20); // request config byte
            wait_output_ready();
            let mut cfg = read_data();
            cfg |= 1 << 0; // set bit0 = IRQ1 enable
            cfg &= !(1 << 6); // set bit 6 = translation OFF... for now
            write_command(0x60); // tell controller we’ll write it
            write_data(cfg);

            /* enable keyboard clock */
            write_command(0xAE);

            if !send_with_ack(0xF4) {
                debug!("ps2: enable-scan failed");
            }

            /* unmask IRQ 1 */
            let mask = io::inb(PIC1_DATA_PORT);
            io::outb(PIC1_DATA_PORT, mask & !(1 << 1));
            debug!("ps2: clock on, IRQ1 unmasked");
        }
    }

    /// Handle a keyboard interrupt: read a scan-code and buffer it.
    pub fn handle_interrupt(&self) {
        loop {
            if unsafe { io::inb(PS2_STATUS_PORT) } & 0x01 == 0 {
                break;
            }
            self.push_code(unsafe { io::inb(PS2_DATA_PORT) });
        }
    }

    /// Pop the next scan-code, or None if buffer is empty.
    pub fn pop_scan_code(&self) -> Option<u8> {
        // Disable interrupts for the whole read-modify-write window so an IRQ
        // can’t change `count`, `tail`, or the buffer between our reads.
        with(|| {
            if self.count.get() == 0 {
                return None;
            }

            let t = self.tail.get();
            let byte = self.buffer.borrow()[t];

            self.tail.set((t + 1) % BUFFER_SIZE);
            self.count.set(self.count.get() - 1);

            Some(byte)
        })
    }

    /// Internal: push a scan-code into the ring buffer, dropping oldest if full.
    fn push_code(&self, byte: u8) {
        // Runs in IRQ context, but `with()` is re-entrant-safe: if `IF` is already
        // clear it leaves it that way.
        with(|| {
            let head = self.head.get();
            self.buffer.borrow_mut()[head] = byte;

            let next_head = (head + 1) % BUFFER_SIZE;
            self.head.set(next_head);

            if self.count.get() < BUFFER_SIZE {
                // Still space – just bump the valid-byte count.
                self.count.set(self.count.get() + 1);
            } else {
                // Buffer was full – advance tail to drop the oldest byte.
                self.tail.set((self.tail.get() + 1) % BUFFER_SIZE);
            }
        });
    }
}

// ---------- Unit tests for ring buffer only ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_basic() {
        let dev = Ps2Controller::new();
        // buffer empty
        assert_eq!(dev.pop_scan_code(), None);
        // simulate push via the internal helper
        dev.push_code(0x10);
        assert_eq!(dev.pop_scan_code(), Some(0x10));
        assert_eq!(dev.pop_scan_code(), None);
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let dev = Ps2Controller::new();
        // fill buffer
        for i in 0..BUFFER_SIZE {
            dev.push_code(i as u8);
        }
        // overflow one slot
        dev.push_code(0xFF);
        // should have dropped the first (0), yielding 1..BUFFER_SIZE-1 then 0xFF
        for expected in 1..BUFFER_SIZE {
            assert_eq!(dev.pop_scan_code(), Some(expected as u8));
        }
        assert_eq!(dev.pop_scan_code(), Some(0xFF));
        assert_eq!(dev.pop_scan_code(), None);
    }
}
