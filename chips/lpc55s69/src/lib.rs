// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

#![no_std]

use cortexm33::{CortexM33, CortexMVariant, unhandled_interrupt};

pub mod gpio;
pub mod chip;

// LPC55S6x has a total of 59 interrupts, but the SDK declares 64 as 59 - 64 might be manual
#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".irqs")]
// `used` ensures that the symbol is kept until the final binary. However, as of
// May 2020, due to the compilation process, there must be some other compiled
// code here to make sure the object file is kept around. That means at minimum
// there must be an `init()` function here so that compiler does not just ignore
// the `IRQS` object. See https://github.com/rust-lang/rust/issues/56639 for a
// related discussion.
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static IRQS: [unsafe extern "C" fn(); 64] = [
    CortexM33::GENERIC_ISR,  // WDT, BOD, FLASH (0)
    CortexM33::GENERIC_ISR,  // SDMA0 (1)
    CortexM33::GENERIC_ISR,  // GPIO_GLOBALINIT0 (2)
    CortexM33::GENERIC_ISR,  // GPIO_GLOBALINIT1  (3)
    CortexM33::GENERIC_ISR,  // GPIO_INT0_IRQ0 (4)
    CortexM33::GENERIC_ISR,  // GPIO_INT0_IRQ1 (5)
    CortexM33::GENERIC_ISR,  // GPIO_INTO_IRQ2 (6)
    CortexM33::GENERIC_ISR,  // GPIO_INTO_IRQ3 (7)
    CortexM33::GENERIC_ISR,  // UTICK (8)
    CortexM33::GENERIC_ISR,  // MRT (9)
    CortexM33::GENERIC_ISR,  // CTIMER0 (10)
    CortexM33::GENERIC_ISR,  // CTIMER1 (11)
    CortexM33::GENERIC_ISR,  // SCT (12)
    CortexM33::GENERIC_ISR,  // CTIMER3 (13)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 0 (14)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 1 (15)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 2 (16)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 3 (17)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 4 (18)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 5 (19)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 6 (20)
    CortexM33::GENERIC_ISR,  // Flexcomm Interface 7 (21)
    CortexM33::GENERIC_ISR,  // ADC (22)
    CortexM33::GENERIC_ISR,  // Reserved (23)
    CortexM33::GENERIC_ISR,  // ACMP (24)
    CortexM33::GENERIC_ISR,  // Reserved (25)
    CortexM33::GENERIC_ISR,  // Reserved (26)
    CortexM33::GENERIC_ISR,  // USB0_NEEDCLK (27)
    CortexM33::GENERIC_ISR,  // USB0 (28)
    CortexM33::GENERIC_ISR,  // RTC (29)
    CortexM33::GENERIC_ISR,  // Reserved (30)
    CortexM33::GENERIC_ISR,  // WAKEUP_IRQn or MAILBOX (31)
    CortexM33::GENERIC_ISR,  // GPIO_INTO_IRQ4 (32)
    CortexM33::GENERIC_ISR,  // GPIO_INT0_IRQ5 (33)
    CortexM33::GENERIC_ISR,  // GPIO_INT0_IRQ6 (34)
    CortexM33::GENERIC_ISR,  // GPIO_INT0_IRQ7 (35)
    CortexM33::GENERIC_ISR,  // CTIMER2 (36)
    CortexM33::GENERIC_ISR,  // CTIMER4 (37)
    CortexM33::GENERIC_ISR,  // OSEVTIMER (38)
    CortexM33::GENERIC_ISR,  // Reserved (39)
    CortexM33::GENERIC_ISR,  // Reserved (40)
    CortexM33::GENERIC_ISR,  // Reserved (41)
    CortexM33::GENERIC_ISR,  // SDIO (42)
    CortexM33::GENERIC_ISR,  // Reserved (43)
    CortexM33::GENERIC_ISR,  // Reserved (44)
    CortexM33::GENERIC_ISR,  // Reserved (45)
    CortexM33::GENERIC_ISR,  // USB1_PHY (46)
    CortexM33::GENERIC_ISR,  // USB1 (47)
    CortexM33::GENERIC_ISR,  // USB1_NEEDCLK (48)
    CortexM33::GENERIC_ISR,  // HYPERADVISOR (49)
    CortexM33::GENERIC_ISR,  // SGPIO_INTO_IRQ0 (50)
    CortexM33::GENERIC_ISR,  // SGPIO_INTO_IRQ1 (51)
    CortexM33::GENERIC_ISR,  // PLU (52)
    CortexM33::GENERIC_ISR,  // SEC_VIO, SECURE_VIOLATION, SEC_VIOLOATION (53)
    CortexM33::GENERIC_ISR,  // HASH (54)
    CortexM33::GENERIC_ISR,  // CASPER (55)
    CortexM33::GENERIC_ISR,  // PUF (56)
    CortexM33::GENERIC_ISR,  // PQ (57)
    CortexM33::GENERIC_ISR,  // SDMA1 (58)
    CortexM33::GENERIC_ISR,  // HS_SPI (59)
    unhandled_interrupt,    // (60)
    unhandled_interrupt,    // (61)
    unhandled_interrupt,    // (62)
    unhandled_interrupt,    // (63)
];

pub unsafe fn init() {
    cortexm33::nvic::disable_all();
    cortexm33::nvic::clear_all_pending();
    cortexm33::nvic::enable_all();
}


