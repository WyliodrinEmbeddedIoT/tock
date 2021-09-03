
use kernel::common::StaticRef;
use kernel::common::registers::{self, register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly};
use kernel::utilities::StaticRef;

register_structs! {
    /// Simple PWM
    PwmRegisters {
        /// Control and status register
        (0x000 => ch0_csr: ReadWrite<u32, CH0_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x004 => ch0_div: ReadWrite<u32, CH0_DIV::Register>),
        /// Direct access to the PWM counter
        (0x008 => ch0_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x00C => ch0_cc: ReadWrite<u32, CH0_CC::Register>),
        /// Counter wrap value
        (0x010 => ch0_top: ReadWrite<u32>),
        /// Control and status register
        (0x014 => ch1_csr: ReadWrite<u32, CH1_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x018 => ch1_div: ReadWrite<u32, CH1_DIV::Register>),
        /// Direct access to the PWM counter
        (0x01C => ch1_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x020 => ch1_cc: ReadWrite<u32, CH1_CC::Register>),
        /// Counter wrap value
        (0x024 => ch1_top: ReadWrite<u32>),
        /// Control and status register
        (0x028 => ch2_csr: ReadWrite<u32, CH2_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x02C => ch2_div: ReadWrite<u32, CH2_DIV::Register>),
        /// Direct access to the PWM counter
        (0x030 => ch2_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x034 => ch2_cc: ReadWrite<u32, CH2_CC::Register>),
        /// Counter wrap value
        (0x038 => ch2_top: ReadWrite<u32>),
        /// Control and status register
        (0x03C => ch3_csr: ReadWrite<u32, CH3_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x040 => ch3_div: ReadWrite<u32, CH3_DIV::Register>),
        /// Direct access to the PWM counter
        (0x044 => ch3_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x048 => ch3_cc: ReadWrite<u32, CH3_CC::Register>),
        /// Counter wrap value
        (0x04C => ch3_top: ReadWrite<u32>),
        /// Control and status register
        (0x050 => ch4_csr: ReadWrite<u32, CH4_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x054 => ch4_div: ReadWrite<u32, CH4_DIV::Register>),
        /// Direct access to the PWM counter
        (0x058 => ch4_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x05C => ch4_cc: ReadWrite<u32, CH4_CC::Register>),
        /// Counter wrap value
        (0x060 => ch4_top: ReadWrite<u32>),
        /// Control and status register
        (0x064 => ch5_csr: ReadWrite<u32, CH5_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x068 => ch5_div: ReadWrite<u32, CH5_DIV::Register>),
        /// Direct access to the PWM counter
        (0x06C => ch5_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x070 => ch5_cc: ReadWrite<u32, CH5_CC::Register>),
        /// Counter wrap value
        (0x074 => ch5_top: ReadWrite<u32>),
        /// Control and status register
        (0x078 => ch6_csr: ReadWrite<u32, CH6_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x07C => ch6_div: ReadWrite<u32, CH6_DIV::Register>),
        /// Direct access to the PWM counter
        (0x080 => ch6_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x084 => ch6_cc: ReadWrite<u32, CH6_CC::Register>),
        /// Counter wrap value
        (0x088 => ch6_top: ReadWrite<u32>),
        /// Control and status register
        (0x08C => ch7_csr: ReadWrite<u32, CH7_CSR::Register>),
        /// INT and FRAC form a fixed-point fractional number.\n
        /// Counting rate is system clock frequency divided by this number.\n
        /// Fractional division uses simple 1st-order sigma-delta.
        (0x090 => ch7_div: ReadWrite<u32, CH7_DIV::Register>),
        /// Direct access to the PWM counter
        (0x094 => ch7_ctr: ReadWrite<u32>),
        /// Counter compare values
        (0x098 => ch7_cc: ReadWrite<u32, CH7_CC::Register>),
        /// Counter wrap value
        (0x09C => ch7_top: ReadWrite<u32>),
        /// This register aliases the CSR_EN bits for all channels.\n
        /// Writing to this register allows multiple channels to be enabled\n
        /// or disabled simultaneously, so they can run in perfect sync.\n
        /// For each channel, there is only one physical EN register bit,\n
        /// which can be accessed through here or CHx_CSR.
        (0x0A0 => en: ReadWrite<u32, EN::Register>),
        /// Raw Interrupts
        (0x0A4 => intr: ReadWrite<u32, INTR::Register>),
        /// Interrupt Enable
        (0x0A8 => inte: ReadWrite<u32, INTE::Register>),
        /// Interrupt Force
        (0x0AC => intf: ReadWrite<u32, INTF::Register>),
        /// Interrupt status after masking & forcing
        (0x0B0 => ints: ReadWrite<u32, INTS::Register>),
        (0x0B4 => @END),
    }
}
register_bitfields![u32,
CH0_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH0_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH0_CTR [
    
    CH0_CTR OFFSET(0) NUMBITS(16) []
],
CH0_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH0_TOP [
    
    CH0_TOP OFFSET(0) NUMBITS(16) []
],
CH1_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH1_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH1_CTR [
    
    CH1_CTR OFFSET(0) NUMBITS(16) []
],
CH1_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH1_TOP [
    
    CH1_TOP OFFSET(0) NUMBITS(16) []
],
CH2_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH2_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH2_CTR [
    
    CH2_CTR OFFSET(0) NUMBITS(16) []
],
CH2_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH2_TOP [
    
    CH2_TOP OFFSET(0) NUMBITS(16) []
],
CH3_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH3_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH3_CTR [
    
    CH3_CTR OFFSET(0) NUMBITS(16) []
],
CH3_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH3_TOP [
    
    CH3_TOP OFFSET(0) NUMBITS(16) []
],
CH4_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH4_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH4_CTR [
    
    CH4_CTR OFFSET(0) NUMBITS(16) []
],
CH4_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH4_TOP [
    
    CH4_TOP OFFSET(0) NUMBITS(16) []
],
CH5_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH5_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH5_CTR [
    
    CH5_CTR OFFSET(0) NUMBITS(16) []
],
CH5_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH5_TOP [
    
    CH5_TOP OFFSET(0) NUMBITS(16) []
],
CH6_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH6_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH6_CTR [
    
    CH6_CTR OFFSET(0) NUMBITS(16) []
],
CH6_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH6_TOP [
    
    CH6_TOP OFFSET(0) NUMBITS(16) []
],
CH7_CSR [
    /// Advance the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    /// at less than full speed (div_int + div_frac / 16 > 1)
    PH_ADV OFFSET(7) NUMBITS(1) [],
    /// Retard the phase of the counter by 1 count, while it is running.\n
    /// Self-clearing. Write a 1, and poll until low. Counter must be ru
    PH_RET OFFSET(6) NUMBITS(1) [],
    
    DIVMODE OFFSET(4) NUMBITS(2) [
        /// Free-running counting at rate dictated by fractional divider
        FreeRunningCountingAtRateDictatedByFractionalDivider = 0,
        /// Fractional divider operation is gated by the PWM B pin.
        FractionalDividerOperationIsGatedByThePWMBPin = 1,
        /// Counter advances with each rising edge of the PWM B pin.
        CounterAdvancesWithEachRisingEdgeOfThePWMBPin = 2,
        /// Counter advances with each falling edge of the PWM B pin.
        CounterAdvancesWithEachFallingEdgeOfThePWMBPin = 3
    ],
    /// Invert output B
    B_INV OFFSET(3) NUMBITS(1) [],
    /// Invert output A
    A_INV OFFSET(2) NUMBITS(1) [],
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    PH_CORRECT OFFSET(1) NUMBITS(1) [],
    /// Enable the PWM channel.
    EN OFFSET(0) NUMBITS(1) []
],
CH7_DIV [
    
    INT OFFSET(4) NUMBITS(8) [],
    
    FRAC OFFSET(0) NUMBITS(4) []
],
CH7_CTR [
    
    CH7_CTR OFFSET(0) NUMBITS(16) []
],
CH7_CC [
    
    B OFFSET(16) NUMBITS(16) [],
    
    A OFFSET(0) NUMBITS(16) []
],
CH7_TOP [
    
    CH7_TOP OFFSET(0) NUMBITS(16) []
],
EN [
    
    CH7 OFFSET(7) NUMBITS(1) [],
    
    CH6 OFFSET(6) NUMBITS(1) [],
    
    CH5 OFFSET(5) NUMBITS(1) [],
    
    CH4 OFFSET(4) NUMBITS(1) [],
    
    CH3 OFFSET(3) NUMBITS(1) [],
    
    CH2 OFFSET(2) NUMBITS(1) [],
    
    CH1 OFFSET(1) NUMBITS(1) [],
    
    CH0 OFFSET(0) NUMBITS(1) []
],
INTR [
    
    CH7 OFFSET(7) NUMBITS(1) [],
    
    CH6 OFFSET(6) NUMBITS(1) [],
    
    CH5 OFFSET(5) NUMBITS(1) [],
    
    CH4 OFFSET(4) NUMBITS(1) [],
    
    CH3 OFFSET(3) NUMBITS(1) [],
    
    CH2 OFFSET(2) NUMBITS(1) [],
    
    CH1 OFFSET(1) NUMBITS(1) [],
    
    CH0 OFFSET(0) NUMBITS(1) []
],
INTE [
    
    CH7 OFFSET(7) NUMBITS(1) [],
    
    CH6 OFFSET(6) NUMBITS(1) [],
    
    CH5 OFFSET(5) NUMBITS(1) [],
    
    CH4 OFFSET(4) NUMBITS(1) [],
    
    CH3 OFFSET(3) NUMBITS(1) [],
    
    CH2 OFFSET(2) NUMBITS(1) [],
    
    CH1 OFFSET(1) NUMBITS(1) [],
    
    CH0 OFFSET(0) NUMBITS(1) []
],
INTF [
    
    CH7 OFFSET(7) NUMBITS(1) [],
    
    CH6 OFFSET(6) NUMBITS(1) [],
    
    CH5 OFFSET(5) NUMBITS(1) [],
    
    CH4 OFFSET(4) NUMBITS(1) [],
    
    CH3 OFFSET(3) NUMBITS(1) [],
    
    CH2 OFFSET(2) NUMBITS(1) [],
    
    CH1 OFFSET(1) NUMBITS(1) [],
    
    CH0 OFFSET(0) NUMBITS(1) []
],
INTS [
    
    CH7 OFFSET(7) NUMBITS(1) [],
    
    CH6 OFFSET(6) NUMBITS(1) [],
    
    CH5 OFFSET(5) NUMBITS(1) [],
    
    CH4 OFFSET(4) NUMBITS(1) [],
    
    CH3 OFFSET(3) NUMBITS(1) [],
    
    CH2 OFFSET(2) NUMBITS(1) [],
    
    CH1 OFFSET(1) NUMBITS(1) [],
    
    CH0 OFFSET(0) NUMBITS(1) []
]
];
const PWM_BASE: StaticRef<PwmRegisters> =
    unsafe { StaticRef::new(0x40050000 as *const PwmRegisters) };
zg 


pub struct Pwm {
    registers: StaticRef<PwmRegisters>,
}

impl Pwm {
    pub const fn new() -> Pwm {
        Pwm {
            registers: PWM_BASE,
        }
    }
}

