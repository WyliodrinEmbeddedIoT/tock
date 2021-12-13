use core::time::Duration;
use kernel::hil::time::SyncronousTime;

pub struct SynchronousTimer<'a,F:kernel::hil::time::Frequency> {
    frequency: &'a F,
}

impl<'a,F> SynchronousTimer<'a, F> {
    pub fn new(
        frequency: &'a F,
    ) -> Self {
        Self {
        frequency
        }
    }
}

impl <'a,F> kernel::hil::time::SyncronousTime for SynchronousTimer<'a,F> {
    fn sleep(&self, duration: Duration) {
        let mut num_nops:u32 = duration.as_nanos()*self.frequency;
        while num_nops>0 {
            rv32i::support::nop();
            num_nops-=1; //?
        }
    }
}