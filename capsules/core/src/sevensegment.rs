use kernel::hil::sevensegment::SevenSegment;

use kernel::ErrorCode;
use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};

use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::SevenSegment as usize;

pub struct SevenSegmentDriver<'a, S: SevenSegment> {
    seven_segment: &'a S,
}

impl<'a, S: SevenSegment> SevenSegmentDriver<'a, S> {
    pub fn new(seven_segment: &'a S) -> Self {
        Self { seven_segment }
    }
}

impl<'a, S: SevenSegment> SyscallDriver for SevenSegmentDriver<'a, S> {
    fn command(
        &self,
        command_number: usize,
        r2: usize,
        r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_number {
            0 => CommandReturn::success(),

            1 => CommandReturn::success_u32(self.seven_segment.num_digits() as u32),

            2 => {
                if r2 > 9999 {
                    CommandReturn::failure(ErrorCode::INVAL)
                } else {
                    let _ = self.seven_segment.write(r2 as u16, r3 as u8);
                    CommandReturn::success()
                }
            }

            3 => {
                let _ = self.seven_segment.clear();
                CommandReturn::success()
            }

            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: ProcessId) -> Result<(), Error> {
        Ok(())
    }
}
