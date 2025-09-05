use crate::gpio;
use crate::gpio::GpioPin;
use crate::gpio::GpioRegisters;
use crate::inputmux;
use crate::iocon;
use crate::pint;
use crate::pint::Pint;
pub use kernel::hil::gpio::{Configure, Input, Interrupt, Output, Pin};
use kernel::utilities::StaticRef;

// #[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]

pub struct Pins<'a> {
    pub pins: [GpioPin<'a>; 64],
    pint: Pint,
}

impl<'a> Pins<'a> {
    pub const fn new() -> Self {
        Self {
            pins: [
                GpioPin::new(gpio::LPCPin::P0_0),
                GpioPin::new(gpio::LPCPin::P0_1),
                GpioPin::new(gpio::LPCPin::P0_2),
                GpioPin::new(gpio::LPCPin::P0_3),
                GpioPin::new(gpio::LPCPin::P0_4),
                GpioPin::new(gpio::LPCPin::P0_5),
                GpioPin::new(gpio::LPCPin::P0_6),
                GpioPin::new(gpio::LPCPin::P0_7),
                GpioPin::new(gpio::LPCPin::P0_8),
                GpioPin::new(gpio::LPCPin::P0_9),
                GpioPin::new(gpio::LPCPin::P0_10),
                GpioPin::new(gpio::LPCPin::P0_11),
                GpioPin::new(gpio::LPCPin::P0_12),
                GpioPin::new(gpio::LPCPin::P0_13),
                GpioPin::new(gpio::LPCPin::P0_14),
                GpioPin::new(gpio::LPCPin::P0_15),
                GpioPin::new(gpio::LPCPin::P0_16),
                GpioPin::new(gpio::LPCPin::P0_17),
                GpioPin::new(gpio::LPCPin::P0_18),
                GpioPin::new(gpio::LPCPin::P0_19),
                GpioPin::new(gpio::LPCPin::P0_20),
                GpioPin::new(gpio::LPCPin::P0_21),
                GpioPin::new(gpio::LPCPin::P0_22),
                GpioPin::new(gpio::LPCPin::P0_23),
                GpioPin::new(gpio::LPCPin::P0_24),
                GpioPin::new(gpio::LPCPin::P0_25),
                GpioPin::new(gpio::LPCPin::P0_26),
                GpioPin::new(gpio::LPCPin::P0_27),
                GpioPin::new(gpio::LPCPin::P0_28),
                GpioPin::new(gpio::LPCPin::P0_29),
                GpioPin::new(gpio::LPCPin::P0_30),
                GpioPin::new(gpio::LPCPin::P0_31),
                GpioPin::new(gpio::LPCPin::P1_0),
                GpioPin::new(gpio::LPCPin::P1_1),
                GpioPin::new(gpio::LPCPin::P1_2),
                GpioPin::new(gpio::LPCPin::P1_3),
                GpioPin::new(gpio::LPCPin::P1_4),
                GpioPin::new(gpio::LPCPin::P1_5),
                GpioPin::new(gpio::LPCPin::P1_6),
                GpioPin::new(gpio::LPCPin::P1_7),
                GpioPin::new(gpio::LPCPin::P1_8),
                GpioPin::new(gpio::LPCPin::P1_9),
                GpioPin::new(gpio::LPCPin::P1_10),
                GpioPin::new(gpio::LPCPin::P1_11),
                GpioPin::new(gpio::LPCPin::P1_12),
                GpioPin::new(gpio::LPCPin::P1_13),
                GpioPin::new(gpio::LPCPin::P1_14),
                GpioPin::new(gpio::LPCPin::P1_15),
                GpioPin::new(gpio::LPCPin::P1_16),
                GpioPin::new(gpio::LPCPin::P1_17),
                GpioPin::new(gpio::LPCPin::P1_18),
                GpioPin::new(gpio::LPCPin::P1_19),
                GpioPin::new(gpio::LPCPin::P1_20),
                GpioPin::new(gpio::LPCPin::P1_21),
                GpioPin::new(gpio::LPCPin::P1_22),
                GpioPin::new(gpio::LPCPin::P1_23),
                GpioPin::new(gpio::LPCPin::P1_24),
                GpioPin::new(gpio::LPCPin::P1_25),
                GpioPin::new(gpio::LPCPin::P1_26),
                GpioPin::new(gpio::LPCPin::P1_27),
                GpioPin::new(gpio::LPCPin::P1_28),
                GpioPin::new(gpio::LPCPin::P1_29),
                GpioPin::new(gpio::LPCPin::P1_30),
                GpioPin::new(gpio::LPCPin::P1_31),
            ],
            pint: Pint::new(),
        }
    }

    pub fn get_pin(&self, id: gpio::LPCPin) -> &GpioPin<'a> {
        &self.pins[id as usize]
    }

    pub fn handle_interrupt(&self) {
        self.pint.handle_interrupt();
    }
}

pub struct GpioPins {
    id: gpio::LPCPin,
    gpio_register: StaticRef<GpioRegisters>,
    iocon_register: StaticRef<iocon::IoconRegisters>,
    pint: pint::Pint,
}

impl GpioPins {
    pub const fn new(id: gpio::LPCPin) -> Self {
        Self {
            id,
            gpio_register: gpio::GPIO_BASE,
            iocon_register: iocon::IOCON_BASE,
            pint: pint::Pint::new(),
        }
    }

    fn pin_mask(&self) -> u32 {
        1 << (self.id as u8 % 32)
    }

    fn port(&self) -> usize {
        (self.id as u8 / 32) as usize
    }

    fn pin_index(&self) -> usize {
        self.id as usize
    }
}

// impl gpio::Pin for GpioPins<'_> {}

impl gpio::Output for GpioPins {
    fn set(&self) {
        todo!()
    }

    fn clear(&self) {
        todo!()
    }

    fn toggle(&self) -> bool {
        todo!()
    }
}
