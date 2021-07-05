//! Tock syscall driver capsule for running bpf code in the kernel space
use kernel::common::cells::TakeCell;
use kernel::common::cells::OptionalCell;

use core::mem;
use kernel::debug;
use kernel::{Read, ProcessId, Upcall, CommandReturn, Driver, ErrorCode, Grant, ReadOnlyAppSlice};
use kernel::hil::gpio;
use kernel::hil::gpio::{Configure, Input, Output, InterruptWithValue};

use crate::bpf::rbpf;

/// Syscall driver number.
use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::Bpf as usize;

#[derive(Default)]
pub struct BpfData {
    callback: Upcall,
    buffer: ReadOnlyAppSlice
}

pub struct BpfDriver<'a, IP: gpio::InterruptPin<'a>> {
    app: Grant<BpfData>,
    pins: &'a [Option<&'a gpio::InterruptValueWrapper<'a, IP>>],
    buttons: &'a [(
            &'a gpio::InterruptValueWrapper<'a, IP>,
            gpio::ActivationMode,
            gpio::FloatingState,
        )],
    process_id: OptionalCell<ProcessId>,
    packet: TakeCell<'static, [u8]>
}

impl<'a, IP: gpio::InterruptPin<'a>> BpfDriver<'a, IP> {
    pub fn new(
        app: Grant<BpfData>,
        pins: &'a [Option<&'a gpio::InterruptValueWrapper<'a, IP>>],
        buttons: &'a [(
            &'a gpio::InterruptValueWrapper<'a, IP>,
            gpio::ActivationMode,
            gpio::FloatingState,
        )],
        packet: &'static mut [u8],
    ) -> Self {
        for (i, &(pin, _, floating_state)) in buttons.iter().enumerate() {
            pin.make_input();
            pin.set_value(i as u32);
            pin.set_floating_state(floating_state);
        }
        Self {
             app: app,
             pins: pins,
             buttons: buttons,
             process_id: OptionalCell::empty(),
             packet: TakeCell::new(packet)
        }
    }
}

impl<'a, IP: gpio::InterruptPin<'a>> Driver for BpfDriver<'a, IP> {
    fn allow_readonly(
        &self,
        _appid: ProcessId,
        allow_num: usize,
        mut slice: ReadOnlyAppSlice,
    ) -> Result<ReadOnlyAppSlice, (ReadOnlyAppSlice, ErrorCode)> {
        match allow_num {
            1 => {
                    let res = self.app
                                .enter(_appid, |app| {
                                    mem::swap(&mut app.buffer, &mut slice);
                                })
                                .map_err(ErrorCode::from);
                    if let Err(e) = res {
                        Err((slice, e))
                    } else {
                        Ok(slice)
                    }
            },
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
        mut callback: Upcall,
        app_id: ProcessId,
    ) -> Result<Upcall, (Upcall, ErrorCode)> {
        match subscribe_num {
            0 => {
                let res = self
                    .app
                    .enter(app_id, |app| {
                        mem::swap(&mut app.callback, &mut callback);
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

    fn command(&self, cmd_num: usize, arg1: usize, _: usize, appid: ProcessId) -> CommandReturn {
        match cmd_num {
            0 => {
                CommandReturn::success()
            },
            1 => {
                // Run bpf code
                if arg1 < self.buttons.len() {
                    self.app
                        .enter(appid, |_app| {
                            let _ = self.buttons[arg1]
                                .0
                                .enable_interrupts(gpio::InterruptEdge::EitherEdge);

                            self.process_id.replace(appid);
                            CommandReturn::success()
                        })
                        .unwrap_or_else(|err| CommandReturn::failure(err.into()))
                } else {
                    CommandReturn::failure(ErrorCode::INVAL) /* impossible button */
                }
            },
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT)
        }
    }
}

impl<'a, IP: gpio::InterruptPin<'a>> gpio::ClientWithValue for BpfDriver<'a, IP> {
    fn fired(&self, pin_num: u32) {
        let pins = self.pins.as_ref();
        if let Some(pin) = pins[12] {
            pin.make_output();
            pin.clear();

            let (pin, _, _) = self.buttons[pin_num as usize];
            let value = pin.read();
            
            self.process_id.map_or_else(
               || {
                   debug!("Process id not set!");
               },
               |process_id| {
                    self.app
                        .enter(*process_id, |app| {
                            let program_len = app.buffer.len();
                            if program_len != 0 {
                                app.buffer.map_or(0, |buf| {
                                    let vm = rbpf::EbpfVmRaw::new(Some(buf)).unwrap();
                                    self.packet.map(|packet| {
                                        packet[0] = value as u8;
                                        let res = vm.execute_program(packet).unwrap();
                                        if res == 0 {
                                            for i in 1..22 {
                                                if packet[i] != 0xff {
                                                    if let Some(pin) = pins[i] {
                                                        pin.make_output();
                                                        if packet[i] == 0 {
                                                            pin.clear();
                                                        } else {
                                                            pin.set();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    });

                                    1
                                });
                            }
                        }).unwrap_or_else(|_err| {
                            debug!("Failed in app.enter");
                        });
                }
            );
            pin.set();
        }

        // let _interrupt_count = Cell::new(0);
        // It's possible we got an interrupt for a process that has since died
        // (and didn't unregister the interrupt). Lazily disable interrupts for
        // this button if so.
        // if interrupt_count.get() == 0 {
        //     self.buttons[pin_num as usize].0.disable_interrupts();
        // }
    }
}