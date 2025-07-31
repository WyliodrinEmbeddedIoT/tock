<<<<<<< HEAD
<<<<<<< HEAD
#![no_std]
#![recursion_limit = "512"]

use cortexm33::{initialize_ram_jump_to_main, unhandled_interrupt, CortexM33, CortexMVariant};

pub mod chip;
pub mod clocks;
pub mod ctimer0;
pub mod gpio;
pub mod inputmux;
pub mod interrupts;
pub mod iocon;
pub mod pint;
// pub mod rtc;
pub mod syscon;

=======
// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

=======
>>>>>>> 2cc808484 (Add initial code for the GPIO)
#![no_std]
#![recursion_limit = "512"]

use cortexm33::{initialize_ram_jump_to_main, unhandled_interrupt, CortexM33, CortexMVariant};

<<<<<<< HEAD
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
=======
pub mod chip;
pub mod clocks;
pub mod gpio;
pub mod gpiopin;
pub mod inputmux;
pub mod interrupts;
pub mod iocon;
pub mod pint;
pub mod syscon;

>>>>>>> 2cc808484 (Add initial code for the GPIO)
extern "C" {
    fn _estack();
}

#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    link_section = ".vectors"
)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static BASE_VECTORS: [unsafe extern "C" fn(); 16] = [
    _estack,
    initialize_ram_jump_to_main,
    unhandled_interrupt,
    CortexM33::HARD_FAULT_HANDLER,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    CortexM33::SVC_HANDLER,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    CortexM33::SYSTICK_HANDLER,
];
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 2cc808484 (Add initial code for the GPIO)

#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".irqs")]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static IRQS: [unsafe extern "C" fn(); 60] = [
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
    CortexM33::GENERIC_ISR,
];

pub unsafe fn init() {
    cortexm33::nvic::disable_all();
    cortexm33::nvic::clear_all_pending();

    cortexm33::scb::set_vector_table_offset(core::ptr::addr_of!(BASE_VECTORS) as *const ());

    cortexm33::nvic::enable_all();
}
<<<<<<< HEAD
=======
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
=======
>>>>>>> 2cc808484 (Add initial code for the GPIO)
