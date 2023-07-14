use kernel::utilities::StaticRef;
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite, WriteOnly};
use kernel::hil;
use crate::syscon;
    
#[repr(C)]
/// General Purpose I/O (GPIO)
struct GpioRegisters{
    /// Direction registers for all port GPIO pins
    dir_0: ReadWrite<u32, DIR0::Register>,
    /// Direction registers for all port GPIO pins
    dir_1: ReadWrite<u32, DIR1::Register>,
    /// Mask register for all port GPIO pins
    mask_0: ReadWrite<u32, MASK0::Register>,
    /// Mask register for all port GPIO pins
    mask_1: ReadWrite<u32, MASK1::Register>,
    /// Port pin register for all port GPIO pins
    pin_0: ReadWrite<u32, PIN0::Register>,
    /// Port pin register for all port GPIO pins
    pin_1: ReadWrite<u32, PIN1::Register>,
    /// Masked port register for all port GPIO pins
    mpin_0: ReadWrite<u32, MPIN0::Register>,
    /// Masked port register for all port GPIO pins
    mpin_1: ReadWrite<u32, MPIN1::Register>,
    /// Write: Set register for port. Read: output bits for port
    set_0: ReadWrite<u32, SET0::Register>,
    /// Write: Set register for port. Read: output bits for port
    set_1: ReadWrite<u32, SET1::Register>,
    /// Clear port for all port GPIO pins
    clr_0: WriteOnly<u32, CLR0::Register>,
    /// Clear port for all port GPIO pins
    clr_1: WriteOnly<u32, CLR1::Register>,
    /// Toggle port for all port GPIO pins
    not_0: WriteOnly<u32, NOT0::Register>,
    /// Toggle port for all port GPIO pins
    not_1: WriteOnly<u32, NOT1::Register>,
    /// Set pin direction bits for port
    dirset_0: WriteOnly<u32, DIRSET0::Register>,
    /// Set pin direction bits for port
    dirset_1: WriteOnly<u32, DIRSET1::Register>,
    /// Clear pin direction bits for port
    dirclr_0: WriteOnly<u32, DIRCLR0::Register>,
    /// Clear pin direction bits for port
    dirclr_1: WriteOnly<u32, DIRCLR1::Register>,
    /// Toggle pin direction bits for port
    dirnot_0: WriteOnly<u32, DIRNOT1::Register>,
    /// Toggle pin direction bits for port
    dirnot_1: WriteOnly<u32, DIRNOT1::Register>,
}

register_bitfields![u32,
    /// For all registers, each bit corresponds to a certain pin: bit 0 = PIOn_0, bit 1 = PIOn_1, etc.
    DIR0 [
        /// Selects pin direction for pin PIOm_n
        DIRP OFFSET(0) NUMBITS(32) []
    ],
    DIR1 [
        /// Selects pin direction for pin PIOm_n
        DIRP OFFSET(0) NUMBITS(32) []
    ],
    MASK0 [
        /// Controls which bits corresponding to PIOm_n are active in the MPORT register 
        MASKP OFFSET(0) NUMBITS(32) []
    ],
    MASK1 [
        /// Controls which bits corresponding to PIOm_n are active in the MPORT register
        MASKP OFFSET(0) NUMBITS(32) []
    ],
    PIN0 [
        /// Reads pin states or loads output bits 
        PORT OFFSET(0) NUMBITS(32) []
    ],
    PIN1 [
        /// Reads pin states or loads output bits 
        PORT OFFSET(0) NUMBITS(32) []
    ],
    MPIN0 [
        /// Masked port register 
        MPORTP OFFSET(0) NUMBITS(32) []
    ],
    MPIN1 [
        /// Masked port register
        MPORTP OFFSET(0) NUMBITS(32) []
    ],
    SET0 [
        /// Read or set output bits
        SETP OFFSET(0) NUMBITS(32) []
    ],
    SET1 [
        /// Read or set output bits
        SETP OFFSET(0) NUMBITS(32) []
    ],
    CLR0 [
        /// Clear output bits
        CLRP OFFSET(0) NUMBITS(32) []
    ],
    CLR1 [
        /// Clear output bits
        CLRP OFFSET(0) NUMBITS(32) []
    ],
    NOT0 [
        /// Toggle output bits
        NOTP OFFSET(0) NUMBITS(32) []
    ],
    NOT1 [
        /// Toggle output bits
        NOTP OFFSET(0) NUMBITS(32) []
    ],
    DIRSET0 [
        /// Set direction bits
        DIRSETP OFFSET(0) NUMBITS(32) []
    ],
    DIRSET1 [
        /// Set direction bits
        DIRSETP OFFSET(0) NUMBITS(32) []
    ],
    DIRCLR0 [
        /// Clear direction bits
        DIRCLRP OFFSET(0) NUMBITS(32) []
    ],
    DIRCLR1 [
        /// Clear direction bits
        DIRCLRP OFFSET(0) NUMBITS(32) []
    ],
    DIRNOT0 [
        /// Toggle direction bits
        DIRNOTP OFFSET(0) NUMBITS(32) []
    ],
    DIRNOT1 [
        /// Toggle direction bits
        DIRNOTP OFFSET(0) NUMBITS(32) []
    ],
];

#[allow(dead_code)]
const GPIO_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x5008C000 as *const GpioRegisters) };

struct GpioClock<'a>(syscon::PeripheralClock<'a>);

pub struct Gpio{
    registers: StaticRef<GpioRegisters>,
    clock: GpioClock<'a>,
}


/// A GPIO pin, like `GPIO3[17]`
///
/// `Pin` implements the `hil::gpio` traits. To acquire a `Pin`,
///
/// - use [`Ports::pin`] to reference a `Pin` by a [`PinId`], or
/// - use a combination of the ports on [`Ports`], and [`Port::pin`]
///
pub struct Pin{
    offset: usize,
}

impl<'a> Pin<'a> {
    const fn new(offset: usize) -> Self{
        Pin {
            offset,
        }
    }
}

// TODO: implement Pin functionalities
impl hil::gpio::Configure for Pin<'_> {
    fn make_output(&self) -> hil::gpio::Configuration{
        unimplemented!();
    }
    fn make_input(&self) -> hil::gpio::Configuration {
        unimplemented!();
    }
    fn deactivate_to_low_power(&self) {
        unimplemented!();
    }
    fn disable_output(&self) -> hil::gpio::Configuration {
        unimplemented!();
    }
    fn disable_input(&self) -> hil::gpio::Configuration {
        unimplemented!();
    }
    fn set_floating_state(&self, _mode: hil::gpio::FloatingState) {
        unimplemented!();
    }
    fn floating_state(&self) -> hil::gpio::FloatingState {
        unimplemented!();
    }
    fn configuration(&self) -> hil::gpio::Configuration {
        unimplemented!();
    }
}

impl hil::gpio::Output for Pin<'_> {
    fn set(&self) {
        unimplemented!();
    }
    fn clear(&self) {
        unimplemented!();
    }
    fn toggle(&self) -> bool {
        unimplemented!();
    }
}

impl hil::gpio::Input for Pin<'_> {
    fn read(&self) -> bool {
        unimplemented!();
    }
}
