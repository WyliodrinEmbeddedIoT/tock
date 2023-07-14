// SVD errors are not yet adressed:
//
// Offset Mismatch at register PRESETCTRLX0 (256 != 260.0)
// Offset Mismatch at register PRESETCTRLX1 (260 != 264.0)
// Offset Mismatch at register PRESETCTRLX2 (264 != 268.0)
// Offset Mismatch at register AHBCLKCTRLX0 (512 != 516.0)
// Offset Mismatch at register AHBCLKCTRLX1 (516 != 520.0)
// Offset Mismatch at register AHBCLKCTRLX2 (520 != 524.0)
// Offset Mismatch at register SYSTICKCLKSELX0 (608 != 612.0)
// Offset Mismatch at register SYSTICKCLKSELX1 (612 != 616.0)
// Offset Mismatch at register CTIMERCLKSELX0 (620 != 624.0)
// Offset Mismatch at register CTIMERCLKSELX1 (624 != 628.0)
// Offset Mismatch at register CTIMERCLKSELX2 (628 != 632.0)
// Offset Mismatch at register CTIMERCLKSELX3 (632 != 636.0)
// Offset Mismatch at register CTIMERCLKSELX4 (636 != 640.0)
// Offset Mismatch at register FCCLKSELX0 (688 != 692.0)
// Offset Mismatch at register FCCLKSELX1 (692 != 696.0)
// Offset Mismatch at register FCCLKSELX2 (696 != 700.0)
// Offset Mismatch at register FCCLKSELX3 (700 != 704.0)
// Offset Mismatch at register FCCLKSELX4 (704 != 708.0)
// Offset Mismatch at register FCCLKSELX5 (708 != 712.0)
// Offset Mismatch at register FCCLKSELX6 (712 != 716.0)
// Offset Mismatch at register FCCLKSELX7 (716 != 720.0)
// Offset Mismatch at register FLEXFRGXCTRL0 (800 != 804.0)
// Offset Mismatch at register FLEXFRGXCTRL1 (804 != 808.0)
// Offset Mismatch at register FLEXFRGXCTRL2 (808 != 812.0)
// Offset Mismatch at register FLEXFRGXCTRL3 (812 != 816.0)
// Offset Mismatch at register FLEXFRGXCTRL4 (816 != 820.0)
// Offset Mismatch at register FLEXFRGXCTRL5 (820 != 824.0)
// Offset Mismatch at register FLEXFRGXCTRL6 (824 != 828.0)
// Offset Mismatch at register FLEXFRGXCTRL7 (828 != 832.0)

use kernel::utilities::StaticRef;
use kernel::utilities::registers::{self, register_bitfields, register_structs, ReadOnly, ReadWrite, WriteOnly};
    
register_structs! {
    /// SYSCON
    SysconRegisters {
        /// Memory Remap control register
        (0x000 => memoryremap: ReadWrite<u32>),
        (0x004 => _reserved0),
        /// AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest
        (0x010 => ahbmatprio: ReadWrite<u32, AHBMATPRIO::Register>),
        (0x014 => _reserved1),
        /// Buffering of write accesses on Synchronous System configuration APB interface -
        (0x020 => bufferingahb2vpb0: ReadWrite<u32, BUFFERINGAHB2VPB0::Register>),
        /// Buffering of write accesses on Synchronous System configuration APB interface -
        (0x024 => bufferingahb2vpb1: ReadWrite<u32, BUFFERINGAHB2VPB1::Register>),
        (0x028 => _reserved2),
        /// System tick calibration for secure part of CPU0
        (0x038 => cpu0stckcal: ReadWrite<u32, CPU0STCKCAL::Register>),
        /// System tick calibration for non-secure part of CPU0
        (0x03C => cpu0nstckcal: ReadWrite<u32, CPU0NSTCKCAL::Register>),
        /// System tick calibration for CPU1
        (0x040 => cpu1tckcal: ReadWrite<u32, CPU1TCKCAL::Register>),
        /// CPU IRQ latency control - SPARE REGISTER - NOT USED
        (0x044 => irqlat: ReadWrite<u32, IRQLAT::Register>),
        /// NMI Source Select
        (0x048 => nmisrc: ReadWrite<u32, NMISRC::Register>),
        (0x04C => _reserved3),
        /// Peripheral reset control 0
        (0x100 => presetctrl0: ReadWrite<u32, PRESETCTRL0::Register>),
        /// Peripheral reset control 1
        (0x104 => presetctrl1: ReadWrite<u32, PRESETCTRL1::Register>),
        /// Peripheral reset control 2
        (0x108 => presetctrl2: ReadWrite<u32, PRESETCTRL2::Register>),
        (0x10C => _reserved4),
        /// Peripheral reset control set register
        (0x120 => presetctrlset_0: ReadWrite<u32>),
        /// Peripheral reset control set register
        (0x124 => presetctrlset_1: ReadWrite<u32>),
        /// Peripheral reset control set register
        (0x128 => presetctrlset_2: ReadWrite<u32>),
        (0x12C => _reserved5),
        /// Peripheral reset contro clearl register
        (0x140 => presetctrlclr_0: ReadWrite<u32>),
        /// Peripheral reset contro clearl register
        (0x144 => presetctrlclr_1: ReadWrite<u32>),
        /// Peripheral reset contro clearl register
        (0x148 => presetctrlclr_2: ReadWrite<u32>),
        (0x14C => _reserved6),
        /// generate a software_reset
        (0x160 => swr_reset: WriteOnly<u32>),
        (0x164 => _reserved7),
        /// AHB Clock control 0
        (0x200 => ahbclkctrl0: ReadWrite<u32, AHBCLKCTRL0::Register>),
        /// AHB Clock control 1
        (0x204 => ahbclkctrl1: ReadWrite<u32, AHBCLKCTRL1::Register>),
        /// AHB Clock control 2
        (0x208 => ahbclkctrl2: ReadWrite<u32, AHBCLKCTRL2::Register>),
        (0x20C => _reserved8),
        /// Peripheral reset control register
        (0x220 => ahbclkctrlset_0: ReadWrite<u32>),
        /// Peripheral reset control register
        (0x224 => ahbclkctrlset_1: ReadWrite<u32>),
        /// Peripheral reset control register
        (0x228 => ahbclkctrlset_2: ReadWrite<u32>),
        (0x22C => _reserved9),
        /// Peripheral reset control register
        (0x240 => ahbclkctrlclr_0: ReadWrite<u32>),
        /// Peripheral reset control register
        (0x244 => ahbclkctrlclr_1: ReadWrite<u32>),
        /// Peripheral reset control register
        (0x248 => ahbclkctrlclr_2: ReadWrite<u32>),
        (0x24C => _reserved10),
        /// System Tick Timer for CPU0 source select
        (0x260 => systickclksel0: ReadWrite<u32>),
        /// System Tick Timer for CPU1 source select
        (0x264 => systickclksel1: ReadWrite<u32>),
        /// Trace clock source select
        (0x268 => traceclksel: ReadWrite<u32>),
        /// CTimer 0 clock source select
        (0x26C => ctimerclksel0: ReadWrite<u32>),
        /// CTimer 1 clock source select
        (0x270 => ctimerclksel1: ReadWrite<u32>),
        /// CTimer 2 clock source select
        (0x274 => ctimerclksel2: ReadWrite<u32>),
        /// CTimer 3 clock source select
        (0x278 => ctimerclksel3: ReadWrite<u32>),
        /// CTimer 4 clock source select
        (0x27C => ctimerclksel4: ReadWrite<u32>),
        /// Main clock A source select
        (0x280 => mainclksela: ReadWrite<u32>),
        /// Main clock source select
        (0x284 => mainclkselb: ReadWrite<u32>),
        /// CLKOUT clock source select
        (0x288 => clkoutsel: ReadWrite<u32>),
        (0x28C => _reserved11),
        /// PLL0 clock source select
        (0x290 => pll0clksel: ReadWrite<u32>),
        /// PLL1 clock source select
        (0x294 => pll1clksel: ReadWrite<u32>),
        (0x298 => _reserved12),
        /// ADC clock source select
        (0x2A4 => adcclksel: ReadWrite<u32>),
        /// FS USB clock source select
        (0x2A8 => usb0clksel: ReadWrite<u32>),
        /// HS USB clock source select - NOT USED
        (0x2AC => usb1clksel: ReadWrite<u32>),
        /// Flexcomm Interface 0 clock source select for Fractional Rate Divider
        (0x2B0 => fcclksel0: ReadWrite<u32>),
        /// Flexcomm Interface 1 clock source select for Fractional Rate Divider
        (0x2B4 => fcclksel1: ReadWrite<u32>),
        /// Flexcomm Interface 2 clock source select for Fractional Rate Divider
        (0x2B8 => fcclksel2: ReadWrite<u32>),
        /// Flexcomm Interface 3 clock source select for Fractional Rate Divider
        (0x2BC => fcclksel3: ReadWrite<u32>),
        /// Flexcomm Interface 4 clock source select for Fractional Rate Divider
        (0x2C0 => fcclksel4: ReadWrite<u32>),
        /// Flexcomm Interface 5 clock source select for Fractional Rate Divider
        (0x2C4 => fcclksel5: ReadWrite<u32>),
        /// Flexcomm Interface 6 clock source select for Fractional Rate Divider
        (0x2C8 => fcclksel6: ReadWrite<u32>),
        /// Flexcomm Interface 7 clock source select for Fractional Rate Divider
        (0x2CC => fcclksel7: ReadWrite<u32>),
        /// HS LSPI clock source select
        (0x2D0 => hslspiclksel: ReadWrite<u32>),
        (0x2D4 => _reserved13),
        /// MCLK clock source select
        (0x2E0 => mclkclksel: ReadWrite<u32>),
        (0x2E4 => _reserved14),
        /// Flash rclk clock source select - FOR INTERNAL USE ONLY
        (0x2EC => flashrclksel: ReadWrite<u32>),
        /// SCTimer/PWM clock source select
        (0x2F0 => sctclksel: ReadWrite<u32>),
        /// Capacitive Touch clock source select
        (0x2F4 => captclksel: ReadWrite<u32>),
        /// SDIO clock source select
        (0x2F8 => sdioclksel: ReadWrite<u32>),
        (0x2FC => _reserved15),
        /// System Tick Timer divider for CPU0
        (0x300 => systickclkdiv0: ReadWrite<u32, SYSTICKCLKDIV0::Register>),
        /// System Tick Timer divider for CPU1
        (0x304 => systickclkdiv1: ReadWrite<u32, SYSTICKCLKDIV1::Register>),
        /// TRACE clock divider
        (0x308 => traceclkdiv: ReadWrite<u32, TRACECLKDIV::Register>),
        (0x30C => _reserved16),
        /// Fractional rate divider for flexcomm 0
        (0x320 => flexfrg0ctrl: ReadWrite<u32, FLEXFRG0CTRL::Register>),
        /// Fractional rate divider for flexcomm 1
        (0x324 => flexfrg1ctrl: ReadWrite<u32, FLEXFRG1CTRL::Register>),
        /// Fractional rate divider for flexcomm 2
        (0x328 => flexfrg2ctrl: ReadWrite<u32, FLEXFRG2CTRL::Register>),
        /// Fractional rate divider for flexcomm 3
        (0x32C => flexfrg3ctrl: ReadWrite<u32, FLEXFRG3CTRL::Register>),
        /// Fractional rate divider for flexcomm 4
        (0x330 => flexfrg4ctrl: ReadWrite<u32, FLEXFRG4CTRL::Register>),
        /// Fractional rate divider for flexcomm 5
        (0x334 => flexfrg5ctrl: ReadWrite<u32, FLEXFRG5CTRL::Register>),
        /// Fractional rate divider for flexcomm 6
        (0x338 => flexfrg6ctrl: ReadWrite<u32, FLEXFRG6CTRL::Register>),
        /// Fractional rate divider for flexcomm 7
        (0x33C => flexfrg7ctrl: ReadWrite<u32, FLEXFRG7CTRL::Register>),
        (0x340 => _reserved17),
        /// System clock divider
        (0x380 => ahbclkdiv: ReadWrite<u32, AHBCLKDIV::Register>),
        /// CLKOUT clock divider
        (0x384 => clkoutdiv: ReadWrite<u32, CLKOUTDIV::Register>),
        /// FRO 96MHz clock divider
        (0x388 => fro96mdiv: ReadWrite<u32, FRO96MDIV::Register>),
        /// WDT clock divider
        (0x38C => wdtclkdiv: ReadWrite<u32, WDTCLKDIV::Register>),
        (0x390 => _reserved18),
        /// ADC clock divider
        (0x394 => adcclkdiv: ReadWrite<u32, ADCCLKDIV::Register>),
        /// USB0 Clock divider
        (0x398 => usb0clkdiv: ReadWrite<u32, USB0CLKDIV::Register>),
        /// USB1 Clock divider - NOT USED
        (0x39C => usb1clkdiv: ReadWrite<u32, USB1CLKDIV::Register>),
        (0x3A0 => _reserved19),
        /// I2S MCLK clock divider
        (0x3AC => mclkdiv: ReadWrite<u32, MCLKDIV::Register>),
        (0x3B0 => _reserved20),
        /// SCT/PWM clock divider
        (0x3B4 => sctclkdiv: ReadWrite<u32, SCTCLKDIV::Register>),
        /// Capacitive Touch clock divider
        (0x3B8 => captclkdiv: ReadWrite<u32, CAPTCLKDIV::Register>),
        /// SDIO clock divider
        (0x3BC => sdioclkdiv: ReadWrite<u32, SDIOCLKDIV::Register>),
        (0x3C0 => _reserved21),
        /// PLL0 clock divider
        (0x3C4 => pll0clkdiv: ReadWrite<u32, PLL0CLKDIV::Register>),
        (0x3C8 => _reserved22),
        /// Control clock configuration registers access (like xxxDIV, xxxSEL)
        (0x3FC => clockgenupdatelockout: ReadWrite<u32>),
        /// FMC configuration register - INTERNAL USE ONLY
        (0x400 => fmccr: ReadWrite<u32, FMCCR::Register>),
        /// ROM access configuration register - INTERNAL USE ONLY
        (0x404 => romcr: ReadWrite<u32>),
        (0x408 => _reserved23),
        /// USB0 clock control
        (0x40C => usb0clkctrl: ReadWrite<u32, USB0CLKCTRL::Register>),
        /// USB0 clock status
        (0x410 => usb0clkstat: ReadWrite<u32, USB0CLKSTAT::Register>),
        /// EZH interrupt hijack - INTERNAL USE ONLY
        (0x414 => ezhint: ReadWrite<u32>),
        (0x418 => _reserved24),
        /// FMCflush control
        (0x41C => fmcflush: WriteOnly<u32>),
        /// MCLK control
        (0x420 => mclkio: ReadWrite<u32>),
        /// USB1 clock control
        (0x424 => usb1clkctrl: ReadWrite<u32, USB1CLKCTRL::Register>),
        /// USB1 clock status
        (0x428 => usb1clkstat: ReadWrite<u32, USB1CLKSTAT::Register>),
        (0x42C => _reserved25),
        /// Flash Banks control
        (0x450 => flashbankenable: ReadWrite<u32, FLASHBANKENABLE::Register>),
        (0x454 => _reserved26),
        /// SDIO CCLKIN phase and delay control
        (0x460 => sdioclkctrl: ReadWrite<u32, SDIOCLKCTRL::Register>),
        (0x464 => _reserved27),
        /// PLL1 550m control
        (0x560 => pll1ctrl: ReadWrite<u32, PLL1CTRL::Register>),
        /// PLL1 550m status
        (0x564 => pll1stat: ReadOnly<u32>),
        /// PLL1 550m N divider
        (0x568 => pll1ndec: ReadWrite<u32, PLL1NDEC::Register>),
        /// PLL1 550m M divider
        (0x56C => pll1mdec: ReadWrite<u32, PLL1MDEC::Register>),
        /// PLL1 550m P divider
        (0x570 => pll1pdec: ReadWrite<u32, PLL1PDEC::Register>),
        /// PLL1 550m functional test control
        (0x574 => pll1_testctrl: ReadWrite<u32, PLL1_TESTCTRL::Register>),
        /// PLL1 550m functional test status
        (0x578 => pll1_teststat: ReadWrite<u32, PLL1_TESTSTAT::Register>),
        (0x57C => _reserved28),
        /// PLL0 550m control
        (0x580 => pll0ctrl: ReadWrite<u32, PLL0CTRL::Register>),
        /// PLL0 550m status
        (0x584 => pll0stat: ReadOnly<u32>),
        /// PLL0 550m N divider
        (0x588 => pll0ndec: ReadWrite<u32, PLL0NDEC::Register>),
        /// PLL0 550m P divider
        (0x58C => pll0pdec: ReadWrite<u32, PLL0PDEC::Register>),
        /// PLL0 Spread Spectrum Wrapper control register 0
        (0x590 => pll0sscg0: ReadWrite<u32, PLL0SSCG0::Register>),
        /// PLL0 Spread Spectrum Wrapper control register 1
        (0x594 => pll0sscg1: ReadWrite<u32, PLL0SSCG1::Register>),
        /// PLL0 550m functional test control - INTERNAL USE ONLY
        (0x598 => pll0_testctrl: ReadWrite<u32, PLL0_TESTCTRL::Register>),
        /// PLL0 550m functional test status
        (0x59C => pll0_teststat: ReadWrite<u32, PLL0_TESTSTAT::Register>),
        (0x5A0 => _reserved29),
        /// eFUSE controller clock enable
        (0x5CC => efuseclkctrl: ReadWrite<u32>),
        (0x5D0 => _reserved30),
        /// Start logic wake-up enable register
        (0x680 => starter0: ReadWrite<u32, STARTER0::Register>),
        /// Start logic wake-up enable register
        (0x684 => starter1: ReadWrite<u32, STARTER1::Register>),
        (0x688 => _reserved31),
        /// Set bits in STARTER
        (0x6A0 => starterset0: WriteOnly<u32, STARTERSET0::Register>),
        /// Set bits in STARTER
        (0x6A4 => starterset1: WriteOnly<u32, STARTERSET1::Register>),
        (0x6A8 => _reserved32),
        /// Clear bits in STARTER
        (0x6C0 => starterclr0: WriteOnly<u32, STARTERCLR0::Register>),
        /// Clear bits in STARTER
        (0x6C4 => starterclr1: WriteOnly<u32, STARTERCLR1::Register>),
        (0x6C8 => _reserved33),
        /// Functional retention control register
        (0x704 => funcretentionctrl: ReadWrite<u32, FUNCRETENTIONCTRL::Register>),
        (0x708 => _reserved34),
        /// Override some powerdown control signals (for debug purposes)
        (0x70C => powerdownsafety: ReadWrite<u32>),
        /// main clock is enable after MAINCLKSAFETY cycle - FOR INTERNAL USE ONLY
        (0x710 => mainclksafety: ReadWrite<u32>),
        (0x714 => _reserved35),
        /// Hardware Sleep control
        (0x780 => hardwaresleep: ReadWrite<u32, HARDWARESLEEP::Register>),
        (0x784 => _reserved36),
        /// CPU Control for multiple processors
        (0x800 => cpuctrl: ReadWrite<u32, CPUCTRL::Register>),
        /// Coprocessor Boot Address
        (0x804 => cpboot: ReadWrite<u32>),
        /// Coprocessor Stack Address
        (0x808 => cpstack: ReadWrite<u32>),
        /// CPU Status
        (0x80C => cpstat: ReadWrite<u32, CPSTAT::Register>),
        (0x810 => _reserved37),
        /// no description available
        (0x900 => dice_reg0: ReadWrite<u32>),
        /// no description available
        (0x904 => dice_reg1: ReadWrite<u32>),
        /// no description available
        (0x908 => dice_reg2: ReadWrite<u32>),
        /// no description available
        (0x90C => dice_reg3: ReadWrite<u32>),
        /// no description available
        (0x910 => dice_reg4: ReadWrite<u32>),
        /// no description available
        (0x914 => dice_reg5: ReadWrite<u32>),
        /// no description available
        (0x918 => dice_reg6: ReadWrite<u32>),
        /// no description available
        (0x91C => dice_reg7: ReadWrite<u32>),
        (0x920 => _reserved38),
        /// Various system clock controls : Flash clock (48 MHz) control, clocks to Frequenc
        (0xA18 => clock_ctrl: ReadWrite<u32, CLOCK_CTRL::Register>),
        (0xA1C => _reserved39),
        /// Comparator Interrupt control
        (0xB10 => comp_int_ctrl: ReadWrite<u32, COMP_INT_CTRL::Register>),
        /// Comparator Interrupt status
        (0xB14 => comp_int_status: ReadWrite<u32, COMP_INT_STATUS::Register>),
        (0xB18 => _reserved40),
        /// Control automatic clock gating
        (0xE04 => autoclkgateoverride: ReadWrite<u32, AUTOCLKGATEOVERRIDE::Register>),
        /// Enable bypass of the first stage of synchonization inside GPIO_INT module
        (0xE08 => gpiopsync: ReadWrite<u32>),
        (0xE0C => _reserved41),
        /// Invert Main clock -- FOR INTERNAL USE
        (0xE20 => invertmainclk: ReadWrite<u32>),
        (0xE24 => _reserved42),
        /// Control write access to security registers -- FOR INTERNAl USE ONLY
        (0xFA0 => debug_lock_en: ReadWrite<u32>),
        /// Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control -- FOR INTE
        (0xFA4 => debug_features: ReadWrite<u32, DEBUG_FEATURES::Register>),
        /// Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE r
        (0xFA8 => debug_features_dp: ReadWrite<u32, DEBUG_FEATURES_DP::Register>),
        (0xFAC => _reserved43),
        /// Security code to allow test (Design for Testability) access -- FOR INTERNAl USE
        (0xFB0 => codesecurityprottest: WriteOnly<u32>),
        /// Security code to allow CPU0 (CM33) Debug Access Port (DAP) -- FOR INTERNAl USE O
        (0xFB4 => codesecurityprotcpu0: WriteOnly<u32>),
        /// Security code to allow CPU1 (Micro CM33) Debug Access Port (DAP) -- FOR INTERNAl
        (0xFB8 => codesecurityprotcpu1: WriteOnly<u32>),
        /// block quiddikey/PUF all index. -- FOR INTERNAL USE ONLY
        (0xFBC => key_block: WriteOnly<u32>),
        /// Debug authentication scratch registers -- FOR INTERNAL USE ONLY
        (0xFC0 => debug_auth_scratch: ReadWrite<u32>),
        (0xFC4 => _reserved44),
        /// CPUs configuration register
        (0xFD4 => cpucfg: ReadWrite<u32>),
        (0xFD8 => _reserved45),
        /// Flash size configuration -- FOR INTERNAL USE ONLY
        (0xFE0 => flashsizecfg: ReadWrite<u32, FLASHSIZECFG::Register>),
        /// Disable write access to FLASHSIZECFG, SRAMSIZECFG, CPUCFG. -- FOR INTERNAL USE O
        (0xFE4 => configlockout: ReadWrite<u32, CONFIGLOCKOUT::Register>),
        /// RAM size -- FOR INTERNAL USE ONLY
        (0xFE8 => ramsizecfg: ReadWrite<u32, RAMSIZECFG::Register>),
        /// peripheral enable configuration -- FOR INTERNAL USE ONLY
        (0xFEC => periphencfg: ReadWrite<u32, PERIPHENCFG::Register>),
        (0xFF0 => _reserved46),
        /// no description available
        (0xFF8 => device_id0: ReadOnly<u32, DEVICE_ID0::Register>),
        /// Chip revision ID and Number
        (0xFFC => dieid: ReadOnly<u32, DIEID::Register>),
        (0x1000 => @END),
    }
}
register_bitfields![u32,
MEMORYREMAP [
    /// Select the location of the vector table :.
    MAP OFFSET(0) NUMBITS(2) [
        /// Vector Table in ROM.
        VectorTableInROM = 0,
        /// Vector Table in RAM.
        VectorTableInRAM = 1,
        /// Vector Table in Flash.
        VectorTableInFlash = 2
    ]
],
AHBMATPRIO [
    /// Teal C-AHB bus.
    PRI_TEAL_CBUS OFFSET(0) NUMBITS(2) [],
    /// Teal S-AHB bus.
    PRI_TEAL_SBUS OFFSET(2) NUMBITS(2) [],
    /// Micro Teal C-AHB bus.
    PRI_UTEAL_CBUS OFFSET(4) NUMBITS(2) [],
    /// Micro Teal S-AHB bus.
    PRI_UTEAL_SBUS OFFSET(6) NUMBITS(2) [],
    /// USB-FS.
    PRI_USB_FS OFFSET(8) NUMBITS(2) [],
    /// DMA0 controller priority.
    PRI_SDMA0 OFFSET(10) NUMBITS(2) [],
    /// EZH B data bus.
    PRI_EZH_B_D OFFSET(12) NUMBITS(2) [],
    /// EZH B instruction bus.
    PRI_EZH_B_I OFFSET(14) NUMBITS(2) [],
    /// SDIO.
    PRI_SDIO OFFSET(16) NUMBITS(2) [],
    /// PQ (Teal HW Accelerator).
    PRI_PQ OFFSET(18) NUMBITS(2) [],
    /// SHA-2.
    PRI_SHA2 OFFSET(20) NUMBITS(2) [],
    /// Master 11 for Dummy.
    PRI_MAT11 OFFSET(22) NUMBITS(2) [],
    /// DMA1 controller priority.
    PRI_SDMA1 OFFSET(24) NUMBITS(2) []
],
BUFFERINGAHB2VPB0 [
    /// Enable buffering of write accesses on Synchronous System configuration APB inter
    SYSCON OFFSET(0) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on IO Configuration APB interface:.
    IOCON OFFSET(1) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on GPIO Global Interrupt APB interface:.
    GPIOGLOBALINT0 OFFSET(2) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on GPIO Global Interrupt APB interface:.
    GPIOGLOBALINT1 OFFSET(3) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on GPIO Int APB interface:.
    GPIOINT OFFSET(4) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on secure GPIO Int APB interface:.
    SECGPIOINT OFFSET(5) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Peripheral Input Mux APB interface:.
    PMUX OFFSET(6) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Counter/Timer0 APB interface:.
    CT32B0 OFFSET(8) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Counter/Timer1 APB interface:.
    CT32B1 OFFSET(9) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Watchdog Timer APB interface:.
    WWDT OFFSET(12) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write ac.
    MRT OFFSET(13) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on micro Tick APB interface:.
    UTICK OFFSET(14) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on analog control APB interface:.
    ANACTRL OFFSET(19) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on eFUSE controller APB interface:.
    OTPC OFFSET(21) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on capacitive touch control APB interface:.
    CAPTOUCH OFFSET(26) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on SCTIUP APB interface:.
    SCTIUP OFFSET(27) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on EZH APB interface:.
    EZH OFFSET(29) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on prob IS (sync) APB interface:.
    PROBSYNC OFFSET(30) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on prob IS (XVC) APB interface:.
    PROBXVC OFFSET(31) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ]
],
BUFFERINGAHB2VPB1 [
    /// Enable buffering of write accesses on Power Management Controller APB interface:
    PMC OFFSET(0) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on PVT monitor APB interface:.
    PVT OFFSET(2) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on system control APB interface:.
    SYSCTL OFFSET(3) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Counter/Timer2 APB interface:.
    CT32B2 OFFSET(8) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Counter/Timer3 APB interface:.
    CT32B3 OFFSET(9) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Counter/Timer4 APB interface:.
    CT32B4 OFFSET(10) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on RTC APB interface:.
    RTC OFFSET(12) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on OS event timer APB interface:.
    OSEVENT OFFSET(13) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Flash Controller APB interface:.
    FLASHCTRL OFFSET(20) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Prince APB interface:.
    PRINCE OFFSET(21) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on USB high speed interface:.
    USB_HS OFFSET(24) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Random Number Generator APB interface:.
    RNG OFFSET(26) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on PUFF interface:.
    PUFF OFFSET(27) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on OS timer1 interface:.
    OSTIMER1 OFFSET(28) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on PLU LUT interface:.
    PLU_LUT OFFSET(29) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ],
    /// Enable buffering of write accesses on Code Patch Unit APB interface:.
    CODEPATCH OFFSET(30) NUMBITS(1) [
        /// Disable buffering.
        DisableBuffering = 0,
        /// Enable buffering.
        EnableBuffering = 1
    ]
],
CPU0STCKCAL [
    /// System tick timer calibration value.
    CAL OFFSET(0) NUMBITS(24) [],
    /// Initial value for the Systick timer.
    SKEW OFFSET(24) NUMBITS(1) [],
    /// Initial value for the Systick timer.
    NOREF OFFSET(25) NUMBITS(1) []
],
CPU0NSTCKCAL [
    /// System tick timer calibration value.
    CAL OFFSET(0) NUMBITS(24) [],
    /// Initial value for the Systick timer.
    SKEW OFFSET(24) NUMBITS(1) [],
    /// Initial value for the Systick timer.
    NOREF OFFSET(25) NUMBITS(1) []
],
CPU1TCKCAL [
    /// System tick timer calibration value.
    CAL OFFSET(0) NUMBITS(24) [],
    /// Initial value for the Systick timer.
    SKEW OFFSET(24) NUMBITS(1) [],
    /// Initial value for the Systick timer.
    NOREF OFFSET(25) NUMBITS(1) []
],
IRQLAT [
    /// CPU0 interrupt latency control.
    CPU0_IRQLAT OFFSET(0) NUMBITS(8) [],
    /// CPU1 interrupt latency control.
    CPU1_IRQLAT OFFSET(8) NUMBITS(8) []
],
NMISRC [
    /// The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) fo
    IRQCPU0 OFFSET(0) NUMBITS(6) [],
    /// The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) fo
    IRQCPU1 OFFSET(8) NUMBITS(6) [],
    /// Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected
    NMIENCPU1 OFFSET(30) NUMBITS(1) [],
    /// Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected
    NMIENCPU0 OFFSET(31) NUMBITS(1) []
],
PRESETCTRL0 [
    /// ROM reset control.
    ROM_RST OFFSET(1) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// SRAM Controller 1 reset control.
    SRAM_CTRL1_RST OFFSET(3) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// SRAM Controller 2 reset control.
    SRAM_CTRL2_RST OFFSET(4) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// SRAM Controller 3 reset control.
    SRAM_CTRL3_RST OFFSET(5) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// SRAM Controller 4 reset control.
    SRAM_CTRL4_RST OFFSET(6) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Flash controller reset control.
    FLASH_RST OFFSET(7) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FMC controller reset control.
    FMC_RST OFFSET(8) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Input Mux 0 reset control.
    MUX0_RST OFFSET(11) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// I/O controller reset control.
    IOCON_RST OFFSET(13) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO0 reset control.
    GPIO0_RST OFFSET(14) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO1 reset control.
    GPIO1_RST OFFSET(15) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO2 reset control.
    GPIO2_RST OFFSET(16) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO3 reset control.
    GPIO3_RST OFFSET(17) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Pin interrupt (PINT) reset control.
    PINT_RST OFFSET(18) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Group interrupt (GINT) reset control.
    GINT_RST OFFSET(19) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// DMA0 reset control.
    DMA0_RST OFFSET(20) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// CRCGEN reset control.
    CRCGEN_RST OFFSET(21) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Watchdog Timer reset control.
    WWDT_RST OFFSET(22) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Real Time Clock (RTC) reset control.
    RTC_RST OFFSET(23) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Inter CPU communication Mailbox reset control.
    MAILBOX_RST OFFSET(26) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// ADC reset control.
    ADC_RST OFFSET(27) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ]
],
PRESETCTRLX0 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
PRESETCTRL1 [
    /// MRT reset control.
    MRT_RST OFFSET(0) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// OS Timer 0 reset control.
    OSTIMER0_RST OFFSET(1) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// SCT0 reset control.
    SCT0_RST OFFSET(2) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// SCTIPU reset control.
    SCTIPU_RST OFFSET(6) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// UTICK0 reset control.
    UTICK0_RST OFFSET(10) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC0 reset control.
    FC0_RST OFFSET(11) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC1 reset control.
    FC1_RST OFFSET(12) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC2 reset control.
    FC2_RST OFFSET(13) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC3 reset control.
    FC3_RST OFFSET(14) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC4 reset control.
    FC4_RST OFFSET(15) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC5 reset control.
    FC5_RST OFFSET(16) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC6 reset control.
    FC6_RST OFFSET(17) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// FC7 reset control.
    FC7_RST OFFSET(18) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Timer 2 reset control.
    TIMER2_RST OFFSET(22) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// USB0 DEV reset control.
    USB0_DEV_RST OFFSET(25) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Timer 0 reset control.
    TIMER0_RST OFFSET(26) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Timer 1 reset control.
    TIMER1_RST OFFSET(27) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// PVT reset control.
    PVT_RST OFFSET(28) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// EZH a reset control.
    EZHA_RST OFFSET(30) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// EZH b reset control.
    EZHB_RST OFFSET(31) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ]
],
PRESETCTRLX1 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
PRESETCTRL2 [
    /// DMA1 reset control.
    DMA1_RST OFFSET(1) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Comparator reset control.
    COMP_RST OFFSET(2) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// SDIO reset control.
    SDIO_RST OFFSET(3) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// USB1 Host reset control.
    USB1_HOST_RST OFFSET(4) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// USB1 dev reset control.
    USB1_DEV_RST OFFSET(5) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// USB1 RAM reset control.
    USB1_RAM_RST OFFSET(6) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// USB1 PHY reset control.
    USB1_PHY_RST OFFSET(7) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Frequency meter reset control.
    FREQME_RST OFFSET(8) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO4 reset control.
    GPIO4_RST OFFSET(9) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO5 reset control.
    GPIO5_RST OFFSET(10) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// OTP reset control.
    OTP_RST OFFSET(12) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// RNG reset control.
    RNG_RST OFFSET(13) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Peripheral Input Mux 1 reset control.
    MUX1_RST OFFSET(14) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// USB0 Host Master reset control.
    USB0_HOSTM_RST OFFSET(16) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// USB0 Host Slave reset control.
    USB0_HOSTS_RST OFFSET(17) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// HASH0 reset control.
    HASH0_RST OFFSET(18) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Power Quad reset control.
    PQ_RST OFFSET(19) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// PLU LUT reset control.
    PLULUT_RST OFFSET(20) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Timer 3 reset control.
    TIMER3_RST OFFSET(21) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Timer 4 reset control.
    TIMER4_RST OFFSET(22) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// PUFF reset control reset control.
    PUFF_RST OFFSET(23) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// Casper reset control.
    CASPER_RST OFFSET(24) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// CAPT0 reset control.
    CAPT0_RST OFFSET(25) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// OS timer 1 reset control.
    OS_TIMER1_RST OFFSET(26) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// analog control reset control.
    ANALOG_CTRL_RST OFFSET(27) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// HS LSPI reset control.
    HS_LSPI_RST OFFSET(28) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO secure reset control.
    GPIO_SEC_RST OFFSET(29) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ],
    /// GPIO secure int reset control.
    GPIO_SEC_INT_RST OFFSET(30) NUMBITS(1) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Bloc is reset.
        BlocIsReset = 1
    ]
],
PRESETCTRLX2 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
SWR_RESET [
    /// Write 0x5A00_0001 to generate a software_reset.
    SWR_RESET OFFSET(0) NUMBITS(32) [
        /// Bloc is not reset.
        BlocIsNotReset = 0,
        /// Generate a software reset.
        GenerateASoftwareReset = 1509949441
    ]
],
AHBCLKCTRL0 [
    /// Enables the clock for the ROM.
    ROM OFFSET(1) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the SRAM Controller 1.
    SRAM_CTRL1 OFFSET(3) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the SRAM Controller 2.
    SRAM_CTRL2 OFFSET(4) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the SRAM Controller 3.
    SRAM_CTRL3 OFFSET(5) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the SRAM Controller 4.
    SRAM_CTRL4 OFFSET(6) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Flash controller.
    FLASH OFFSET(7) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FMC controller.
    FMC OFFSET(8) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Input Mux 0.
    MUX0 OFFSET(11) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the I/O controller.
    IOCON OFFSET(13) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO0.
    GPIO0 OFFSET(14) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO1.
    GPIO1 OFFSET(15) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO2.
    GPIO2 OFFSET(16) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO3.
    GPIO3 OFFSET(17) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Pin interrupt (PINT).
    PINT OFFSET(18) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Group interrupt (GINT).
    GINT OFFSET(19) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the DMA0.
    DMA0 OFFSET(20) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the CRCGEN.
    CRCGEN OFFSET(21) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Watchdog Timer.
    WWDT OFFSET(22) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Real Time Clock (RTC).
    RTC OFFSET(23) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Inter CPU communication Mailbox.
    MAILBOX OFFSET(26) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the ADC.
    ADC OFFSET(27) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ]
],
AHBCLKCTRLX0 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRL1 [
    /// Enables the clock for the MRT.
    MRT OFFSET(0) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the OS Timer 0.
    OSTIMER0 OFFSET(1) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the SCT0.
    SCT0 OFFSET(2) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the SCTIPU.
    SCTIPU OFFSET(6) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the UTICK0.
    UTICK0 OFFSET(10) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC0.
    FC0 OFFSET(11) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC1.
    FC1 OFFSET(12) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC2.
    FC2 OFFSET(13) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC3.
    FC3 OFFSET(14) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC4.
    FC4 OFFSET(15) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC5.
    FC5 OFFSET(16) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC6.
    FC6 OFFSET(17) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the FC7.
    FC7 OFFSET(18) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Timer 2.
    TIMER2 OFFSET(22) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the USB0 DEV.
    USB0_DEV OFFSET(25) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Timer 0.
    TIMER0 OFFSET(26) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Timer 1.
    TIMER1 OFFSET(27) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the PVT.
    PVT OFFSET(28) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the EZH a.
    EZHA OFFSET(30) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the EZH b.
    EZHB OFFSET(31) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ]
],
AHBCLKCTRLX1 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRL2 [
    /// Enables the clock for the DMA1.
    DMA1 OFFSET(1) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Comparator.
    COMP OFFSET(2) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the SDIO.
    SDIO OFFSET(3) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the USB1 Host.
    USB1_HOST OFFSET(4) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the USB1 dev.
    USB1_DEV OFFSET(5) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the USB1 RAM.
    USB1_RAM OFFSET(6) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the USB1 PHY.
    USB1_PHY OFFSET(7) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Frequency meter.
    FREQME OFFSET(8) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO4.
    GPIO4 OFFSET(9) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO5.
    GPIO5 OFFSET(10) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the OTP.
    OTP OFFSET(12) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the RNG.
    RNG OFFSET(13) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Peripheral Input Mux 1.
    MUX1 OFFSET(14) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the USB0 Host Master.
    USB0_HOSTM OFFSET(16) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the USB0 Host Slave.
    USB0_HOSTS OFFSET(17) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the HASH0.
    HASH0 OFFSET(18) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Power Quad.
    PQ OFFSET(19) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the PLU LUT.
    PLULUT OFFSET(20) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Timer 3.
    TIMER3 OFFSET(21) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Timer 4.
    TIMER4 OFFSET(22) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the PUFF reset control.
    PUFF OFFSET(23) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the Casper.
    CASPER OFFSET(24) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the CAPT0.
    CAPT0 OFFSET(25) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the OS timer 1.
    OS_TIMER1 OFFSET(26) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the analog control.
    ANALOG_CTRL OFFSET(27) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the HS LSPI.
    HS_LSPI OFFSET(28) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO secure.
    GPIO_SEC OFFSET(29) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ],
    /// Enables the clock for the GPIO secure int.
    GPIO_SEC_INT OFFSET(30) NUMBITS(1) [
        /// Disable Clock.
        DisableClock = 0,
        /// Enable Clock.
        EnableClock = 1
    ]
],
AHBCLKCTRLX2 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
SYSTICKCLKSEL0 [
    /// System Tick Timer for CPU0 source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// System Tick 0 divided clock.
        SystemTick0DividedClock = 0,
        /// FRO 1MHz clock.
        FRO1MHzClock = 1,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 2,
        /// No clock.
        NoClock = 3
    ]
],
SYSTICKCLKSELX0 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
SYSTICKCLKSEL1 [
    /// System Tick Timer for CPU1 source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// System Tick 1 divided clock.
        SystemTick1DividedClock = 0,
        /// FRO 1MHz clock.
        FRO1MHzClock = 1,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 2,
        /// No clock.
        NoClock = 3
    ]
],
SYSTICKCLKSELX1 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
TRACECLKSEL [
    /// Trace clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Trace divided clock.
        TraceDividedClock = 0,
        /// FRO 1MHz clock.
        FRO1MHzClock = 1,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 2,
        /// No clock.
        NoClock = 3
    ]
],
CTIMERCLKSEL0 [
    /// CTimer 0 clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// No clock.
        NoClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 6
    ]
],
CTIMERCLKSELX0 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
CTIMERCLKSEL1 [
    /// CTimer 1 clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// No clock.
        NoClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 6
    ]
],
CTIMERCLKSELX1 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
CTIMERCLKSEL2 [
    /// CTimer 2 clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// No clock.
        NoClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 6
    ]
],
CTIMERCLKSELX2 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
CTIMERCLKSEL3 [
    /// CTimer 3 clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// No clock.
        NoClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 6
    ]
],
CTIMERCLKSELX3 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
CTIMERCLKSEL4 [
    /// CTimer 4 clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// No clock.
        NoClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 6
    ]
],
CTIMERCLKSELX4 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
MAINCLKSELA [
    /// Main clock A source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// FRO 12 MHz clock.
        FRO12MHzClock = 0,
        /// CLKIN clock.
        CLKINClock = 1,
        /// FRO 1MHz clock.
        FRO1MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3
    ]
],
MAINCLKSELB [
    /// Main clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main Clock A.
        MainClockA = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// PLL1 clock.
        PLL1Clock = 2,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 3
    ]
],
CLKOUTSEL [
    /// CLKOUT clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// CLKIN clock.
        CLKINClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// PLL1 clock.
        PLL1Clock = 5,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
PLL0CLKSEL [
    /// PLL0 clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// FRO 12 MHz clock.
        FRO12MHzClock = 0,
        /// CLKIN clock.
        CLKINClock = 1,
        /// FRO 1MHz clock.
        FRO1MHzClock = 2,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 3,
        /// No clock.
        NoClock = 4
    ]
],
PLL1CLKSEL [
    /// PLL1 clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// FRO 12 MHz clock.
        FRO12MHzClock = 0,
        /// CLKIN clock.
        CLKINClock = 1,
        /// FRO 1MHz clock.
        FRO1MHzClock = 2,
        /// Oscillator 32kHz clock.
        Oscillator32kHzClock = 3,
        /// No clock.
        NoClock = 4
    ]
],
ADCCLKSEL [
    /// ADC clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 2,
        /// FRO 1 MHz clock.
        FRO1MHzClock = 3,
        /// No clock.
        NoClock = 4
    ]
],
USB0CLKSEL [
    /// FS USB clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// No clock.
        NoClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// PLL1 clock.
        PLL1Clock = 5
    ]
],
USB1CLKSEL [
    /// HS USB clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// CLKIN clock.
        CLKINClock = 2,
        /// No clock.
        NoClock = 3,
        /// PLL1 clock.
        PLL1Clock = 5
    ]
],
FCCLKSEL0 [
    /// Flexcomm Interface 0 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX0 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FCCLKSEL1 [
    /// Flexcomm Interface 1 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX1 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FCCLKSEL2 [
    /// Flexcomm Interface 2 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX2 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FCCLKSEL3 [
    /// Flexcomm Interface 3 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX3 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FCCLKSEL4 [
    /// Flexcomm Interface 4 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX4 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FCCLKSEL5 [
    /// Flexcomm Interface 5 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX5 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FCCLKSEL6 [
    /// Flexcomm Interface 6 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX6 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FCCLKSEL7 [
    /// Flexcomm Interface 7 clock source select for Fractional Rate Divider.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// MCLK clock.
        MCLKClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6,
        /// No clock.
        NoClock = 7
    ]
],
FCCLKSELX7 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
HSLSPICLKSEL [
    /// HS LSPI clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// system PLL divided clock.
        SystemPLLDividedClock = 1,
        /// FRO 12 MHz clock.
        FRO12MHzClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// FRO 1MHz clock.
        FRO1MHzClock = 4,
        /// No clock.
        NoClock = 5,
        /// Oscillator 32 kHz clock.
        Oscillator32KHzClock = 6
    ]
],
MCLKCLKSEL [
    /// MCLK clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// FRO 96 MHz clock.
        FRO96MHzClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// FRO 1 MHz clock.
        FRO1MHzClock = 2,
        /// Main clock.
        MainClock = 3,
        /// No clock.
        NoClock = 4
    ]
],
FLASHRCLKSEL [
    /// MCLK clock source select.
    SEL OFFSET(0) NUMBITS(1) [
        /// FRO 48 MHz clock.
        FRO48MHzClock = 0,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 1
    ]
],
SCTCLKSEL [
    /// SCTimer/PWM clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// CLKIN clock.
        CLKINClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// No clock.
        NoClock = 4,
        /// MCLK clock.
        MCLKClock = 5
    ]
],
CAPTCLKSEL [
    /// Capacitive Touch clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// FRO 12 MHz clock.
        FRO12MHzClock = 0,
        /// FRO 1 MHz clock.
        FRO1MHzClock = 1,
        /// No clock.
        NoClock = 2
    ]
],
SDIOCLKSEL [
    /// SDIO clock source select.
    SEL OFFSET(0) NUMBITS(3) [
        /// Main clock.
        MainClock = 0,
        /// PLL0 clock.
        PLL0Clock = 1,
        /// CLKIN clock.
        CLKINClock = 2,
        /// FRO 96 MHz clock.
        FRO96MHzClock = 3,
        /// No clock.
        NoClock = 4,
        /// MCLK clock.
        MCLKClock = 5
    ]
],
SYSTICKCLKDIV0 [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
SYSTICKCLKDIV1 [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
TRACECLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
FLEXFRG0CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL0 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FLEXFRG1CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL1 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FLEXFRG2CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL2 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FLEXFRG3CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL3 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FLEXFRG4CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL4 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FLEXFRG5CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL5 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FLEXFRG6CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL6 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
FLEXFRG7CTRL [
    /// Denominator of the fractional rate divider.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Numerator of the fractional rate divider.
    MULT OFFSET(8) NUMBITS(8) []
],
FLEXFRGXCTRL7 [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
CLKOUTDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
FRO96MDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
WDTCLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(6) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
ADCCLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(3) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
USB0CLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
USB1CLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
MCLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
SCTCLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
CAPTCLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
SDIOCLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
PLL0CLKDIV [
    /// Clock divider value.
    DIV OFFSET(0) NUMBITS(8) [],
    /// Resets the divider counter.
    RESET OFFSET(29) NUMBITS(1) [
        /// Divider is not reset.
        DividerIsNotReset = 0,
        /// Divider is reset.
        DividerIsReset = 1
    ],
    /// Halts the divider counter.
    HALT OFFSET(30) NUMBITS(1) [
        /// Divider clock is running.
        DividerClockIsRunning = 0,
        /// Divider clock is stoped.
        DividerClockIsStoped = 1
    ],
    /// Divider status flag.
    REQFLAG OFFSET(31) NUMBITS(1) [
        /// Divider clock is stable.
        DividerClockIsStable = 0,
        /// Clock frequency is not stable.
        ClockFrequencyIsNotStable = 1
    ]
],
CLOCKGENUPDATELOCKOUT [
    /// Control clock configuration registers access (like xxxDIV, xxxSEL).
    CLOCKGENUPDATELOCKOUT OFFSET(0) NUMBITS(32) [
        /// all hardware clock configruration are freeze.
        AllHardwareClockConfigrurationAreFreeze = 0,
        /// update all clock configuration.
        UpdateAllClockConfiguration = 1
    ]
],
FMCCR [
    /// Fetch control
    FETCHCTL OFFSET(0) NUMBITS(2) [
        /// No buffering (bypass always used) for Fetch cycles
        NoBufferingBypassAlwaysUsedForFetchCycles = 0,
        /// One buffer is used for all Fetch cycles
        OneBufferIsUsedForAllFetchCycles = 1,
        /// All buffers can be used for Fetch cycles
        AllBuffersCanBeUsedForFetchCycles = 2
    ],
    /// Data control
    DATACTL OFFSET(2) NUMBITS(2) [
        /// No buffering (bypass always used) for Data cycles
        NoBufferingBypassAlwaysUsedForDataCycles = 0,
        /// One buffer is used for all Data cycles
        OneBufferIsUsedForAllDataCycles = 1,
        /// All buffers can be used for Data cycles
        AllBuffersCanBeUsedForDataCycles = 2
    ],
    /// ACCEL
    ACCEL OFFSET(4) NUMBITS(1) [],
    /// Pref enable
    PREFEN OFFSET(5) NUMBITS(1) [],
    /// Pref ovr
    PREFOVR OFFSET(6) NUMBITS(1) [],
    /// Pref CRI
    PREFCRI OFFSET(8) NUMBITS(3) [],
    /// TMC time
    FMCTIM OFFSET(12) NUMBITS(5) [],
    /// When set, prefetch uses LRU buffer replacement policy
    PFISLRU OFFSET(17) NUMBITS(1) [],
    /// When set, prefetch will adaptively select between parent and LRU buffer replacem
    PFADAP OFFSET(18) NUMBITS(1) []
],
ROMCR [
    /// An extra-wait state between ARM master and ROM access.
    WAIT OFFSET(0) NUMBITS(1) [
        /// No extra-wait state between ARM master and ROM access.
        NoExtraWaitStateBetweenARMMasterAndROMAccess = 0,
        /// An extra-wait state between ARM master and ROM access.
        AnExtraWaitStateBetweenARMMasterAndROMAccess = 1
    ]
],
USB0CLKCTRL [
    /// USB0 Device USB0_NEEDCLK signal control:.
    AP_FS_DEV_CLK OFFSET(0) NUMBITS(1) [
        /// Under hardware control.
        UnderHardwareControl = 0,
        /// Forced high.
        ForcedHigh = 1
    ],
    /// USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:.
    POL_FS_DEV_CLK OFFSET(1) NUMBITS(1) [
        /// Falling edge of device USB0_NEEDCLK triggers wake-up.
        FallingEdgeOfDeviceUSB0_NEEDCLKTriggersWakeUp = 0,
        /// Rising edge of device USB0_NEEDCLK triggers wake-up.
        RisingEdgeOfDeviceUSB0_NEEDCLKTriggersWakeUp = 1
    ],
    /// USB0 Host USB0_NEEDCLK signal control:.
    AP_FS_HOST_CLK OFFSET(2) NUMBITS(1) [
        /// Under hardware control.
        UnderHardwareControl = 0,
        /// Forced high.
        ForcedHigh = 1
    ],
    /// USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:.
    POL_FS_HOST_CLK OFFSET(3) NUMBITS(1) [
        /// Falling edge of device USB0_NEEDCLK triggers wake-up.
        FallingEdgeOfDeviceUSB0_NEEDCLKTriggersWakeUp = 0,
        /// Rising edge of device USB0_NEEDCLK triggers wake-up.
        RisingEdgeOfDeviceUSB0_NEEDCLKTriggersWakeUp = 1
    ],
    /// Internal pull-up disable control.
    PU_DISABLE OFFSET(4) NUMBITS(1) [
        /// Internal pull-up enable.
        InternalPullUpEnable = 0,
        /// Internal pull-up disable.
        InternalPullUpDisable = 1
    ]
],
USB0CLKSTAT [
    /// USB0 Device USB0_NEEDCLK signal status:.
    DEV_NEED_CLKST OFFSET(0) NUMBITS(1) [
        /// USB0 Device clock is low.
        USB0DeviceClockIsLow = 0,
        /// USB0 Device clock is high.
        USB0DeviceClockIsHigh = 1
    ],
    /// USB0 Host USB0_NEEDCLK signal status:.
    HOST_NEED_CLKST OFFSET(1) NUMBITS(1) [
        /// USB0 Host clock is low.
        USB0HostClockIsLow = 0,
        /// USB0 Host clock is high.
        USB0HostClockIsHigh = 1
    ]
],
EZHINT [
    /// EZH interrupt hijack.
    EZHINT OFFSET(0) NUMBITS(32) []
],
FMCFLUSH [
    /// no description available
    FLUSH OFFSET(0) NUMBITS(1) []
],
MCLKIO [
    /// MCLK control.
    MCLKIO OFFSET(0) NUMBITS(32) [
        /// input mode.
        InputMode = 0,
        /// output mode.
        OutputMode = 1
    ]
],
USB1CLKCTRL [
    /// USB1 Device need_clock signal control:.
    AP_HS_DEV_CLK OFFSET(0) NUMBITS(1) [
        /// Under hardware control.
        UnderHardwareControl = 0,
        /// Forced high.
        ForcedHigh = 1
    ],
    /// USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt:.
    POL_HS_DEV_CLK OFFSET(1) NUMBITS(1) [
        /// Falling edge of device need_clock triggers wake-up.
        FallingEdgeOfDeviceNeed_clockTriggersWakeUp = 0,
        /// Rising edge of device need_clock triggers wake-up.
        RisingEdgeOfDeviceNeed_clockTriggersWakeUp = 1
    ],
    /// USB1 Host need_clock signal control:.
    AP_HS_HOST_CLK OFFSET(2) NUMBITS(1) [
        /// Under hardware control.
        UnderHardwareControl = 0,
        /// Forced high.
        ForcedHigh = 1
    ],
    /// USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt: 0 Falli
    POL_HS_HOST_CLK OFFSET(3) NUMBITS(1) [
        /// Falling edge of device need_clock triggers wake-up.
        FallingEdgeOfDeviceNeed_clockTriggersWakeUp = 0,
        /// Rising edge of device need_clock triggers wake-up.
        RisingEdgeOfDeviceNeed_clockTriggersWakeUp = 1
    ],
    /// External user wake-up signal for device mode; asserting this signal (active low)
    HS_DEV_WAKEUP_N OFFSET(4) NUMBITS(1) [
        /// Forces USB1 PHY to wake-up.
        ForcesUSB1PHYToWakeUp = 0,
        /// Normal USB1 PHY behavior.
        NormalUSB1PHYBehavior = 1
    ]
],
USB1CLKSTAT [
    /// USB1 Device need_clock signal status:.
    DEV_NEED_CLKST OFFSET(0) NUMBITS(1) [
        /// USB1 Device clock is low.
        USB1DeviceClockIsLow = 0,
        /// USB1 Device clock is high.
        USB1DeviceClockIsHigh = 1
    ],
    /// USB1 Host need_clock signal status:.
    HOST_NEED_CLKST OFFSET(1) NUMBITS(1) [
        /// USB1 Host clock is low.
        USB1HostClockIsLow = 0,
        /// USB1 Host clock is high.
        USB1HostClockIsHigh = 1
    ]
],
FLASHBANKENABLE [
    /// Flash Bank0 control.
    BANK0 OFFSET(0) NUMBITS(4) [
        /// Flash BANK0 checker is enabled (all Flash pages inside this bank cannot be erase
        ENABLE = 0,
        /// 1010: Flash BANK0 checker is disabled (all Flash pages inside this bank can be e
        DISABLE = 10
    ],
    /// Flash Bank1 control.
    BANK1 OFFSET(4) NUMBITS(4) [
        /// Flash BANK1 checker is enabled (all Flash pages inside this bank cannot be erase
        ENABLE = 0,
        /// 1010: Flash BANK1 checker is disabled (all Flash pages inside this bank can be e
        DISABLE = 10
    ],
    /// Flash Bank2 control.
    BANK2 OFFSET(8) NUMBITS(4) [
        /// Flash BANK2 checker is enabled (all Flash pages inside this bank cannot be erase
        ENABLE = 0,
        /// 1010: Flash BANK2 checker is disabled (all Flash pages inside this bank can be e
        DISABLE = 10
    ]
],
SDIOCLKCTRL [
    /// Programmable delay value by which cclk_in_drv is phase-shifted with regard to cc
    CCLK_DRV_PHASE OFFSET(0) NUMBITS(2) [
        /// 0 degree shift.
        _0DegreeShift = 0,
        /// 90 degree shift.
        _90DegreeShift = 1,
        /// 180 degree shift.
        _180DegreeShift = 2,
        /// 270 degree shift.
        _270DegreeShift = 3
    ],
    /// Programmable delay value by which cclk_in_sample is delayed with regard to cclk_
    CCLK_SAMPLE_PHASE OFFSET(2) NUMBITS(2) [
        /// 0 degree shift.
        _0DegreeShift = 0,
        /// 90 degree shift.
        _90DegreeShift = 1,
        /// 180 degree shift.
        _180DegreeShift = 2,
        /// 270 degree shift.
        _270DegreeShift = 3
    ],
    /// Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE.
    PHASE_ACTIVE OFFSET(7) NUMBITS(1) [
        /// Bypassed.
        Bypassed = 0,
        /// Activates phase shift logic. When active, the clock divider is active and phase
        PH_SHIFT = 1
    ],
    /// Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in.
    CCLK_DRV_DELAY OFFSET(16) NUMBITS(5) [],
    /// Enables drive delay, as controlled by the CCLK_DRV_DELAY field.
    CCLK_DRV_DELAY_ACTIVE OFFSET(23) NUMBITS(1) [
        /// Disable drive delay.
        DisableDriveDelay = 0,
        /// Enable drive delay.
        EnableDriveDelay = 1
    ],
    /// Programmable delay value by which cclk_in_sample is delayed with regard to cclk_
    CCLK_SAMPLE_DELAY OFFSET(24) NUMBITS(5) [],
    /// Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.
    CCLK_SAMPLE_DELAY_ACTIVE OFFSET(31) NUMBITS(1) [
        /// Disables sample delay.
        DisablesSampleDelay = 0,
        /// Enables sample delay.
        EnablesSampleDelay = 1
    ]
],
PLL1CTRL [
    /// Bandwidth select R value
    SELR OFFSET(0) NUMBITS(4) [],
    /// Bandwidth select I value.
    SELI OFFSET(4) NUMBITS(6) [],
    /// Bandwidth select P value
    SELP OFFSET(10) NUMBITS(5) [],
    /// PLL bypass control.
    BYPASS OFFSET(15) NUMBITS(1) [
        /// Bypass disabled. PLL CCO is sent to the PLL post-dividers.
        BypassDisabledPLLCCOIsSentToThePLLPostDividers = 0,
        /// Bypass enabled. PLL input clock is sent directly to the PLL output (default).
        BypassEnabledPLLInputClockIsSentDirectlyToThePLLOutputDefault = 1
    ],
    /// Bypass feedback clock divide by 2.
    BYPASSCCODIV2 OFFSET(16) NUMBITS(1) [
        /// Divide by 2. The CCO feedback clock is divided by 2 in addition to the programme
        DivideBy2TheCCOFeedbackClockIsDividedBy2InAdditionToTheProgrammedMDivide = 0,
        /// Bypass. The CCO feedback clock is divided only by the programmed M divide.
        BypassTheCCOFeedbackClockIsDividedOnlyByTheProgrammedMDivide = 1
    ],
    /// Disable upper frequency limiter.
    UPLIMOFF OFFSET(17) NUMBITS(1) [
        /// Normal mode.
        NormalMode = 0,
        /// Upper frequency limiter disabled.
        UpperFrequencyLimiterDisabled = 1
    ],
    /// PLL filter control. Set this bit to one when the spread spectrum controller is d
    BANDSEL OFFSET(18) NUMBITS(1) [
        /// SSCG control. The PLL filter uses the parameters derived from the spread spectru
        SSCGControlThePLLFilterUsesTheParametersDerivedFromTheSpreadSpectrumController = 0,
        /// MDEC control. The PLL filter uses the programmable fields SELP, SELR, and SELI i
        MDEC_CONTROL = 1
    ],
    /// PLL0 direct input enable
    DIRECTI OFFSET(19) NUMBITS(1) [
        /// Disabled. The PLL input divider (N divider) output is used to drive the PLL CCO.
        DisabledThePLLInputDividerNDividerOutputIsUsedToDriveThePLLCCO = 0,
        /// Enabled. The PLL input divider (N divider) is bypassed. the PLL input clock is u
        ENABLED = 1
    ],
    /// PLL0 direct output enable.
    DIRECTO OFFSET(20) NUMBITS(1) [
        /// Disabled. The PLL output divider (P divider) is used to create the PLL output.
        DisabledThePLLOutputDividerPDividerIsUsedToCreateThePLLOutput = 0,
        /// Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is u
        EnabledThePLLOutputDividerPDividerIsBypassedThePLLCCOOutputIsUsedAsThePLLOutput = 1
    ]
],
PLL1STAT [
    /// PLL0 lock indicator
    LOCK OFFSET(0) NUMBITS(1) []
],
PLL1NDEC [
    /// Decoded N-divider coefficient value.
    NDEC OFFSET(0) NUMBITS(10) [],
    /// NDEC reload request. When a 1 is written to this bit, the NDEC value is loaded i
    NREQ OFFSET(10) NUMBITS(1) []
],
PLL1MDEC [
    /// Decoded M-divider coefficient value.
    MDEC OFFSET(0) NUMBITS(17) [],
    /// MDEC reload request.
    MREQ OFFSET(17) NUMBITS(1) []
],
PLL1PDEC [
    /// Decoded P-divider coefficient value.
    PDEC OFFSET(0) NUMBITS(7) [],
    /// PDEC reload request. When a 1 is written to this bit, the PDEC value is loaded i
    PREQ OFFSET(7) NUMBITS(1) []
],
PLL1_TESTCTRL [
    /// input to functional test the pre-divider (N-divider).
    NFUNCTEST OFFSET(0) NUMBITS(7) [],
    /// input to functional test the feedback-divider (M-divider).
    MFUNCTEST OFFSET(7) NUMBITS(15) [],
    /// input to functional test the post-divider (P-divider).
    PFUNCTEST OFFSET(22) NUMBITS(4) []
],
PLL1_TESTSTAT [
    /// output to observe the functional pre-divider test.
    NMOTEST OFFSET(0) NUMBITS(1) [],
    /// output to observe the functional feedback-divider test.
    MMOTEST OFFSET(1) NUMBITS(1) [],
    /// output to observe the functional post-divider test.
    PMOTEST OFFSET(2) NUMBITS(1) []
],
PLL0CTRL [
    /// Bandwidth select R value
    SELR OFFSET(0) NUMBITS(4) [],
    /// Bandwidth select I value.
    SELI OFFSET(4) NUMBITS(6) [],
    /// Bandwidth select P value
    SELP OFFSET(10) NUMBITS(5) [],
    /// PLL bypass control.
    BYPASS OFFSET(15) NUMBITS(1) [
        /// Bypass disabled. PLL CCO is sent to the PLL post-dividers.
        BypassDisabledPLLCCOIsSentToThePLLPostDividers = 0,
        /// Bypass enabled. PLL input clock is sent directly to the PLL output (default).
        BypassEnabledPLLInputClockIsSentDirectlyToThePLLOutputDefault = 1
    ],
    /// Bypass feedback clock divide by 2.
    BYPASSCCODIV2 OFFSET(16) NUMBITS(1) [
        /// Divide by 2. The CCO feedback clock is divided by 2 in addition to the programme
        DivideBy2TheCCOFeedbackClockIsDividedBy2InAdditionToTheProgrammedMDivide = 0,
        /// Bypass. The CCO feedback clock is divided only by the programmed M divide.
        BypassTheCCOFeedbackClockIsDividedOnlyByTheProgrammedMDivide = 1
    ],
    /// Disable upper frequency limiter.
    UPLIMOFF OFFSET(17) NUMBITS(1) [
        /// Normal mode.
        NormalMode = 0,
        /// Upper frequency limiter disabled.
        UpperFrequencyLimiterDisabled = 1
    ],
    /// PLL filter control. Set this bit to one when the spread spectrum controller is d
    BANDSEL OFFSET(18) NUMBITS(1) [
        /// SSCG control. The PLL filter uses the parameters derived from the spread spectru
        SSCGControlThePLLFilterUsesTheParametersDerivedFromTheSpreadSpectrumController = 0,
        /// MDEC control. The PLL filter uses the programmable fields SELP, SELR, and SELI i
        MDEC_CONTROL = 1
    ],
    /// PLL0 direct input enable
    DIRECTI OFFSET(19) NUMBITS(1) [
        /// Disabled. The PLL input divider (N divider) output is used to drive the PLL CCO.
        DisabledThePLLInputDividerNDividerOutputIsUsedToDriveThePLLCCO = 0,
        /// Enabled. The PLL input divider (N divider) is bypassed. the PLL input clock is u
        ENABLED = 1
    ],
    /// PLL0 direct output enable.
    DIRECTO OFFSET(20) NUMBITS(1) [
        /// Disabled. The PLL output divider (P divider) is used to create the PLL output.
        DisabledThePLLOutputDividerPDividerIsUsedToCreateThePLLOutput = 0,
        /// Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is u
        EnabledThePLLOutputDividerPDividerIsBypassedThePLLCCOOutputIsUsedAsThePLLOutput = 1
    ]
],
PLL0STAT [
    /// PLL0 lock indicator
    LOCK OFFSET(0) NUMBITS(1) []
],
PLL0NDEC [
    /// Decoded N-divider coefficient value.
    NDEC OFFSET(0) NUMBITS(10) [],
    /// NDEC reload request. When a 1 is written to this bit, the NDEC value is loaded i
    NREQ OFFSET(10) NUMBITS(1) []
],
PLL0PDEC [
    /// Decoded P-divider coefficient value.
    PDEC OFFSET(0) NUMBITS(7) [],
    /// PDEC reload request. When a 1 is written to this bit, the PDEC value is loaded i
    PREQ OFFSET(7) NUMBITS(1) []
],
PLL0SSCG0 [
    /// Decoded M-divider coefficient value.
    MDEC OFFSET(0) NUMBITS(17) [],
    /// MDEC reload request. When a 1 is written to this bit, the MDEC value is loaded i
    MREQ OFFSET(17) NUMBITS(1) [],
    /// Select spread spectrum mode. Selects the source of the feedback divider value. F
    SEL_EXT OFFSET(18) NUMBITS(1) []
],
PLL0SSCG1 [
    /// M- divider value with fraction. MD[18:11]: integer portion of the feedback divid
    MD OFFSET(0) NUMBITS(19) [],
    /// MD reload request. When a 1 is written to this bit, the MD value is loaded into
    MDREQ OFFSET(19) NUMBITS(1) [],
    /// Programmable modulation frequency fm = Fref/Nss with Fref = Fin/N 0b000 => Nss =
    MF OFFSET(20) NUMBITS(3) [],
    /// Programmable frequency modulation depth. 0 = no spread. _fmodpk-pk = Fref x k/Fc
    MR OFFSET(23) NUMBITS(3) [],
    /// Modulation waveform control. 0 = no compensation. Compensation for low pass filt
    MC OFFSET(26) NUMBITS(2) [],
    /// Spread spectrum power-down.
    PD OFFSET(28) NUMBITS(1) [
        /// Enabled. Spread spectrum controller is enabled
        EnabledSpreadSpectrumControllerIsEnabled = 0,
        /// Disabled. Spread spectrum controller is disabled.
        DisabledSpreadSpectrumControllerIsDisabled = 1
    ],
    /// Select modulation frequency.
    DITHER OFFSET(29) NUMBITS(1) [
        /// Fixed. Fixed modulation frequency.
        FixedFixedModulationFrequency = 0,
        /// Dither. Randomly dither between two modulation frequencies.
        DitherRandomlyDitherBetweenTwoModulationFrequencies = 1
    ]
],
PLL0_TESTCTRL [
    /// input to functional test the pre-divider (N-divider).
    NFUNCTEST OFFSET(0) NUMBITS(7) [],
    /// input to functional test the feedback-divider (M-divider).
    MFUNCTEST OFFSET(7) NUMBITS(15) [],
    /// input to functional test the post-divider (P-divider).
    PFUNCTEST OFFSET(22) NUMBITS(4) []
],
PLL0_TESTSTAT [
    /// output to observe the functional pre-divider test.
    NMOTEST OFFSET(0) NUMBITS(1) [],
    /// output to observe the functional feedback-divider test.
    MMOTEST OFFSET(1) NUMBITS(1) [],
    /// output to observe the functional post-divider test.
    PMOTEST OFFSET(2) NUMBITS(1) []
],
EFUSECLKCTRL [
    /// eFUSE controller clock enable.
    EFUSECLKENA OFFSET(0) NUMBITS(1) []
],
STARTER0 [
    /// SYS interrupt wake-up.
    SYS OFFSET(0) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SDMA0 interrupt wake-up.
    SDMA0 OFFSET(1) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_GLOBALINT0 interrupt wake-up.
    GPIO_GLOBALINT0 OFFSET(2) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_GLOBALINT1 interrupt wake-up.
    GPIO_GLOBALINT1 OFFSET(3) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_INT00 interrupt wake-up.
    GPIO_INT00 OFFSET(4) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_INT01 interrupt wake-up.
    GPIO_INT01 OFFSET(5) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_INT02 interrupt wake-up.
    GPIO_INT02 OFFSET(6) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_INT03 interrupt wake-up.
    GPIO_INT03 OFFSET(7) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// UTICK0 interrupt wake-up.
    UTICK0 OFFSET(8) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// MRT0 interrupt wake-up.
    MRT0 OFFSET(9) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// CTIMER0 interrupt wake-up.
    CTIMER0 OFFSET(10) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// CTIMER1 interrupt wake-up.
    CTIMER1 OFFSET(11) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SCT0 interrupt wake-up.
    SCT0 OFFSET(12) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// CTIMER3 interrupt wake-up.
    CTIMER3 OFFSET(13) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT0 interrupt wake-up.
    FLEXINT0 OFFSET(14) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT1 interrupt wake-up.
    FLEXINT1 OFFSET(15) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT2 interrupt wake-up.
    FLEXINT2 OFFSET(16) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT3 interrupt wake-up.
    FLEXINT3 OFFSET(17) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT4 interrupt wake-up.
    FLEXINT4 OFFSET(18) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT5 interrupt wake-up.
    FLEXINT5 OFFSET(19) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT6 interrupt wake-up.
    FLEXINT6 OFFSET(20) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// FLEXINT7 interrupt wake-up.
    FLEXINT7 OFFSET(21) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// ADC0 interrupt wake-up.
    ADC0 OFFSET(22) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// ADC0_THCMP_OVR interrupt wake-up.
    ADC0_THCMP_OVR OFFSET(24) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// USB0_NEEDCLK interrupt wake-up.
    USB0_NEEDCLK OFFSET(27) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// USB0 interrupt wake-up.
    USB0 OFFSET(28) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// RTC_LITE0 interrupt wake-up.
    RTC_LITE0 OFFSET(29) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// EZH_ARCH_B0 interrupt wake-up.
    EZH_ARCH_B0 OFFSET(30) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// WAKEUP_MAILBOX0 interrupt wake-up.
    WAKEUP_MAILBOX0 OFFSET(31) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ]
],
STARTER1 [
    /// GPIO_INT04 interrupt wake-up.
    GPIO_INT04 OFFSET(0) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_INT05 interrupt wake-up.
    GPIO_INT05 OFFSET(1) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_INT06 interrupt wake-up.
    GPIO_INT06 OFFSET(2) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// GPIO_INT07 interrupt wake-up.
    GPIO_INT07 OFFSET(3) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// CTIMER2 interrupt wake-up.
    CTIMER2 OFFSET(4) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// CTIMER4 interrupt wake-up.
    CTIMER4 OFFSET(5) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// OS_EVENT interrupt wake-up.
    OS_EVENT OFFSET(6) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SDIO interrupt wake-up.
    SDIO OFFSET(10) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// USB1 interrupt wake-up.
    USB1 OFFSET(15) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// USB1_NEEDCLK interrupt wake-up.
    USB1_NEEDCLK OFFSET(16) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SEC_HYPERVISOR_CALL interrupt wake-up.
    SEC_HYPERVISOR_CALL OFFSET(17) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SEC_GPIO_INT00 interrupt wake-up.
    SEC_GPIO_INT00 OFFSET(18) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SEC_GPIO_INT01 interrupt wake-up.
    SEC_GPIO_INT01 OFFSET(19) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// PLU interrupt wake-up.
    PLU OFFSET(20) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SEC_VIO interrupt wake-up.
    SEC_VIO OFFSET(21) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SHA interrupt wake-up.
    SHA OFFSET(22) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// CASER interrupt wake-up.
    CASER OFFSET(23) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// QDDKEY interrupt wake-up.
    QDDKEY OFFSET(24) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// PQ interrupt wake-up.
    PQ OFFSET(25) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// SDMA1 interrupt wake-up.
    SDMA1 OFFSET(26) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// LSPI_HS interrupt wake-up.
    LSPI_HS OFFSET(27) NUMBITS(1) [
        /// Wake-up disabled.
        WakeUpDisabled = 0,
        /// Wake-up enabled.
        WakeUpEnabled = 1
    ],
    /// WAKEUPPADS interrupt wake-up.
    WAKEUPPADS OFFSET(31) NUMBITS(1) []
],
STARTERSET0 [
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    SYS_SET OFFSET(0) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    SDMA0_SET OFFSET(1) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    GPIO_GLOBALINT0_SET OFFSET(2) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    GPIO_GLOBALINT1_SET OFFSET(3) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    GPIO_INT00_SET OFFSET(4) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    GPIO_INT01_SET OFFSET(5) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    GPIO_INT02_SET OFFSET(6) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    GPIO_INT03_SET OFFSET(7) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    UTICK0_SET OFFSET(8) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    MRT0_SET OFFSET(9) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    CTIMER0_SET OFFSET(10) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    CTIMER1_SET OFFSET(11) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    SCT0_SET OFFSET(12) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    CTIMER3_SET OFFSET(13) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT0_SET OFFSET(14) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT1_SET OFFSET(15) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT2_SET OFFSET(16) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT3_SET OFFSET(17) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT4_SET OFFSET(18) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT5_SET OFFSET(19) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT6_SET OFFSET(20) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    FLEXINT7_SET OFFSET(21) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    ADC0_SET OFFSET(22) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    ADC0_THCMP_OVR_SET OFFSET(24) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    USB0_NEEDCLK_SET OFFSET(27) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    USB0_SET OFFSET(28) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    RTC_LITE0_SET OFFSET(29) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    EZH_ARCH_B0_SET OFFSET(30) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER0 registe
    WAKEUP_MAILBOX0_SET OFFSET(31) NUMBITS(1) []
],
STARTERSET1 [
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    GPIO_INT04_SET OFFSET(0) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    GPIO_INT05_SET OFFSET(1) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    GPIO_INT06_SET OFFSET(2) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    GPIO_INT07_SET OFFSET(3) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    CTIMER2_SET OFFSET(4) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    CTIMER4_SET OFFSET(5) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    OS_EVENT_SET OFFSET(6) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    SDIO_SET OFFSET(10) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    USB1_SET OFFSET(15) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    USB1_NEEDCLK_SET OFFSET(16) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    SEC_HYPERVISOR_CALL_SET OFFSET(17) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    SEC_GPIO_INT00_SET OFFSET(18) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    SEC_GPIO_INT01_SET OFFSET(19) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    PLU_SET OFFSET(20) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    SEC_VIO_SET OFFSET(21) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    SHA_SET OFFSET(22) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    CASER_SET OFFSET(23) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    QDDKEY_SET OFFSET(24) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    PQ_SET OFFSET(25) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    SDMA1_SET OFFSET(26) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    LSPI_HS_SET OFFSET(27) NUMBITS(1) [],
    /// Writing ones to this register sets the corresponding bit in the STARTER1 registe
    WAKEUPPADS_SET OFFSET(31) NUMBITS(1) []
],
STARTERCLR0 [
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    SYS_CLR OFFSET(0) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    SDMA0_CLR OFFSET(1) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    GPIO_GLOBALINT0_CLR OFFSET(2) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    GPIO_GLOBALINT1_CLR OFFSET(3) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    GPIO_INT00_CLR OFFSET(4) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    GPIO_INT01_CLR OFFSET(5) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    GPIO_INT02_CLR OFFSET(6) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    GPIO_INT03_CLR OFFSET(7) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    UTICK0_CLR OFFSET(8) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    MRT0_CLR OFFSET(9) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    CTIMER0_CLR OFFSET(10) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    CTIMER1_CLR OFFSET(11) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    SCT0_CLR OFFSET(12) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    CTIMER3_CLR OFFSET(13) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT0_CLR OFFSET(14) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT1_CLR OFFSET(15) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT2_CLR OFFSET(16) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT3_CLR OFFSET(17) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT4_CLR OFFSET(18) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT5_CLR OFFSET(19) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT6_CLR OFFSET(20) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    FLEXINT7_CLR OFFSET(21) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    ADC0_CLR OFFSET(22) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    ADC0_THCMP_OVR_CLR OFFSET(24) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    USB0_NEEDCLK_CLR OFFSET(27) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    USB0_CLR OFFSET(28) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    RTC_LITE0_CLR OFFSET(29) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    EZH_ARCH_B0_CLR OFFSET(30) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER0 regis
    WAKEUP_MAILBOX0_CLR OFFSET(31) NUMBITS(1) []
],
STARTERCLR1 [
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    GPIO_INT04_CLR OFFSET(0) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    GPIO_INT05_CLR OFFSET(1) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    GPIO_INT06_CLR OFFSET(2) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    GPIO_INT07_CLR OFFSET(3) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    CTIMER2_CLR OFFSET(4) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    CTIMER4_CLR OFFSET(5) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    OS_EVENT_CLR OFFSET(6) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    SDIO_CLR OFFSET(10) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    USB1_CLR OFFSET(15) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    USB1_NEEDCLK_CLR OFFSET(16) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    SEC_HYPERVISOR_CALL_CLR OFFSET(17) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    SEC_GPIO_INT00_CLR OFFSET(18) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    SEC_GPIO_INT01_CLR OFFSET(19) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    PLU_CLR OFFSET(20) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    SEC_VIO_CLR OFFSET(21) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    SHA_CLR OFFSET(22) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    CASER_CLR OFFSET(23) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    QDDKEY_CLR OFFSET(24) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    PQ_CLR OFFSET(25) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    SDMA1_CLR OFFSET(26) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    LSPI_HS_CLR OFFSET(27) NUMBITS(1) [],
    /// Writing ones to this register clears the corresponding bit in the STARTER1 regis
    WAKEUPPADS_CLR OFFSET(31) NUMBITS(1) []
],
FUNCRETENTIONCTRL [
    /// functional retention in power down only.
    FUNCRETENA OFFSET(0) NUMBITS(1) [
        /// disable functional retention.
        DisableFunctionalRetention = 0,
        /// enable functional retention.
        EnableFunctionalRetention = 1
    ],
    /// Start address divided by 4 inside SRAMX bank.
    RET_START OFFSET(1) NUMBITS(13) [],
    /// lenth of Scan chains to save.
    RET_LENTH OFFSET(14) NUMBITS(10) []
],
POWERDOWNSAFETY [
    /// Overrides the fro_is_dead' signal in Sleepcon module, in case this doesn't work
    OVERRIDEFRO OFFSET(0) NUMBITS(1) []
],
MAINCLKSAFETY [
    /// main clock is enable after MAINCLKSAFETY cycle.
    MAINCLKSAFETY OFFSET(0) NUMBITS(32) []
],
HARDWARESLEEP [
    /// Force peripheral clocking to stay on during Deep Sleep and Power-down modes.
    FORCED OFFSET(0) NUMBITS(1) [],
    /// Wake for Flexcomms.
    PERIPHERALS OFFSET(1) NUMBITS(1) [],
    /// Wake for DMA0.
    SDMA0 OFFSET(3) NUMBITS(1) [],
    /// Wake for DMA1.
    SDMA1 OFFSET(5) NUMBITS(1) []
],
CPUCTRL [
    /// CPU1 clock enable.
    CPU1CLKEN OFFSET(3) NUMBITS(1) [
        /// The CPU1 clock is not enabled.
        TheCPU1ClockIsNotEnabled = 0,
        /// The CPU1 clock is enabled.
        TheCPU1ClockIsEnabled = 1
    ],
    /// CPU1 reset.
    CPU1RSTEN OFFSET(5) NUMBITS(1) [
        /// The CPU1 is not being reset.
        TheCPU1IsNotBeingReset = 0,
        /// The CPU1 is being reset.
        TheCPU1IsBeingReset = 1
    ]
],
CPBOOT [
    /// Coprocessor Boot Address for CPU1.
    CPBOOT OFFSET(0) NUMBITS(32) []
],
CPSTACK [
    /// Coprocessor Stack Address. -- NOT USED
    CPSTACK OFFSET(0) NUMBITS(32) []
],
CPSTAT [
    /// The CPU0 sleeping state.
    CPU0SLEEPING OFFSET(0) NUMBITS(1) [
        /// the CPU is not sleeping.
        TheCPUIsNotSleeping = 0,
        /// the CPU is sleeping.
        TheCPUIsSleeping = 1
    ],
    /// The CPU1 sleeping state.
    CPU1SLEEPING OFFSET(1) NUMBITS(1) [
        /// the CPU is not sleeping.
        TheCPUIsNotSleeping = 0,
        /// the CPU is sleeping.
        TheCPUIsSleeping = 1
    ],
    /// The CPU0 lockup state.
    CPU0LOCKUP OFFSET(2) NUMBITS(1) [
        /// the CPU is not in lockup.
        TheCPUIsNotInLockup = 0,
        /// the CPU is in lockup.
        TheCPUIsInLockup = 1
    ],
    /// The CPU1 lockup state.
    CPU1LOCKUP OFFSET(3) NUMBITS(1) [
        /// the CPU is not in lockup.
        TheCPUIsNotInLockup = 0,
        /// the CPU is in lockup.
        TheCPUIsInLockup = 1
    ]
],
DICE_REG0 [
    /// no description available
    DICE_REG0 OFFSET(0) NUMBITS(32) []
],
DICE_REG1 [
    /// no description available
    DICE_REG1 OFFSET(0) NUMBITS(32) []
],
DICE_REG2 [
    /// no description available
    DICE_REG2 OFFSET(0) NUMBITS(32) []
],
DICE_REG3 [
    /// no description available
    DICE_REG3 OFFSET(0) NUMBITS(32) []
],
DICE_REG4 [
    /// no description available
    DICE_REG4 OFFSET(0) NUMBITS(32) []
],
DICE_REG5 [
    /// no description available
    DICE_REG5 OFFSET(0) NUMBITS(32) []
],
DICE_REG6 [
    /// no description available
    DICE_REG6 OFFSET(0) NUMBITS(32) []
],
DICE_REG7 [
    /// no description available
    DICE_REG7 OFFSET(0) NUMBITS(32) []
],
CLOCK_CTRL [
    /// Enable Flash 48 MHz clock.
    FLASH48MHZ_ENA OFFSET(0) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable XTAL32MHz clock for Frequency Measure module.
    XTAL32MHZ_FREQM_ENA OFFSET(1) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable FRO 1MHz clock for Frequency Measure module.
    FRO1MHZ_FREQM_ENA OFFSET(2) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable FRO 12MHz clock for Frequency Measure module.
    FRO12MHZ_FREQM_ENA OFFSET(3) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable FRO 96MHz clock for Frequency Measure module.
    FRO96MHZ_FREQM_ENA OFFSET(4) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable clock_in clock for clock module.
    CLKIN_FREQM_ENA OFFSET(5) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable FRO 1MHz clock for clock muxing in clock gen.
    FRO_LS_CLK_ENA OFFSET(6) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable FRO 12MHz clock for analog control of the FRO 192MHz.
    ANA_FRO12M_CLK_ENA OFFSET(7) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable clock for cristal oscilator calibration.
    XO_CAL_CLK_ENA OFFSET(8) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ],
    /// Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching.
    PLU_DEGLITCH_CLK_ENA OFFSET(9) NUMBITS(1) [
        /// The clock is not enabled.
        TheClockIsNotEnabled = 0,
        /// The clock is enabled.
        TheClockIsEnabled = 1
    ]
],
COMP_INT_CTRL [
    /// Analog Comparator interrupt enable control:.
    INT_ENABLE OFFSET(0) NUMBITS(1) [
        /// interrupt disable.
        InterruptDisable = 0,
        /// interrupt enable.
        InterruptEnable = 1
    ],
    /// Analog Comparator interrupt clear.
    INT_CLEAR OFFSET(1) NUMBITS(1) [
        /// No effect.
        NoEffect = 0,
        /// Clear the interrupt. Self-cleared bit.
        ClearTheInterruptSelfClearedBit = 1
    ],
    /// Comparator interrupt type selector:.
    INT_CTRL OFFSET(2) NUMBITS(3) [
        /// The analog comparator interrupt edge sensitive is disabled.
        TheAnalogComparatorInterruptEdgeSensitiveIsDisabled = 0,
        /// The analog comparator interrupt level sensitive is disabled.
        TheAnalogComparatorInterruptLevelSensitiveIsDisabled = 1,
        /// analog comparator interrupt is rising edge sensitive.
        AnalogComparatorInterruptIsRisingEdgeSensitive = 2,
        /// Analog Comparator interrupt is high level sensitive.
        AnalogComparatorInterruptIsHighLevelSensitive = 3,
        /// analog comparator interrupt is falling edge sensitive.
        AnalogComparatorInterruptIsFallingEdgeSensitive = 4,
        /// Analog Comparator interrupt is low level sensitive.
        AnalogComparatorInterruptIsLowLevelSensitive = 5,
        /// analog comparator interrupt is rising and falling edge sensitive.
        AnalogComparatorInterruptIsRisingAndFallingEdgeSensitive = 6
    ],
    /// Select which Analog comparator output (filtered our un-filtered) is used for int
    INT_SOURCE OFFSET(5) NUMBITS(1) [
        /// Select Analog Comparator filtered output as input for interrupt detection.
        SelectAnalogComparatorFilteredOutputAsInputForInterruptDetection = 0,
        /// Select Analog Comparator raw output (unfiltered) as input for interrupt detectio
        RAW_INT = 1
    ]
],
COMP_INT_STATUS [
    /// Interrupt status BEFORE Interrupt Enable.
    STATUS OFFSET(0) NUMBITS(1) [
        /// no interrupt pending.
        NoInterruptPending = 0,
        /// interrupt pending.
        InterruptPending = 1
    ],
    /// Interrupt status AFTER Interrupt Enable.
    INT_STATUS OFFSET(1) NUMBITS(1) [
        /// no interrupt pending.
        NoInterruptPending = 0,
        /// interrupt pending.
        InterruptPending = 1
    ],
    /// comparator analog output.
    VAL OFFSET(2) NUMBITS(1) [
        /// P+ is smaller than P-.
        PIsSmallerThanP = 0,
        /// P+ is greater than P-.
        PIsGreaterThanP = 1
    ]
],
AUTOCLKGATEOVERRIDE [
    /// Control automatic clock gating of ROM controller.
    ROM OFFSET(0) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of RAMX controller.
    RAMX_CTRL OFFSET(1) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of RAM0 controller.
    RAM0_CTRL OFFSET(2) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of RAM1 controller.
    RAM1_CTRL OFFSET(3) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of RAM2 controller.
    RAM2_CTRL OFFSET(4) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of RAM3 controller.
    RAM3_CTRL OFFSET(5) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of RAM4 controller.
    RAM4_CTRL OFFSET(6) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of synchronous bridge controller 0.
    SYNC0_APB OFFSET(7) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of synchronous bridge controller 1.
    SYNC1_APB OFFSET(8) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of FLASH controller.
    FLASH OFFSET(9) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of FMC controller.
    FMC OFFSET(10) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of CRCGEN controller.
    CRCGEN OFFSET(11) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of DMA0 controller.
    SDMA0 OFFSET(12) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of DMA1 controller.
    SDMA1 OFFSET(13) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of USB controller.
    USB OFFSET(14) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// Control automatic clock gating of synchronous system controller registers bank.
    SYSCON OFFSET(15) NUMBITS(1) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 1
    ],
    /// The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields update
    ENABLEUPDATE OFFSET(16) NUMBITS(16) [
        /// Automatic clock gating is not overridden.
        AutomaticClockGatingIsNotOverridden = 0,
        /// Automatic clock gating is overridden (Clock gating is disabled).
        AutomaticClockGatingIsOverriddenClockGatingIsDisabled = 49374
    ]
],
GPIOPSYNC [
    /// Enable bypass of the first stage of synchonization inside GPIO_INT module.
    PSYNC OFFSET(0) NUMBITS(1) [
        /// use the first stage of synchonization inside GPIO_INT module.
        UseTheFirstStageOfSynchonizationInsideGPIO_INTModule = 0,
        /// bypass of the first stage of synchonization inside GPIO_INT module.
        BypassOfTheFirstStageOfSynchonizationInsideGPIO_INTModule = 1
    ]
],
INVERTMAINCLK [
    /// Invert main_clock (AHB system clock).
    INVERT OFFSET(0) NUMBITS(1) []
],
DEBUG_LOCK_EN [
    /// Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITY
    LOCK_ALL OFFSET(0) NUMBITS(4) [
        /// Any other value than b1010: disable write access to all 6 registers.
        AnyOtherValueThanB1010DisableWriteAccessToAll6Registers = 0,
        /// 1010: Enable write access to all 6 registers.
        _1010EnableWriteAccessToAll6Registers = 10
    ]
],
DEBUG_FEATURES [
    /// CM33 (CPU0) Invasive debug control:.
    CM33_DBGEN OFFSET(0) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// CM33 (CPU0) Non Invasive debug control:.
    CM33_NIDEN OFFSET(2) NUMBITS(2) [
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// CM33 (CPU0) Secure Invasive debug control:.
    CM33_SPIDEN OFFSET(4) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// CM33 (CPU0) Secure Non Invasive debug control:.
    CM33_SPNIDEN OFFSET(6) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// Micro-CM33 (CPU1) Invasive debug control:.
    MCM33_DBGEN OFFSET(8) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// Micro-CM33 (CPU1) Non Invasive debug control:.
    MCM33_NIDEN OFFSET(10) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ]
],
DEBUG_FEATURES_DP [
    /// CM33 (CPU0) Invasive debug control:.
    CM33_DBGEN OFFSET(0) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// CM33 (CPU0) Non Invasive debug control:.
    CM33_NIDEN OFFSET(2) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// CM33 (CPU0) Secure Invasive debug control:.
    CM33_SPIDEN OFFSET(4) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// CM33 (CPU0) Secure Non Invasive debug control:.
    CM33_SPNIDEN OFFSET(6) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// Micro-CM33 (CPU1) Invasive debug control:.
    MCM33_DBGEN OFFSET(8) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ],
    /// Micro-CM33 (CPU1) Non Invasive debug control:.
    MCM33_NIDEN OFFSET(10) NUMBITS(2) [
        /// Any other value than b10: invasive debug is disable.
        AnyOtherValueThanB10InvasiveDebugIsDisable = 1,
        /// 10: Invasive debug is enabled.
        _10InvasiveDebugIsEnabled = 2
    ]
],
CODESECURITYPROTTEST [
    /// Security code to allow test access : 0x12345678.
    SEC_CODE OFFSET(0) NUMBITS(32) [
        /// test access is not allowed.
        TestAccessIsNotAllowed = 0,
        /// Security code to allow test access.
        SecurityCodeToAllowTestAccess = 305419896
    ]
],
CODESECURITYPROTCPU0 [
    /// Security code to allow CPU0 DAP: 0x12345678.
    SEC_CODE OFFSET(0) NUMBITS(32) [
        /// CPU0 DAP is not allowed.
        CPU0DAPIsNotAllowed = 0,
        /// Security code to allow CPU0 DAP.
        SecurityCodeToAllowCPU0DAP = 305419896
    ]
],
CODESECURITYPROTCPU1 [
    /// Security code to allow CPU1 DAP: 0x12345678.
    SEC_CODE OFFSET(0) NUMBITS(32) [
        /// CPU1 DAP is not allowed.
        CPU1DAPIsNotAllowed = 0,
        /// Security code to allow CPU1 DAP.
        SecurityCodeToAllowCPU1DAP = 305419896
    ]
],
KEY_BLOCK [
    /// Write a value to block quiddikey/PUF all index.
    KEY_BLOCK OFFSET(0) NUMBITS(32) []
],
DEBUG_AUTH_SCRATCH [
    /// Set by the debug authentication code in ROM to pass the debug beacons (Credentia
    SCRATCH OFFSET(0) NUMBITS(32) []
],
CPUCFG [
    /// Enable CPU1.
    CPU1ENABLE OFFSET(2) NUMBITS(1) [
        /// CPU1 is disable (Processor in reset).
        CPU1IsDisableProcessorInReset = 0,
        /// CPU1 is enable.
        CPU1IsEnable = 1
    ]
],
FLASHSIZECFG [
    /// Flash_size.
    FLASHSIZE OFFSET(0) NUMBITS(9) [
        /// 128KB when 8'b00000000.
        _128KBWhen8B00000000 = 0,
        /// 256KB when 8'b00000001.
        _256KBWhen8B00000001 = 1,
        /// 512KB when 8'b00000010.
        _512KBWhen8B00000010 = 2,
        /// 640KB when others.
        _640KBWhenOthers = 511
    ],
    /// Flash start address.
    FLASHSTART OFFSET(9) NUMBITS(1) [
        /// Private flash start 32kB before last address.
        PrivateFlashStart32kBBeforeLastAddress = 0,
        /// Private flash start 64kB before last address.
        PrivateFlashStart64kBBeforeLastAddress = 1
    ]
],
CONFIGLOCKOUT [
    /// Disable write access to FLASHSIZECFG, SRAMSIZECFG, DEVICE_ID0 and DEVICE_ID1.
    LOCK OFFSET(0) NUMBITS(1) [
        /// Enable write access to FLASHSIZECFG, SRAMSIZECFG, DEVICE_ID0 and DEVICE_ID1.
        EnableWriteAccessToFLASHSIZECFGSRAMSIZECFGDEVICE_ID0AndDEVICE_ID1 = 0,
        /// Disable write access to FLASHSIZECFG, SRAMSIZECFG, DEVICE_ID0 and DEVICE_ID1.
        DisableWriteAccessToFLASHSIZECFGSRAMSIZECFGDEVICE_ID0AndDEVICE_ID1 = 1
    ],
    /// Disable write access to FLASHBENKENABLE.
    FLASHBANK0_LOCK OFFSET(2) NUMBITS(1) [
        /// Enable write access to FLASHBENKENABLE.
        EnableWriteAccessToFLASHBENKENABLE = 0,
        /// Disable write access to FLASHBENKENABLE.
        DisableWriteAccessToFLASHBENKENABLE = 1
    ],
    /// no description available
    FLASHBANK1_LOCK OFFSET(3) NUMBITS(1) [],
    /// no description available
    FLASHBANK2_LOCK OFFSET(4) NUMBITS(1) []
],
RAMSIZECFG [
    /// RAMX size:.
    RAMX_SIZE OFFSET(0) NUMBITS(2) [
        /// RAMX size is 0 KByte.
        RAMXSizeIs0KByte = 0,
        /// RAMX size is 16 KByte.
        RAMXSizeIs16KByte = 1,
        /// RAMX size is 32 KByte.
        RAMXSizeIs32KByte = 3
    ],
    /// RAM0 size:.
    RAM0_SIZE OFFSET(2) NUMBITS(2) [
        /// RAM0 size is 0 KByte.
        RAM0SizeIs0KByte = 0,
        /// RAM0 size is 32 Kbyte.
        RAM0SizeIs32Kbyte = 1,
        /// RAM0 size is 64 Kbyte.
        RAM0SizeIs64Kbyte = 3
    ],
    /// RAM1 size:.
    RAM1_SIZE OFFSET(4) NUMBITS(2) [
        /// RAM1 size is 0 KByte.
        RAM1SizeIs0KByte = 0,
        /// RAM1 size is 32 Kbyte.
        RAM1SizeIs32Kbyte = 1,
        /// RAM1 size is 64 Kbyte.
        RAM1SizeIs64Kbyte = 3
    ],
    /// RAM2 size:.
    RAM2_SIZE OFFSET(6) NUMBITS(2) [
        /// RAM2 size is 0 KByte.
        RAM2SizeIs0KByte = 0,
        /// RAM2 size is 32 Kbyte.
        RAM2SizeIs32Kbyte = 1,
        /// RAM2 size is 64 Kbyte.
        RAM2SizeIs64Kbyte = 3
    ],
    /// RAM3 size:.
    RAM3_SIZE OFFSET(8) NUMBITS(2) [
        /// RAM3 size is 0 KByte.
        RAM3SizeIs0KByte = 0,
        /// RAM3 size is 32 Kbyte.
        RAM3SizeIs32Kbyte = 1,
        /// RAM3 size is 64 Kbyte.
        RAM3SizeIs64Kbyte = 3
    ],
    /// RAM4 size:.
    RAM4_SIZE OFFSET(10) NUMBITS(2) [
        /// RAM4 size is 0 KByte.
        RAM4SizeIs0KByte = 0,
        /// RAM4 size is 8 Kbyte.
        RAM4SizeIs8Kbyte = 1,
        /// RAM4 size is 16 Kbyte.
        RAM4SizeIs16Kbyte = 3
    ]
],
PERIPHENCFG [
    /// SCT enable.
    SCTEN OFFSET(0) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ],
    /// ADC enable.
    ADCEN OFFSET(1) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ],
    /// USB0 enable.
    USB0EN OFFSET(2) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ],
    /// Puff enable.
    PUFFEN OFFSET(6) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ],
    /// USB1 enable.
    USB1EN OFFSET(10) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ],
    /// SDIO enable.
    SDIOEN OFFSET(11) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ],
    /// HASH enable.
    HASHEN OFFSET(12) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ],
    /// PRINCE enable.
    PRINCEEN OFFSET(14) NUMBITS(1) [
        /// peripheral is disable.
        PeripheralIsDisable = 0,
        /// peripheral is enable.
        PeripheralIsEnable = 1
    ]
],
DEVICE_ID0 [
    /// no description available
    PARTCONFIG OFFSET(0) NUMBITS(8) [],
    /// no description available
    SRAM_SIZE OFFSET(8) NUMBITS(4) [],
    /// no description available
    FLASH_SIZE OFFSET(12) NUMBITS(3) [],
    /// no description available
    ROM_REV_MINOR OFFSET(20) NUMBITS(4) [],
    /// no description available
    MODELNUM_EXTENTION OFFSET(24) NUMBITS(3) []
],
DIEID [
    /// Chip Metal Revision ID.
    REV_ID OFFSET(0) NUMBITS(4) [],
    /// Chip Number.
    MCO_NUM_IN_DIE_ID OFFSET(4) NUMBITS(20) []
],
PRESETCTRLSET[0] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
PRESETCTRLSET[1] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
PRESETCTRLSET[2] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
PRESETCTRLCLR[0] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
PRESETCTRLCLR[1] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
PRESETCTRLCLR[2] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRLSET[0] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRLSET[1] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRLSET[2] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRLCLR[0] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRLCLR[1] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
],
AHBCLKCTRLCLR[2] [
    /// Data array value
    DATA OFFSET(0) NUMBITS(32) []
]
];
const SYSCON_BASE: StaticRef<SysconRegisters> =
    unsafe { StaticRef::new(0x50000000 as *const SysconRegisters) };

pub struct Syscon{
    registers: StaticRef<SysconRegisters>,
}

impl Syscon{
    pub const fn new() -> Syscon{
        Syscon {
            registers: SYSCON_BASE,
        }
    }

    // IOCON 
    pub fn is_enabled_iocon(&self){
        self.registers.ahbclkctrl0.is_set(AHBCLKCTRL0::IOCON)
    }

    pub fn enable_iocon(&self){
        self.registers.ahbclkctrl0.modify(AHBCLKCTRL0::IOCON::SET);
    }

    pub fn disable_iocon(&self){
        self.registers.ahbclkctrl0.is_set(AHBCLKCTRL0::IOCON::CLEAR);
    }
}

// TODO: add remaining clocks
enum ClockGate {
    AHBCLKCTRL0(CLKCTRL0),
    AHBCLKCTRL1(CLKCTRL1),
    AHBCLKCTRL2(CLKCTRL2),
}

// TODO: add remaining clocks
pub enum CLKCTRL0 {
    IOCON,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
}
pub enum CLKCTRL1 {
    DemoClock,
}
pub enum CLKCTRL2 {
    DemoClock,
}

pub struct PeripheralClock<'a>{
    syscon: &'a syscon,
    clock_gate: ClockGate,
}

impl ClockInterface for PeripheralClock<'_> {
    fn is_enabled(&self) -> bool{
        match self.clock_gate {
            ClockGate::AHBCLKCTRL0(ref v) => match v {
                CLKCTRL0::IOCON => self.sscon.is_enabled_iocon(),
                CLKCTRL0::GPIO0 => unimplemented!(),
                CLKCTRL0::GPIO1 => unimplemented!(),
                CLKCTRL0::GPIO2 => unimplemented!(),
                CLKCTRL0::GPIO3 => unimplemented!(),
            },
            ClockGate::AHBCLKCTRL1(ref v) => unimplemented!(),
            ClockGate::AHBCLKCTRL2(ref v) => unimplemented!(),
        }
    }

    fn enable(&self) {
        match self.clock_gate {
            ClockGate::AHBCLKCTRL0(ref v) => match v {
                CLKCTRL0::IOCON => self.syscon.enable_iocon(),
                CLKCTRL0::GPIO0 => unimplemented!(),
                CLKCTRL0::GPIO1 => unimplemented!(),
                CLKCTRL0::GPIO2 => unimplemented!(),
                CLKCTRL0::GPIO3 => unimplemented!(),
            },
            ClockGate::AHBCLKCTRL1(ref v) => unimplemented!(),
            ClockGate::AHBCLKCTRL2(ref v) => unimplemented!(),
        }
    }

    fn disable(&self) {
        match self.clock_gate {
            ClockGate::AHBCLKCTRL0(ref v) => match v {
                CLKCTRL0::IOCON => self.syscon.disable_iocon(),
                CLKCTRL0::GPIO0 => unimplemented!(),
                CLKCTRL0::GPIO1 => unimplemented!(),
                CLKCTRL0::GPIO2 => unimplemented!(),
                CLKCTRL0::GPIO3 => unimplemented!(),
            },
            ClockGate::AHBCLKCTRL1(ref v) => unimplemented!(),
            ClockGate::AHBCLKCTRL2(ref v) => unimplemented!(),
        }
    }
}
