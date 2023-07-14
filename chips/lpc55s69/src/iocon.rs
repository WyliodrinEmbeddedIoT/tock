// I/O Pin Configuration (IOCON)
// TODO: implement functions to initialize GPIO pins

use kernel::utilities::StaticRef;
use kernel::utilities::registers::{self, register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly};
    
register_structs! {
    /// I/O pin configuration (IOCON)
    IoconRegisters {
        /// Digital I/O control for port 0 pins PIO0_0
        (0x000 => pio0_0: ReadWrite<u32, PIO0_0::Register>),
        /// Digital I/O control for port 0 pins PIO0_1
        (0x004 => pio0_1: ReadWrite<u32, PIO0_1::Register>),
        /// Digital I/O control for port 0 pins PIO0_2
        (0x008 => pio0_2: ReadWrite<u32, PIO0_2::Register>),
        /// Digital I/O control for port 0 pins PIO0_3
        (0x00C => pio0_3: ReadWrite<u32, PIO0_3::Register>),
        /// Digital I/O control for port 0 pins PIO0_4
        (0x010 => pio0_4: ReadWrite<u32, PIO0_4::Register>),
        /// Digital I/O control for port 0 pins PIO0_5
        (0x014 => pio0_5: ReadWrite<u32, PIO0_5::Register>),
        /// Digital I/O control for port 0 pins PIO0_6
        (0x018 => pio0_6: ReadWrite<u32, PIO0_6::Register>),
        /// Digital I/O control for port 0 pins PIO0_7
        (0x01C => pio0_7: ReadWrite<u32, PIO0_7::Register>),
        /// Digital I/O control for port 0 pins PIO0_8
        (0x020 => pio0_8: ReadWrite<u32, PIO0_8::Register>),
        /// Digital I/O control for port 0 pins PIO0_9
        (0x024 => pio0_9: ReadWrite<u32, PIO0_9::Register>),
        /// Digital I/O control for port 0 pins PIO0_10
        (0x028 => pio0_10: ReadWrite<u32, PIO0_10::Register>),
        /// Digital I/O control for port 0 pins PIO0_11
        (0x02C => pio0_11: ReadWrite<u32, PIO0_11::Register>),
        /// Digital I/O control for port 0 pins PIO0_12
        (0x030 => pio0_12: ReadWrite<u32, PIO0_12::Register>),
        /// Digital I/O control for port 0 pins PIO0_13
        (0x034 => pio0_13: ReadWrite<u32, PIO0_13::Register>),
        /// Digital I/O control for port 0 pins PIO0_14
        (0x038 => pio0_14: ReadWrite<u32, PIO0_14::Register>),
        /// Digital I/O control for port 0 pins PIO0_15
        (0x03C => pio0_15: ReadWrite<u32, PIO0_15::Register>),
        /// Digital I/O control for port 0 pins PIO0_16
        (0x040 => pio0_16: ReadWrite<u32, PIO0_16::Register>),
        /// Digital I/O control for port 0 pins PIO0_17
        (0x044 => pio0_17: ReadWrite<u32, PIO0_17::Register>),
        /// Digital I/O control for port 0 pins PIO0_18
        (0x048 => pio0_18: ReadWrite<u32, PIO0_18::Register>),
        /// Digital I/O control for port 0 pins PIO0_19
        (0x04C => pio0_19: ReadWrite<u32, PIO0_19::Register>),
        /// Digital I/O control for port 0 pins PIO0_20
        (0x050 => pio0_20: ReadWrite<u32, PIO0_20::Register>),
        /// Digital I/O control for port 0 pins PIO0_21
        (0x054 => pio0_21: ReadWrite<u32, PIO0_21::Register>),
        /// Digital I/O control for port 0 pins PIO0_22
        (0x058 => pio0_22: ReadWrite<u32, PIO0_22::Register>),
        /// Digital I/O control for port 0 pins PIO0_23
        (0x05C => pio0_23: ReadWrite<u32, PIO0_23::Register>),
        /// Digital I/O control for port 0 pins PIO0_24
        (0x060 => pio0_24: ReadWrite<u32, PIO0_24::Register>),
        /// Digital I/O control for port 0 pins PIO0_25
        (0x064 => pio0_25: ReadWrite<u32, PIO0_25::Register>),
        /// Digital I/O control for port 0 pins PIO0_26
        (0x068 => pio0_26: ReadWrite<u32, PIO0_26::Register>),
        /// Digital I/O control for port 0 pins PIO0_27
        (0x06C => pio0_27: ReadWrite<u32, PIO0_27::Register>),
        /// Digital I/O control for port 0 pins PIO0_28
        (0x070 => pio0_28: ReadWrite<u32, PIO0_28::Register>),
        /// Digital I/O control for port 0 pins PIO0_29
        (0x074 => pio0_29: ReadWrite<u32, PIO0_29::Register>),
        /// Digital I/O control for port 0 pins PIO0_30
        (0x078 => pio0_30: ReadWrite<u32, PIO0_30::Register>),
        /// Digital I/O control for port 0 pins PIO0_31
        (0x07C => pio0_31: ReadWrite<u32, PIO0_31::Register>),
        /// Digital I/O control for port 1 pins PIO1_0
        (0x080 => pio1_0: ReadWrite<u32, PIO1_0::Register>),
        /// Digital I/O control for port 1 pins PIO1_1
        (0x084 => pio1_1: ReadWrite<u32, PIO1_1::Register>),
        /// Digital I/O control for port 1 pins PIO1_2
        (0x088 => pio1_2: ReadWrite<u32, PIO1_2::Register>),
        /// Digital I/O control for port 1 pins PIO1_3
        (0x08C => pio1_3: ReadWrite<u32, PIO1_3::Register>),
        /// Digital I/O control for port 1 pins PIO1_4
        (0x090 => pio1_4: ReadWrite<u32, PIO1_4::Register>),
        /// Digital I/O control for port 1 pins PIO1_5
        (0x094 => pio1_5: ReadWrite<u32, PIO1_5::Register>),
        /// Digital I/O control for port 1 pins PIO1_6
        (0x098 => pio1_6: ReadWrite<u32, PIO1_6::Register>),
        /// Digital I/O control for port 1 pins PIO1_7
        (0x09C => pio1_7: ReadWrite<u32, PIO1_7::Register>),
        /// Digital I/O control for port 1 pins PIO1_8
        (0x0A0 => pio1_8: ReadWrite<u32, PIO1_8::Register>),
        /// Digital I/O control for port 1 pins PIO1_9
        (0x0A4 => pio1_9: ReadWrite<u32, PIO1_9::Register>),
        /// Digital I/O control for port 1 pins PIO1_10
        (0x0A8 => pio1_10: ReadWrite<u32, PIO1_10::Register>),
        /// Digital I/O control for port 1 pins PIO1_11
        (0x0AC => pio1_11: ReadWrite<u32, PIO1_11::Register>),
        /// Digital I/O control for port 1 pins PIO1_12
        (0x0B0 => pio1_12: ReadWrite<u32, PIO1_12::Register>),
        /// Digital I/O control for port 1 pins PIO1_13
        (0x0B4 => pio1_13: ReadWrite<u32, PIO1_13::Register>),
        /// Digital I/O control for port 1 pins PIO1_14
        (0x0B8 => pio1_14: ReadWrite<u32, PIO1_14::Register>),
        /// Digital I/O control for port 1 pins PIO1_15
        (0x0BC => pio1_15: ReadWrite<u32, PIO1_15::Register>),
        /// Digital I/O control for port 1 pins PIO1_16
        (0x0C0 => pio1_16: ReadWrite<u32, PIO1_16::Register>),
        /// Digital I/O control for port 1 pins PIO1_17
        (0x0C4 => pio1_17: ReadWrite<u32, PIO1_17::Register>),
        /// Digital I/O control for port 1 pins PIO1_18
        (0x0C8 => pio1_18: ReadWrite<u32, PIO1_18::Register>),
        /// Digital I/O control for port 1 pins PIO1_19
        (0x0CC => pio1_19: ReadWrite<u32, PIO1_19::Register>),
        /// Digital I/O control for port 1 pins PIO1_20
        (0x0D0 => pio1_20: ReadWrite<u32, PIO1_20::Register>),
        /// Digital I/O control for port 1 pins PIO1_21
        (0x0D4 => pio1_21: ReadWrite<u32, PIO1_21::Register>),
        /// Digital I/O control for port 1 pins PIO1_22
        (0x0D8 => pio1_22: ReadWrite<u32, PIO1_22::Register>),
        /// Digital I/O control for port 1 pins PIO1_23
        (0x0DC => pio1_23: ReadWrite<u32, PIO1_23::Register>),
        /// Digital I/O control for port 1 pins PIO1_24
        (0x0E0 => pio1_24: ReadWrite<u32, PIO1_24::Register>),
        /// Digital I/O control for port 1 pins PIO1_25
        (0x0E4 => pio1_25: ReadWrite<u32, PIO1_25::Register>),
        /// Digital I/O control for port 1 pins PIO1_26
        (0x0E8 => pio1_26: ReadWrite<u32, PIO1_26::Register>),
        /// Digital I/O control for port 1 pins PIO1_27
        (0x0EC => pio1_27: ReadWrite<u32, PIO1_27::Register>),
        /// Digital I/O control for port 1 pins PIO1_28
        (0x0F0 => pio1_28: ReadWrite<u32, PIO1_28::Register>),
        /// Digital I/O control for port 1 pins PIO1_29
        (0x0F4 => pio1_29: ReadWrite<u32, PIO1_29::Register>),
        /// Digital I/O control for port 1 pins PIO1_30
        (0x0F8 => pio1_30: ReadWrite<u32, PIO1_30::Register>),
        /// Digital I/O control for port 1 pins PIO1_31
        (0x0FC => pio1_31: ReadWrite<u32, PIO1_31::Register>),
        /// Digital I/O control for port 2 pins PIO2_0
        (0x100 => pio2_0: ReadWrite<u32, PIO2_0::Register>),
        /// Digital I/O control for port 2 pins PIO2_1
        (0x104 => pio2_1: ReadWrite<u32, PIO2_1::Register>),
        /// Digital I/O control for port 2 pins PIO2_2
        (0x108 => pio2_2: ReadWrite<u32, PIO2_2::Register>),
        /// Digital I/O control for port 2 pins PIO2_3
        (0x10C => pio2_3: ReadWrite<u32, PIO2_3::Register>),
        /// Digital I/O control for port 2 pins PIO2_4
        (0x110 => pio2_4: ReadWrite<u32, PIO2_4::Register>),
        /// Digital I/O control for port 2 pins PIO2_5
        (0x114 => pio2_5: ReadWrite<u32, PIO2_5::Register>),
        /// Digital I/O control for port 2 pins PIO2_6
        (0x118 => pio2_6: ReadWrite<u32, PIO2_6::Register>),
        /// Digital I/O control for port 2 pins PIO2_7
        (0x11C => pio2_7: ReadWrite<u32, PIO2_7::Register>),
        /// Digital I/O control for port 2 pins PIO2_8
        (0x120 => pio2_8: ReadWrite<u32, PIO2_8::Register>),
        /// Digital I/O control for port 2 pins PIO2_9
        (0x124 => pio2_9: ReadWrite<u32, PIO2_9::Register>),
        /// Digital I/O control for port 2 pins PIO2_10
        (0x128 => pio2_10: ReadWrite<u32, PIO2_10::Register>),
        /// Digital I/O control for port 2 pins PIO2_11
        (0x12C => pio2_11: ReadWrite<u32, PIO2_11::Register>),
        /// Digital I/O control for port 2 pins PIO2_12
        (0x130 => pio2_12: ReadWrite<u32, PIO2_12::Register>),
        /// Digital I/O control for port 2 pins PIO2_13
        (0x134 => pio2_13: ReadWrite<u32, PIO2_13::Register>),
        /// Digital I/O control for port 2 pins PIO2_14
        (0x138 => pio2_14: ReadWrite<u32, PIO2_14::Register>),
        /// Digital I/O control for port 2 pins PIO2_15
        (0x13C => pio2_15: ReadWrite<u32, PIO2_15::Register>),
        /// Digital I/O control for port 2 pins PIO2_16
        (0x140 => pio2_16: ReadWrite<u32, PIO2_16::Register>),
        /// Digital I/O control for port 2 pins PIO2_17
        (0x144 => pio2_17: ReadWrite<u32, PIO2_17::Register>),
        /// Digital I/O control for port 2 pins PIO2_18
        (0x148 => pio2_18: ReadWrite<u32, PIO2_18::Register>),
        /// Digital I/O control for port 2 pins PIO2_19
        (0x14C => pio2_19: ReadWrite<u32, PIO2_19::Register>),
        /// Digital I/O control for port 2 pins PIO2_20
        (0x150 => pio2_20: ReadWrite<u32, PIO2_20::Register>),
        /// Digital I/O control for port 2 pins PIO2_21
        (0x154 => pio2_21: ReadWrite<u32, PIO2_21::Register>),
        /// Digital I/O control for port 2 pins PIO2_22
        (0x158 => pio2_22: ReadWrite<u32, PIO2_22::Register>),
        /// Digital I/O control for port 2 pins PIO2_23
        (0x15C => pio2_23: ReadWrite<u32, PIO2_23::Register>),
        /// Digital I/O control for port 2 pins PIO2_24
        (0x160 => pio2_24: ReadWrite<u32, PIO2_24::Register>),
        /// Digital I/O control for port 2 pins PIO2_25
        (0x164 => pio2_25: ReadWrite<u32, PIO2_25::Register>),
        /// Digital I/O control for port 2 pins PIO2_26
        (0x168 => pio2_26: ReadWrite<u32, PIO2_26::Register>),
        /// Digital I/O control for port 2 pins PIO2_27
        (0x16C => pio2_27: ReadWrite<u32, PIO2_27::Register>),
        /// Digital I/O control for port 2 pins PIO2_28
        (0x170 => pio2_28: ReadWrite<u32, PIO2_28::Register>),
        /// Digital I/O control for port 2 pins PIO2_29
        (0x174 => pio2_29: ReadWrite<u32, PIO2_29::Register>),
        /// Digital I/O control for port 2 pins PIO2_30
        (0x178 => pio2_30: ReadWrite<u32, PIO2_30::Register>),
        /// Digital I/O control for port 2 pins PIO2_31
        (0x17C => pio2_31: ReadWrite<u32, PIO2_31::Register>),
        /// Digital I/O control for port 3 pins PIO3_0
        (0x180 => pio3_0: ReadWrite<u32, PIO3_0::Register>),
        /// Digital I/O control for port 3 pins PIO3_1
        (0x184 => pio3_1: ReadWrite<u32, PIO3_1::Register>),
        /// Digital I/O control for port 3 pins PIO3_2
        (0x188 => pio3_2: ReadWrite<u32, PIO3_2::Register>),
        /// Digital I/O control for port 3 pins PIO3_3
        (0x18C => pio3_3: ReadWrite<u32, PIO3_3::Register>),
        /// Digital I/O control for port 3 pins PIO3_4
        (0x190 => pio3_4: ReadWrite<u32, PIO3_4::Register>),
        /// Digital I/O control for port 3 pins PIO3_5
        (0x194 => pio3_5: ReadWrite<u32, PIO3_5::Register>),
        /// Digital I/O control for port 3 pins PIO3_6
        (0x198 => pio3_6: ReadWrite<u32, PIO3_6::Register>),
        /// Digital I/O control for port 3 pins PIO3_7
        (0x19C => pio3_7: ReadWrite<u32, PIO3_7::Register>),
        /// Digital I/O control for port 3 pins PIO3_8
        (0x1A0 => pio3_8: ReadWrite<u32, PIO3_8::Register>),
        /// Digital I/O control for port 3 pins PIO3_9
        (0x1A4 => pio3_9: ReadWrite<u32, PIO3_9::Register>),
        /// Digital I/O control for port 3 pins PIO3_10
        (0x1A8 => pio3_10: ReadWrite<u32, PIO3_10::Register>),
        /// Digital I/O control for port 3 pins PIO3_11
        (0x1AC => pio3_11: ReadWrite<u32, PIO3_11::Register>),
        /// Digital I/O control for port 3 pins PIO3_12
        (0x1B0 => pio3_12: ReadWrite<u32, PIO3_12::Register>),
        /// Digital I/O control for port 3 pins PIO3_13
        (0x1B4 => pio3_13: ReadWrite<u32, PIO3_13::Register>),
        /// Digital I/O control for port 3 pins PIO3_14
        (0x1B8 => pio3_14: ReadWrite<u32, PIO3_14::Register>),
        /// Digital I/O control for port 3 pins PIO3_15
        (0x1BC => pio3_15: ReadWrite<u32, PIO3_15::Register>),
        /// Digital I/O control for port 3 pins PIO3_16
        (0x1C0 => pio3_16: ReadWrite<u32, PIO3_16::Register>),
        /// Digital I/O control for port 3 pins PIO3_17
        (0x1C4 => pio3_17: ReadWrite<u32, PIO3_17::Register>),
        /// Digital I/O control for port 3 pins PIO3_18
        (0x1C8 => pio3_18: ReadWrite<u32, PIO3_18::Register>),
        /// Digital I/O control for port 3 pins PIO3_19
        (0x1CC => pio3_19: ReadWrite<u32, PIO3_19::Register>),
        /// Digital I/O control for port 3 pins PIO3_20
        (0x1D0 => pio3_20: ReadWrite<u32, PIO3_20::Register>),
        /// Digital I/O control for port 3 pins PIO3_21
        (0x1D4 => pio3_21: ReadWrite<u32, PIO3_21::Register>),
        /// Digital I/O control for port 3 pins PIO3_22
        (0x1D8 => pio3_22: ReadWrite<u32, PIO3_22::Register>),
        /// Digital I/O control for port 3 pins PIO3_23
        (0x1DC => pio3_23: ReadWrite<u32, PIO3_23::Register>),
        /// Digital I/O control for port 3 pins PIO3_24
        (0x1E0 => pio3_24: ReadWrite<u32, PIO3_24::Register>),
        /// Digital I/O control for port 3 pins PIO3_25
        (0x1E4 => pio3_25: ReadWrite<u32, PIO3_25::Register>),
        /// Digital I/O control for port 3 pins PIO3_26
        (0x1E8 => pio3_26: ReadWrite<u32, PIO3_26::Register>),
        /// Digital I/O control for port 3 pins PIO3_27
        (0x1EC => pio3_27: ReadWrite<u32, PIO3_27::Register>),
        /// Digital I/O control for port 3 pins PIO3_28
        (0x1F0 => pio3_28: ReadWrite<u32, PIO3_28::Register>),
        /// Digital I/O control for port 3 pins PIO3_29
        (0x1F4 => pio3_29: ReadWrite<u32, PIO3_29::Register>),
        /// Digital I/O control for port 3 pins PIO3_30
        (0x1F8 => pio3_30: ReadWrite<u32, PIO3_30::Register>),
        /// Digital I/O control for port 3 pins PIO3_31
        (0x1FC => pio3_31: ReadWrite<u32, PIO3_31::Register>),
        /// Digital I/O control for port 4 pins PIO4_0
        (0x200 => pio4_0: ReadWrite<u32, PIO4_0::Register>),
        /// Digital I/O control for port 4 pins PIO4_1
        (0x204 => pio4_1: ReadWrite<u32, PIO4_1::Register>),
        /// Digital I/O control for port 4 pins PIO4_2
        (0x208 => pio4_2: ReadWrite<u32, PIO4_2::Register>),
        /// Digital I/O control for port 4 pins PIO4_3
        (0x20C => pio4_3: ReadWrite<u32, PIO4_3::Register>),
        /// Digital I/O control for port 4 pins PIO4_4
        (0x210 => pio4_4: ReadWrite<u32, PIO4_4::Register>),
        /// Digital I/O control for port 4 pins PIO4_5
        (0x214 => pio4_5: ReadWrite<u32, PIO4_5::Register>),
        /// Digital I/O control for port 4 pins PIO4_6
        (0x218 => pio4_6: ReadWrite<u32, PIO4_6::Register>),
        /// Digital I/O control for port 4 pins PIO4_7
        (0x21C => pio4_7: ReadWrite<u32, PIO4_7::Register>),
        /// Digital I/O control for port 4 pins PIO4_8
        (0x220 => pio4_8: ReadWrite<u32, PIO4_8::Register>),
        /// Digital I/O control for port 4 pins PIO4_9
        (0x224 => pio4_9: ReadWrite<u32, PIO4_9::Register>),
        /// Digital I/O control for port 4 pins PIO4_10
        (0x228 => pio4_10: ReadWrite<u32, PIO4_10::Register>),
        /// Digital I/O control for port 4 pins PIO4_11
        (0x22C => pio4_11: ReadWrite<u32, PIO4_11::Register>),
        /// Digital I/O control for port 4 pins PIO4_12
        (0x230 => pio4_12: ReadWrite<u32, PIO4_12::Register>),
        /// Digital I/O control for port 4 pins PIO4_13
        (0x234 => pio4_13: ReadWrite<u32, PIO4_13::Register>),
        /// Digital I/O control for port 4 pins PIO4_14
        (0x238 => pio4_14: ReadWrite<u32, PIO4_14::Register>),
        /// Digital I/O control for port 4 pins PIO4_15
        (0x23C => pio4_15: ReadWrite<u32, PIO4_15::Register>),
        /// Digital I/O control for port 4 pins PIO4_16
        (0x240 => pio4_16: ReadWrite<u32, PIO4_16::Register>),
        /// Digital I/O control for port 4 pins PIO4_17
        (0x244 => pio4_17: ReadWrite<u32, PIO4_17::Register>),
        /// Digital I/O control for port 4 pins PIO4_18
        (0x248 => pio4_18: ReadWrite<u32, PIO4_18::Register>),
        /// Digital I/O control for port 4 pins PIO4_19
        (0x24C => pio4_19: ReadWrite<u32, PIO4_19::Register>),
        /// Digital I/O control for port 4 pins PIO4_20
        (0x250 => pio4_20: ReadWrite<u32, PIO4_20::Register>),
        /// Digital I/O control for port 4 pins PIO4_21
        (0x254 => pio4_21: ReadWrite<u32, PIO4_21::Register>),
        /// Digital I/O control for port 4 pins PIO4_22
        (0x258 => pio4_22: ReadWrite<u32, PIO4_22::Register>),
        /// Digital I/O control for port 4 pins PIO4_23
        (0x25C => pio4_23: ReadWrite<u32, PIO4_23::Register>),
        /// Digital I/O control for port 4 pins PIO4_24
        (0x260 => pio4_24: ReadWrite<u32, PIO4_24::Register>),
        /// Digital I/O control for port 4 pins PIO4_25
        (0x264 => pio4_25: ReadWrite<u32, PIO4_25::Register>),
        /// Digital I/O control for port 4 pins PIO4_26
        (0x268 => pio4_26: ReadWrite<u32, PIO4_26::Register>),
        /// Digital I/O control for port 4 pins PIO4_27
        (0x26C => pio4_27: ReadWrite<u32, PIO4_27::Register>),
        /// Digital I/O control for port 4 pins PIO4_28
        (0x270 => pio4_28: ReadWrite<u32, PIO4_28::Register>),
        /// Digital I/O control for port 4 pins PIO4_29
        (0x274 => pio4_29: ReadWrite<u32, PIO4_29::Register>),
        /// Digital I/O control for port 4 pins PIO4_30
        (0x278 => pio4_30: ReadWrite<u32, PIO4_30::Register>),
        /// Digital I/O control for port 4 pins PIO4_31
        (0x27C => pio4_31: ReadWrite<u32, PIO4_31::Register>),
        /// Digital I/O control for port 5 pins PIO5_0
        (0x280 => pio5_0: ReadWrite<u32, PIO5_0::Register>),
        /// Digital I/O control for port 5 pins PIO5_1
        (0x284 => pio5_1: ReadWrite<u32, PIO5_1::Register>),
        /// Digital I/O control for port 5 pins PIO5_2
        (0x288 => pio5_2: ReadWrite<u32, PIO5_2::Register>),
        /// Digital I/O control for port 5 pins PIO5_3
        (0x28C => pio5_3: ReadWrite<u32, PIO5_3::Register>),
        /// Digital I/O control for port 5 pins PIO5_4
        (0x290 => pio5_4: ReadWrite<u32, PIO5_4::Register>),
        /// Digital I/O control for port 5 pins PIO5_5
        (0x294 => pio5_5: ReadWrite<u32, PIO5_5::Register>),
        /// Digital I/O control for port 5 pins PIO5_6
        (0x298 => pio5_6: ReadWrite<u32, PIO5_6::Register>),
        /// Digital I/O control for port 5 pins PIO5_7
        (0x29C => pio5_7: ReadWrite<u32, PIO5_7::Register>),
        /// Digital I/O control for port 5 pins PIO5_8
        (0x2A0 => pio5_8: ReadWrite<u32, PIO5_8::Register>),
        /// Digital I/O control for port 5 pins PIO5_9
        (0x2A4 => pio5_9: ReadWrite<u32, PIO5_9::Register>),
        /// Digital I/O control for port 5 pins PIO5_10
        (0x2A8 => pio5_10: ReadWrite<u32, PIO5_10::Register>),
        /// Digital I/O control for port 5 pins PIO5_11
        (0x2AC => pio5_11: ReadWrite<u32, PIO5_11::Register>),
        /// Digital I/O control for port 5 pins PIO5_12
        (0x2B0 => pio5_12: ReadWrite<u32, PIO5_12::Register>),
        /// Digital I/O control for port 5 pins PIO5_13
        (0x2B4 => pio5_13: ReadWrite<u32, PIO5_13::Register>),
        /// Digital I/O control for port 5 pins PIO5_14
        (0x2B8 => pio5_14: ReadWrite<u32, PIO5_14::Register>),
        /// Digital I/O control for port 5 pins PIO5_15
        (0x2BC => pio5_15: ReadWrite<u32, PIO5_15::Register>),
        /// Digital I/O control for port 5 pins PIO5_16
        (0x2C0 => pio5_16: ReadWrite<u32, PIO5_16::Register>),
        /// Digital I/O control for port 5 pins PIO5_17
        (0x2C4 => pio5_17: ReadWrite<u32, PIO5_17::Register>),
        /// Digital I/O control for port 5 pins PIO5_18
        (0x2C8 => pio5_18: ReadWrite<u32, PIO5_18::Register>),
        /// Digital I/O control for port 5 pins PIO5_19
        (0x2CC => pio5_19: ReadWrite<u32, PIO5_19::Register>),
        /// Digital I/O control for port 5 pins PIO5_20
        (0x2D0 => pio5_20: ReadWrite<u32, PIO5_20::Register>),
        /// Digital I/O control for port 5 pins PIO5_21
        (0x2D4 => pio5_21: ReadWrite<u32, PIO5_21::Register>),
        /// Digital I/O control for port 5 pins PIO5_22
        (0x2D8 => pio5_22: ReadWrite<u32, PIO5_22::Register>),
        /// Digital I/O control for port 5 pins PIO5_23
        (0x2DC => pio5_23: ReadWrite<u32, PIO5_23::Register>),
        /// Digital I/O control for port 5 pins PIO5_24
        (0x2E0 => pio5_24: ReadWrite<u32, PIO5_24::Register>),
        /// Digital I/O control for port 5 pins PIO5_25
        (0x2E4 => pio5_25: ReadWrite<u32, PIO5_25::Register>),
        /// Digital I/O control for port 5 pins PIO5_26
        (0x2E8 => pio5_26: ReadWrite<u32, PIO5_26::Register>),
        /// Digital I/O control for port 5 pins PIO5_27
        (0x2EC => pio5_27: ReadWrite<u32, PIO5_27::Register>),
        /// Digital I/O control for port 5 pins PIO5_28
        (0x2F0 => pio5_28: ReadWrite<u32, PIO5_28::Register>),
        /// Digital I/O control for port 5 pins PIO5_29
        (0x2F4 => pio5_29: ReadWrite<u32, PIO5_29::Register>),
        /// Digital I/O control for port 5 pins PIO5_30
        (0x2F8 => pio5_30: ReadWrite<u32, PIO5_30::Register>),
        /// Digital I/O control for port 5 pins PIO5_31
        (0x2FC => pio5_31: ReadWrite<u32, PIO5_31::Register>),
        (0x300 => @END),
    }
}
register_bitfields![u32,
PIO0_0 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_1 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_2 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_3 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_4 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_5 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_6 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_7 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_8 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_9 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_10 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_11 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_12 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_13 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ],
    /// Supply Selection bit.
    SSEL OFFSET(11) NUMBITS(1) [
        /// 3V3 Signaling in I2C Mode.
        _3V3SignalingInI2CMode = 0,
        /// 1V8 Signaling in I2C Mode.
        _1V8SignalingInI2CMode = 1
    ],
    /// Controls input glitch filter.
    FILTEROFF OFFSET(12) NUMBITS(1) [
        /// Filter enabled. Noise pulses below approximately 10 ns are filtered out.
        FilterEnabledNoisePulsesBelowApproximately10NsAreFilteredOut = 0,
        /// Filter disabled. No input filtering is done.
        FilterDisabledNoInputFilteringIsDone = 1
    ],
    /// Pull-up current source enable in IIC mode.
    ECS OFFSET(13) NUMBITS(1) [
        /// Enabled. Pull resistor is conencted.
        EnabledPullResistorIsConencted = 0,
        /// Disabled. IO is in open drain.
        DisabledIOIsInOpenDrain = 1
    ],
    /// Controls slew rate of I2C pad.
    I2CSLEW OFFSET(14) NUMBITS(1) [
        /// I2C mode.
        I2CMode = 0,
        /// GPIO mode.
        GPIOMode = 1
    ],
    /// Configures I2C features for standard mode, fast mode, and Fast Mode Plus operati
    I2CFILTER OFFSET(15) NUMBITS(1) [
        /// Enabled. I2C 50 ns glitch filter enabled.
        EnabledI2C50NsGlitchFilterEnabled = 0,
        /// Disabled. I2C 50 ns glitch filter disabled.
        DisabledI2C50NsGlitchFilterDisabled = 1
    ]
],
PIO0_14 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ],
    /// Supply Selection bit.
    SSEL OFFSET(11) NUMBITS(1) [
        /// 3V3 Signaling in I2C Mode.
        _3V3SignalingInI2CMode = 0,
        /// 1V8 Signaling in I2C Mode.
        _1V8SignalingInI2CMode = 1
    ],
    /// Controls input glitch filter.
    FILTEROFF OFFSET(12) NUMBITS(1) [
        /// Filter enabled. Noise pulses below approximately 10 ns are filtered out.
        FilterEnabledNoisePulsesBelowApproximately10NsAreFilteredOut = 0,
        /// Filter disabled. No input filtering is done.
        FilterDisabledNoInputFilteringIsDone = 1
    ],
    /// Pull-up current source enable in IIC mode.
    ECS OFFSET(13) NUMBITS(1) [
        /// Enabled. Pull resistor is conencted.
        EnabledPullResistorIsConencted = 0,
        /// Disabled. IO is in open drain.
        DisabledIOIsInOpenDrain = 1
    ],
    /// Controls slew rate of I2C pad.
    I2CSLEW OFFSET(14) NUMBITS(1) [
        /// I2C mode.
        I2CMode = 0,
        /// GPIO mode.
        GPIOMode = 1
    ],
    /// Configures I2C features for standard mode, fast mode, and Fast Mode Plus operati
    I2CFILTER OFFSET(15) NUMBITS(1) [
        /// Enabled. I2C 50 ns glitch filter enabled.
        EnabledI2C50NsGlitchFilterEnabled = 0,
        /// Disabled. I2C 50 ns glitch filter disabled.
        DisabledI2C50NsGlitchFilterDisabled = 1
    ]
],
PIO0_15 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_16 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_17 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_18 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_19 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_20 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_21 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_22 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_23 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO0_24 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_25 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_26 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_27 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_28 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_29 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_30 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO0_31 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO1_0 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO1_1 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_2 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_3 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_4 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_5 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_6 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_7 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_8 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO1_9 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO1_10 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_11 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_12 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_13 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_14 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO1_15 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_16 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_17 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_18 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_19 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO1_20 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_21 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_22 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_23 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_24 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_25 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_26 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_27 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_28 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_29 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_30 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO1_31 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_0 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_1 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_2 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_3 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_4 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_5 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_6 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_7 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_8 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_9 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_10 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_11 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_12 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_13 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_14 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_15 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_16 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_17 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_18 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_19 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_20 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_21 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_22 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_23 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_24 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_25 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_26 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ],
    /// Analog switch input control. Usable only if DIGIMODE = 0b0
    ASW OFFSET(10) NUMBITS(1) [
        /// Analog switch is open.
        AnalogSwitchIsOpen = 0,
        /// Analog switch is closed.
        AnalogSwitchIsClosed = 1
    ]
],
PIO2_27 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_28 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_29 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_30 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO2_31 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_0 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_1 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_2 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_3 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_4 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_5 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_6 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_7 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_8 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_9 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_10 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_11 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_12 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_13 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_14 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_15 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_16 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_17 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_18 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_19 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_20 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_21 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_22 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_23 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_24 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_25 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_26 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_27 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_28 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_29 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_30 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO3_31 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_0 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_1 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_2 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_3 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_4 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_5 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_6 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_7 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_8 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_9 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_10 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_11 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_12 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_13 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_14 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_15 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_16 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_17 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_18 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_19 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_20 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_21 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_22 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_23 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_24 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_25 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_26 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_27 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_28 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_29 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_30 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO4_31 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_0 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_1 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_2 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_3 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_4 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_5 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_6 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_7 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_8 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_9 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_10 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_11 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_12 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_13 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_14 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_15 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_16 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_17 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_18 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_19 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_20 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_21 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_22 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_23 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_24 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_25 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_26 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_27 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_28 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_29 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_30 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
],
PIO5_31 [
    /// Selects pin function.
    FUNC OFFSET(0) NUMBITS(4) [
        /// Alternative connection 0.
        AlternativeConnection0 = 0,
        /// Alternative connection 1.
        AlternativeConnection1 = 1,
        /// Alternative connection 2.
        AlternativeConnection2 = 2,
        /// Alternative connection 3.
        AlternativeConnection3 = 3,
        /// Alternative connection 4.
        AlternativeConnection4 = 4,
        /// Alternative connection 5.
        AlternativeConnection5 = 5,
        /// Alternative connection 6.
        AlternativeConnection6 = 6,
        /// Alternative connection 7.
        AlternativeConnection7 = 7
    ],
    /// Selects function mode (on-chip pull-up/pull-down resistor control).
    MODE OFFSET(4) NUMBITS(2) [
        /// Inactive. Inactive (no pull-down/pull-up resistor enabled).
        InactiveInactiveNoPullDownPullUpResistorEnabled = 0,
        /// Pull-down. Pull-down resistor enabled.
        PullDownPullDownResistorEnabled = 1,
        /// Pull-up. Pull-up resistor enabled.
        PullUpPullUpResistorEnabled = 2,
        /// Repeater. Repeater mode.
        RepeaterRepeaterMode = 3
    ],
    /// Driver slew rate.
    SLEW OFFSET(6) NUMBITS(1) [
        /// Standard mode, output slew rate control is enabled. More outputs can be switched
        STANDARD = 0,
        /// Fast mode, slew rate control is disabled. Refer to the appropriate specific devi
        FAST = 1
    ],
    /// Input polarity.
    INVERT OFFSET(7) NUMBITS(1) [
        /// Disabled. Input function is not inverted.
        DisabledInputFunctionIsNotInverted = 0,
        /// Enabled. Input is function inverted.
        EnabledInputIsFunctionInverted = 1
    ],
    /// Select Digital mode.
    DIGIMODE OFFSET(8) NUMBITS(1) [
        /// Analog mode, digital input is disabled.
        AnalogModeDigitalInputIsDisabled = 0,
        /// Digital mode, digital input is enabled.
        DigitalModeDigitalInputIsEnabled = 1
    ],
    /// Controls open-drain mode.
    OD OFFSET(9) NUMBITS(1) [
        /// Normal. Normal push-pull output
        NormalNormalPushPullOutput = 0,
        /// Open-drain. Simulated open-drain output (high drive disabled).
        OpenDrainSimulatedOpenDrainOutputHighDriveDisabled = 1
    ]
]
];
const IOCON_BASE: StaticRef<IoconRegisters> =
    unsafe { StaticRef::new(0x50001000 as *const IoconRegisters) };
