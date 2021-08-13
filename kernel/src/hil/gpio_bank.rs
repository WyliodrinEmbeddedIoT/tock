//! HIL for General Purpose Input-Output (GPIO) bank read.

// use core::cell::Cell;

// use crate::utilities::cells::TakeCell;
// use crate::hil;
// use crate::ErrorCode;

// pub struct GpioBank<'a> {
//     pin_array: TakeCell<'a, [u8]>,
// }

// TakeCell<'a, [u8]>

pub trait GpioBank<'a> {
    fn read_pins(&self, pin_array: &'a mut [u8]);
    fn write_pins(&self, pin_array: &'a mut [u8]);
}