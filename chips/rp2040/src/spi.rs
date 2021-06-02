use kernel::common::registers::{
    self, register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly,
};
use kernel::common::StaticRef;
register_structs! {
    SpiRegisters {
        /// Control register 0, SSPCR0 on page 3-4
        (0x000 => sspcr0: ReadWrite<u32, SSPCR0::Register>),
        /// Control register 1, SSPCR1 on page 3-5
        (0x004 => sspcr1: ReadWrite<u32, SSPCR1::Register>),
        /// Data register, SSPDR on page 3-6
        (0x008 => sspdr: ReadWrite<u32>),
        /// Status register, SSPSR on page 3-7
        (0x00C => sspsr: ReadOnly<u32, SSPSR::Register>),              //READONLY??
        /// Clock prescale register, SSPCPSR on page 3-8
        (0x010 => sspcpsr: ReadWrite<u32>),
        /// Interrupt mask set or clear register, SSPIMSC on page 3-9
        (0x014 => sspimsc: ReadWrite<u32, SSPIMSC::Register>),
        /// Raw interrupt status register, SSPRIS on page 3-10
        (0x018 => sspris: ReadOnly<u32, SSPRIS::Register>),            //readonly
        /// Masked interrupt status register, SSPMIS on page 3-11
        (0x01C => sspmis: ReadOnly<u32, SSPMIS::Register>),            //readonly
        /// Interrupt clear register, SSPICR on page 3-11
        (0x020 => sspicr: ReadWrite<u32, SSPICR::Register>),
        /// DMA control register, SSPDMACR on page 3-12
        (0x024 => sspdmacr: ReadWrite<u32, SSPDMACR::Register>),
        (0x028 => _reserved0),
        /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
        (0xFE0 => sspperiphid0: ReadOnly<u32>, SSPPERIPHID0::Register>),                       //RO
        /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
        (0xFE4 => sspperiphid1: ReadOnly<u32, SSPPERIPHID1::Register>), //RO
        /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
        (0xFE8 => sspperiphid2: ReadOnly<u32, SSPPERIPHID2::Register>), //RO
        /// Peripheral identification registers, SSPPeriphID0-3 on page 3-13
        (0xFEC => sspperiphid3: ReadOnly<u32>, SSPPERIPHID3::Register>),                       //RO
        /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
        (0xFF0 => ssppcellid0: ReadOnly<u32>, SSPPCELLID0::Register>),                        //RO
        /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
        (0xFF4 => ssppcellid1: ReadOnly<u32>, SSPPCELLID1::Register>),                        //RO
        /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
        (0xFF8 => ssppcellid2: ReadOnly<u32>, SSPPCELLID2::Register>),                        //RO
        /// PrimeCell identification registers, SSPPCellID0-3 on page 3-16
        (0xFFC => ssppcellid3: ReadOnly<u32>, SSPPCELLID3::Register>),                       //RO
        (0x1000 => @END),
    }
}
register_bitfields![u32,
/// Control register 0
SSPCR0 [
    /// Serial clock rate.
    SCR OFFSET(8) NUMBITS(8) [],
    /// SSPCLKOUT phase
    SPH OFFSET(7) NUMBITS(1) [],
    /// SSPCLKOUT polarity
    SPO OFFSET(6) NUMBITS(1) [],
    /// Frame format
    FRF OFFSET(4) NUMBITS(2) [
        MOTOROLA_SPI = 0b00,
        TI_SINC_SERIAL = 0b01,
        NAT_MICROWIRE = 0b10,
        RESERVED = 0b11
    ],
    /// Data Size Select
    DSS OFFSET(0) NUMBITS(4) [
        RESERVED_0 = 0b0000,
        RESERVED_1 = 0b0001,
        RESERVED_2 = 0b0010,
        DATA_4_BIT = 0b0011,
        DATA_5_BIT = 0b0100,
        DATA_6_BIT = 0b0101,
        DATA_7_BIT = 0b0110,
        DATA_8_BIT = 0b0111,
        DATA_9_BIT = 0b1000,
        DATA_10_BIT = 0b1001,
        DATA_11_BIT = 0b1010,
        DATA_12_BIT = 0b1011,
        DATA_13_BIT = 0b1100,
        DATA_14_BIT = 0b1101,
        DATA_15_BIT = 0b1110,
        DATA_16_BIT = 0b1111
    ]
],
/// Control register 1
SSPCR1 [
    /// Slave-mode output disable
    SOD OFFSET(3) NUMBITS(1) [],
    /// Master or slave mode select
    MS OFFSET(2) NUMBITS(1) [],
    /// Synchronous serial port enable
    SSE OFFSET(1) NUMBITS(1) [],
    /// Loop back mode
    LBM OFFSET(0) NUMBITS(1) []
],
/// Data register
SSPDR [
    /// Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO.
    DATA OFFSET(0) NUMBITS(16) []
],
/// Status register
SSPSR [
    /// PrimeCell SSP busy flag
    BSY OFFSET(4) NUMBITS(1) [],
    /// Receive FIFO full, RO
    RFF OFFSET(3) NUMBITS(1) [],
    /// Receive FIFO not empty
    RNE OFFSET(2) NUMBITS(1) [],
    /// Transmit FIFO not full
    TNF OFFSET(1) NUMBITS(1) [],
    /// Transmit FIFO empty
    TFE OFFSET(0) NUMBITS(1) []
],
/// Clock prescale register
SSPCPSR [
    /// Clock prescale divisor
    CPSDVSR OFFSET(0) NUMBITS(8) []
],
/// Interrupt mask set or clear register
SSPIMSC [
    /// Transmit FIFO interrupt mask
    TXIM OFFSET(3) NUMBITS(1) [],
    /// Receive FIFO interrupt mask
    RXIM OFFSET(2) NUMBITS(1) [],
    /// Receive timeout interrupt mask
    RTIM OFFSET(1) NUMBITS(1) [],
    /// Receive overrun interrupt mask
    RORIM OFFSET(0) NUMBITS(1) []
],
/// Raw interrupt status register
SSPRIS [
    /// Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt
    TXRIS OFFSET(3) NUMBITS(1) [],
    /// Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt
    RXRIS OFFSET(2) NUMBITS(1) [],
    /// Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt
    RTRIS OFFSET(1) NUMBITS(1) [],
    /// Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt
    RORRIS OFFSET(0) NUMBITS(1) []
],
/// Masked interrupt status register
SSPMIS [
    /// Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR
    TXMIS OFFSET(3) NUMBITS(1) [],
    /// Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR i
    RXMIS OFFSET(2) NUMBITS(1) [],
    /// Gives the receive timeout masked interrupt state, after masking, of the SSPRTINT
    RTMIS OFFSET(1) NUMBITS(1) [],
    /// Gives the receive over run masked interrupt status, after masking, of the SSPROR
    RORMIS OFFSET(0) NUMBITS(1) []
],
/// Interrupt clear register
SSPICR [
    /// Clears the SSPRTINTR interrupt
    RTIC OFFSET(1) NUMBITS(1) [],
    /// Clears the SSPRORINTR interrupt
    RORIC OFFSET(0) NUMBITS(1) []
],
/// DMA control register
SSPDMACR [
    /// Transmit DMA Enable
    TXDMAE OFFSET(1) NUMBITS(1) [],
    /// Receive DMA Enable
    RXDMAE OFFSET(0) NUMBITS(1) []
],
/// Peripheral identification registers
SSPPERIPHID0 [
    /// These bits read back as 0x22
    PARTNUMBER0 OFFSET(0) NUMBITS(8) []
],
/// Peripheral identification registers
SSPPERIPHID1 [
    /// These bits read back as 0x1
    DESIGNER0 OFFSET(4) NUMBITS(4) [],
    /// These bits read back as 0x0
    PARTNUMBER1 OFFSET(0) NUMBITS(4) []
],
/// Peripheral identification registers
SSPPERIPHID2 [
    /// These bits return the peripheral revision
    REVISION OFFSET(4) NUMBITS(4) [],
    /// These bits read back as 0x4
    DESIGNER1 OFFSET(0) NUMBITS(4) []
],
/// Peripheral identification registers
SSPPERIPHID3 [
    /// These bits read back as 0x00
    CONFIGURATION OFFSET(0) NUMBITS(8) []
],
/// PrimeCell identification registers
SSPPCELLID0 [
    /// These bits read back as 0x0D
    SSPPCELLID0 OFFSET(0) NUMBITS(8) []
],
/// PrimeCell identification registers
SSPPCELLID1 [
    /// These bits read back as 0xF0
    SSPPCELLID1 OFFSET(0) NUMBITS(8) []
],
/// PrimeCell identification registers
SSPPCELLID2 [
    /// These bits read back as 0x05
    SSPPCELLID2 OFFSET(0) NUMBITS(8) []
],
/// PrimeCell identification registers
SSPPCELLID3 [
    /// These bits read back as 0xB1
    SSPPCELLID3 OFFSET(0) NUMBITS(8) []
]
];
const SPI0_BASE: StaticRef<Spi1Registers> =
    unsafe { StaticRef::new(0x4003C000 as *const SpiRegisters) };

const SPI1_BASE: StaticRef<Spi1Registers> =
    unsafe { StaticRef::new(0x40040000 as *const SpiRegisters) };
