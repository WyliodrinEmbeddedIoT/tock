use kernel::utilities::StaticRef;
use kernel::utilities::registers::{self, register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly};
    
register_structs! {
    /// General Purpose I/O (GPIO)
    GpioRegisters {
        (0x000 => _reserved0),
        /// Direction registers for all port GPIO pins
        (0x2000 => dir_0: ReadWrite<u32>),
        /// Direction registers for all port GPIO pins
        (0x2004 => dir_1: ReadWrite<u32>),
        /// Direction registers for all port GPIO pins
        (0x2008 => dir_2: ReadWrite<u32>),
        /// Direction registers for all port GPIO pins
        (0x200C => dir_3: ReadWrite<u32>),
        (0x2010 => _reserved1),
        /// Mask register for all port GPIO pins
        (0x2080 => mask_0: ReadWrite<u32>),
        /// Mask register for all port GPIO pins
        (0x2084 => mask_1: ReadWrite<u32>),
        /// Mask register for all port GPIO pins
        (0x2088 => mask_2: ReadWrite<u32>),
        /// Mask register for all port GPIO pins
        (0x208C => mask_3: ReadWrite<u32>),
        (0x2090 => _reserved2),
        /// Port pin register for all port GPIO pins
        (0x2100 => pin_0: ReadWrite<u32>),
        /// Port pin register for all port GPIO pins
        (0x2104 => pin_1: ReadWrite<u32>),
        /// Port pin register for all port GPIO pins
        (0x2108 => pin_2: ReadWrite<u32>),
        /// Port pin register for all port GPIO pins
        (0x210C => pin_3: ReadWrite<u32>),
        (0x2110 => _reserved3),
        /// Masked port register for all port GPIO pins
        (0x2180 => mpin_0: ReadWrite<u32>),
        /// Masked port register for all port GPIO pins
        (0x2184 => mpin_1: ReadWrite<u32>),
        /// Masked port register for all port GPIO pins
        (0x2188 => mpin_2: ReadWrite<u32>),
        /// Masked port register for all port GPIO pins
        (0x218C => mpin_3: ReadWrite<u32>),
        (0x2190 => _reserved4),
        /// Write: Set register for port. Read: output bits for port
        (0x2200 => set_0: ReadWrite<u32>),
        /// Write: Set register for port. Read: output bits for port
        (0x2204 => set_1: ReadWrite<u32>),
        /// Write: Set register for port. Read: output bits for port
        (0x2208 => set_2: ReadWrite<u32>),
        /// Write: Set register for port. Read: output bits for port
        (0x220C => set_3: ReadWrite<u32>),
        (0x2210 => _reserved5),
        /// Clear port for all port GPIO pins
        (0x2280 => clr_0: WriteOnly<u32>),
        /// Clear port for all port GPIO pins
        (0x2284 => clr_1: WriteOnly<u32>),
        /// Clear port for all port GPIO pins
        (0x2288 => clr_2: WriteOnly<u32>),
        /// Clear port for all port GPIO pins
        (0x228C => clr_3: WriteOnly<u32>),
        (0x2290 => _reserved6),
        /// Toggle port for all port GPIO pins
        (0x2300 => not_0: WriteOnly<u32>),
        /// Toggle port for all port GPIO pins
        (0x2304 => not_1: WriteOnly<u32>),
        /// Toggle port for all port GPIO pins
        (0x2308 => not_2: WriteOnly<u32>),
        /// Toggle port for all port GPIO pins
        (0x230C => not_3: WriteOnly<u32>),
        (0x2310 => _reserved7),
        /// Set pin direction bits for port
        (0x2380 => dirset_0: WriteOnly<u32>),
        /// Set pin direction bits for port
        (0x2384 => dirset_1: WriteOnly<u32>),
        /// Set pin direction bits for port
        (0x2388 => dirset_2: WriteOnly<u32>),
        /// Set pin direction bits for port
        (0x238C => dirset_3: WriteOnly<u32>),
        (0x2390 => _reserved8),
        /// Clear pin direction bits for port
        (0x2400 => dirclr_0: WriteOnly<u32>),
        /// Clear pin direction bits for port
        (0x2404 => dirclr_1: WriteOnly<u32>),
        /// Clear pin direction bits for port
        (0x2408 => dirclr_2: WriteOnly<u32>),
        /// Clear pin direction bits for port
        (0x240C => dirclr_3: WriteOnly<u32>),
        (0x2410 => _reserved9),
        /// Toggle pin direction bits for port
        (0x2480 => dirnot_0: WriteOnly<u32>),
        /// Toggle pin direction bits for port
        (0x2484 => dirnot_1: WriteOnly<u32>),
        /// Toggle pin direction bits for port
        (0x2488 => dirnot_2: WriteOnly<u32>),
        /// Toggle pin direction bits for port
        (0x248C => dirnot_3: WriteOnly<u32>),
        (0x2490 => @END),
    }
}
register_bitfields![u32,
DIR[0] [
    /// Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Sup
    DIRP OFFSET(0) NUMBITS(32) []
],
DIR[1] [
    /// Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Sup
    DIRP OFFSET(0) NUMBITS(32) []
],
DIR[2] [
    /// Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Sup
    DIRP OFFSET(0) NUMBITS(32) []
],
DIR[3] [
    /// Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Sup
    DIRP OFFSET(0) NUMBITS(32) []
],
MASK[0] [
    /// Controls which bits corresponding to PIOm_n are active in the MPORT register (bi
    MASKP OFFSET(0) NUMBITS(32) []
],
MASK[1] [
    /// Controls which bits corresponding to PIOm_n are active in the MPORT register (bi
    MASKP OFFSET(0) NUMBITS(32) []
],
MASK[2] [
    /// Controls which bits corresponding to PIOm_n are active in the MPORT register (bi
    MASKP OFFSET(0) NUMBITS(32) []
],
MASK[3] [
    /// Controls which bits corresponding to PIOm_n are active in the MPORT register (bi
    MASKP OFFSET(0) NUMBITS(32) []
],
PIN[0] [
    /// Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Su
    PORT OFFSET(0) NUMBITS(32) []
],
PIN[1] [
    /// Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Su
    PORT OFFSET(0) NUMBITS(32) []
],
PIN[2] [
    /// Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Su
    PORT OFFSET(0) NUMBITS(32) []
],
PIN[3] [
    /// Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Su
    PORT OFFSET(0) NUMBITS(32) []
],
MPIN[0] [
    /// Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    MPORTP OFFSET(0) NUMBITS(32) []
],
MPIN[1] [
    /// Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    MPORTP OFFSET(0) NUMBITS(32) []
],
MPIN[2] [
    /// Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    MPORTP OFFSET(0) NUMBITS(32) []
],
MPIN[3] [
    /// Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    MPORTP OFFSET(0) NUMBITS(32) []
],
SET[0] [
    /// Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins d
    SETP OFFSET(0) NUMBITS(32) []
],
SET[1] [
    /// Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins d
    SETP OFFSET(0) NUMBITS(32) []
],
SET[2] [
    /// Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins d
    SETP OFFSET(0) NUMBITS(32) []
],
SET[3] [
    /// Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins d
    SETP OFFSET(0) NUMBITS(32) []
],
CLR[0] [
    /// Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends
    CLRP OFFSET(0) NUMBITS(32) []
],
CLR[1] [
    /// Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends
    CLRP OFFSET(0) NUMBITS(32) []
],
CLR[2] [
    /// Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends
    CLRP OFFSET(0) NUMBITS(32) []
],
CLR[3] [
    /// Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends
    CLRP OFFSET(0) NUMBITS(32) []
],
NOT[0] [
    /// Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    NOTP OFFSET(0) NUMBITS(32) []
],
NOT[1] [
    /// Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    NOTP OFFSET(0) NUMBITS(32) []
],
NOT[2] [
    /// Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    NOTP OFFSET(0) NUMBITS(32) []
],
NOT[3] [
    /// Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    NOTP OFFSET(0) NUMBITS(32) []
],
DIRSET[0] [
    /// Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    DIRSETP OFFSET(0) NUMBITS(29) []
],
DIRSET[1] [
    /// Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    DIRSETP OFFSET(0) NUMBITS(29) []
],
DIRSET[2] [
    /// Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    DIRSETP OFFSET(0) NUMBITS(29) []
],
DIRSET[3] [
    /// Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depend
    DIRSETP OFFSET(0) NUMBITS(29) []
],
DIRCLR[0] [
    /// Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    DIRCLRP OFFSET(0) NUMBITS(29) []
],
DIRCLR[1] [
    /// Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    DIRCLRP OFFSET(0) NUMBITS(29) []
],
DIRCLR[2] [
    /// Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    DIRCLRP OFFSET(0) NUMBITS(29) []
],
DIRCLR[3] [
    /// Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depe
    DIRCLRP OFFSET(0) NUMBITS(29) []
],
DIRNOT[0] [
    /// Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins dep
    DIRNOTP OFFSET(0) NUMBITS(29) []
],
DIRNOT[1] [
    /// Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins dep
    DIRNOTP OFFSET(0) NUMBITS(29) []
],
DIRNOT[2] [
    /// Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins dep
    DIRNOTP OFFSET(0) NUMBITS(29) []
],
DIRNOT[3] [
    /// Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins dep
    DIRNOTP OFFSET(0) NUMBITS(29) []
]
];

const GPIO_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x5008C000 as *const GpioRegisters) };
