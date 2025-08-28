use crate::ctimer0::LPCTimer;
use crate::gpio::Pins;
use crate::uart::Uart;
use core::cell::Cell;
use core::fmt::Write;
use cortex_m_semihosting::hprintln;
use cortexm33::{CortexM33, CortexMVariant};
use kernel::hil::uart::Transmit;
use kernel::hil::uart::{self, TransmitClient};
use kernel::platform::chip::Chip;
use kernel::platform::chip::InterruptService;

use crate::{ctimer0, interrupts};

static mut UART_TX_BUFFER: [u8; 128] = [0; 128];

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

pub struct Lpc55s69DefaultPeripheral<'a> {
    pub uart: Uart<'a>,
    pub ctimer0: LPCTimer<'a>,
    pub pins: Pins<'a>,
}

impl Lpc55s69DefaultPeripheral<'_> {
    pub fn new() -> Self {
        Self {
            uart: Uart::new_uart0(),
            ctimer0: LPCTimer::new(),
            pins: Pins::new(),
        }
    }

    pub fn resolve_dependencies(&'static self) {}
}

impl InterruptService for Lpc55s69DefaultPeripheral<'_> {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        match interrupt {
            interrupts::GPIO_INT0_IRQ0 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt0 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ1 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt1 active!");
                true
            }

            interrupts::GPIO_INT0_IRQ2 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt2 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ3 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt3 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ4 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt4 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ5 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt5 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ6 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt6 active!");
                true
            }
            interrupts::GPIO_INT0_IRQ7 => {
                self.pins.handle_interrupt();
                // hprintln!("Interrupt7 active!");
                true
            }
            interrupts::FLEXCOMM0 => {
                self.uart.handle_interrupt();
                true
            }

            _ => false,
        }
    }
}
