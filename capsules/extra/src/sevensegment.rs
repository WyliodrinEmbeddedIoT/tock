use core::cell::Cell;
use kernel::ErrorCode;
use kernel::hil::sevensegment::SevenSegment;
use kernel::hil::shift_register::ShiftRegister;
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};

use crate::seven_segment;

pub struct SevenSegmentShiftRegister<'a, Sr: ShiftRegister, A: Alarm<'a>> {
    shift_register: &'a Sr,
    segment_map_digit: [u8; 10],
    seven_segment: [u8; 4],
    alarm: &'a A,
    number: Cell<usize>,
    position: Cell<usize>,
    timing: u8,
    activ: Cell<bool>,
}

impl<'a, Sr: ShiftRegister, A: Alarm<'a>> SevenSegmentShiftRegister<'a, Sr, A> {
    pub fn new(
        shift_register: &'static Sr,
        segment_map_digit: [u8; 10],
        seven_segment: [u8; 4],
        alarm: &'a A,
        number: Cell<usize>,
        position: Cell<usize>,
        timing: u8,
        refresh_rate: usize,
        activ: Cell<bool>,
    ) -> Self {
        Self {
            shift_register,
            segment_map_digit,
            seven_segment,
            alarm,
            number,
            position,
            timing: (1000 / (refresh_rate * 4)) as u8,
            activ,
        }
    }

    pub fn init(&self) {
        self.next_digit();
    }

    fn next_digit(&self) {
        let number = self.number.get();
        let pos = self.position.get();
        let aux: usize = 10;

        if self.activ.get() {
            let _ = self
                .shift_register
                .write(self.segment_map_digit[(number / aux.pow((3 - pos) as u32)) % 10]);
            let _ = self.shift_register.write(self.seven_segment[pos]);

            self.position.set((pos + 1) % 4);
        } else {
            let _ = self.shift_register.write(0x00);
            let _ = self.shift_register.write(0x00);
        }
        let interval = self.alarm.ticks_from_ms(self.timing as u32);
        self.alarm.set_alarm(self.alarm.now(), interval);
    }
}

impl<'a, Sr: ShiftRegister, A: Alarm<'a>> AlarmClient for SevenSegmentShiftRegister<'a, Sr, A> {
    fn alarm(&self) {
        self.next_digit();
    }
}

impl<'a, Sr: ShiftRegister, A: Alarm<'a>> SevenSegment for SevenSegmentShiftRegister<'a, Sr, A> {
    fn num_digits(&self) -> u8 {
        4
    }

    fn has_point(&self) -> bool {
        false
    }

    fn write(&self, number: u16, _point: u8) -> Result<(), ErrorCode> {
        self.number.set(number as usize);
        self.activ.set(true);
        Ok(())
    }

    fn clear(&self) -> Result<(), ErrorCode> {
        self.activ.set(false);
        Ok(())
    }
}
