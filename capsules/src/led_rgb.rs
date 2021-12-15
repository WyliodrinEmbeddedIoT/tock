use kernel::{debug, ErrorCode, ProcessId};
use kernel::hil::gpio::Pin;
use kernel::hil::led::LedRGB;
use kernel::hil::time::{Ticks, Time};
use kernel::process::Error;
use kernel::syscall::{CommandReturn, SyscallDriver};


use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::LedRGB as usize;


pub struct LedRGBDriver<'a, L: LedRGB > {
    led: &'a L,

}

impl<'a, L: LedRGB> LedRGBDriver<'a, L> {
    pub fn new(
        led: &'a L,
    ) -> Self {
        Self { led, }
    }

    pub fn init(&self){
        self.led.init()
    }
    /*
    pub fn on(&self,r:u8,g:u8,b:u8){
        self.led.on(r,g,b)
    }

     */

}

impl<L: LedRGB> SyscallDriver for LedRGBDriver<'_, L,> {
    fn command(&self, command_num: usize, r2: usize, _r3: usize, _process_id: ProcessId) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),

            // on
            1 => {
                let r = (r2>>16) as u8;
                let g = ((r2>>8) & 0b1111_1111) as u8;
                let b = (r2 & 0b1111_1111) as u8;
                self.led.on(r,g,b);
                CommandReturn::success()
            }

            // off
            2 => {
                self.led.off();
                CommandReturn::success()
            }

            // default
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, process_id: ProcessId) -> Result<(), Error> {
        Ok(())
    }
}











