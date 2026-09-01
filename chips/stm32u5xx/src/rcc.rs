// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2024.
// Copyright OxidOS Automotive 2026.

use kernel::utilities::StaticRef;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::{ReadWrite, register_bitfields, register_structs};

register_structs! {
    pub RccRegisters {
        /// Control register
        (0x000 => cr: ReadWrite<u32, CR::Register>),
        (0x004 => _reserved0: [u32; 9]),
        (0x028 => pll1cfgr: ReadWrite<u32, PLL1CFGR::Register>),
        (0x02C => _reserved_b: [u32; 2]),
        /// PLL1 Div Register
        (0x034 => pll1divr: ReadWrite<u32, PLL1DIVR::Register>),
        (0x038 => _reserved_c: [u32; 20]),
        /// AHB1 peripheral clock enable register
        (0x088 => ahb1enr: ReadWrite<u32, AHB1ENR::Register>),
        /// AHB2 peripheral clock enable register 1
        (0x08C => ahb2enr1: ReadWrite<u32, AHB2ENR1::Register>),
        (0x090 => _reserved1: [u32; 1]),
        /// AHB3 peripheral clock enable register
        (0x094 => ahb3enr: ReadWrite<u32, AHB3ENR::Register>),
        (0x098 => _reserved2: [u32; 1]),
        /// APB1 peripheral clock enable register 1
        (0x09C => apb1enr1: ReadWrite<u32, APB1ENR1::Register>),
        (0x0A0 => apb1enr2: ReadWrite<u32, APB1ENR2::Register>),
        /// APB2 peripheral clock enable register
        (0x0A4 => apb2enr: ReadWrite<u32, APB2ENR::Register>),
        /// APB3 peripheral clock enable register
        (0x0A8 => apb3enr: ReadWrite<u32, APB3ENR::Register>),
        (0x0AC => _reserved4: [u32; 13]),
        /// Peripherals independent clock configuration register 1
        (0x0E0 => ccipr1: ReadWrite<u32, CCIPR1::Register>),
        (0x0E4 => ccipr2: ReadWrite<u32, CCIPR1::Register>),
        /// Peripherals independent clock configuration register 3
        (0x0E8 => ccipr3: ReadWrite<u32, CCIPR3::Register>),
        (0x0EC => @END),
    }
}

register_bitfields![u32,
    pub CR [
        HSION OFFSET(8) NUMBITS(1) [],
        HSIRDY OFFSET(10) NUMBITS(1) [],
        HSEON OFFSET(16) NUMBITS(1) [],
        HSERDY OFFSET(17) NUMBITS(1) [],
        HSEBYP OFFSET(18) NUMBITS(1) [],
        HSEEXT OFFSET(20) NUMBITS(1) [],
        PLL1ON OFFSET(24) NUMBITS(1) [],
        PLL1RDY OFFSET(25) NUMBITS(1) [],
    ],
    pub AHB1ENR [
        GPDMA1EN OFFSET(0) NUMBITS(1) []
    ],
    pub AHB2ENR1 [
        GPIOAEN OFFSET(0) NUMBITS(1) [],
        GPIOBEN OFFSET(1) NUMBITS(1) [],
        GPIOCEN OFFSET(2) NUMBITS(1) [],
        GPIODEN OFFSET(3) NUMBITS(1) [],
        GPIOEEN OFFSET(4) NUMBITS(1) [],
        GPIOFEN OFFSET(5) NUMBITS(1) [],
        GPIOGEN OFFSET(6) NUMBITS(1) [],
        GPIOHEN OFFSET(7) NUMBITS(1) [],
        GPIOIEN OFFSET(8) NUMBITS(1) [],
        GPIOJEN OFFSET(9) NUMBITS(1) [],
        ADC12EN OFFSET(10) NUMBITS(1) [],
        HASHEN OFFSET(17) NUMBITS(1) [],
        TRNGEN  OFFSET(18) NUMBITS(1) [],
    ],
    pub AHB3ENR [
        PWREN OFFSET(2) NUMBITS(1) [],
        DAC1EN OFFSET(6) NUMBITS(1) [],
    ],
    pub APB1ENR1 [
        TIM2EN OFFSET(0) NUMBITS(1) [],
        TIM3EN OFFSET(1) NUMBITS(1) []
    ],
    pub APB1ENR2 [
        FDCAN1EN OFFSET(9) NUMBITS(1) [],
    ],
    pub APB2ENR [
        USART1EN OFFSET(14) NUMBITS(1) []
    ],
    pub APB3ENR [
        SYSCFGEN OFFSET(1) NUMBITS(1) []
    ],
    pub CCIPR1 [
        USART1SEL OFFSET(0) NUMBITS(2) [
            PCLK = 0,
            SYSCLK = 1,
            HSI16 = 2,
            LSE = 3
        ],
        FDCAN1SEL OFFSET(24) NUMBITS(2) [
            HSE = 0,
            PLL1_Q = 1,
            PLL2_P = 2,
        ]
    ],
    pub CCIPR3 [
        ADCDACSEL OFFSET(12) NUMBITS(3) [
            HCLK = 0,
            SYSCLK = 1,
            PLL2_R_CK = 2,
            HSE = 3,
            HSI16 = 4,
            MSIK = 5
        ],
        DAC1SEL OFFSET(15) NUMBITS(1) [
            LSE = 0,
            LSI = 1
        ]
    ],
    pub PLL1CFGR [
            //HSI16 is 16mhz, we feed directly into pll1 and then feed into fdcan thru pll_q
            PLL1SRC OFFSET(0) NUMBITS(2) [
                NONE = 0,
                MSIS = 1,
                HSI16 = 2,
                HSE = 3,
            ],
            //pll1 input range, 00-01-10 is 4-8mhz and 11 is 8-16mhz
            PLL1RGE OFFSET(2) NUMBITS(2) [
                LOW = 0,
                HIGH = 3
            ],
            //input clock divider
            PLL1M OFFSET(8) NUMBITS(2) [],
            PLL1QEN OFFSET(17) NUMBITS(1) [],
        ],

        pub PLL1DIVR [
            PLL1N OFFSET(0) NUMBITS(9) [],
            PLL1Q OFFSET(16) NUMBITS(7) []
        ]
];

/// Base address for RCC in Nonsecure mode
pub const RCC_BASE: StaticRef<RccRegisters> =
    unsafe { StaticRef::new(0x46020C00 as *const RccRegisters) };

pub struct Rcc {
    registers: StaticRef<RccRegisters>,
}

impl Rcc {
    pub const fn new(base: StaticRef<RccRegisters>) -> Self {
        Self { registers: base }
    }

    pub fn enable_dma1(&self) {
        self.registers.ahb1enr.modify(AHB1ENR::GPDMA1EN::SET);
    }

    pub fn enable_gpioa(&self) {
        self.registers.ahb2enr1.modify(AHB2ENR1::GPIOAEN::SET);
    }

    pub fn enable_gpioc(&self) {
        self.registers.ahb2enr1.modify(AHB2ENR1::GPIOCEN::SET);
    }

    pub fn enable_usart1(&self) {
        self.registers.apb2enr.modify(APB2ENR::USART1EN::SET);
    }

    pub fn enable_tim2(&self) {
        self.registers.apb1enr1.modify(APB1ENR1::TIM2EN::SET);
    }

    pub fn enable_tim3(&self) {
        self.registers.apb1enr1.modify(APB1ENR1::TIM3EN::SET);
    }

    pub fn enable_syscfg(&self) {
        self.registers.apb3enr.modify(APB3ENR::SYSCFGEN::SET);
    }

    pub fn enable_pwr(&self) {
        self.registers.ahb3enr.modify(AHB3ENR::PWREN::SET);
    }

    pub fn enable_hsi16(&self) {
        self.registers.cr.modify(CR::HSION::SET);

        // Wait for oscillator ready
        while !self.registers.cr.is_set(CR::HSIRDY) {}
    }

    pub fn enable_adc1(&self) {
        self.registers.ahb2enr1.modify(AHB2ENR1::ADC12EN::SET);
    }

    pub fn enable_trng(&self) {
        self.registers.ahb2enr1.modify(AHB2ENR1::TRNGEN::SET);
    }

    pub fn set_usart1_source_pclk(&self) {
        self.registers.ccipr1.modify(CCIPR1::USART1SEL::PCLK);
    }

    pub fn set_adcdacsel_source_hsi16(&self) {
        self.registers.ccipr3.modify(CCIPR3::ADCDACSEL::HSI16);
    }

    pub fn enable_dac1(&self) {
        self.registers.ahb3enr.modify(AHB3ENR::DAC1EN::SET);
    }

    pub fn enable_hash(&self) {
        self.registers.ahb2enr1.modify(AHB2ENR1::HASHEN::SET);
    }

    pub fn enable_fdcan(&self) {
        //enable HSI16
        self.registers.cr.modify(CR::HSION::SET);
        //wait for HSI16 to be ready
        while !self.registers.cr.is_set(CR::HSIRDY) {}
        //route HSI16 to PLL1
        self.registers.pll1cfgr.modify(PLL1CFGR::PLL1SRC::HSI16);
        //configure PLL1, 16mhz*8/16=8mhz out of pll1q
        self.registers.pll1cfgr.modify(PLL1CFGR::PLL1M.val(0));
        self.registers.pll1divr.modify(PLL1DIVR::PLL1N.val(7));
        self.registers.pll1divr.modify(PLL1DIVR::PLL1Q.val(15));
        //enable PLL1
        self.registers.pll1cfgr.modify(PLL1CFGR::PLL1QEN::SET);
        self.registers.pll1cfgr.modify(PLL1CFGR::PLL1RGE::HIGH);
        self.registers.cr.modify(CR::PLL1ON::SET);
        //wait for PLL1 to be ready
        while !self.registers.cr.is_set(CR::PLL1RDY) {}
        //route FDCAN1 to pll1q
        self.registers.ccipr1.modify(CCIPR1::FDCAN1SEL::PLL1_Q);
        //self.registers.ccipr1.modify(CCIPR1::FDCAN1SEL::HSE);
        self.registers.apb1enr2.modify(APB1ENR2::FDCAN1EN::SET);
    }
}
