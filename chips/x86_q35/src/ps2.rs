// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

use core::cell::{Cell, RefCell};
use core::marker::PhantomData;
use kernel::debug;
use kernel::utilities::registers::register_bitfields;
use tock_registers::LocalRegisterCopy;
use x86::registers::io;

/// PS/2 controller ports
const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;

/// Timeout limit for spin loops
const TIMEOUT_LIMIT: usize = 1_000_000;

/// Depth of the scan-code ring buffer
const BUFFER_SIZE: usize = 32;

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
pub(crate) fn wait_input_ready() {
    let mut status = LocalRegisterCopy::<u8, STATUS::Register>::new(0);
    let mut cnt = 0;
    while {
        // reload the latest port value into our local copy
        status.set(unsafe { io::inb(PS2_STATUS_PORT) });
        // loop while input‐full bit is set
        status.is_set(STATUS::INPUT_FULL)
    } {
        cnt += 1;
        if cnt >= TIMEOUT_LIMIT {
            debug!("PS/2 wait_input_ready timed out");
            break;
        }
    }
}
/// Data-ready events trigger IRQ1, handled asynchronously in `handle_interrupt()`.
/// See OSDev documentation:
/// https://wiki.osdev.org/I8042_PS/2_Controller#Status_Register
pub(crate) fn wait_output_ready() {
    let mut status = LocalRegisterCopy::<u8, STATUS::Register>::new(0);
    let mut cnt = 0;
    while {
        status.set(unsafe { io::inb(PS2_STATUS_PORT) });
        // loop while output‐full bit is *not* set
        !status.is_set(STATUS::OUTPUT_FULL)
    } {
        cnt += 1;
        if cnt >= TIMEOUT_LIMIT {
            debug!("PS/2 wait_output_ready timed out");
            break;
        }
    }
}

/// Read one byte from the data port (0x60).
pub fn read_data() -> u8 {
    wait_output_ready();
    unsafe { io::inb(PS2_DATA_PORT) }
}

/// Send a command byte to the controller (port 0x64).
pub fn write_command(cmd: u8) {
    wait_input_ready();
    unsafe { io::outb(PS2_STATUS_PORT, cmd) };
}

/// Write a data byte to the data port (0x60).
pub fn write_data(data: u8) {
    wait_input_ready();
    unsafe { io::outb(PS2_DATA_PORT, data) };
}

/// PS/2 controller driver (the “8042” peripheral)
pub struct Ps2Controller {
    buffer: RefCell<[u8; BUFFER_SIZE]>,
    head: Cell<usize>,
    tail: Cell<usize>,
    _marker: PhantomData<()>,
}

impl Ps2Controller {
    /// Create a new PS/2 controller instance.
    pub fn new() -> Self {
        Ps2Controller {
            buffer: RefCell::new([0; BUFFER_SIZE]),
            head: Cell::new(0),
            tail: Cell::new(0),
            _marker: PhantomData,
        }
    }

    pub fn init(&self) {
        unsafe {
            // 1) disable both channels
            write_command(0xAD); // keyboard
            write_command(0xA7); // auxiliary

            // 2) flush output buffer
            let mut status = LocalRegisterCopy::<u8, STATUS::Register>::new(0);
            while {
                status.set(io::inb(PS2_STATUS_PORT));
                status.is_set(STATUS::OUTPUT_FULL)
            } {
                let _ = read_data();
            }

            // 3) controller self-test
            write_command(0xAA);
            wait_output_ready();
            if read_data() != 0x55 {
                debug!("PS/2 self-test failed");
            }
            // Read-Modify-Write config byte to enable keyboard IRQ1
            write_command(0x20);
            let mut cfg = read_data();
            cfg |= 1 << 0;
            write_command(0x60);
            write_data(cfg);

            //Test the keyboard port (0xAB -> expect 0x00)
            write_command(0xAB);
            wait_output_ready();
            if read_data() != 0x00 {
                debug!("PS/2 keyboard‐port test failed");
            }

            // 6) enable scanning on the keyboard
            write_data(0xF4);
            wait_output_ready();
            if read_data() != 0xFA {
                debug!("PS/2 enable‐scan ACK failed");
            }

            write_command(0xAE); // re-enable keyboard

            // 8) unmask IRQ1 on the master PIC
            const PIC1_DATA: u16 = 0x21;
            let mask = io::inb(PIC1_DATA);
            io::outb(PIC1_DATA, mask & !(1 << 1));
        }
    }

    /// Handle a keyboard interrupt: read a scan-code and buffer it.
    pub fn handle_interrupt(&self) {
        let code = read_data();
        self.push_code(code);
    }

    /// Pop the next scan-code, or None if buffer is empty.
    pub fn pop_scan_code(&self) -> Option<u8> {
        let head = self.head.get();
        let tail = self.tail.get();
        if head == tail {
            None
        } else {
            let byte = self.buffer.borrow()[tail];
            self.tail.set((tail + 1) % BUFFER_SIZE);
            Some(byte)
        }
    }

    /// Internal: push a scan-code into the ring buffer, dropping oldest if full.
    fn push_code(&self, byte: u8) {
        let head = self.head.get();
        let next = (head + 1) % BUFFER_SIZE;
        if next == self.tail.get() {
            // buffer full: drop the oldest
            self.tail.set((self.tail.get() + 1) % BUFFER_SIZE);
        }
        self.buffer.borrow_mut()[head] = byte;
        self.head.set(next);
    }
}
