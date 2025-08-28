#![no_std]
#![no_main]

use capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm;
use core::arch::asm;
use core::panic::PanicInfo;
use cortexm33;
use kernel::component::Component;
use kernel::hil::uart::{Configure, Error, Parameters, Parity, StopBits, Width};
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::process::ProcessArray;
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::{capabilities, create_capability, static_init};
use kernel::{debug, syscall};
use lpc55s6x::chip::{Lpc55s69, Lpc55s69DefaultPeripheral};
use lpc55s6x::clocks::{self, Clock};
use lpc55s6x::ctimer0::LPCTimer;
use lpc55s6x::gpio::LPCPin;
use lpc55s6x::iocon::{Config, Function, Pull, Slew};
use lpc55s6x::pint::Edge;
use lpc55s6x::uart::{self, Uart};
use lpc55s6x::{flexcomm, interrupts};

#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

#[no_mangle]
#[link_section = ".app_memory"]
pub static mut APP_MEMORY: [u8; 0] = [0; 0];

static mut PROCESS_PRINTER: Option<&'static capsules_system::process_printer::ProcessPrinterText> =
    None;

unsafe fn get_peripherals() -> &'static mut Lpc55s69DefaultPeripheral<'static> {
    static_init!(Lpc55s69DefaultPeripheral, Lpc55s69DefaultPeripheral::new())
}

const FAULT_RESPONSE: capsules_system::process_policies::PanicFaultPolicy =
    capsules_system::process_policies::PanicFaultPolicy {};

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 4;
const INPUTMUX_SRC: u8 = 41;

/// Static variables used by io.rs.
static mut PROCESSES: Option<&'static ProcessArray<NUM_PROCS>> = None;
static mut CHIP: Option<&'static Lpc55s69<Lpc55s69DefaultPeripheral>> = None;

pub struct Lpc55s69evk {
    alarm: &'static capsules_core::alarm::AlarmDriver<
        'static,
        VirtualMuxAlarm<'static, lpc55s6x::ctimer0::LPCTimer<'static>>,
    >,
    gpio: &'static capsules_core::gpio::GPIO<'static, lpc55s6x::gpio::GpioPin<'static>>,
    scheduler: &'static RoundRobinSched<'static>,
    systick: cortexm33::systick::SysTick,
    console: &'static capsules_core::console::Console<'static>,
}

impl SyscallDriverLookup for Lpc55s69evk {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::syscall::SyscallDriver>) -> R,
    {
        match driver_num {
            capsules_core::console::DRIVER_NUM => f(Some(self.console)),
            capsules_core::alarm::DRIVER_NUM => f(Some(self.alarm)),
            // capsules_core::led::DRIVER_NUM => f(Some(self.led)),
            // capsules_core::button::DRIVER_NUM => f(Some(self.button)),
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
    lpc55s6x::init();

    let peripherals = get_peripherals();

    let pins = static_init!(lpc55s6x::gpio::Pins, lpc55s6x::gpio::Pins::new());
    pins.init();

    let processes = components::process_array::ProcessArrayComponent::new()
        .finalize(components::process_array_component_static!(NUM_PROCS));
    PROCESSES = Some(processes);
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(processes.as_slice()));

    // Alarm
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

    // GPIO
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

    // USART
    let clock = static_init!(clocks::Clock, clocks::Clock::new());
    let flexcomm0 = static_init!(flexcomm::Flexcomm, flexcomm::Flexcomm::new_id(0).unwrap());

    let uart = &peripherals.uart;

    uart.set_clocks(clock);
    uart.set_flexcomm(flexcomm0);
    uart.setup_deferred_call();

    let uart_pin_config = Config {
        // Configuration for digital mode, responding to FLEXCOMM0 signals
        function: Function::Alt1,
        pull: Pull::None,
        digital_mode: true,
        slew: Slew::Standard,
        invert: false,
        open_drain: false,
    };

    pins.iocon.configure_pin(LPCPin::P0_29, uart_pin_config);
    pins.iocon.configure_pin(LPCPin::P0_30, uart_pin_config);

    peripherals.pins.inputmux.set_pintsel(0, INPUTMUX_SRC);

    peripherals.pins.pint.configure_interrupt(0, Edge::Rising);

    let params = Parameters {
        // USART initial configuration, using default settings
        baud_rate: 9600,
        width: Width::Eight,
        stop_bits: StopBits::One,
        parity: Parity::None,
        hw_flow_control: false,
    };
    uart.configure(params).unwrap();

    let uart_mux = components::console::UartMuxComponent::new(uart, 9600)
        .finalize(components::uart_mux_component_static!());

    let chip = static_init!(
        Lpc55s69<Lpc55s69DefaultPeripheral>,
        Lpc55s69::new(peripherals)
    );

    // Console
    let console = components::console::ConsoleComponent::new(
        board_kernel,
        capsules_core::console::DRIVER_NUM,
        uart_mux,
    )
    .finalize(components::console_component_static!());

    // Process console
    components::debug_writer::DebugWriterComponent::new(
        uart_mux,
        create_capability!(capabilities::SetDebugWriterCapability),
    )
    .finalize(components::debug_writer_component_static!());

    let process_printer = components::process_printer::ProcessPrinterTextComponent::new()
        .finalize(components::process_printer_text_component_static!());
    PROCESS_PRINTER = Some(process_printer);

    let process_console = components::process_console::ProcessConsoleComponent::new(
        board_kernel,
        uart_mux,
        mux_alarm,
        process_printer,
        None,
    )
    .finalize(components::process_console_component_static!(
        LPCTimer<'static>
    ));
    let _ = process_console.start();

    debug!("Tock");

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

    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(processes)
        .finalize(components::round_robin_component_static!(NUM_PROCS));

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
        debug!("Error loading processes!");
        debug!("{:?}", err);
    });

    let lpc55 = Lpc55s69evk {
        alarm,
        gpio,
        scheduler,
        systick: cortexm33::systick::SysTick::new_with_calibration(12_000_000),
        console: console,
    };

    board_kernel.kernel_loop(
        &lpc55,
        chip,
        None::<kernel::ipc::IPC<{ NUM_PROCS as u8 }>>.as_ref(),
        &main_loop_capability,
    );
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("bkpt") };
    }
}
