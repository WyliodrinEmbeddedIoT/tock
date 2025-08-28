#![no_std]
#![no_main]

use capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm;
use core::cell::Cell;
use core::panic::PanicInfo;
use cortex_m_semihosting::hprintln;
use cortexm33::nvic::Nvic;
use kernel::capabilities::ProcessManagementCapability;
use kernel::hil::uart::Configure;
use kernel::hil::uart::{Error, Parameters, Parity, StopBits, Width};
use kernel::hil::uart::{Receive, Transmit};
use kernel::hil::uart::{ReceiveClient, TransmitClient};
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::process::ProcessArray;
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::utilities::cells::OptionalCell;
use kernel::ErrorCode;
use kernel::{capabilities, create_capability, static_init};
use kernel::{debug, syscall};
use lpc55s6x::ctimer0::LPCTimer;
use lpc55s6x::uart::{self, Uart};
use lpc55s6x::{flexcomm, interrupts};

use core::arch::asm;
use core::ptr::write_volatile;
use cortex_m::peripheral::NVIC;
use cortex_m::peripheral::SCB;
use cortex_m::{asm, interrupt};
use cortexm33;
use kernel::component::Component;
use lpc55s6x::chip::{Lpc55s69, Lpc55s69DefaultPeripheral};

// Import your HAL drivers
use lpc55s6x::clocks::{self, Clock};

#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

#[no_mangle]
#[link_section = ".app_memory"]
pub static mut APP_MEMORY: [u8; 0] = [0; 0];

static mut PROCESS_PRINTER: Option<&'static capsules_system::process_printer::ProcessPrinterText> =
    None;

// This function is marked with `#[pre_init]`.
// `cortex-m-rt` will execute this function BEFORE it initializes RAM (.data and .bss)
// and before it calls main. This is our Rust equivalent of `SystemInit`.
// #[pre_init]
// unsafe fn system_init() {
//     // This is the absolute first code to run.
//     // We enable clocks for all the peripherals we will need, especially SRAM.
//     // If we don't enable SRAM clocks, the program will fault when the runtime
//     // tries to set up the stack.
//     write_volatile(
//         AHBCLKCTRLSET0_ADDR,
//         SRAM1_CLK
//             | SRAM2_CLK
//             | SRAM3_CLK
//             | SRAM4_CLK
//             | IOCON_CLK
//             | GPIO1_CLK
//             | PINT_CLK
//             | INPUTMUX_CLK,
//     );

//     // let clocks = Clock::new();
//     // clocks.start_gpio_clocks();
//     core::ptr::write_volatile(CTIMERCLKSEL0_ADDR, 0);
//     core::ptr::write_volatile(AHBCLKCTRLSET1_ADDR, CTIMER0_CLK);
//     core::ptr::write_volatile(PRESETCTRLCLR1, CTIMER0_CLK);

//     // Add a memory barrier to ensure all writes are committed before proceeding.
//     asm::dmb();
//     asm::isb();
// }

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
            // capsules_core::gpio::DRIVER_NUM => f(Some(self.gpio)),
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

    // Setup space to store the core kernel data structure.
    lpc55s6x::init();

    let peripherals = get_peripherals();

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
    let clock = static_init!(clocks::Clock, clocks::Clock::new());
    let flexcomm0 = static_init!(flexcomm::Flexcomm, flexcomm::Flexcomm::new_id(0).unwrap());

    let uart = &peripherals.uart;

    uart.set_clocks(clock);
    uart.set_flexcomm(flexcomm0);

    unsafe {
        let nvic_interrupt = Nvic::new(interrupts::FLEXCOMM0);
        nvic_interrupt.enable();
    }
    uart.setup_deferred_call();

    const IOCON_BASE: u32 = 0x4000_1000;
    unsafe {
        ((IOCON_BASE + 0x74) as *mut u32).write_volatile(0x101);
    }
    unsafe {
        ((IOCON_BASE + 0x78) as *mut u32).write_volatile(0x101);
    }
    let params = Parameters {
        baud_rate: 9600,
        width: Width::Eight,
        stop_bits: StopBits::One,
        parity: Parity::None,
        hw_flow_control: false,
    };
    uart.configure(params).unwrap();

    let chip = static_init!(
        Lpc55s69<Lpc55s69DefaultPeripheral>,
        Lpc55s69::new(peripherals)
    );

    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(processes)
        .finalize(components::round_robin_component_static!(NUM_PROCS));

    let uart_mux = components::console::UartMuxComponent::new(uart, 9600)
        .finalize(components::uart_mux_component_static!());

    let console = components::console::ConsoleComponent::new(
        board_kernel,
        capsules_core::console::DRIVER_NUM,
        uart_mux,
    )
    .finalize(components::console_component_static!());

    // Create the debugger object that handles calls to `debug!()`.
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

    let lpc55 = Lpc55s69evk {
        alarm,
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
