// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OxidOS Automotive 2026.

use kernel::platform::watchdog::WatchDog;
use kernel::utilities::StaticRef;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{Readable, Writeable};
use kernel::utilities::registers::{
    ReadOnly, ReadWrite, WriteOnly, register_bitfields, register_structs,
};

register_structs! {
    IwdgRegisters {
        // Key register
        (0x00 => kr: WriteOnly<u32, KR::Register>),
        // Prescaler register
        (0x04 => pr: ReadWrite<u32, PR::Register>),
        // Reload register
        (0x08 => rlr: ReadWrite<u32, RLR::Register>),
        // Status register
        (0x0C => sr: ReadOnly<u32, SR::Register>),
        // Window register
        (0x10 => winr: ReadWrite<u32, WINR::Register>),
        // Early wake-up register
        (0x14 => ewcr: ReadWrite<u32, EWCR::Register>),
        (0x18 => @END),
    }
}

register_bitfields! [u32,
    KR [
        KEY OFFSET(0) NUMBITS(16) [
            Reload = 0xAAAA, // Reset the timer
            Unlock = 0x5555, // Unlock registers for writing
            Start = 0xCCCC, // Start timer
        ],
    ],

    PR [
        // Prescaler divider
        PR OFFSET(0) NUMBITS(4) [
            DivideBy4 = 0b0000,
            DivideBy8 = 0b0001,
            DivideBy16 = 0b0010,
            DivideBy32 = 0b0011,
            DivideBy64 = 0b0100,
            DivideBy128 = 0b0101,
            DivideBy256 = 0b0110,
            DivideBy512 = 0b0111,
            DivideBy1024 = 0b1000,
        ],
    ],

    RLR [
        // Watchdog counter reload value
        RL OFFSET(0) NUMBITS(12) [],
    ],

    SR [
        // Watchdog early interrupt flag
        EWIF OFFSET(14) NUMBITS(1) [],

        // Watchdog interrupt comparartor value update
        EWU OFFSET(3) NUMBITS(1) [],

        // Watchdog counter window value update
        WVU OFFSET(2) NUMBITS(1) [],

        // Watchdog counter reload value update
        RVU OFFSET(1) NUMBITS(1) [],

        // Watchdog prescaler value update
        PVU OFFSET(0) NUMBITS(1) [],
    ],

    WINR [
        // Watchdog counter window value
        WIN OFFSET(0) NUMBITS(12) [],
    ],

    EWCR [

        // Watchdog early interrupt enable
        EWIE OFFSET(15) NUMBITS(1) [],

        // Watchdog early interrupt acknowledge
        EWIC OFFSET(14) NUMBITS(1) [],

        // Watchdog counter early wake-up interrupt value
        EWIT OFFSET(0) NUMBITS(12) [],
    ]
];

const IWDG_BASE: StaticRef<IwdgRegisters> =
    unsafe { StaticRef::new(0x40003000 as *const IwdgRegisters) };

/// Helps us to implement a "wake up and tickle" workaround for sleep mode
pub trait IwdgWaker {
    fn wakeup(&self);
}

/// The timer trait to be implemented by RTC (or other timer in future)
pub trait WakeupTimer {
    fn enable_watchdog_wakeup(&self, seconds: u16);
    fn disable_watchdog_wakeup(&self);
}

pub struct Iwdg<'a> {
    registers: StaticRef<IwdgRegisters>,
    wakeup_timer: OptionalCell<&'a dyn WakeupTimer>,
}

impl<'a> Iwdg<'a> {
    pub const fn new() -> Iwdg<'a> {
        Iwdg {
            registers: IWDG_BASE,
            wakeup_timer: OptionalCell::empty(),
        }
    }

    pub fn set_wakeup_timer(&self, timer: &'a dyn WakeupTimer) {
        self.wakeup_timer.set(timer);
    }

    /// Setup registers for normal operation
    fn configure_normal(&self) {
        // 32khz / 32 = 1khz => one tick per ms => 1000 to RL register means 1 second timeout before reboot
        self.registers.pr.write(PR::PR::DivideBy32);
        self.registers.rlr.set(1000);
    }

    /// Setup registers for opertion during sleep mode (as slow as possible)
    fn configure_sleep(&self) {
        // 32 kHz / 1024 = 31.25 Hz => 31.25 ticks per second. 4095 / 31.25 = 131s before reboot
        self.registers.pr.write(PR::PR::DivideBy1024);
        self.registers.rlr.set(4095); // maximum 12-bit number
    }
}

impl WatchDog for Iwdg<'_> {
    fn setup(&self) {
        self.registers.kr.write(KR::KEY::Unlock);

        while self.registers.sr.is_set(SR::PVU) || self.registers.sr.is_set(SR::RVU) {
            // Block the executor until hardware is ready
        }

        self.configure_normal();

        self.registers.kr.write(KR::KEY::Start);
        self.tickle();
    }

    fn tickle(&self) {
        // Reset the counter
        self.registers.kr.write(KR::KEY::Reload);
    }

    /// This function is called before going into sleep mode
    /// However, the on this chip the suspending is not supported
    /// As a workaround, we just reconfigure the watchdog, making it as slow as possible,
    /// and then schedule an interrupt to wake up and tickle just before running out of time
    /// The most we can get is 131s (divider of 1024 and reload value of 4095)
    fn suspend(&self) {
        self.registers.kr.write(KR::KEY::Unlock);

        while self.registers.sr.is_set(SR::PVU) || self.registers.sr.is_set(SR::RVU) {
            // Block the executor until hardware is ready
        }

        self.configure_sleep();

        self.registers.kr.write(KR::KEY::Start);

        self.wakeup_timer
            .map(|timer| timer.enable_watchdog_wakeup(125));

        self.tickle();
    }

    /// This function is called when going back from sleep mode
    /// It reconfigures the watchdog back to normal mode
    fn resume(&self) {
        self.wakeup_timer
            .map(|timer| timer.disable_watchdog_wakeup());
        self.setup();
    }
}

impl IwdgWaker for Iwdg<'_> {
    fn wakeup(&self) {
        self.tickle();
    }
}
