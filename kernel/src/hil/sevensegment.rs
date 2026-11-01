use crate::ErrorCode;

pub trait SevenSegment {
    fn num_digits(&self) -> u8;
    fn has_point(&self) -> bool;

    fn write(&self, number: u16, point: u8) -> Result<(), ErrorCode>;

    fn clear(&self) -> Result<(), ErrorCode>;
}
