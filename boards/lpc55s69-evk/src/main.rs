<<<<<<< HEAD
#![no_std]
#![no_main]

mod io;

use capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm;
use components::led::LedsComponent;
use core::arch::asm;
use core::panic;
use core::ptr::write_volatile;
use cortex_m::asm;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::pre_init;
use cortex_m_semihosting::hprintln;
use cortexm33;
use kernel::component::Component;
use kernel::hil::led::{LedHigh, LedLow};
use kernel::hil::time::{Alarm, Time};
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::process::ProcessArray;
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::{capabilities, create_capability, static_init};
use lpc55s6x::chip::{Lpc55s69, Lpc55s69DefaultPeripheral};
use lpc55s6x::clocks::Clock;
use lpc55s6x::gpio::{Configure, GpioPin, Input, LPCPin, Output};
use lpc55s6x::interrupts::GPIO_INT0_IRQ0;
use lpc55s6x::iocon::{Config, Function, Iocon, Pull, Slew};
use lpc55s6x::pint::{Edge, Pint};

#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x4000] = [0; 0x4000];

static mut PROCESS_PRINTER: Option<&'static capsules_system::process_printer::ProcessPrinterText> =
    None;

// // --- Manual Register Definitions for System Initialization ---
// const SYSCON_BASE: usize = 0x5000_0000; // Secure alias for SYSCON
// const CTIMER0_BASE: usize = 0x50008000;
// const AHBCLKCTRLSET0_ADDR: *mut u32 = (SYSCON_BASE + 0x200) as *mut u32;
// const AHBCLKCTRLSET1_ADDR: *mut u32 = (SYSCON_BASE + 0x204) as *mut u32;
// const TIMER: *mut u32 = (SYSCON_BASE + 0x26C) as *mut u32;
// const INPUTMUX_BASE: usize = 0x5000_6000;
// const PINTSEL0_ADDR: *mut u32 = (INPUTMUX_BASE + 0x0C0) as *mut u32;

// // These are the bitmasks from the C code's SystemInit()
// const SRAM1_CLK: u32 = 1 << 5; // Corresponds to SRAM_CTRL1
// const SRAM2_CLK: u32 = 1 << 6; // Corresponds to SRAM_CTRL2
// const SRAM3_CLK: u32 = 1 << 7; // Corresponds to SRAM_CTRL3
// const SRAM4_CLK: u32 = 1 << 8; // Corresponds to SRAM_CTRL4
// const IOCON_CLK: u32 = 1 << 13;
// const GPIO1_CLK: u32 = 1 << 15;
// const PINT_CLK: u32 = 1 << 18;
// const INPUTMUX_CLK: u32 = 1 << 11;
// const CTIMER0_CLK: u32 = 1 << 26;
// const CTIMERCLKSEL0: u32 = 0;
// const CTIMERCLKSEL0_ADDR: *mut u32 = (SYSCON_BASE + 0x26C) as *mut u32;
// const INTPIN: u64 = 1 << 41;
// const PRESETCTRLCLR1: *mut u32 = (SYSCON_BASE + 0x288) as *mut u32;

// #[pre_init]
fn system_init() {
    // This is the absolute first code to run.
    // We enable clocks for all the peripherals we will need, especially SRAM.
    // If we don't enable SRAM clocks, the program will fault when the runtime
    // tries to set up the stack.
    // write_volatile(
    //     AHBCLKCTRLSET0_ADDR,
    //     SRAM1_CLK
    //         | SRAM2_CLK
    //         | SRAM3_CLK
    //         | SRAM4_CLK
    //         | IOCON_CLK
    //         | GPIO1_CLK
    //         | PINT_CLK
    //         | INPUTMUX_CLK,
    // );

    // write_volatile(AHBCLKCTRLSET1_ADDR, CTIMER0_CLK);

    // Gate clock

    // core::ptr::write_volatile(CTIMERCLKSEL0_ADDR, 0);
    // // 2) Gate clock on for CTIMER0.
    // core::ptr::write_volatile(AHBCLKCTRLSET1_ADDR, CTIMER0_CLK);

    // // 3) Release CTIMER0 reset.
    // core::ptr::write_volatile(PRESETCTRLCLR1, CTIMER0_CLK);
    // write_volatile(AHBCLKCTRLSET1_ADDR, ALARM_CLK);
    let clocks = Clock::new();
    clocks.start_gpio_clocks();
    clocks.start_timer_clocks();

    // Add a memory barrier to ensure all writes are committed before proceeding.
    // asm::dmb();
    // asm::isb();
}

unsafe fn get_peripherals() -> &'static mut Lpc55s69DefaultPeripheral<'static> {
    static_init!(Lpc55s69DefaultPeripheral, Lpc55s69DefaultPeripheral::new())
}

const FAULT_RESPONSE: capsules_system::process_policies::PanicFaultPolicy =
    capsules_system::process_policies::PanicFaultPolicy {};

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 4;

/// Static variables used by io.rs.
static mut PROCESSES: Option<&'static ProcessArray<NUM_PROCS>> = None;
static mut CHIP: Option<&'static Lpc55s69<Lpc55s69DefaultPeripheral>> = None;

pub struct Lpc55s69evk {
    alarm: &'static capsules_core::alarm::AlarmDriver<
        'static,
        VirtualMuxAlarm<'static, lpc55s6x::ctimer0::LPCTimer<'static>>,
    >,
    gpio: &'static capsules_core::gpio::GPIO<'static, lpc55s6x::gpio::GpioPin<'static>>,
    led: &'static capsules_core::led::LedDriver<'static, LedLow<'static, GpioPin<'static>>, 1>,
    button: &'static capsules_core::button::Button<'static, lpc55s6x::gpio::GpioPin<'static>>,
    scheduler: &'static RoundRobinSched<'static>,
    systick: cortexm33::systick::SysTick,
}

impl SyscallDriverLookup for Lpc55s69evk {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::syscall::SyscallDriver>) -> R,
    {
        match driver_num {
            // capsules_core::console::DRIVER_NUM => f(Some(self.console)),
            capsules_core::alarm::DRIVER_NUM => f(Some(self.alarm)),
            capsules_core::led::DRIVER_NUM => f(Some(self.led)),
            capsules_core::button::DRIVER_NUM => f(Some(self.button)),
            capsules_core::gpio::DRIVER_NUM => f(Some(self.gpio)),
            _ => f(None),
        }
    }
}

impl KernelResources<Lpc55s69<'static, Lpc55s69DefaultPeripheral<'static>>> for Lpc55s69evk {
    type SyscallDriverLookup = Self;
    type SyscallFilter = ();
    type ProcessFault = ();
    type Scheduler = RoundRobinSched<'static>;
    type SchedulerTimer = cortexm33::systick::SysTick;
    type WatchDog = ();
    type ContextSwitchCallback = ();

    fn syscall_driver_lookup(&self) -> &Self::SyscallDriverLookup {
        self
    }
    fn syscall_filter(&self) -> &Self::SyscallFilter {
        &()
    }
    fn process_fault(&self) -> &Self::ProcessFault {
        &()
    }
    fn scheduler(&self) -> &Self::Scheduler {
        self.scheduler
    }
    fn scheduler_timer(&self) -> &Self::SchedulerTimer {
        &self.systick
    }
    fn watchdog(&self) -> &Self::WatchDog {
        &()
    }
    fn context_switch_callback(&self) -> &Self::ContextSwitchCallback {
        &()
    }
}

#[no_mangle]
unsafe fn main() -> ! {
    cortexm33::scb::set_vector_table_offset(0x00000000 as *const ());
    // By the time we get here, `system_init` has already run.
    // All necessary clocks are enabled.
    // unsafe {
    //     system_init();
    // }

    system_init();

    // let clocks = Clock::new();
    // clocks.start_gpio_clocks();

    // Create an array to hold process references.

    let peripherals = get_peripherals();

    peripherals.pins.init();

    let chip = static_init!(
        Lpc55s69<Lpc55s69DefaultPeripheral>,
        Lpc55s69::new(peripherals)
    );

    // Setup space to store the core kernel data structure.

    // let pint = Pint::new();
    cortexm33::nvic::enable_all();

    // lpc55s6x::init();

    let processes = components::process_array::ProcessArrayComponent::new()
        .finalize(components::process_array_component_static!(NUM_PROCS));
    PROCESSES = Some(processes);
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(processes.as_slice()));

    peripherals.ctimer0.init(96_000_000);

    let mux_alarm = components::alarm::AlarmMuxComponent::new(&peripherals.ctimer0).finalize(
        components::alarm_mux_component_static!(lpc55s6x::ctimer0::LPCTimer),
    );

    let alarm = components::alarm::AlarmDriverComponent::new(
        board_kernel,
        capsules_core::alarm::DRIVER_NUM,
        mux_alarm,
    )
    .finalize(components::alarm_component_static!(
        lpc55s6x::ctimer0::LPCTimer
    ));

    // let alarmTest = lpc55s6x::ctimer0::LPCTimer::new();
    // // hprintln!("PR: {}", alarmTest.get_pr());
    // // hprintln!("{}", alarmTest.is_armed());
    // loop {
    //     alarmTest.now();
    // }

    // Allocate the GpioLPCPin statically so its reference is valid
    let gpio = components::gpio::GpioComponent::new(
        board_kernel,
        capsules_core::gpio::DRIVER_NUM,
        components::gpio_component_helper!(
            lpc55s6x::gpio::GpioPin,
            0 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_0),
            1 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_1),
            2 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_2),
            3 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_3),
            5 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_4),
            6 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_5),
            7 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_6),
            8 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_7),
            9 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_8),
            10 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_9),
            11 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_10),
            12 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_11),
            13 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_12),
            14 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_13),
            15 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_14),
            16 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_15),
            17 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_16),
            18 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_17),
            19 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_18),
            20 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_19),
            21 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_20),
            22 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_21),
            23 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_22),
            24 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_23),
            25 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_24),
            26 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_25),
            27 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_26),
            28 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_27),
            29 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_28),
            30 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_29),
            31 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_30),
            32 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P0_31),
            33 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_0),
            34 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_1),
            35 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_2),
            36 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_3),
            // This is the blue LED: 37 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_4),
            38 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_5),
            // 39 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_6),
            40 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_7),
            41 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_8),
            //This is the button:  42 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_9),
            43 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_10),
            44 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_11),
            45 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_12),
            46 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_13),
            47 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_14),
            48 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_15),
            49 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_16),
            50 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_17),
            51 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_18),
            52 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_19),
            53 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_20),
            54 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_21),
            55 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_22),
            56 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_23),
            57 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_24),
            58 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_25),
            59 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_26),
            60 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_27),
            61 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_28),
            62 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_29),
            63 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_30),
            64 => peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_31),
        ),
    )
    .finalize(components::gpio_component_static!(lpc55s6x::gpio::GpioPin));

    let button_pin = peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_9);

    let button = components::button::ButtonComponent::new(
        board_kernel,
        capsules_core::button::DRIVER_NUM,
        components::button_component_helper!(
            GpioPin,
            (
                button_pin,
                kernel::hil::gpio::ActivationMode::ActiveLow,
                kernel::hil::gpio::FloatingState::PullUp
            ),
        ),
    )
    .finalize(components::button_component_static!(
        lpc55s6x::gpio::GpioPin
    ));

    let led_pin = peripherals.pins.get_pin(lpc55s6x::gpio::LPCPin::P1_4);

    let led = LedsComponent::new().finalize(components::led_component_static!(
        LedLow<'static, GpioPin>,
        LedLow::new(led_pin)
    ));

    const INPUTMUX_SRC: u8 = 41;
    // // unsafe {
    // //     write_volatile(LPCPinTSEL0_ADDR, INPUTMUX_SRC.try_into().unwrap());
    // // }

    peripherals.pins.inputmux.set_pintsel(0, INPUTMUX_SRC);

    peripherals.pins.pint.configure_interrupt(0, Edge::Rising);

    // cortexm33::nvic::clear_all_pending();

    // pint.configure_interrupt(0, Edge::Rising);

    // These symbols are defined in the linker script.
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
        /// End of the ROM region containing app images.
        static _eapps: u8;
        /// Beginning of the RAM region for app memory.
        static mut _sappmem: u8;
        /// End of the RAM region for app memory.
        static _eappmem: u8;
    }

    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);

    kernel::process::load_processes(
        board_kernel,
        chip,
        core::slice::from_raw_parts(
            core::ptr::addr_of!(_sapps),
            core::ptr::addr_of!(_eapps) as usize - core::ptr::addr_of!(_sapps) as usize,
        ),
        core::slice::from_raw_parts_mut(
            core::ptr::addr_of_mut!(_sappmem),
            core::ptr::addr_of!(_eappmem) as usize - core::ptr::addr_of!(_sappmem) as usize,
        ),
        &FAULT_RESPONSE,
        &process_management_capability,
    )
    .unwrap_or_else(|err| {
        kernel::debug!("Error loading processes!");
        kernel::debug!("{:?}", err);
    });

    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(processes)
        .finalize(components::round_robin_component_static!(NUM_PROCS));

    let lpc55 = Lpc55s69evk {
        alarm,
        gpio,
        button,
        led,
        scheduler,
        systick: cortexm33::systick::SysTick::new_with_calibration(12_000_000),
    };

    board_kernel.kernel_loop(
        &lpc55,
        chip,
        None::<kernel::ipc::IPC<{ NUM_PROCS as u8 }>>.as_ref(),
        &main_loop_capability,
    );
=======
// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

#![no_std]
#![no_main]
//#![deny(missing_docs)]

mod io;

#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

#[allow(unused)]
use lpc55s6x::BASE_VECTORS;

#[no_mangle]
pub unsafe fn main() {
    loop {
        cortexm33::support::nop();
    }
>>>>>>> 9d726ec54 (Adding support for NXP-LPC55)
}
