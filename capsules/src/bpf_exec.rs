//! Tock syscall driver capsule for running bpf code in the kernel space
use core::mem;
use kernel::debug;
use kernel::{AppId, Callback, CommandResult, Driver, ErrorCode, Grant, ReadWriteAppSlice};

/// Syscall driver number.
use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::Bpf as usize;

pub struct BpfData {
    callback: Callback,
    buffer: ReadWriteAppSlice,
}

impl Default for BpfData {
    fn default() -> BpfData {
        BpfData {
            callback: Callback::default(),
            buffer: ReadWriteAppSlice::default()
        }
    }
}

pub struct BpfDriver {
    app: Grant<BpfData>
}

impl BpfDriver {
    pub const fn new(app: Grant<BpfData>) -> BpfDriver {
        BpfDriver {
             app: app
        }
    }
}

impl<'a> Driver for BpfDriver {
    // Allow readwrite in the memory grant 
    fn allow_readwrite(
        &self,
        appid: AppId,
        allow_num: usize,
        mut slice: ReadWriteAppSlice,
    ) -> Result<ReadWriteAppSlice, (ReadWriteAppSlice, ErrorCode)> {
        match allow_num {
            2 => {
                if self.multi_touch.is_some() {
                    let res = self
                        .apps
                        .enter(appid, |app, _| {
                            mem::swap(&mut app.events_buffer, &mut slice);
                        })
                        .map_err(ErrorCode::from);
                    match res {
                        Err(e) => Err((slice, e)),
                        Ok(_) => Ok(slice),
                    }
                } else {
                    Err((slice, ErrorCode::NOSUPPORT))
                }
            }
            _ => Err((slice, ErrorCode::NOSUPPORT)),
        }
    }

    /// Subscribe to timer expiration
    ///
    /// ### `_subscribe_num`
    ///
    /// - `0`: Subscribe to bpf_exec
    fn subscribe(
        &self,
        subscribe_num: usize,
        mut callback: Callback,
        app_id: AppId,
    ) -> Result<Callback, (Callback, ErrorCode)> {
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

    fn command(&self, cmd_num: usize, _arg1: usize, _: usize, _appid: AppId) -> CommandResult {
        match cmd_num {
            0 => CommandResult::success(),
            1 => {
                // Run bpf code
                let res = self
                    .app
                    .enter(_appid, |td, _allocator| {
                        // let last_value: Ticks16 = Ticks16::from(td.last_value.get());
                        debug!("Am primit ");
                        // debug!("Command 2 dupa last_value");
                        
                        td.callback.schedule(0, 0, 0);
                    })
                    .map_err(ErrorCode::from);

                if let Err(e) = res {
                    // debug!("Error: {:?}", e);
                    CommandResult::failure(e)
                } else {
                    // debug!("Succes cica? {:?}", diff_ms);
                    CommandResult::success()
                }
            },
            _ => CommandResult::failure(ErrorCode::NOSUPPORT)
        }
    }
}