//! Tock syscall driver capsule for running bpf code in the kernel space
use kernel::utilities::cells::TakeCell;
use kernel::utilities::cells::OptionalCell;

use core::mem;
use kernel::debug;
use kernel::grant::Grant;
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::processbuffer::{ReadOnlyProcessBuffer, ReadableProcessBuffer};
use kernel::{ProcessId, ErrorCode};
use kernel::hil::gpio;
use kernel::hil::gpio::{Configure, Input, Output, InterruptWithValue};
// use kernel::hil::gpio_bank::GpioBank;

use crate::bpf::rbpf;

/// Syscall driver number.
use crate::driver;
pub const DRIVER_NUM: usize = driver::NUM::Bpf as usize;
pub const MAX_BPF_PROG_SIZE: usize = 1024;

#[derive(Default)]
pub struct BpfData {
    // callback: Upcall,
    program_buffer: ReadOnlyProcessBuffer,
    program_len: usize
}

pub struct BpfDriver<'a, IP: gpio::InterruptPin<'a>> {
    app: Grant<BpfData, 0>,
    pins: &'a [Option<&'a gpio::InterruptValueWrapper<'a, IP>>],
    // buttons: &'a [(
    //         &'a gpio::InterruptValueWrapper<'a, IP>,
    //         gpio::ActivationMode,
    //         gpio::FloatingState,
    //     )],
    process_id: OptionalCell<ProcessId>,
    packet: TakeCell<'static, [u8]>,
    program: TakeCell<'static, [u8]>
}

impl<'a, IP: gpio::InterruptPin<'a>> BpfDriver<'a, IP> {
    pub fn new(
        app: Grant<BpfData, 0>,
        pins: &'a [Option<&'a gpio::InterruptValueWrapper<'a, IP>>],
        // buttons: &'a [(
        //     &'a gpio::InterruptValueWrapper<'a, IP>,
        //     gpio::ActivationMode,
        //     gpio::FloatingState,
        // )],
        packet: &'static mut [u8],
        program: &'static mut [u8],
    ) -> Self {
        // for (i, &(pin, _, floating_state)) in buttons.iter().enumerate() {
        //     pin.make_input();
        //     pin.set_value(i as u32);
        //     pin.set_floating_state(floating_state);
        // }
        Self {
             app: app,
             pins: pins,
             // buttons: buttons,
             process_id: OptionalCell::empty(),
             packet: TakeCell::new(packet),
             program: TakeCell::new(program)
        }
    }
}

impl<'a, IP: gpio::InterruptPin<'a>> SyscallDriver for BpfDriver<'a, IP> {
    fn allow_readonly(
        &self,
        _appid: ProcessId,
        allow_num: usize,
        mut slice: ReadOnlyProcessBuffer,
    ) -> Result<ReadOnlyProcessBuffer, (ReadOnlyProcessBuffer, ErrorCode)> {
        match allow_num {
            1 => {
                    let res = self.app
                            .enter(_appid, |app, _| {
                                mem::swap(&mut app.program_buffer, &mut slice);

                                let buffer_len = app.program_buffer.len();
                                let res = app.program_buffer.enter(|source_buffer| {
                                    self.program.take().map_or(Err(ErrorCode::BUSY), |buffer| {
                                        if buffer_len > MAX_BPF_PROG_SIZE {
                                            return Err(ErrorCode::SIZE);
                                        }
                                        for n in 0..buffer_len {
                                            buffer[n] = source_buffer[n].get();
                                        }

                                        self.program.replace(buffer);
                                        Ok(())
                                    })
                                });
                                app.program_len = buffer_len;

                                match res {
                                    Ok(Ok(())) => Ok(()),
                                    Ok(Err(err)) => Err(err),
                                    Err(err) => err.into(),
                                }
                            })
                            .map_err(ErrorCode::from);

                    // match res {
                    //     Ok(Ok(())) => return CommandReturn::success(),
                    //     Ok(Err(err)) => return CommandReturn::failure(err.into()),
                    //     Err(err) => return CommandReturn::failure(err.into()),
                    // };
                    if let Err(e) = res {
                        Err((slice, e))
                    } else {
                        Ok(slice)
                    }
            },
            _ => Err((slice, ErrorCode::NOSUPPORT)),
        }
    }

    fn command(&self, cmd_num: usize, arg1: usize, arg2: usize, appid: ProcessId) -> CommandReturn {
        let pins = self.pins.as_ref();
        match cmd_num {
            0 => { 
                // Return maximum bpf program size
                CommandReturn::success_u32(MAX_BPF_PROG_SIZE as u32)
            },
            1 => {
                // Configure pin as input
                let pin_index = arg1;
                if pin_index >= pins.len() {
                    /* impossible pin */
                    CommandReturn::failure(ErrorCode::INVAL)
                } else {
                    // debug!("Set input for {:?}", pin_index);
                    let maybe_pin = self.pins[pin_index as usize];
                    if let Some(pin) = maybe_pin {
                        pin.make_input();
                        match arg2 {
                            0 => {
                                pin.set_floating_state(gpio::FloatingState::PullNone);
                                CommandReturn::success()
                            }
                            1 => {
                                pin.set_floating_state(gpio::FloatingState::PullUp);
                                CommandReturn::success()
                            }
                            2 => {
                                pin.set_floating_state(gpio::FloatingState::PullDown);
                                CommandReturn::success()
                            }
                            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
                        }
                    } else {
                        CommandReturn::failure(ErrorCode::NODEVICE)
                    }
                }
            },
            2 => {
                // Enable interrupt and copy bpf buffer
                // if arg1 < self.buttons.len() {
                let pin_index = arg1;
                self.app
                    .enter(appid, |_app, _| {
                        let maybe_pin = self.pins[pin_index as usize];
                        // debug!("Set interrupt for {:?}", pin_index);
                        if let Some(pin) = maybe_pin {
                            let _ = pin.enable_interrupts(gpio::InterruptEdge::EitherEdge);
                        // let _ = self.buttons[arg1]
                        //     .0
                        //     .enable_interrupts(gpio::InterruptEdge::EitherEdge);

                            self.process_id.replace(appid);
                            CommandReturn::success()
                        } else {
                            CommandReturn::failure(ErrorCode::NODEVICE)
                        }

                    })
                    .unwrap_or_else(|err| CommandReturn::failure(err.into()))

                // } else {
                //     CommandReturn::failure(ErrorCode::INVAL) /* impossible button */
                // }
            },
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT)
        }
    }

    fn allocate_grant(&self, processid: ProcessId) -> Result<(), kernel::process::Error> {
        self.app.enter(processid, |_, _| {})
    }
}

impl<'a, IP: gpio::InterruptPin<'a>> gpio::ClientWithValue for BpfDriver<'a, IP> {
    fn fired(&self, _pin_num: u32) {
        // let pins = self.pins.as_ref();
        // let (pin, _, _) = self.buttons[pin_num as usize];
        // let value = pin.read();
        // debug!("Pin fired: {:?} ", pin_num);
        self.process_id.map_or_else(
           || {
               debug!("Process id not set!");
           },
           |process_id| {
                self.app
                    .enter(*process_id, |app, _| {
                        self.program.map(|program| {
                            let program_len = app.program_len;
                            if program_len != 0 {
                                let vm = rbpf::EbpfVmRaw::new(Some(&program[0..program_len])).unwrap();
                                self.packet.map(|packet| {
                                    // let mut state: [u8; 15] = [0; 15];
                                    // if let Some(pin_9) = self.pins[9] {
                                    //     pin_9.make_output();
                                    //     pin_9.set();
                                        // debug!("Sunt aici");
                                        for (i, maybe_pin) in self.pins.iter().enumerate() {
                                            if let Some(pin) = maybe_pin {
                                                if pin.is_input() {
                                                    // debug!("{:?} e input", i);
                                                    packet[i/4] &= !(1 << (2 * (3 - (i % 4)) + 1));
                                                    let val = pin.read();
                                                    // if i == 34 {
                                                    //     debug!("i=34, value {:?} ", val);
                                                    // }
                                                    if val {
                                                        packet[i/4] |= 1 << (2 * (3 - (i % 4)));
                                                    } else {
                                                        packet[i/4] &= !(1 << (2 * (3 - (i % 4))));
                                                    }
                                                } else {
                                                    // if i == 12 {
                                                    //     debug!("{:?} e output", i);
                                                    // }
                                                    packet[i/4] |= 1 << (2 * (3 - (i % 4)) + 1);
                                                }
                                                // state[i/4] = packet[i/4];
                                            }
                                            // debug!("i: {:?}", i);
                                        }
                                        // pin_9.source.read_pins(packet);
                                        // pin_9.clear();
                                        // debug!("inainte");
                                        // for i in 0..15 {
                                        //     debug!("{:?} ", packet[i]);
                                        // }
                                        let res = vm.execute_program(packet).unwrap();
                                        // debug!("dupa");
                                        // for i in 0..15 {
                                        //     debug!("{:?} {:?}", state[i], packet[i]);
                                        // }

                                        // debug!("\n\n\n\n");

                                        if res == 0 {
                                            for i in 0..self.pins.len() {
                                                let value = (packet[i/4] & ( 1 << (2 * (3 - (i % 4))) )) >> (2 * (3 - (i % 4)));
                                                let mode = (packet[i/4] & ( 1 << (2 * (3 - (i % 4)) + 1) )) >> (2 * (3 - (i % 4)) + 1);

                                                // let value_i = (state[i/4] & ( 1 << (2 * (3 - (i % 4))) )) >> (2 * (3 - (i % 4)));
                                                // let mode_i = (state[i/4] & ( 1 << (2 * (3 - (i % 4)) + 1) )) >> (2 * (3 - (i % 4)) + 1);
                                                // if i == 9 {
                                                //     debug!("Packet: {:?} si state: {:?}", packet[i/4], state[i/4]);
                                                //     debug!("i=9, value {:?} mode {:?}", value, mode);
                                                //     debug!("i=9, value_i {:?} mode_i {:?}", value_i, mode_i);
                                                // }
                                                if let Some(pin) = self.pins[i] {
                                                    // if mode != mode_i {
                                                    if mode == 1 {
                                                        pin.make_output();
                                                        // if i == 9 {
                                                        //     debug!("i=9, value {:?} mode {:?}", value, mode);
                                                        // }
                                                        // if value != value_i {
                                                        // if i != 9 {
                                                            if value == 0 {
                                                                // debug!("{:?} Clear!", i);
                                                                pin.clear();
                                                            } else {
                                                                // debug!("{:?} Set!", i);
                                                                pin.set();
                                                            }
                                                        // }
                                                    } else {
                                                        pin.make_input();
                                                    }
                                                    // }
                                                }
                                            }
                                        }
                                        // pin.clear();
                                    // }
                                });
                            }
                        });
                    }).unwrap_or_else(|_err| {
                        debug!("Failed in app.enter");
                    });
            }
        );

        // let _interrupt_count = Cell::new(0);
        // It's possible we got an interrupt for a process that has since died
        // (and didn't unregister the interrupt). Lazily disable interrupts for
        // this button if so.
        // if interrupt_count.get() == 0 {
        //     self.buttons[pin_num as usize].0.disable_interrupts();
        // }
    }
}