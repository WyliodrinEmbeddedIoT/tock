//! SysReg driver.

use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

pub const SYS_REG_BASE: StaticRef<SysRegRegisters> =
    unsafe { StaticRef::new(0x600C_0000 as *const SysRegRegisters) };

register_structs! {
    pub SysRegRegisters {
        (0x000 => _reserved0),
        (0x008 => cpu_per_conf: ReadWrite<u32, CPU_PER_CONF::Register>),
        (0x00c => _reserved1),
        (0x010 => perip_clk_en0: ReadWrite<u32, PERIP_CLK_EN0::Register>),
        (0x014 => _reserved3),
        (0x018 => perip_rst_en0: ReadWrite<u32, PERIP_RST_EN0::Register>),
        (0x01C => _reserved4),
        (0x058 => sysclk_config: ReadWrite<u32, SYSCLK_CONFIG::Register>),
        (0x05C => _reserved_unimplemented_yet),
        (0x1000 => @END),
    }
}

register_bitfields![u32,
    PERIP_CLK_EN0 [
        LEDC OFFSET(11) NUMBITS(1) [],
        TIMERGROUP0 OFFSET(13) NUMBITS(1) [],
    ],
    PERIP_RST_EN0 [
        LEDC OFFSET(11) NUMBITS(1) []
    ],
    CPU_PER_CONF [
        CPUPERIOD_SEL OFFSET(0) NUMBITS(2) [
            MHz80 = 0,
            MHz160 = 1,
        ],
        PLL_FREQ_SEL OFFSET(2) NUMBITS(1) [
            MHz320 = 0,
            MHz480 = 1,
        ],
        CPU_WAIT_MODE_FORCE_ON OFFSET(3) NUMBITS(1) [],
        CPU_WAIT_DELAY_NUM OFFSET(4) NUMBITS(4) [],
    ],
    SYSCLK_CONFIG [
        PRE_DIV_CNT OFFSET(0) NUMBITS(10) [],
        SOC_CLK_SEL OFFSET(10) NUMBITS(2) [
            Xtal = 0,
            Pll = 1,
            RcFast = 2,
        ],
        CLK_XTAL_FREQ OFFSET(12) NUMBITS(6) [],
    ],
];

#[repr(u32)]
pub enum PllFrequency {
    MHz320 = 0,
    MHz480 = 1,
}

impl From<u32> for PllFrequency {
    fn from(value: u32) -> Self {
        match value {
            0 => PllFrequency::MHz320,
            1 => PllFrequency::MHz480,
            _ => unreachable!(),
        }
    }
}

#[repr(u32)]
pub enum CpuFrequency {
    MHz80 = 0,
    MHz160 = 1,
}

impl From<u32> for CpuFrequency {
    fn from(value: u32) -> Self {
        match value {
            0 => CpuFrequency::MHz80,
            1 => CpuFrequency::MHz160,
            _ => unreachable!(),
        }
    }
}

pub struct ClockPrescaler(u32);

impl ClockPrescaler {
    pub fn new(prescaler: u32) -> Self {
        ClockPrescaler(match prescaler >= 1024 {
            true => 1,
            false => prescaler,
        })
    }
}

pub enum CpuClock {
    Xtal(ClockPrescaler),
    Pll(PllFrequency, CpuFrequency),
    RcFast(ClockPrescaler),
}

pub struct SysReg {
    registers: StaticRef<SysRegRegisters>,
}

impl SysReg {
    pub const fn new() -> Self {
        SysReg {
            registers: SYS_REG_BASE,
        }
    }

    pub fn set_clock_source(&self, clock: CpuClock) {
        match clock {
            CpuClock::Xtal(ClockPrescaler(prescaler)) => self.registers.sysclk_config.modify(
                SYSCLK_CONFIG::SOC_CLK_SEL::Xtal + SYSCLK_CONFIG::PRE_DIV_CNT.val(prescaler as u32),
            ),
            CpuClock::Pll(pll_frequency, cpu_frequency) => {
                self.registers
                    .sysclk_config
                    .modify(SYSCLK_CONFIG::SOC_CLK_SEL::Pll);
                self.registers.cpu_per_conf.modify(
                    CPU_PER_CONF::PLL_FREQ_SEL.val(pll_frequency as u32)
                        + CPU_PER_CONF::CPUPERIOD_SEL.val(cpu_frequency as u32),
                );
            }
            CpuClock::RcFast(ClockPrescaler(prescaler)) => self.registers.sysclk_config.modify(
                SYSCLK_CONFIG::SOC_CLK_SEL::RcFast
                    + SYSCLK_CONFIG::PRE_DIV_CNT.val(prescaler as u32),
            ),
        }
    }

    pub fn get_clock_source(&self) -> Option<CpuClock> {
        match self
            .registers
            .sysclk_config
            .read_as_enum(SYSCLK_CONFIG::SOC_CLK_SEL)
        {
            Some(SYSCLK_CONFIG::SOC_CLK_SEL::Value::Xtal) => Some(CpuClock::Xtal(ClockPrescaler(
                self.registers
                    .sysclk_config
                    .read(SYSCLK_CONFIG::PRE_DIV_CNT),
            ))),
            Some(SYSCLK_CONFIG::SOC_CLK_SEL::Value::Pll) => Some(CpuClock::Pll(
                PllFrequency::from(self.registers.cpu_per_conf.read(CPU_PER_CONF::PLL_FREQ_SEL)),
                CpuFrequency::from(
                    self.registers
                        .cpu_per_conf
                        .read(CPU_PER_CONF::CPUPERIOD_SEL),
                ),
            )),
            Some(SYSCLK_CONFIG::SOC_CLK_SEL::Value::RcFast) => {
                Some(CpuClock::RcFast(ClockPrescaler(
                    self.registers
                        .sysclk_config
                        .read(SYSCLK_CONFIG::PRE_DIV_CNT),
                )))
            }
            None => None,
        }
    }

    //Enable the APB_CLK signal to LED PWM
    pub fn enable_ledc(&self) {
        self.registers
            .perip_clk_en0
            .modify(PERIP_CLK_EN0::LEDC::SET);
    }

    //Disable the APB_CLK signal to LED PWM
    pub fn disable_ledc(&self) {
        self.registers
            .perip_clk_en0
            .modify(PERIP_CLK_EN0::LEDC::CLEAR);
    }

    pub fn is_enabled_ledc(&self) -> bool {
        self.registers.perip_clk_en0.is_set(PERIP_CLK_EN0::LEDC)
    }

    //Reset the APB_CLK signal for LED PWM
    pub fn reset_ledc(&self) {
        self.registers
            .perip_rst_en0
            .modify(PERIP_RST_EN0::LEDC::SET);
    }

    pub fn enable_timg0(&self) {
        self.registers
            .perip_clk_en0
            .modify(PERIP_CLK_EN0::TIMERGROUP0::SET);
    }

    pub fn disable_timg0(&self) {
        self.registers
            .perip_clk_en0
            .modify(PERIP_CLK_EN0::TIMERGROUP0::CLEAR);
    }

    pub fn is_enabled_timg0(&self) -> bool {
        self.registers
            .perip_clk_en0
            .is_set(PERIP_CLK_EN0::TIMERGROUP0)
    }
}
