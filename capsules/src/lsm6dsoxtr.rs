#![allow(non_camel_case_types)]
use core::cell::Cell;

use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::hil::i2c;
use kernel::procs::State;
//use kernel::hil::sensors;
use kernel::{CommandReturn, Driver, ErrorCode, Grant, ProcessId};
use crate::lsm303agr::Lsm303agrI2C;
use crate::lsm6ds::{
    //LSM6DSOXTRAccelRegisters, LSM6DSOXAccelDataRate, LSM6DSOXGyroDataRate, LSM6DSOXAccelRange, LSM6DSOXTRGyroRange,
};
use crate::driver;


use crate::lsm6ds;
enum State {
    Idle,
    IsPresent,
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
    pub fn is_present(&self) {
        self.buffer.take().map(|buf| {
            buf[0] = 0x0F;
            self.i2c_accelerometer.enable();
            let id = self.i2c_accelerometer.write_read(buf, 1, 1);
        });
    }
}

impl i2c::I2CClient for Lsm6dsoxtrI2C<'_> {
    fn command_complete(&self, buffer: &'static mut [u8], status: Result<(), i2c::Error>) {
        match self.state.get() {
            State::IsPresent => {
                let present = if status == Ok(()) && buffer[0] == 60 {
                    true
                } else {
                    false
                };
            }

            _ => {
                self.i2c_gyroscope.disable();
                self.i2c_accelerometer.disable();
                self.buffer.replace(buffer);
            }
            State::Idle => todo!(),
        }
    }
}