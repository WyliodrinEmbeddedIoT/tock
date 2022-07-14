/// Syscall driver number.

pub const DRIVER_NUM: usize = 0xa0000;

use kernel::grant::{AllowRoCount, AllowRwCount, Grant, UpcallCount};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{ErrorCode, ProcessId};

pub struct Packed {
    apps: Grant<(), UpcallCount<1>, AllowRoCount<10>, AllowRwCount<10>>,
}

impl Packed {
    pub fn new(grant: Grant<(), UpcallCount<1>, AllowRoCount<10>, AllowRwCount<10>>) -> Self {
        Self { apps: grant }
    }
}

impl SyscallDriver for Packed {
    fn command(
        &self,
        command_num: usize,
        data1: usize,
        data2: usize,
        processid: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),

            1 => {
                let _ = self.apps.enter(processid, |_data, kernel| {
                    kernel.schedule_upcall(0, (0, data1, data2))
                });
                CommandReturn::success_u32_u32(data1 as u32, data2 as u32)
            }

            // default
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, processid: ProcessId) -> Result<(), kernel::process::Error> {
        self.apps.enter(processid, |_, _| {})
    }
}
