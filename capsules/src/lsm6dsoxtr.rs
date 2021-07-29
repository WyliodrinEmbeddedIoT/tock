#![allow(non_camel_case_types)]

use core::cell::Cell;
use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::hil::i2c;
use kernel::hil::sensors;
use kernel::{CommandReturn, Driver, ErrorCode, Grant, ProcessId};

use crate::kernel;
use crate::driver;
use crate::lsm6dsox;
pub struct Lsm6dsoxtrI2C<'a> {
    i2c_accelerometer: &'a dyn i2c::I2CDevice,
    i2c_gyroscope: &'a dyn i2c::I2CDevice,
    buffer: TakeCell<'static, [u8]>,
    apps: Grant<App, 1>,
}

#[derive(Default)] 
pub struct App{}

impl<'a> Lsm6dsoxtrI2C<'a> {
    fn is_present(&self) {
        self.buffer.take().map(|buf| {
            buf[0] = 0x0F;
            self.i2c_accelerometer.enable();
            let id = self.i2c_accelerometer.write_read(buf, 1, 1);
        });
    }
}