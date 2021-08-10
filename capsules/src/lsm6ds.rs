//! LSM6DSOXTR sensors
//!

#![allow(non_camel_case_types)]

use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
//use kernel::common::registers::register_bitfields;

pub const CHIP_ID: u8 = 0x6C;
pub const ACCELEROMETER_BASE_ADDRESS: u8 = 0x10;
pub const GYROSCOPE_BASE_ADDRESS: u8 = 0x11;

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum LSM6DSOXGyroDataRate {
        LSMDSOX_GYRO_RATE_SHUTDOWN = 0,
        LSM6DSOX_GYRO_RATE_12_5_HZ = 1,
        LSM6DSOX_GYRO_RATE_26_HZ = 2,
        LSM6DSOX_GYRO_RATE_52_HZ = 3,
        LSM6DSOX_GYRO_RATE_104_HZ = 4,
        LSM6DSOX_GYRO_RATE_208_HZ = 5,
        LSM6DSOX_GYRO_RATE_416_HZ = 6,
        LSM6DSOX_GYRO_RATE_833_HZ = 7,
        LSM6DSOX_GYRO_RATE_1_66k_HZ = 8,
        LSM6DSOX_GYRO_RATE_3_33K_HZ = 9,
        LSM6DSOX_GYRO_RATE_6_66K_HZ = 10
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum LSM6DSOXAccelDataRate {
        LSMDSOX_ACCEL_RATE_SHUTDOWN = 0,
        LSM6DSOX_ACCEL_RATE_12_5_HZ = 1,
        LSM6DSOX_ACCEL_RATE_26_HZ = 2,
        LSM6DSOX_ACCEL_RATE_52_HZ = 3,
        LSM6DSOX_ACCEL_RATE_104_HZ = 4,
        LSM6DSOX_ACCEL_RATE_208_HZ = 5,
        LSM6DSOX_ACCEL_RATE_416_HZ = 6,
        LSM6DSOX_ACCEL_RATE_833_HZ = 7,
        LSM6DSOX_ACCEL_RATE_1_66k_HZ = 8,
        LSM6DSOX_ACCEL_RATE_3_33K_HZ = 9,
        LSM6DSOX_ACCEL_RATE_6_66K_HZ = 10
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum LSM6DSOXAccelRange {
        LSM6DSOX_ACCEL_RANGE_2_G = 0,
        LSM6DSOX_ACCEL_RANGE_16_G = 1,
        LSM6DSOX_ACCEL_RANGE_4_G = 2,
        LSM6DSOX_ACCEL_RANGE_8_G = 3
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum LSM6DSOXTRGyroRange {
        LSM6DSOX_GYRO_RANGE_250_DPS = 0,
        LSM6DSOX_GYRO_RANGE_500_DPS = 1,
        LSM6DSOX_GYRO_RANGE_1000_DPS = 2,
        LSM6DSOX_GYRO_RANGE_2000_DPS = 3
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum LSM6DSOXTRAccelRegisters {
        CTRL_REG1 = 0x11,
        CTRL_REG2 = 0x16,
        OUT_X_L_G = 0x22,
        OUT_X_H_G = 0x23,
        OUT_Y_L_G = 0x24,
        OUT_Y_H_G = 0x25,
        OUT_Z_L_G = 0x26,
        OUT_Z_H_G = 0x27
    }
}

