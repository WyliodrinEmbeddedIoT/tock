use crate::{LPCPin, CHIP, PROCESSES, PROCESS_PRINTER};
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut};
use cortex_m_semihosting::hprint;
use kernel::debug::{self, IoWrite};
use kernel::hil::gpio::*;
use kernel::hil::led::LedHigh;
use lpc55s6x::gpio::GpioPin;
use lpc55s6x::iocon::{Config, Function, Iocon, Pull, Slew};
use lpc55s6x::pint::{Edge, Pint};

pub struct Writer;

/// Global static for debug writer
pub static mut WRITER: Writer = Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.as_bytes() {
            hprint!("{}", *byte as char);
        }
        Ok(())
    }
}

impl IoWrite for Writer {
    fn write(&mut self, buf: &[u8]) -> usize {
        for byte in buf {
            hprint!("{}", *byte as char);
        }
        buf.len()
    }
}

#[panic_handler]
pub unsafe fn panic_fmt(panic_info: &PanicInfo) -> ! {
    let iocon_ctrl = Iocon::new();
    let led_pin_config = Config {
        function: Function::GPIO,
        pull: Pull::Up,
        slew: Slew::Standard,
        invert: false,
        digital_mode: true,
        open_drain: false,
    };
    iocon_ctrl.configure_pin(LPCPin::P1_6, led_pin_config);
    let red_led = GpioPin::new(LPCPin::P1_6);
    red_led.make_output();
    let led = &mut LedHigh::new(&red_led);
    // red_led.set();
    // red_led.clear();
    let writer = &mut *addr_of_mut!(WRITER);

    debug::panic(
        &mut [led],
        writer,
        panic_info,
        &cortexm33::support::nop,
        PROCESSES.unwrap().as_slice(),
        &*addr_of!(CHIP),
        &*addr_of!(PROCESS_PRINTER),
    )
}
