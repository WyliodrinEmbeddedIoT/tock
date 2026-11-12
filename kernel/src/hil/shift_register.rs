use crate::ErrorCode;

pub trait ShiftRegister {
    fn write(&self, value: usize) -> Result<(), ErrorCode>;
}
