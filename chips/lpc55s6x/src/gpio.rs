// use cortex_m_semihosting::hprintln;
use kernel::hil::gpio;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite, WriteOnly};
use kernel::utilities::StaticRef;
// FINAL CORRECTED SYNTAX:
// The macro expects a tuple `(offset => field: type)` for every entry.
// For reserved space, we define a field of type `[u8; size]` or `[u32; size]`.
// The macro calculates padding based on the offset of the *next* defined register.
register_structs! {
    pub GpioRegisters {
        (0x0000 => _reserved0: [u8; 0x2000]),
        (0x2000 => dir_0: ReadWrite<u32, DIR::Register>),
        (0x2004 => dir_1: ReadWrite<u32, DIR::Register>),
        (0x2008 => _reserved1: [u8; 0x78]),
        (0x2080 => mask_0: ReadWrite<u32, MASK::Register>),
        (0x2084 => mask_1: ReadWrite<u32, MASK::Register>),
        (0x2088 => _reserved2: [u8; 0x78]),
        (0x2100 => pin_0: ReadWrite<u32, PIN::Register>),
        (0x2104 => pin_1: ReadWrite<u32, PIN::Register>),
        (0x2108 => _reserved3: [u8; 0x78]),
        (0x2180 => mpin_0: ReadWrite<u32, MPIN::Register>),
        (0x2184 => mpin_1: ReadWrite<u32, MPIN::Register>),
        (0x2188 => _reserved4: [u8; 0x78]),
        (0x2200 => set_0: WriteOnly<u32, SET::Register>),
        (0x2204 => set_1: WriteOnly<u32, SET::Register>),
        (0x2208 => _reserved5: [u8; 0x78]),
        (0x2280 => clr_0: WriteOnly<u32, CLR::Register>),
        (0x2284 => clr_1: WriteOnly<u32, CLR::Register>),
        (0x2288 => _reserved6: [u8; 0x78]),
        (0x2300 => not_0: WriteOnly<u32, NOT::Register>),
        (0x2304 => not_1: WriteOnly<u32, NOT::Register>),
        (0x2308 => _reserved7: [u8; 0x78]),
        (0x2380 => dirset_0: WriteOnly<u32, DIRSET::Register>),
        (0x2384 => dirset_1: WriteOnly<u32, DIRSET::Register>),
        (0x2388 => _reserved8: [u8; 0x78]),
        (0x2400 => dirclr_0: WriteOnly<u32, DIRCLR::Register>),
        (0x2404 => dirclr_1: WriteOnly<u32, DIRCLR::Register>),
        (0x2408 => _reserved9: [u8; 0x78]),
        (0x2480 => dirnot_0: WriteOnly<u32, DIRNOT::Register>),
        (0x2484 => dirnot_1: WriteOnly<u32, DIRNOT::Register>),
        (0x2488 => @END),
    }
}

register_bitfields![u32,
    DIR [ DIRP OFFSET(0) NUMBITS(32) [] ], MASK [ MASKP OFFSET(0) NUMBITS(32) [] ],
    PIN [ PORT OFFSET(0) NUMBITS(32) [] ], MPIN [ MPORTP OFFSET(0) NUMBITS(32) [] ],
    SET [ SETP OFFSET(0) NUMBITS(32) [] ], CLR [ CLRP OFFSET(0) NUMBITS(32) [] ],
    NOT [ NOTP OFFSET(0) NUMBITS(32) [] ], DIRSET [ DIRSETP OFFSET(0) NUMBITS(32) [] ],
    DIRCLR [ DIRCLRP OFFSET(0) NUMBITS(32) [] ], DIRNOT [ DIRNOTP OFFSET(0) NUMBITS(32) [] ]
];

pub(crate) const GPIO_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x5008_C000 as *const GpioRegisters) };

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LPCPin {
    P0_0,
    P0_1,
    P0_2,
    P0_3,
    P0_4,
    P0_5,
    P0_6,
    P0_7,
    P0_8,
    P0_9,
    P0_10,
    P0_11,
    P0_12,
    P0_13,
    P0_14,
    P0_15,
    P0_16,
    P0_17,
    P0_18,
    P0_19,
    P0_20,
    P0_21,
    P0_22,
    P0_23,
    P0_24,
    P0_25,
    P0_26,
    P0_27,
    P0_28,
    P0_29,
    P0_30,
    P0_31,
    P1_0,
    P1_1,
    P1_2,
    P1_3,
    P1_4,
    P1_5,
    P1_6,
    P1_7,
    P1_8,
    P1_9,
    P1_10,
    P1_11,
    P1_12,
    P1_13,
    P1_14,
    P1_15,
    P1_16,
    P1_17,
    P1_18,
    P1_19,
    P1_20,
    P1_21,
    P1_22,
    P1_23,
    P1_24,
    P1_25,
    P1_26,
    P1_27,
    P1_28,
    P1_29,
    P1_30,
    P1_31,
}

pub struct GpioPin<'a> {
    registers: StaticRef<GpioRegisters>,
    port: u8,
    pin: u8,
    pint_channel: OptionalCell<u8>,
    client: OptionalCell<&'a dyn gpio::Client>,
    inputmux: Inputmux,
    iocon: Iocon,
    pint: Pint,
}

pub use kernel::hil::gpio::{Configure, Input, Interrupt, Output, Pin};

use crate::inputmux::Inputmux;
use crate::iocon::Iocon;
use crate::pint::{self, Edge, Pint};

impl GpioPin<'_> {
    pub const fn new(pin_name: LPCPin) -> Self {
        let pin_num = pin_name as u8;
        Self {
            registers: GPIO_BASE,
            port: pin_num / 32,
            pin: pin_num % 32,
            pint_channel: OptionalCell::empty(),
            client: OptionalCell::empty(),
            inputmux: Inputmux::new(),
            iocon: Iocon::new(),
            pint: Pint::new(),
        }
    }

    fn pin_mask(&self) -> u32 {
        1 << self.pin
    }

    fn is_output(&self) -> bool {
        match self.port {
            0 => (self.registers.dir_0.get() & self.pin_mask()) != 0,
            1 => (self.registers.dir_1.get() & self.pin_mask()) != 0,
            _ => false,
        }
    }

    pub fn get_pin_num(&self) -> usize {
        (self.port as usize * 32) + self.pin as usize
    }
}

impl gpio::Output for GpioPin<'_> {
    fn set(&self) {
        match self.port {
            0 => self.registers.set_0.write(SET::SETP.val(self.pin_mask())),
            1 => self.registers.set_1.write(SET::SETP.val(self.pin_mask())),
            _ => {}
        }
    }

    fn clear(&self) {
        match self.port {
            0 => self.registers.clr_0.write(CLR::CLRP.val(self.pin_mask())),
            1 => self.registers.clr_1.write(CLR::CLRP.val(self.pin_mask())),
            _ => {}
        }
    }

    fn toggle(&self) -> bool {
        match self.port {
            0 => self.registers.not_0.write(NOT::NOTP.val(self.pin_mask())),
            1 => self.registers.not_1.write(NOT::NOTP.val(self.pin_mask())),
            _ => {}
        }
        self.read()
    }
}

impl gpio::Input for GpioPin<'_> {
    fn read(&self) -> bool {
        match self.port {
            0 => self.registers.pin_0.get() & self.pin_mask() != 0,
            1 => self.registers.pin_1.get() & self.pin_mask() != 0,
            _ => false,
        }
    }
}

impl gpio::Configure for GpioPin<'_> {
    fn make_output(&self) -> gpio::Configuration {
        match self.port {
            0 => self
                .registers
                .dirset_0
                .write(DIRSET::DIRSETP.val(self.pin_mask())),
            1 => self
                .registers
                .dirset_1
                .write(DIRSET::DIRSETP.val(self.pin_mask())),
            _ => {}
        }
        gpio::Configuration::Output
    }

    fn make_input(&self) -> gpio::Configuration {
        // hprintln!("Making input for pin {}", self.pin_mask());
        match self.port {
            0 => self
                .registers
                .dirclr_0
                .write(DIRCLR::DIRCLRP.val(self.pin_mask())),
            1 => self
                .registers
                .dirclr_1
                .write(DIRCLR::DIRCLRP.val(self.pin_mask())),
            _ => {}
        }
        gpio::Configuration::Input
    }

    fn configuration(&self) -> gpio::Configuration {
        if self.is_output() {
            gpio::Configuration::Output
        } else {
            gpio::Configuration::Input
        }
    }

    fn set_floating_state(&self, _state: gpio::FloatingState) {}
    fn floating_state(&self) -> gpio::FloatingState {
        gpio::FloatingState::PullNone
    }
    fn disable_input(&self) -> gpio::Configuration {
        self.make_output()
    }
    fn disable_output(&self) -> gpio::Configuration {
        self.make_input()
    }
    fn deactivate_to_low_power(&self) {
        let state = gpio::FloatingState::PullNone;
        self.make_input();
    }
}

impl<'a> gpio::Interrupt<'a> for GpioPin<'a> {
    fn set_client(&self, client: &'a dyn gpio::Client) {
        self.client.set(client);
    }

    fn enable_interrupts(&self, mode: gpio::InterruptEdge) {
        match mode {
            gpio::InterruptEdge::RisingEdge => {
                self.pint.configure_interrupt(0, Edge::Rising);
            }
            gpio::InterruptEdge::FallingEdge => {
                self.pint.configure_interrupt(0, Edge::Falling);
            }
            gpio::InterruptEdge::EitherEdge => {
                self.pint.configure_interrupt(0, Edge::Both);
            }
        }
    }

    fn disable_interrupts(&self) {
        self.pint.disable_interrupt(0);
    }

    fn is_pending(&self) -> bool {
        todo!()
    }
}

// impl<'a> gpio::Interrupt<'a> for GpioPin<'a> {
//     fn set_client(&self, client: &'a dyn gpio::Client) {
//         self.client.set(client)
//     }

//     // fn enable_interrupts(&self, mode: gpio::InterruptEdge) {
//     //     if self.pint_channel.is_none() {
//     //         if let Some(channel) = PINT.find_and_take_channel() {
//     //             self.pint_channel.set(channel);
//     //             PINT.select_pin(self.get_pin_num(), channel);
//     //         }
//     //     }

//     //     self.pint_channel.map(|channel|{
//     //         self.client.map(|client| PINT.set_client(channel, client));

//     //         let edge = match mode {
//     //             gpio::InterruptEdge::RisingEdge => Edge::Rising,
//     //             gpio::InterruptEdge::FallingEdge => Edge:: Falling,
//     //             gpio::InterruptEdge::EitherEdge => Edge::Both,
//     //         };
//     //         PINT.configure_interrupt(channel.into(), edge);
//     //     });
//     // }

//     // fn is_interrupt_enabled(&self) -> bool {
//     //     self.pint_channel.is_some()
//     // }

//     // fn disable_interrupts(&self) {
//     //     self.pint_channel.map(|channel| {
//     //         PINT.disable_and_free_channel(channel);
//     //     });
//     //     self.pint_channel.clear();
//     // }

//     fn is_pending(&self) -> bool {
//         false
//     }
// }
