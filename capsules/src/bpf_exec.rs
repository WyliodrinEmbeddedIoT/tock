//! Tock syscall driver capsule for running bpf code in the kernel space
use kernel::common::cells::OptionalCell;
use core::cell::Cell;
use core::mem;
use kernel::debug;
use kernel::{Read, ProcessId, Upcall, CommandReturn, Driver, ErrorCode, Grant, ReadOnlyAppSlice};
use kernel::hil::gpio;
use kernel::hil::gpio::{Configure, Output, InterruptWithValue};

use crate::bpf::rbpf;

/// Syscall driver number.
use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::Bpf as usize;

#[derive(Default)]
pub struct BpfData {
    callback: Upcall,
    buffer: ReadOnlyAppSlice
}

// impl Default for BpfData {
//     fn default() -> BpfData {
//         BpfData {
//             callback: Upcall::default(),
//             buffer: TakeCell::new(buffer),
//         }
//     }
// }

pub struct BpfDriver<'a, IP: gpio::InterruptPin<'a>> {
    app: Grant<BpfData>,
    pins: &'a [Option<&'a gpio::InterruptValueWrapper<'a, IP>>],
    buttons: &'a [(
            &'a gpio::InterruptValueWrapper<'a, IP>,
            gpio::ActivationMode,
            gpio::FloatingState,
        )],
    process_id: OptionalCell<ProcessId>
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
             process_id: OptionalCell::empty()
        }
    }

    // fn get_button_state(&self, pin_num: u32) -> gpio::ActivationState {
    //     let pin = &self.buttons[pin_num as usize];
    //     pin.0.read_activation(kernel::hil::gpio::ActivationMode::ActiveLow)
    // }
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
                // debug!("Command received! {}", arg1);
                if arg1 < self.buttons.len() {
                    self.app
                        .enter(appid, |_app| {
                            // debug!("{:?}", self.buttons);
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
        // Read the value of the pin and get the button state.
        // debug!("Client fired!");
        // let _button_state = self.get_button_state(pin_num);
        let interrupt_count = Cell::new(0);
        let packet1 = &mut [ 
            // start data structure passed to eBPF prpgram
            self.pins.len() as u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            // beginning of stack
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00u8
        ];

        self.process_id.map_or_else(
           || {
               debug!("Process id not set!");
           },
           |process_id| {
                self.app
                    .enter(*process_id, |app| {
                        let program_len = app.buffer.len();
                        // debug!("In self.app");
                        if program_len != 0 {
                            let response = app.buffer.map_or(0, |buf| {
                                let vm = rbpf::EbpfVmRaw::new(Some(buf)).unwrap();
                                let _res = vm.execute_program(packet1).unwrap();

                                1
                            });

                            if response != 0 {
                                let pins = self.pins.as_ref();

                                for i in 1..22 {
                                    if packet1[i] != 0xff {
                                        if let Some(pin) = pins[i] {
                                            pin.make_output();
                                            if packet1[i] == 0 {
                                                pin.clear();
                                            } else {
                                                pin.set();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }).unwrap_or_else(|_err| {
                        debug!("Failed in app.enter");
                    });
            }
        );

        // It's possible we got an interrupt for a process that has since died
        // (and didn't unregister the interrupt). Lazily disable interrupts for
        // this button if so.
        if interrupt_count.get() == 0 {
            self.buttons[pin_num as usize].0.disable_interrupts();
        }
    }
}