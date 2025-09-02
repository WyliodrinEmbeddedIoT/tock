use core::cell::{Cell, RefCell};
use core::marker::PhantomData;
use kernel::errorcode::ErrorCode;

use crate::ps2::Ps2Controller;
use crate::ps2::{read_data, wait_ob_full, write_command, write_data};

use crate::ps2::{PS2_DATA_PORT, PS2_STATUS_PORT};
use x86::registers::io;

const RAW_BUF_SIZE: usize = 32; //rawbuf size
const PACKET_BUF_SIZE: usize = 16; // packetbuf size
const MAX_CMD: usize = 8; // maxcmd size

/// One mouse movement/button event (decoded from a 3-byte packet)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MouseEvent {
    pub buttons: u8,    // bit0=L, bit1=R, bit2=Mid
    pub x_movement: i8, // X
    pub y_movement: i8, // Y
}

// small FIFOs
struct RawFifo {
    buf: [u8; RAW_BUF_SIZE],
    head: usize,
    tail: usize,
    full: bool,
}
impl RawFifo {
    const fn new() -> Self {
        Self {
            buf: [0; RAW_BUF_SIZE],
            head: 0,
            tail: 0,
            full: false,
        }
    }
    fn push(&mut self, b: u8) {
        self.buf[self.head] = b;
        self.head = (self.head + 1) % RAW_BUF_SIZE;
        if self.full {
            self.tail = (self.tail + 1) % RAW_BUF_SIZE;
        } else if self.head == self.tail {
            self.full = true;
        }
    }
    fn pop(&mut self) -> Option<u8> {
        if !self.full && self.head == self.tail {
            None
        } else {
            let b = self.buf[self.tail];
            self.tail = (self.tail + 1) % RAW_BUF_SIZE;
            self.full = false;
            Some(b)
        }
    }
}

struct PacketFifo {
    buf: [[u8; 3]; PACKET_BUF_SIZE],
    head: usize,
    tail: usize,
    full: bool,
}
impl PacketFifo {
    const fn new() -> Self {
        Self {
            buf: [[0; 3]; PACKET_BUF_SIZE],
            head: 0,
            tail: 0,
            full: false,
        }
    }
    fn push(&mut self, pkt: [u8; 3]) {
        self.buf[self.head] = pkt;
        self.head = (self.head + 1) % PACKET_BUF_SIZE;
        if self.full {
            self.tail = (self.tail + 1) % PACKET_BUF_SIZE;
        } else if self.head == self.tail {
            self.full = true;
        }
    }
    fn pop(&mut self) -> Option<[u8; 3]> {
        if !self.full && self.head == self.tail {
            None
        } else {
            let pkt = self.buf[self.tail];
            self.tail = (self.tail + 1) % PACKET_BUF_SIZE;
            self.full = false;
            Some(pkt)
        }
    }
}

// cmd helper

#[derive(Copy, Clone, Debug)]
struct Resp {
    buf: [u8; MAX_CMD],
    len: usize,
}
impl Resp {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

/// Send bytes to the mouse (each prefixed with 0xD4) and collect `resp_len` reply bytes.
/// Retries the whole sequence up to 3 times on RESEND (0xFE).
fn send_mouse(cmd: &[u8], resp_len: usize) -> Result<Resp, ErrorCode> {
    const MAX_RETRIES: usize = 3;
    debug_assert!(cmd.len() <= MAX_CMD);
    debug_assert!(resp_len <= MAX_CMD);

    let mut retries = 0;

    'retry: loop {
        // host → mouse (ACK after each byte)
        for &b in cmd {
            write_command(0xD4).map_err(|_| ErrorCode::FAIL)?; // “send next byte to mouse”
            write_data(b).map_err(|_| ErrorCode::FAIL)?;

            wait_ob_full().map_err(|_| ErrorCode::FAIL)?;
            match read_data() {
                Ok(0xFA) => {} // ACK
                Ok(0xFE) => {
                    // RESEND → restart whole sequence
                    retries += 1;
                    if retries > MAX_RETRIES {
                        return Err(ErrorCode::FAIL);
                    }
                    continue 'retry;
                }
                _ => return Err(ErrorCode::FAIL),
            }
        }

        // mouse → host (payload)
        let mut r = Resp {
            buf: [0; MAX_CMD],
            len: 0,
        };
        for _ in 0..resp_len {
            wait_ob_full().map_err(|_| ErrorCode::FAIL)?;
            match read_data() {
                Ok(byte) => {
                    r.buf[r.len] = byte;
                    r.len += 1;
                }
                Err(_) => {
                    return Err(ErrorCode::FAIL);
                }
            }
        }
        return Ok(r);
    }
}
//mouse driver
pub struct Mouse<'a> {
    controller: &'a Ps2Controller,
    raw: RefCell<RawFifo>,
    packet_fifo: RefCell<PacketFifo>,
    state: Cell<usize>, // bytes collected so far in current packet (0..=2)
    pkt: Cell<[u8; 3]>, // scratch for assembling a packet
    _marker: PhantomData<&'a ()>,
}

impl<'a> Mouse<'a> {
    pub fn new(controller: &'a Ps2Controller) -> Self {
        Self {
            controller,
            raw: RefCell::new(RawFifo::new()),
            packet_fifo: RefCell::new(PacketFifo::new()),
            state: Cell::new(0),
            pkt: Cell::new([0; 3]),
            _marker: PhantomData,
        }
    }

    /// Top-half: call from the PIC IRQ stub for mouse/PS2.
    /// Drains a byte from the shared controller buffer and assembles 3-byte packets.
    pub fn handle_interrupt(&self) {
        loop {
            let status = unsafe { io::inb(PS2_STATUS_PORT) };
            // no data?
            if (status & 0x01) == 0 {
                break;
            }
            // not from mouse? leave it for the keyboard ISR
            if (status & 0x20) == 0 {
                return; // changed break; to return;
            } // bit5 = AUX

            // this byte is mouse data
            let b = unsafe { io::inb(PS2_DATA_PORT) };

            // assemble 3-byte packets with header resync
            let mut st = self.state.get();
            let mut cur = self.pkt.get();

            if st == 0 {
                // byte 0 must have bit3 set in standard PS/2 packets
                if (b & 0x08) == 0 {
                    // desync, drop until we see a header
                    continue;
                }
            }

            cur[st] = b;
            st += 1;

            if st == 3 {
                self.packet_fifo.borrow_mut().push(cur);
                st = 0;
            }
            self.pkt.set(cur);
            self.state.set(st);
        }
    }

    /// Bottom-half: try to decode one packet into a `MouseEvent` (non-blocking).
    pub fn poll(&self) -> Option<MouseEvent> {
        let pkt = self.packet_fifo.borrow_mut().pop()?;
        Some(MouseEvent {
            buttons: pkt[0] & 0x07,
            x_movement: pkt[1] as i8,
            y_movement: -(pkt[2] as i8), // old: y_movement: pkt[2] as i8
        })
    }
    // dv cmd helpers
    pub fn enable_streaming(&self) -> Result<(), ErrorCode> {
        send_mouse(&[0xF4], 0).map(|_| ())
    }

    pub fn disable_streaming(&self) -> Result<(), ErrorCode> {
        send_mouse(&[0xF5], 0).map(|_| ())
    }

    pub fn set_sample_rate(&self, rate: u8) -> Result<(), ErrorCode> {
        send_mouse(&[0xF3, rate], 0).map(|_| ())
    }

    pub fn set_resolution(&self, res: u8) -> Result<(), ErrorCode> {
        send_mouse(&[0xE8, res], 0).map(|_| ())
    }

    pub fn set_scaling_1_1(&self) -> Result<(), ErrorCode> {
        send_mouse(&[0xE6], 0).map(|_| ())
    }

    pub fn set_scaling_2_1(&self) -> Result<(), ErrorCode> {
        send_mouse(&[0xE7], 0).map(|_| ())
    }

    pub fn status_request(&self) -> Result<[u8; 3], ErrorCode> {
        let resp = send_mouse(&[0xE9], 3)?;
        let mut out = [0u8; 3];
        out[..resp.len()].copy_from_slice(resp.as_slice());
        Ok(out)
    }

    pub fn reset(&self) -> Result<(), ErrorCode> {
        // Some mice send 0xAA,0x00; we accept just the 0xAA
        let resp = send_mouse(&[0xFF], 1)?;
        if resp.as_slice() == [0xAA] {
            Ok(())
        } else {
            Err(ErrorCode::FAIL)
        }
    }
}
//must fix issue where the kernel freezes!
//must review the mouse driver and ensure it does not block or cause deadlocks
