use core::time::Duration;
use kernel::hil::time::{Frequency, SynchronousTime,Time};

pub struct SynchronousTimer<> {
}

pub const FREQ:u128 = 20_000_000;
impl<> SynchronousTimer<> {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
}

impl <> kernel::hil::time::SynchronousTime for SynchronousTimer<> {
    fn sleep(&self, duration: Duration) {
        let mut num_nops:u128 = duration.as_nanos()*FREQ/1000_000_000;
        while num_nops>0 {
            rv32i::support::nop();
            num_nops-=1; //?
        }
    }
}