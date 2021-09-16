#![allow(non_camel_case_types)]
use core::cell::Cell;
use core::convert::TryInto;

use enum_primitive::cast::FromPrimitive;
use kernel::debug;
use kernel::grant::Grant;
use kernel::hil::i2c;
use kernel::hil::sensors::{NineDof, NineDofClient};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::utilities::registers::LocalRegisterCopy;
use kernel::{ErrorCode, ProcessId};
use kernel::hil::sensors;
use crate::{driver, ninedof, temperature};
use crate::lsm6ds::{CTRL1_XL, CTRL2_G, LSM6DSOXAccelDataRate, LSM6DSOXAccelRange, LSM6DSOXGyroDataRate, LSM6DSOXTRAccelRegisters, LSM6DSOXTRGyroRange, LSM6DSOXTRGyroRegisters, SCALE_FACTOR_ACCEL, SCALE_FACTOR_GYRO, LSM6DSOXTRTempRegisters, TEMP_SENSITIVITY_FACTOR};

pub const DRIVER_NUM: usize = driver::NUM::Lsm6dsoxtr as usize;


#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Idle,
    IsPresent,
    ReadAccelerationXYZ,
    ReadGyroscopeXYZ,
    ReadTemperature,
    SetPowerModeAccel,
    SetPowerModeGyro
    
}
pub struct Lsm6dsoxtrI2C<'a> {
    i2c: &'a dyn i2c::I2CDevice,
    state: Cell<State>,
    config_in_progress: Cell<bool>,
    gyro_data_rate: Cell<LSM6DSOXGyroDataRate>,
    accel_data_rate: Cell<LSM6DSOXAccelDataRate>,
    accel_scale: Cell<LSM6DSOXAccelRange>,
    gyro_range: Cell<LSM6DSOXTRGyroRange>,
    low_power: bool,
    temperature: Cell<bool>,
    nine_dof_client: OptionalCell<&'a dyn sensors::NineDofClient>,
    temperature_client: OptionalCell<&'a dyn sensors::TemperatureClient>,
    buffer: TakeCell<'static, [u8]>,
    apps: Grant<App, 1>,
}

#[derive(Default)]
pub struct App {}

impl<'a> Lsm6dsoxtrI2C<'a> {
    pub fn new(
        i2c: &'a dyn i2c::I2CDevice,
        buffer: &'static mut [u8],
        grant: Grant<App, 1>,
    ) -> Lsm6dsoxtrI2C<'a> {
        Lsm6dsoxtrI2C {
            i2c: i2c,
            state: Cell::new(State::Idle),
            config_in_progress: Cell::new(false),
            gyro_data_rate: Cell::new(LSM6DSOXGyroDataRate::LSM6DSOX_GYRO_RATE_12_5_HZ),
            accel_data_rate: Cell::new(LSM6DSOXAccelDataRate::LSM6DSOX_ACCEL_RATE_12_5_HZ),
            accel_scale: Cell::new(LSM6DSOXAccelRange::LSM6DSOX_ACCEL_RANGE_2_G),
            gyro_range: Cell::new(LSM6DSOXTRGyroRange::LSM6DSOX_GYRO_RANGE_250_DPS),
            low_power:false,
            temperature:Cell::new(false),
            nine_dof_client: OptionalCell::empty(),
            temperature_client: OptionalCell::empty(),
            buffer: TakeCell::new(buffer),
            apps: grant,
        }
    }

    pub fn configure (&self,  gyro_data_rate:LSM6DSOXGyroDataRate, accel_data_rate: LSM6DSOXAccelDataRate, accel_scale: LSM6DSOXAccelRange, gyro_range:LSM6DSOXTRGyroRange, low_power: bool) -> Result<(), ErrorCode>{
        if self.state.get() == State::Idle {
           
        self.config_in_progress.set(true);
        self.gyro_data_rate.set(gyro_data_rate);
        self.accel_data_rate.set(accel_data_rate);
        self.accel_scale.set(accel_scale);
        self.gyro_range.set(gyro_range);
        self.temperature.set(true);
        if let Err(error) = self.set_accelerometer_power_mode(accel_data_rate, low_power) {
            self.state.set(State::Idle);self.state.set(State::Idle);
            self.config_in_progress.set(false);
            Err(error)
        } else {
            Ok(())
        }
        } else {
                Err(ErrorCode::BUSY)
            }
        }
    

    pub fn is_present(&self) -> Result<(), ErrorCode> {
        debug!("{:?}", self.state.get());
        if self.state.get() == State::Idle {
            self.state.set(State::IsPresent);
            self.buffer.take().map_or(Err(ErrorCode::NOMEM), |buf| {
                // turn on i2c to send commands
                buf[0] = 0x0F;
                self.i2c.enable();
                debug!("{:?}", self.state.get());
                if let Err((error, buf)) = self.i2c.write_read(buf, 1, 1) {
                    self.state.set(State::Idle);
                    self.buffer.replace(buf);
                    self.i2c.disable();
                    Err(error.into())
                } else {
                    Ok(())
                }
            })
        } else {
            Err(ErrorCode::BUSY)
        }
    }


    pub fn set_accelerometer_power_mode(
        &self,
        data_rate: LSM6DSOXAccelDataRate,
        low_power: bool,
    ) -> Result<(), ErrorCode> {
        if self.state.get() == State::Idle {
            self.buffer.take().map_or(Err(ErrorCode::NOMEM), |buf| {
                self.state.set(State::SetPowerModeAccel);
                buf[0] = LSM6DSOXTRAccelRegisters::CTRL1_XL as u8;
                let mut reg: LocalRegisterCopy<u8, CTRL1_XL::Register> = LocalRegisterCopy::new(0);
                reg.modify(CTRL1_XL::ODR.val(data_rate as u8));
                reg.modify(CTRL1_XL::LPF.val(low_power as u8));
                reg.modify(CTRL1_XL::FS.val(0));

                


                buf[1] = reg.get();
                self.i2c.enable();
                if let Err((error, buf)) = self.i2c.write(buf, 2) {
                    self.state.set(State::Idle);
                    self.i2c.disable();
                    self.buffer.replace(buf);
                    Err(error.into())
                } else {
                    Ok(())
                }
            })
        } else {
            Err(ErrorCode::BUSY)
        }
    }

    pub fn set_gyroscope_power_mode(
        &self,
        data_rate: LSM6DSOXGyroDataRate,
        low_power: bool,
    ) -> Result<(), ErrorCode> {
        if self.state.get() == State::Idle {
            self.buffer.take().map_or(Err(ErrorCode::NOMEM), |buf| {
                self.state.set(State::SetPowerModeGyro);
                buf[0] = LSM6DSOXTRGyroRegisters::CTRL2_G as u8;
                let mut reg: LocalRegisterCopy<u8, CTRL2_G::Register> = LocalRegisterCopy::new(0);
                reg.modify(CTRL2_G::ODR.val(data_rate as u8));
                reg.modify(CTRL2_G::LPF.val(low_power as u8));
                reg.modify(CTRL2_G::FS.val(0));

                

                buf[1] = reg.get();
                self.i2c.enable();
                if let Err((error, buf)) = self.i2c.write(buf, 2) {
                    self.state.set(State::Idle);
                    self.i2c.disable();
                    self.buffer.replace(buf);
                    Err(error.into())
                } else {
                    Ok(())
                }
            })
        } else {
            Err(ErrorCode::BUSY)
        }
    }

    pub fn read_acceleration_xyz(&self) -> Result<(), ErrorCode> {
        if self.state.get() == State::Idle {
            self.state.set(State::ReadAccelerationXYZ);
            self.buffer.take().map_or(Err(ErrorCode::NOMEM), |buf| {
                buf[0] = LSM6DSOXTRAccelRegisters::OUT_X_L_A as u8;
                debug!("{:?}", buf[0]);
                self.i2c.enable();
                if let Err((error, buf)) = self.i2c.write_read(buf, 1, 6) {
                    self.state.set(State::Idle);
                    self.buffer.replace(buf);
                    self.i2c.disable();
                    Err(error.into())
                } else {
                   //debug!("read acceleration reading");
                    Ok(())
                }
            })
        } else {
            //debug!("read acceleration busy");
            Err(ErrorCode::BUSY)
        }
    }

   pub fn read_gyroscope_xyz(&self) -> Result<(), ErrorCode> {
        if self.state.get() == State::Idle {
            self.state.set(State::ReadGyroscopeXYZ);
            self.buffer.take().map_or(Err(ErrorCode::NOMEM), |buf| {
                buf[0] = LSM6DSOXTRGyroRegisters::OUT_X_L_G as u8;
                self.i2c.enable();
                if let Err((error, buf)) = self.i2c.write_read(buf, 1, 6) {
                    self.state.set(State::Idle);
                    self.buffer.replace(buf);
                    self.i2c.disable();
                    Err(error.into())
                } else {
                    Ok(())
                }
            })
        } else {
            Err(ErrorCode::BUSY)
        }
    }


pub fn read_temperature(&self) -> Result<(), ErrorCode> {
    if self.state.get() == State::Idle {
        self.state.set(State::ReadTemperature);
        self.buffer.take().map_or(Err(ErrorCode::NOMEM), |buf| {
            buf[0] = LSM6DSOXTRTempRegisters::OUT_TEMP_L as u8;
            self.i2c.enable();
            if let Err((error, buf)) = self.i2c.write_read(buf, 1, 6) {
                self.state.set(State::Idle);
                self.buffer.replace(buf);
                self.i2c.disable();
                Err(error.into())
            } else {
                Ok(())
            }
        })
    } else {
        Err(ErrorCode::BUSY)
    }
}
}


impl i2c::I2CClient for Lsm6dsoxtrI2C<'_> {
    fn command_complete(&self, buffer: &'static mut [u8], status: Result<(), i2c::Error>) {
        match self.state.get() {
            State::IsPresent => {
                //debug!("{:?}", buffer);
                if status == Ok(()) {
                    
                }else {
                    //Err(status.into());
                    
                };
            }
            State::Idle => {
                
                if status != Ok(()) {
                    status; // Err(status.into)? statusul in sine poate fi de tip i2c::Error
            }
        }           //should never get here
            ,
            State::ReadAccelerationXYZ => {
                let mut x: usize = 0;
                let mut y: usize = 0;
                let mut z: usize = 0;
                debug!("acceleration {:?}", buffer);
                 if status == Ok(()) {
                self.nine_dof_client.map(|nine_dof_client|{
                    debug!("ninedof scaling");
                    let scale_factor = self.accel_scale.get() as usize;
                    x = ((((buffer[0] as u16 + ((buffer[1] as u16) << 8)) as i16) as isize) * (SCALE_FACTOR_ACCEL[scale_factor] as isize) / 1000) as usize;
                    //debug!("{}", x);
                    y = ((((buffer[2] as u16 + ((buffer[3] as u16) << 8)) as i16) as isize) * (SCALE_FACTOR_ACCEL[scale_factor] as isize) / 1000) as usize;
                       
                    z = ((((buffer[4] as u16 + ((buffer[5] as u16) << 8)) as i16) as isize) * (SCALE_FACTOR_ACCEL[scale_factor] as isize) / 1000) as usize;
                    nine_dof_client.callback(x, y, z)
                } );
              
                
            } else {
                self.nine_dof_client.map(|client| {
                    client.callback(0, 0, 0);
                });
                status; // Err(status.into)? statusul in sine poate fi de tip i2c::Error
            };
                self.buffer.replace(buffer);
                self.i2c.disable();
                self.state.set(State::Idle);
               
            }

            State::ReadGyroscopeXYZ => {
                let mut x: usize = 0;
                let mut y: usize = 0;
                let mut z: usize = 0;
                if status == Ok(()) {
                self.nine_dof_client.map(|nine_dof_client|{
                    debug!("ninedof scaling");
                    let scale_factor = self.gyro_range.get() as usize;
                    x = ((((buffer[0] as u16 + ((buffer[1] as u16) << 8)) as i16)) as isize * (SCALE_FACTOR_GYRO[scale_factor] as isize) / 100) as usize;
                    debug!("{}", x);
                    y =((((buffer[2] as u16 + ((buffer[3] as u16) << 8)) as i16))as isize * (SCALE_FACTOR_GYRO[scale_factor] as isize)  / 100) as usize;
                       
                    z =((((buffer[4] as u16 + ((buffer[5] as u16) << 8)) as i16)) as isize * (SCALE_FACTOR_GYRO[scale_factor] as isize)  / 100) as usize;
                    nine_dof_client.callback(x, y, z)
                } );
                
            } else {
                self.nine_dof_client.map(|client| {
                    client.callback(0, 0, 0);
                });
                status;// Err(status.into)? statusul in sine poate fi de tip i2c::Error
            };
                self.buffer.replace(buffer);
                self.i2c.disable();
                self.state.set(State::Idle);
            }

            State::ReadTemperature => {
                let mut temperature: usize = 0;
                
                if status == Ok(()) {
                self.temperature_client.map(|client|{
                    debug!("ninedof scaling");
                   
                     temperature= ((((buffer[0] as u16 + ((buffer[1] as u16) << 8)) as i16)) as isize /(TEMP_SENSITIVITY_FACTOR as isize) + 25) as usize;
                    client.callback(temperature);
                } );
                
            } else {
                self.temperature_client.map(|client| {
                    client.callback(0);
                });
                status;
                
            };
                self.buffer.replace(buffer);
                self.i2c.disable();
                self.state.set(State::Idle);
            }
            

            State::SetPowerModeAccel => {
                //debug!("{:?}", buffer);
                
                self.buffer.replace(buffer);
                self.i2c.disable();
                self.state.set(State::Idle);
                if status == Ok(()) {
                if self.config_in_progress.get() {
                    if let Err(error) = self.set_gyroscope_power_mode(self.gyro_data_rate.get(), self.low_power) {
                        self.config_in_progress.set(false);

                    } 
                }
            } else {
                status;
            }
        
            }

            State::SetPowerModeGyro => {
                self.buffer.replace(buffer);
                self.i2c.disable();
                self.state.set(State::Idle);
                self.config_in_progress.set(false);
            }

            _ => {
                self.i2c.disable();
                self.i2c.disable();
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

            2 => {
                if self.state.get() == State::Idle {
                    if let Some(data_rate) = LSM6DSOXAccelDataRate::from_usize(data1) {
                        match self.set_accelerometer_power_mode(data_rate, if data2 != 0 { true } else { false })
                        {
                            Ok(()) => CommandReturn::success(),
                            Err(error) => CommandReturn::failure(error),
                        }
                    } else {
                        CommandReturn::failure(ErrorCode::INVAL)
                    }
                    
                } else {
                    CommandReturn::failure(ErrorCode::BUSY)
                }
            }

            3 => {
                if self.state.get() == State::Idle {
                    if let Some(data_rate) = LSM6DSOXGyroDataRate::from_usize(data1) {
                        match self.set_gyroscope_power_mode(data_rate, if data2 != 0 { true } else { false })
                        {
                            Ok(()) => CommandReturn::success(),
                            Err(error) => CommandReturn::failure(error),
                        }
                    } else {
                        CommandReturn::failure(ErrorCode::INVAL)
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
        self.nine_dof_client.replace(nine_dof_client);
    }

    fn read_accelerometer(&self) -> Result<(), ErrorCode> {
        self.read_acceleration_xyz()
    }

    fn read_gyroscope(&self) -> Result<(), ErrorCode> {
        // Err(ErrorCode::NOSUPPORT)
        self.read_gyroscope_xyz()
    }
}

impl<'a> sensors::TemperatureDriver<'a> for Lsm6dsoxtrI2C<'a> {
    fn set_client(&self, temperature_client: &'a dyn sensors::TemperatureClient) {
        self.temperature_client.replace(temperature_client);
    }

    fn read_temperature(&self) -> Result<(), ErrorCode> {
        self.read_temperature()
    }
}
