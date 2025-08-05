// boards/lpc55s69-evk/src/main.rs

#![no_std]
#![no_main]
// Add the checksum feature
//#![feature(linkage)]
//#[linkage = "weak"]
//#[no_mangle]
//static __checksum: u32 = 0;

use cortex_m_semihosting::hprintln;
use cortexm33::nvic::Nvic;
use kernel::hil::uart::{Parameters, Parity, StopBits, Width, Error};
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::hil::uart::{ReceiveClient,TransmitClient};
use kernel::utilities::cells::OptionalCell;
use kernel::ErrorCode;
use kernel::process::ProcessArray;
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::{capabilities, create_capability, static_init};
use lpc55s6x::{flexcomm, interrupts};
use lpc55s6x::uart::{self, Uart};
use core::panic::PanicInfo;
use kernel::hil::uart::Configure;
use core::cell::Cell;
use kernel::hil::uart::{Receive,Transmit};

use core::arch::asm;
use core::ptr::write_volatile;
use cortex_m::{asm, interrupt};
use cortex_m::peripheral::NVIC;
// use cortex_m_semihosting::hprintln;
use cortexm33;
use kernel::component::Component;
use lpc55s6x::chip::{Lpc55s69, Lpc55s69DefaultPeripheral};

// Import your HAL drivers
use lpc55s6x::clocks::{self, Clock};

#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x2000] = [0; 0x2000];

static mut UART_TX_BUFFER: [u8; 128] = [0; 128];
static mut UART_RX_BUFFER: [u8; 128] = [0; 128];


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


pub struct Main {
    uart: &'static uart::Uart<'static>,
    tx_busy: Cell<bool>,
    rx_busy: Cell<bool>
}

impl TransmitClient for Main {
    fn transmitted_buffer(
        &self,
        _buffer: &'static mut [u8],
        _tx_len: usize,
        _result: Result<(), kernel::ErrorCode>,
    ) {
        self.tx_busy.set(false);
    }
}

impl ReceiveClient for Main {
    fn received_buffer(
        &self,
        buffer: &'static mut [u8],
        rx_len: usize,
        _result: Result<(), ErrorCode>,
        _error: Error,
    ) {
        // --- ECHO LOGIC ---
        // Try to send back what we just received.
        if !self.tx_busy.get() {
            // The transmitter is free, let's echo.
            unsafe {
                let echo_len = rx_len.min(UART_TX_BUFFER.len());
                UART_TX_BUFFER[..echo_len].copy_from_slice(&buffer[..echo_len]);

                // Mark the transmitter as busy and start the echo transmission.
                // The `transmitted_buffer` callback will set tx_busy back to false.
                self.tx_busy.set(true);
                let _ = self.uart.transmit_buffer(&mut UART_TX_BUFFER, echo_len);
            }
        } else {
            // The UART is busy sending something else. We'll just drop this echo.
            hprintln!("TX busy, dropping echo.");
        }
        let _ = self.uart.receive_buffer(buffer, buffer.len());
    }
}

impl Main {
    pub fn start_uart_tx(&'static self) {
        if self.tx_busy.get() {
            return;
        }

        unsafe {
            let message = b"Hello from main.rs! This is a longer message to test the capabilities of sending messages.\r\n";
            let len = message.len();
            UART_TX_BUFFER[..len].copy_from_slice(message);
            self.tx_busy.set(true);
            let _ = self.uart.transmit_buffer(&mut UART_TX_BUFFER, len);
        }
    }

    pub fn start_uart_rx(&'static self) {
        if self.rx_busy.get() {
            return;
        }

        self.rx_busy.set(true);
        unsafe {
            let _ = self.uart.receive_buffer(&mut UART_RX_BUFFER, UART_RX_BUFFER.len());
        }
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


    lpc55s6x::init();

    let peripherals = get_peripherals();

        let clock = static_init!(clocks::Clock, clocks::Clock::new());
        let flexcomm0 = static_init!(flexcomm::Flexcomm, flexcomm::Flexcomm::new_id(0).unwrap());

        let uart = &peripherals.uart;

        uart.set_clocks(clock);
        uart.set_flexcomm(flexcomm0);

        unsafe {
            let nvic_interrupt = Nvic::new(interrupts::FLEXCOMM0);
            nvic_interrupt.enable();
        }


        let main_app = static_init!(
            Main,
            Main {
                uart: uart,
                tx_busy: Cell::new(false),
                rx_busy: Cell::new(false),
            }
        );

        

        uart.set_transmit_client(main_app);
        uart.set_receive_client(main_app);

        const IOCON_BASE: u32 = 0x4000_1000;
        unsafe { ((IOCON_BASE + 0x74) as *mut u32).write_volatile(0x101); }
        unsafe { ((IOCON_BASE + 0x78) as *mut u32).write_volatile(0x101); }
        let params = Parameters { baud_rate: 9600, width: Width::Eight, stop_bits: StopBits::One, parity: Parity::None, hw_flow_control: false };
        uart.configure(params).unwrap();

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

        main_app.start_uart_tx();
        main_app.start_uart_rx();

        // loop {
        //     if uart.uart_is_readable() {
        //         let received_byte = uart.receive_byte();
        //         while !uart.uart_is_writable() {}
        //         uart.send_byte(received_byte);
        //     }
        // }     

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


