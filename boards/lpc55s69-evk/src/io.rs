use crate::LPCPin;
use core::panic::PanicInfo;
use kernel::hil::gpio::*;
use lpc55s6x::gpio::GpioPin;
use lpc55s6x::iocon::{Config, Function, Iocon, Pull, Slew};
use lpc55s6x::pint::{Edge, Pint};

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
    red_led.set();
    red_led.clear();
    loop {}
}
