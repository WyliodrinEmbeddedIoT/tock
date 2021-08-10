#![allow(non_camel_case_types)]
use core::cell::Cell;

use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::hil::i2c;
use kernel::grant::Grant;
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{ErrorCode, ProcessId};
use kernel::hil::sensors::{NineDof, NineDofClient};
use kernel::debug;
//use kernel::hil::sensors;
use crate::lsm6ds;
use crate::driver;

pub const DRIVER_NUM: usize = driver::NUM::Lsm6dsoxtr as usize;


#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Idle,
    IsPresent,
    ReadAccelerationXYZ,
    ReadGyroscopeXYZ,
}
pub struct Lsm6dsoxtrI2C<'a> {
    i2c_accelerometer: &'a dyn i2c::I2CDevice,
    i2c_gyroscope: &'a dyn i2c::I2CDevice,
    state: Cell<State>,
    buffer: TakeCell<'static, [u8]>,
    apps: Grant<App, 1>,
}



#[derive(Default)] 
pub struct App{}

impl<'a> Lsm6dsoxtrI2C<'a> {
    pub fn new(i2c_accelerometer: &'a dyn i2c::I2CDevice,
    i2c_gyroscope: &'a dyn i2c::I2CDevice,
    buffer: &'static mut [u8],
    grant: Grant<App, 1>,) -> Lsm6dsoxtrI2C<'a> {
        Lsm6dsoxtrI2C {
            i2c_accelerometer: i2c_accelerometer,
            i2c_gyroscope: i2c_gyroscope,
            state: Cell::new(State::Idle),
            buffer: TakeCell::new(buffer),
            apps: grant, 
        }
    }
    pub fn is_present(&self) -> Result<(), ErrorCode>  {
        debug! ("{:?}", self.state.get());
        if self.state.get() == State::Idle {
            self.state.set(State::IsPresent);
            self.buffer.take().map_or(Err(ErrorCode::NOMEM), |buf| {
                // turn on i2c to send commands
                buf[0] = 0x0F;
                self.i2c_accelerometer.enable();
                debug! ("{:?}", self.state.get());
                if let Err((error, buf)) = self.i2c_accelerometer.write_read(buf, 1, 1) {
                    self.state.set(State::Idle);
                    self.buffer.replace(buf);
                    self.i2c_accelerometer.disable();
                    Err(error.into())
                } else {
                    debug! ("{:?}", "Ok aici");
                    Ok(())
                }
            })
        } else {
            Err(ErrorCode::BUSY)
        }
    }

    fn read_acceleration_xyz(&self) -> Result<(), ErrorCode> {
        if self.state.get() == State::Idle {
            self.state.set(State::ReadAccelerationXYZ);
           Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }
}

impl i2c::I2CClient for Lsm6dsoxtrI2C<'_> {
    fn command_complete(&self, buffer: &'static mut [u8], status: Result<(), i2c::Error>) {
        debug! ("{:?}", status);
        match self.state.get() {
            State::IsPresent => {
                debug! ("{:?}", buffer);
                let present = if status == Ok(()) && buffer[0] == 15 {
                    true
                } else {
                    false
                };
            }
            State::Idle => todo!(),
            State::ReadAccelerationXYZ => {
                let mut x: usize = 0;
                let mut y: usize = 0;
                let mut z: usize = 0;




            }

            _ => {
                self.i2c_gyroscope.disable();
                self.i2c_accelerometer.disable();
                self.buffer.replace(buffer);
            }
        }
    }
}

impl SyscallDriver for Lsm6dsoxtrI2C<'_> {
    fn command(
        &self,
        command_num: usize,
        data1: usize,
        data2: usize,
        process_id: ProcessId,
    ) -> CommandReturn {
        if command_num == 0 {
            // Handle this first as it should be returned
            // unconditionally
            return CommandReturn::success();
        }

        match command_num {
            // Check is sensor is correctly connected
            1 => {
                if self.state.get() == State::Idle {
                    match self.is_present() {
                        Ok(()) => CommandReturn::success(),
                        Err(error) => CommandReturn::failure(error),
                    }
                } else {
                    CommandReturn::failure(ErrorCode::BUSY)
                }
            }

            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }
        fn allocate_grant(&self, _: ProcessId) -> Result<(), kernel::process::Error> {
            todo!()
        }
    }

impl<'a> NineDof<'a> for Lsm6dsoxtrI2C<'a> {
    fn set_client(&self, nine_dof_client: &'a dyn NineDofClient) {
        // self.nine_dof_client.replace(nine_dof_client);
    }
}