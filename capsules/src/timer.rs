//! Tock syscall driver capsule for Latency timer
//! 

use core::cell::Cell;
use core::mem;
// use kernel::debug;
use kernel::hil::time::{Counter, Ticks, Ticks16, Freq16KHz, Frequency};
use kernel::{AppId, Upcall, CommandReturn, Driver, ErrorCode, Grant};

/// Syscall driver number.
use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::Timer as usize;

#[derive(Clone)]
pub struct TimerData {
    callback: Upcall,
    last_value: Cell<u16>,
    command_to_subscribe: Cell<usize>
}

impl Default for TimerData {
    fn default() -> TimerData {
        TimerData {
            callback: Upcall::default(),
            last_value: Cell::new(0),
            command_to_subscribe: Cell::new(0)
        }
    }
}

pub struct TimerDriver<'a, C: Counter<'a>> {
    counter: &'a C,
    app: Grant<TimerData>
}

impl<'a, C: Counter<'a>> TimerDriver<'a, C> {
    pub const fn new(counter: &'a C, app: Grant<TimerData>) -> TimerDriver<'a, C> {
        TimerDriver {
             counter: counter,
             app: app
        }
    }
}

impl<'a, C: Counter<'a>> Driver for TimerDriver<'a, C> {
    /// Subscribe to latency timer expiration
    ///
    /// ### `_subscribe_num`
    ///
    /// - `0`: Subscribe to timer
    fn subscribe(
        &self,
        subscribe_num: usize,
        mut callback: Upcall,
        app_id: AppId,
    ) -> Result<Upcall, (Upcall, ErrorCode)> {
        match subscribe_num {
            0 => {
                let res = self
                    .app
                    .enter(app_id, |td, _allocator| {
                        mem::swap(&mut td.callback, &mut callback);
                    })
                    .map_err(ErrorCode::from);
                
                if let Err(e) = res {
                    Err((callback, e))
                } else {
                    Ok(callback)
                }
            }
            _ => Err((callback, ErrorCode::NOSUPPORT)),
        }
    }

    fn command(&self, cmd_num: usize, _arg1: usize, _: usize, _appid: AppId) -> CommandReturn{
        match cmd_num {
            // Check if driver exists
            0 => CommandReturn::success(),
            // Used for command-to-subscribe_callback
            // Saves current value of counter as last_value 
            // and schedule callback
            1 => {
                let res = self
                    .app
                    .enter(_appid, |td, _allocator| {
                        let now = self.counter.now();
                        td.last_value.set(now.into_u32() as u16);
                        td.callback.schedule(0, now.into_usize(), 0);
                        // debug!("Command 1, now: {:?}", now.into_usize());
                    })
                    .map_err(ErrorCode::from);

                if let Err(e) = res {
                    CommandReturn::failure(e)
                } else {
                    CommandReturn::success()
                }
            },
            // Used for command-to-command
            // Saves current value of counter as last_value
            2 => {
                let res = self
                    .app
                    .enter(_appid, |td, _allocator| {
                        let now = self.counter.now();
                        td.last_value.set(now.into_u32() as u16);
                        // debug!("Command 2, now: {:?}", now.into_usize());
                    })
                    .map_err(ErrorCode::from);

                if let Err(e) = res {
                    CommandReturn::failure(e)
                } else {
                    CommandReturn::success()
                }
            },
            // Used for both command-to-command and command-to-subscribe_callback
            // Computes difference between current now and last_value
            // Returns microseconds as success_u32
            3 => {
                let mut diff_us: u32 = 0;
                let res = self
                    .app
                    .enter(_appid, |td, _allocator| {
                        let last_value: Ticks16 = Ticks16::from(td.last_value.get());


                        let new: Ticks16 = Ticks16::from(self.counter.now().into_u32());
                        let mut diff: Ticks16;

                        if new.into_u32() < last_value.into_u32() {
                            diff = Ticks16::max_value().wrapping_sub(last_value);
                            diff = diff.wrapping_add(new);
                        }
                        else {
                            diff = new.wrapping_sub(last_value);
                        }

                        // microseconds
                        diff_us = ((diff.into_u32() * 1000000)/ Freq16KHz::frequency()) as u32;
                    })
                    .map_err(ErrorCode::from);

                if let Err(e) = res {
                    CommandReturn::failure(e)
                } else {
                    CommandReturn::success_u32(diff_us)
                }
            },
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT)
        }
    }
}
