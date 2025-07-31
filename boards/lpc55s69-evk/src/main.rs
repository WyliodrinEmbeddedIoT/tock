#![no_std]
#![no_main]

mod io;

use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::process::ProcessArray;
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::{capabilities, create_capability, static_init};

use core::arch::asm;
use core::ptr::write_volatile;
use cortex_m::asm;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::pre_init;
use cortexm33;
use kernel::component::Component;
use lpc55s6x::chip::{Lpc55s69, Lpc55s69DefaultPeripheral};

use lpc55s6x::clocks::Clock;
use lpc55s6x::gpio::{Configure, GpioPin, Input, LPCPin, Output};
use lpc55s6x::interrupts::GPIO_INT0_IRQ0;
use lpc55s6x::iocon::{Config, Function, Iocon, Pull, Slew};
use lpc55s6x::pint::{Edge, Pint};

#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

// --- Manual Register Definitions for System Initialization ---
const SYSCON_BASE: usize = 0x5000_0000; // Secure alias for SYSCON
const AHBCLKCTRLSET0_ADDR: *mut u32 = (SYSCON_BASE + 0x200) as *mut u32;
const INPUTMUX_BASE: usize = 0x5000_6000;
const PINTSEL0_ADDR: *mut u32 = (INPUTMUX_BASE + 0x0C0) as *mut u32;

// These are the bitmasks from the C code's SystemInit()
const SRAM1_CLK: u32 = 1 << 5; // Corresponds to SRAM_CTRL1
const SRAM2_CLK: u32 = 1 << 6; // Corresponds to SRAM_CTRL2
const SRAM3_CLK: u32 = 1 << 7; // Corresponds to SRAM_CTRL3
const SRAM4_CLK: u32 = 1 << 8; // Corresponds to SRAM_CTRL4
const IOCON_CLK: u32 = 1 << 13;
const GPIO1_CLK: u32 = 1 << 15;
const PINT_CLK: u32 = 1 << 18;
const INPUTMUX_CLK: u32 = 1 << 11;

const INTPIN: u64 = 1 << 41;

// This function is marked with `#[pre_init]`.
// `cortex-m-rt` will execute this function BEFORE it initializes RAM (.data and .bss)
// and before it calls main. This is our Rust equivalent of `SystemInit`.
// #[pre_init]
unsafe fn system_init() {
    // This is the absolute first code to run.
    // We enable clocks for all the peripherals we will need, especially SRAM.
    // If we don't enable SRAM clocks, the program will fault when the runtime
    // tries to set up the stack.
    write_volatile(
        AHBCLKCTRLSET0_ADDR,
        SRAM1_CLK
            | SRAM2_CLK
            | SRAM3_CLK
            | SRAM4_CLK
            | IOCON_CLK
            | GPIO1_CLK
            | PINT_CLK
            | INPUTMUX_CLK,
    );

    // let clocks = Clock::new();
    // clocks.start_gpio_clocks();

    // Add a memory barrier to ensure all writes are committed before proceeding.
    asm::dmb();
    asm::isb();
}

unsafe fn get_peripherals() -> &'static mut Lpc55s69DefaultPeripheral {
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
            // capsules_core::alarm::DRIVER_NUM => f(Some(self.alarm)),
            // capsules_core::led::DRIVER_NUM => f(Some(self.led)),
            // capsules_core::button::DRIVER_NUM => f(Some(self.button)),
            // capsules_core::gpio::DRIVER_NUM => f(Some(self.gpio)),
            _ => f(None),
        }
    }
}

impl KernelResources<Lpc55s69<'static, Lpc55s69DefaultPeripheral>> for Lpc55s69evk {
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
    unsafe {
        system_init();
    }

    // Create an array to hold process references.
    let processes = components::process_array::ProcessArrayComponent::new()
        .finalize(components::process_array_component_static!(NUM_PROCS));
    PROCESSES = Some(processes);

    // Setup space to store the core kernel data structure.
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(processes.as_slice()));

    let pint = Pint::new();
    // cortexm33::nvic::enable_all();

    lpc55s6x::init();

    // 1. Configure the pin function using your IOCON driver.
    let iocon_ctrl = Iocon::new();
    let led_pin_config = Config {
        function: Function::GPIO,
        pull: Pull::Up,
        slew: Slew::Standard,
        invert: false,
        digital_mode: true,
        open_drain: false,
    };
    iocon_ctrl.configure_pin(LPCPin::P1_4, led_pin_config);
    let blue_led = GpioPin::new(LPCPin::P1_4);
    blue_led.make_output();
    blue_led.set();
    // blue_led.clear(); // Turn off the LED initially
    // unsafe {
    //     const NOT_1: *mut u32 = 0x5008E304 as *mut u32;
    //     const LED: u32 = 1 << 6;

    //     write_volatile(NOT_1, LED);
    // }
    // unsafe {
    //     const NOT_1: *mut u32 = 0x5008E304 as *mut u32;
    //     const LED: u32 = 1 << 6;

    //     write_volatile(NOT_1, LED);
    // }

    let button_pin_config = Config {
        function: Function::GPIO,
        pull: Pull::Up,
        slew: Slew::Standard,
        invert: false,
        digital_mode: true,
        open_drain: false,
    };
    iocon_ctrl.configure_pin(LPCPin::P1_9, button_pin_config);
    let button = GpioPin::new(LPCPin::P1_9);
    button.make_input();

    const INPUTMUX_SRC: u8 = 41;
    // unsafe {
    //     write_volatile(PINTSEL0_ADDR, INPUTMUX_SRC.try_into().unwrap());
    // }

    let inputmux = lpc55s6x::inputmux::Inputmux::new();
    inputmux.set_pintsel(0, INPUTMUX_SRC);

    pint.configure_interrupt(0, Edge::Rising);

    // loop {
    //     if pint.read_interrupt() != 0 {
    //         pint.handle_interrupt();
    //         blue_led.set(); // Toggle the LED state
    //     } else {
    //         blue_led.clear();
    //     }

    //     // hprintln!("{}", pint.read_interrupt());
    // }

    let peripherals = get_peripherals();

    let chip = static_init!(
        Lpc55s69<Lpc55s69DefaultPeripheral>,
        Lpc55s69::new(peripherals)
    );

    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(processes)
        .finalize(components::round_robin_component_static!(NUM_PROCS));

    let lpc55 = Lpc55s69evk {
        scheduler,
        systick: cortexm33::systick::SysTick::new_with_calibration(12_000_000),
    };

    board_kernel.kernel_loop(
        &lpc55,
        chip,
        None::<kernel::ipc::IPC<{ NUM_PROCS as u8 }>>.as_ref(),
        &main_loop_capability,
    );
}
// Delay loop tuned for the default 12 MHz clock.
fn delay_ms(ms: u32) {
    for _ in 0..ms {
        for _ in 0..3000 {
            asm::nop();
        }
    }
}

//Write on PINSEL register
