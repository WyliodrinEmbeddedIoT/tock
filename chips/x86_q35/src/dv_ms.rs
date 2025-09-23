use core::cell::{Cell, RefCell};
use core::marker::PhantomData;
use kernel::errorcode::ErrorCode;

use crate::ps2::{
    read_data, wait_ob_full, write_command, write_data, Ps2Controller, Ps2MouseClient,
}; // change after it

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

    /// Bottom-half: try to decode one packet into a `MouseEvent` (non-blocking).
    pub fn poll(&self) -> Option<MouseEvent> {
        let pkt = self.packet_fifo.borrow_mut().pop()?;
        Some(MouseEvent {
            buttons: pkt[0] & 0x07,
            x_movement: pkt[1] as i8,
            y_movement: -(pkt[2] as i8), // screen coords: +Y down // change after it
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

impl Ps2MouseClient for Mouse<'_> {
    fn handle_mouse_packet(&self, pkt: [u8; 3]) {
        self.packet_fifo.borrow_mut().push(pkt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk(b0: u8, b1: u8, b2: u8) -> [u8; 3] {
        [b0, b1, b2]
    }

    #[test]
    fn mouse_client_to_poll_basic() {
        // Create a dummy controller (we won't touch hardware in tests)
        let ctrl = Ps2Controller::new();
        let mouse = Mouse::new(&ctrl);

        // Simulate one well-formed 3-byte packet delivered by the controller BH
        // byte0: bit3 must be set (0x08), buttons low 3 bits; dx, dy are 2’s complement.
        mouse.handle_mouse_packet(mk(0x0B, 0x05, 0xFB)); // buttons=0b011, dx=+5, dy=-5

        // Poll should return one event; Y should be inverted (+5 on screen)
        let ev = mouse.poll().expect("expected one MouseEvent");
        assert_eq!(ev.buttons, 0b011);
        assert_eq!(ev.x_movement, 5);
        assert_eq!(ev.y_movement, 5);

        // And then empty
        assert!(mouse.poll().is_none());
    }

    #[test]
    fn mouse_packet_ordering_multiple() {
        let ctrl = Ps2Controller::new();
        let mouse = Mouse::new(&ctrl);

        // Push three packets in order
        let p1 = mk(0x08, 1, 2);
        let p2 = mk(0x09, 3, 4);
        let p3 = mk(0x0A, 5, 6);

        mouse.handle_mouse_packet(p1);
        mouse.handle_mouse_packet(p2);
        mouse.handle_mouse_packet(p3);

        // Pop them back in FIFO order
        let e1 = mouse.poll().unwrap();
        assert_eq!(e1.buttons, p1[0] & 0x07);
        assert_eq!(e1.x_movement, p1[1] as i8);
        assert_eq!(e1.y_movement, -(p1[2] as i8));

        let e2 = mouse.poll().unwrap();
        assert_eq!(e2.buttons, p2[0] & 0x07);
        assert_eq!(e2.x_movement, p2[1] as i8);
        assert_eq!(e2.y_movement, -(p2[2] as i8));

        let e3 = mouse.poll().unwrap();
        assert_eq!(e3.buttons, p3[0] & 0x07);
        assert_eq!(e3.x_movement, p3[1] as i8);
        assert_eq!(e3.y_movement, -(p3[2] as i8));

        assert!(mouse.poll().is_none());
    }

    #[test]
    fn packet_fifo_overflow_drops_oldest() {
        // Directly exercise PacketFifo overflow behavior
        let mut q = PacketFifo::new();

        // Fill to capacity
        for i in 0..PACKET_BUF_SIZE {
            q.push([0x08, i as u8, i as u8]);
        }
        // Push one more distinct packet; this should drop the oldest
        let extra = [0x08, 0xFE, 0xEF];
        q.push(extra);

        // First popped should be the second inserted element (index 1)
        for i in 1..PACKET_BUF_SIZE {
            let pkt = q.pop().expect("packet expected");
            assert_eq!(pkt, [0x08, i as u8, i as u8]);
        }
        // Then the extra packet
        assert_eq!(q.pop().unwrap(), extra);
        // Now empty
        assert!(q.pop().is_none());
    }

    #[test]
    fn raw_fifo_basic_and_overflow() {
        let mut q = RawFifo::new();

        assert!(q.pop().is_none());

        q.push(0xAA);
        assert_eq!(q.pop(), Some(0xAA));
        assert!(q.pop().is_none());

        for i in 0..RAW_BUF_SIZE {
            q.push(i as u8);
        }
        q.push(0xFF); 

        for i in 1..RAW_BUF_SIZE {
            assert_eq!(q.pop(), Some(i as u8));
        }
        assert_eq!(q.pop(), Some(0xFF));
        assert!(q.pop().is_none());
    }
}
