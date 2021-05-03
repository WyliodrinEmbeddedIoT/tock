use kernel::hil::led::Led;
use kernel::{CommandReturn, Driver, ErrorCode, ProcessId};

pub const DRIVER_NUM: usize = 0xa0001;

pub struct LedBar<'a> {
    leds: &'a [&'a dyn Led],
}

impl<'a> LedBar<'a> {
    pub fn new(leds: &'a [&'a dyn Led]) -> Self {
        LedBar { leds }
    }
}

impl<'a> Driver for LedBar<'a> {
    fn command(
        &self,
        command_num: usize,
        num_leds_on: usize,
        _r3: usize,
        _caller_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success_u32(self.leds.len() as u32),
            1 => {
                if num_leds_on <= self.leds.len() {
                    // for led in self.leds {
                    //     led.off();
                    // }
                    for i in 0..self.leds.len() {
                        if i < num_leds_on {
                            self.leds[i].on();
                        } else {
                            self.leds[i].off();
                        }
                    }
                    CommandReturn::success()
                } else {
                    CommandReturn::failure(ErrorCode::INVAL)
                }
            }
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }
}
