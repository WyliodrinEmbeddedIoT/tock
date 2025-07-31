use core::fmt::Write;
<<<<<<< HEAD
use core::panic;
use cortex_m_semihosting::hprint;
use cortex_m_semihosting::hprintln;
// use cortex_m_semihosting::hprintln;
=======
>>>>>>> 2cc808484 (Add initial code for the GPIO)
// use cortex_m_semihosting::hprintln;
use cortexm33::{CortexM33, CortexMVariant};
use kernel::platform::chip::Chip;
use kernel::platform::chip::InterruptService;

<<<<<<< HEAD
use crate::ctimer0::LPCTimer;
use crate::gpio::GpioPin;
use crate::gpio::LPCPin;
use crate::gpio::Pins;
=======
use crate::gpio::LPCPin;
>>>>>>> 2cc808484 (Add initial code for the GPIO)
use crate::interrupts;
use crate::iocon::Iocon;
use crate::pint::Pint;

#[repr(u8)]
pub enum Processor {
    Processor0 = 0,
    Processor1 = 1,
}

pub struct Lpc55s69<'a, I: InterruptService + 'a> {
    mpu: cortexm33::mpu::MPU<8>,
    userspace_kernel_boundary: cortexm33::syscall::SysCall,
    interrupt_service: &'a I,
}

impl<'a, I: InterruptService> Lpc55s69<'a, I> {
    pub unsafe fn new(interrupt_service: &'a I) -> Self {
        Self {
            mpu: cortexm33::mpu::new(),
            userspace_kernel_boundary: cortexm33::syscall::SysCall::new(),
            interrupt_service,
        }
    }
}

impl<I: InterruptService> Chip for Lpc55s69<'_, I> {
    type MPU = cortexm33::mpu::MPU<8>;
    type UserspaceKernelBoundary = cortexm33::syscall::SysCall;

    fn service_pending_interrupts(&self) {
        unsafe {
            while let Some(interrupt) = cortexm33::nvic::next_pending() {
                if !self.interrupt_service.service_interrupt(interrupt) {
                    panic!("unhandled interrupt {}", interrupt);
                }

                let n = cortexm33::nvic::Nvic::new(interrupt);
                n.clear_pending();
                n.enable();
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm33::nvic::has_pending() }
    }

    fn mpu(&self) -> &Self::MPU {
        &self.mpu
    }

    fn userspace_kernel_boundary(&self) -> &Self::UserspaceKernelBoundary {
        &self.userspace_kernel_boundary
    }

    fn sleep(&self) {
        unsafe {
            cortex_m::asm::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm33::support::atomic(f)
    }

    unsafe fn print_state(&self, writer: &mut dyn Write) {
        CortexM33::print_cortexm_state(writer);
    }
}

<<<<<<< HEAD
pub struct Lpc55s69DefaultPeripheral<'a> {
    pub pins: Pins<'a>,
    pub ctimer0: LPCTimer<'a>,
}

impl<'a> Lpc55s69DefaultPeripheral<'a> {
    pub fn new() -> Self {
        Self {
            pins: Pins::new(),
            ctimer0: LPCTimer::new(),
=======
pub struct Lpc55s69DefaultPeripheral {
    pub iocon: Iocon,
    pub pint: Pint,
}

impl Lpc55s69DefaultPeripheral {
    pub fn new() -> Self {
        Self {
            iocon: Iocon::new(),
            pint: Pint::new(),
>>>>>>> 2cc808484 (Add initial code for the GPIO)
        }
    }

    pub fn resolve_dependencies(&'static self) {}
}

<<<<<<< HEAD
impl<'a> InterruptService for Lpc55s69DefaultPeripheral<'a> {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        // hprintln!("Interrupt: {}\n", interrupt);
        match interrupt {
            interrupts::GPIO_INT0_IRQ0 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt0 active!");
                // panic!("Interrupt0 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ1 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt1 active!");
                // panic!("Interrupt1 active!");

=======
impl InterruptService for Lpc55s69DefaultPeripheral {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        match interrupt {
            interrupts::GPIO_INT0_IRQ0 => {
                self.pint.handle_interrupt();
                // hprintln!("Interrupt0 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ1 => {
                self.pint.handle_interrupt();
                // hprintln!("Interrupt1 active!");
>>>>>>> 2cc808484 (Add initial code for the GPIO)
                true
            }

            interrupts::GPIO_INT0_IRQ2 => {
<<<<<<< HEAD
                self.pins.handle_interrupt();
                // hprintln!("Interrupt2 active!");
                // panic!("Interrupt2 active!");

                true
            }
            interrupts::GPIO_INT0_IRQ3 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt3 active!");
                // panic!("Interrupt3 active!");    }

                true
            }
            interrupts::GPIO_INT0_IRQ4 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt4 active!");
                // panic!("Interrupt4 active!");

                true
            }
            interrupts::GPIO_INT0_IRQ5 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt5 active!");
                // panic!("Interrupt5 active!");

                true
            }
            interrupts::GPIO_INT0_IRQ6 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt6 active!");
                // panic!("Interrupt6 active!");

                true
            }
            interrupts::GPIO_INT0_IRQ7 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt7 active!");
                // panic!("Interrupt7 active!");

                true
            }

            interrupts::CTIMER0 => {
                self.ctimer0.handle_interrupt();
                // hprintln!("Interrupt7 active!");
                // panic!("Interrupt7 active!");

=======
                self.pint.handle_interrupt();
                // hprintln!("Interrupt2 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ3 => {
                self.pint.handle_interrupt();
                // hprintln!("Interrupt3 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ4 => {
                self.pint.handle_interrupt();
                // hprintln!("Interrupt4 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ5 => {
                self.pint.handle_interrupt();
                // hprintln!("Interrupt5 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ6 => {
                self.pint.handle_interrupt();
                // hprintln!("Interrupt6 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ7 => {
                self.pint.handle_interrupt();
                // hprintln!("Interrupt7 active!");
>>>>>>> 2cc808484 (Add initial code for the GPIO)
                true
            }

            _ => true,
        }
    }
}
