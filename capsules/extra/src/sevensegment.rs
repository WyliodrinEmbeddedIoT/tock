use core::cell::Cell;
use kernel::ErrorCode;
use kernel::hil::sevensegment::SevenSegment;
use kernel::hil::shift_register::ShiftRegister;
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};

use crate::seven_segment;

pub struct SevenSegmentShiftRegister<'a, Sr: ShiftRegister, A: Alarm<'a>, const DIGITS: usize> {
    shift_register: &'a Sr,
    segment_map_digit: [u8; 10],
    seven_segment: [u8; DIGITS],
    alarm: &'a A,
    number: Cell<usize>,
    position: Cell<usize>,
    timing: u8,
    activ: Cell<bool>,
}

impl<'a, Sr: ShiftRegister, A: Alarm<'a>, const DIGITS: usize>
    SevenSegmentShiftRegister<'a, Sr, A, DIGITS>
{
    pub fn new(
        shift_register: &'static Sr,
        segment_map_digit: [u8; 10],
        seven_segment: [u8; DIGITS],
        alarm: &'a A,
        number: Cell<usize>,
        position: Cell<usize>,
        timing: usize,
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
        self.next_digit(DIGITS);
    }

    /// Displays one digit at a time
    /// Number of digits must be given as a parameter
    fn next_digit(&self, nr_digits: usize) {
        let number = self.number.get();
        let pos = self.position.get();
        let aux: usize = 10;

        if self.activ.get() {
            let _ = self.shift_register.write(
                self.segment_map_digit[(number / aux.pow((nr_digits - pos - 1) as u32)) % 10]
                    as usize,
            );
            let _ = self.shift_register.write(self.seven_segment[pos] as usize);

            self.position.set((pos + 1) % nr_digits);
        } else {
            let _ = self.shift_register.write(0x00);
            let _ = self.shift_register.write(0x00);
        }
        let interval = self.alarm.ticks_from_ms(self.timing as u32);
        self.alarm.set_alarm(self.alarm.now(), interval);
    }
}

impl<'a, Sr: ShiftRegister, A: Alarm<'a>, const DIGITS: usize> AlarmClient
    for SevenSegmentShiftRegister<'a, Sr, A, DIGITS>
{
    fn alarm(&self) {
        self.next_digit(DIGITS);
    }
}

impl<'a, Sr: ShiftRegister, A: Alarm<'a>, const DIGITS: usize> SevenSegment
    for SevenSegmentShiftRegister<'a, Sr, A, DIGITS>
{
    fn num_digits(&self) -> u8 {
        DIGITS as u8
    }

    fn has_point(&self) -> bool {
        false
    }

    fn write(&self, number: usize, _point: u8) -> Result<(), ErrorCode> {
        self.number.set(number as usize);
        self.activ.set(true);
        Ok(())
    }

    fn clear(&self) -> Result<(), ErrorCode> {
        self.activ.set(false);
        Ok(())
    }
}
