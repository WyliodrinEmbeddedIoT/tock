// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use kernel::platform::chip::{Chip, InterruptService};
use core::fmt::Write;
use cortexm33::{CortexM33, CortexMVariant};

extern "C" {
    fn _start_trap();
}

pub struct LPC55S69<I: InterruptService + 'static>{
    mpu: cortexm33::mpu::MPU,
    userspace_kernel_boundary: cortexm33::syscall::SysCall,
    interrupt_service: &'static I
}

impl<I: InterruptService + 'static> LPC55S69<I> {
    pub unsafe fn new(interrupt_service: &'static I) -> Self {
        LPC55S69 {
            mpu: cortexm33::mpu::MPU::new(),
            userspace_kernel_boundary: cortexm33::syscall::SysCall::new(),
            interrupt_service,
        }
    }
}


pub struct LPC55S69DefaultPeripherals {
}

impl<'a> LPC55S69DefaultPeripherals {
    pub fn new() -> Self {
        Self {
        }
    }

    // Resolves any circular dependencies and sets up deferred calls
    pub fn init(&'static self) {
        // Register deferred calls
    }
}

impl InterruptService for LPC55S69DefaultPeripherals{
    unsafe fn service_interrupt(&self, _interrupt:u32) -> bool {
        true
    }
}

impl<I: InterruptService + 'static> Chip for LPC55S69<I> {
    type MPU = cortexm33::mpu::MPU;
    type UserspaceKernelBoundary = cortexm33::syscall::SysCall;

    fn service_pending_interrupts(&self){
        unsafe{
            loop{
                if let Some(interrupt) = cortexm33::nvic::next_pending(){
                    let handled = self.interrupt_service.service_interrupt(interrupt);
                    assert!(handled, "Unhandled interrupt number {}", interrupt);
                    let n = cortexm33::nvic::Nvic::new(interrupt);
                    n.clear_pending();
                    n.enable();
                } else {
                    break;
                }
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm33::nvic::has_pending() }
    }

    fn mpu(&self) -> &cortexm33::mpu::MPU {
        &self.mpu
    }

    fn userspace_kernel_boundary(&self) -> &cortexm33::syscall::SysCall {
        &self.userspace_kernel_boundary
    }

    fn sleep(&self) {
        unsafe {
            cortexm33::scb::unset_sleepdeep();
            cortexm33::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm33::support::atomic(f)
    }

    unsafe fn print_state(&self, write: &mut dyn Write) {
        CortexM33::print_cortexm_state(write);
    }
}
