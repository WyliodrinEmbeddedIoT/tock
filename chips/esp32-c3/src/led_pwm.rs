use kernel::utilities::registers::interfaces::ReadWriteable;
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

pub const LED_PWM_BASE: StaticRef<LedPwmRegisters> =
    unsafe { StaticRef::new(0x6001_9000 as *const LedPwmRegisters) };

register_structs! {
    pub LedPwmRegisters { //<- possible naming or "LedcRegisters"
        (0x0000 => ledc_ch0_conf0: ReadWrite<u32, LEDC_CH_CONF0::Register>),
        (0x0004 => ledc_ch0_hpoint: ReadWrite<u32, LEDC_CH_HPOINT::Register>),
        (0x0008 => ledc_ch0_duty: ReadWrite<u32, LEDC_CH_DUTY::Register>),
        (0x000C => ledc_ch0_conf1: ReadWrite<u32, LEDC_CH_CONF1::Register>),
        (0x0010 => ledc_ch0_duty_r: ReadWrite<u32, LEDC_CH_DUTY_R::Register>),
        (0x0014 => ledc_ch1_conf0: ReadWrite<u32, LEDC_CH_CONF0::Register>),
        (0x0018 => ledc_ch1_hpoint: ReadWrite<u32, LEDC_CH_HPOINT::Register>),
        (0x001C => ledc_ch1_duty: ReadWrite<u32, LEDC_CH_DUTY::Register>),
        (0x0020 => ledc_ch1_conf1: ReadWrite<u32, LEDC_CH_CONF1::Register>),
        (0x0024 => ledc_ch1_duty_r: ReadWrite<u32, LEDC_CH_DUTY_R::Register>),
        (0x0028 => ledc_ch2_conf0: ReadWrite<u32, LEDC_CH_CONF0::Register>),
        (0x002C => ledc_ch2_hpoint: ReadWrite<u32, LEDC_CH_HPOINT::Register>),
        (0x0030 => ledc_ch2_duty: ReadWrite<u32, LEDC_CH_DUTY::Register>),
        (0x0034 => ledc_ch2_conf1: ReadWrite<u32, LEDC_CH_CONF1::Register>),
        (0x0038 => ledc_ch2_duty_r: ReadWrite<u32, LEDC_CH_DUTY_R::Register>),
        (0x003C => ledc_ch3_conf0: ReadWrite<u32, LEDC_CH_CONF0::Register>),
        (0x0040 => ledc_ch3_hpoint: ReadWrite<u32, LEDC_CH_HPOINT::Register>),
        (0x0044 => ledc_ch3_duty: ReadWrite<u32, LEDC_CH_DUTY::Register>),
        (0x0048 => ledc_ch3_conf1: ReadWrite<u32, LEDC_CH_CONF1::Register>),
        (0x004C => ledc_ch3_duty_r: ReadWrite<u32, LEDC_CH_DUTY_R::Register>),
        (0x0050 => ledc_ch4_conf0: ReadWrite<u32, LEDC_CH_CONF0::Register>),
        (0x0054 => ledc_ch4_hpoint: ReadWrite<u32, LEDC_CH_HPOINT::Register>),
        (0x0058 => ledc_ch4_duty: ReadWrite<u32, LEDC_CH_DUTY::Register>),
        (0x005C => ledc_ch4_conf1: ReadWrite<u32, LEDC_CH_CONF1::Register>),
        (0x0060 => ledc_ch4_duty_r: ReadWrite<u32, LEDC_CH_DUTY_R::Register>),
        (0x0064 => ledc_ch5_conf0: ReadWrite<u32, LEDC_CH_CONF0::Register>),
        (0x0068 => ledc_ch5_hpoint: ReadWrite<u32, LEDC_CH_HPOINT::Register>),
        (0x006C => ledc_ch5_duty: ReadWrite<u32, LEDC_CH_DUTY::Register>),
        (0x0070 => ledc_ch5_conf1: ReadWrite<u32, LEDC_CH_CONF1::Register>),
        (0x0074 => ledc_ch5_duty_r: ReadWrite<u32, LEDC_CH_DUTY_R::Register>),
        (0x0078 => _reserved0),
        (0x00A0 => ledc_timer0_conf: ReadWrite<u32, LEDC_TIMER_CONF::Register>),
        (0x00A4 => ledc_timer0_value: ReadWrite<u32, LEDC_TIMER_VALUE::Register>),
        (0x00A8 => ledc_timer1_conf: ReadWrite<u32, LEDC_TIMER_CONF::Register>),
        (0x00AC => ledc_timer1_value: ReadWrite<u32, LEDC_TIMER_VALUE::Register>),
        (0x00B0 => ledc_timer2_conf: ReadWrite<u32, LEDC_TIMER_CONF::Register>),
        (0x00B4 => ledc_timer2_value: ReadWrite<u32, LEDC_TIMER_VALUE::Register>),
        (0x00B8 => ledc_timer3_conf: ReadWrite<u32, LEDC_TIMER_CONF::Register>),
        (0x00BC => ledc_timer3_value: ReadWrite<u32, LEDC_TIMER_VALUE::Register>),
        (0x00C0 => ledc_int_raw: ReadWrite<u32, LEDC_INT::Register>),
        (0x00C4 => ledc_int_st: ReadWrite<u32, LEDC_INT::Register>),
        (0x00C8 => ledc_int_ena: ReadWrite<u32, LEDC_INT::Register>),
        (0x00CC => ledc_int_clr: ReadWrite<u32, LEDC_INT::Register>),
        (0x00D0 => ledc_conf: ReadWrite<u32, LEDC_CONF::Register>),
        (0x00D4 => _reserved1),
        (0x00FC => ledc_date: ReadWrite<u32, LEDC_DATE::Register>),
        (0x0100 => @END),
    }
}

register_bitfields! [u32,
    //($x) value used for timers, value between [0, 3]
    //($n) value used for pwm generators, value between [0, 5]

    //naming suggestions are appreciated
    LEDC_CH_CONF0 [
        //used to select one of the timers on channel ($n)
        //($n): select Timer($n)
        TIMER_SEL_CH OFFSET(0) NUMBITS(2) [],
        //set to enable signal output on channel ($n)
        SIG_OUT_EN_CH OFFSET(2) NUMBITS(1) [],
        //used to control the output value when channel ($n) is inactive(LEDC_SIG_OUT_EN_CH($n) == 0)
        IDLE_LV_CH OFFSET(3) NUMBITS(1) [],
        //used to update the listed fields below for channel ($n); is automatically cleared by hardware
        //HPOINT_CHn, DUTY_START_CHn, SIG_OUT_EN_CHn, TIMER_SEL_CHn, DUTY_NUM_CHn, DUTY_CYCLE_CHn,
        //DUTY_SCALE_CHn, DUTY_INC_CHn and OVF_CNT_EN_CHn
        PARA_UP_CH OFFSET(4) NUMBITS(1) [],
        //used to configure maximum times of overflow - 1
        //LEDC_OVF_CNT_CH($n)_INT will be triggered when channel ($n) overflows for (LEDC_OVF_NUM_CH($n) + 1) times
        OVF_NUM_CH OFFSET(5) NUMBITS(10) [],
        //used to count the number of times when the timer selected by channel ($n) overflows
        OVF_CNT_EN_CH OFFSET(15) NUMBITS(1) [],
        //set to reset timer-overflow counter of channel ($n)
        OVF_CNT_RESET_CH OFFSET(16) NUMBITS(1) [],
    ],
    LEDC_CH_CONF1 [
        DUTY_SCALE_CH OFFSET(0) NUMBITS(10) [],
        DUTY_CYCLE_CH OFFSET(10) NUMBITS(10) [],
        DUTY_NUM_CH OFFSET(20) NUMBITS(10) [],
        DUTY_INC_CH OFFSET(30) NUMBITS(1) [],
        DUTY_START_CH OFFSET(31) NUMBITS(1) [],
    ],
    LEDC_CONF [
        //used to select the clock source for all 4 timers
        APB_CLK_SEL OFFSET(0) NUMBITS(2) [
            APB_CLK = 1,
            FOSC_CLK = 2,
            XTAL_CLK = 3,
        ],
        //used to control clock
        //0: support clock only when application writes registers
        //1: force clock on register
        CLK_EN OFFSET(31) NUMBITS(1) [],
    ],
    LEDC_CH_HPOINT [
        HPOINT_CH OFFSET(0) NUMBITS(14) [],
    ],
    LEDC_CH_DUTY [
        DUTY_CH OFFSET(0) NUMBITS(19) [],
    ],
    LEDC_CH_DUTY_R [
        DUTY_R_CH OFFSET(0) NUMBITS(19) [],
    ],
    LEDC_TIMER_CONF [
        //used to control range of counter of timer ($x)
        TIMER_DUTY_RES OFFSET(0) NUMBITS(4) [],
        //used to configure the divisor for the divider of timer ($x)
        CLK_DIV_TIMER OFFSET(4) NUMBITS(18) [],
        //used to suspend the counter of timer ($x)
        TIMER_PAUSE OFFSET(22) NUMBITS(1) [],
        //used to reset timer ($x); counter will show 0 after reset
        TIMER_RST OFFSET(23) NUMBITS(1) [],
        //set to update LEDC_CLK_DIV_TIMER($x) and LEDC_TIMER($x)_DUTY_RES
        TIMER_PARA_UP OFFSET(25) NUMBITS(1) [],
    ],
    LEDC_TIMER_VALUE [
        //stores the current counter value of timer ($x)
        TIMER_CNT OFFSET(0) NUMBITS(14) [],
    ],
    LEDC_INT [
        //RAW:
        //triggered when timer ($x) has reached its maximum counter value
        //ST:
        //masked interrupt status bit for the LEDC_TIMER($x)_OVF_INT interrupt
        //when LEDC_TIMER($x)_OVF_INT_ENA is set to 1
        //ENA:
        //set to enable LEDC_TIMER($x)_OVF_INT interrupt
        //CLR:
        //set to clear LEDC_TIMER($x)_OVF_INT interrupt
        TIMER0_OVF OFFSET(0) NUMBITS(1) [],
        TIMER1_OVF OFFSET(1) NUMBITS(1) [],
        TIMER2_OVF OFFSET(2) NUMBITS(1) [],
        TIMER3_OVF OFFSET(3) NUMBITS(1) [],
        //RAW:
        //interrupt raw bit for channel ($n)
        //triggered when the gradual change of duty has finished
        //ST:
        //masked interrupt status bit for LEDC_DUTY_CHNG_END_CH($n)_INT interrupt
        //when LEDC_DUTY_CHNG_CH($n)_INT_ENA is set to 1
        //ENA:
        //set to enable LEDC_DUTY_CHNG_END_CH($n)_INT interrupt
        //CLR:
        //set to clear LEDC_DUTY_CHNG_END_CH($n)_INT interrupt
        DUTY_CHNG_END_CH0 OFFSET(4) NUMBITS(1) [],
        DUTY_CHNG_END_CH1 OFFSET(5) NUMBITS(1) [],
        DUTY_CHNG_END_CH2 OFFSET(6) NUMBITS(1) [],
        DUTY_CHNG_END_CH3 OFFSET(7) NUMBITS(1) [],
        DUTY_CHNG_END_CH4 OFFSET(8) NUMBITS(1) [],
        DUTY_CHNG_END_CH5 OFFSET(9) NUMBITS(1) [],
        //RAW:
        //interrupt raw bit for channel ($n)
        //triggered when overflow counter has reached value specified by LEDC_OVF_NUM_CH($n)
        //ST:
        //masked interrupt status bit for LEDC_DUTY_CNT_CH($n)_INT interrupt
        //when LEDC_OVF_CNT_CH($n)_INT_ENA is set to 1
        //ENA:
        //set to enable LEDC_OVF_CH($n)_INT interrupt
        //CLR:
        //set to clear LEDC_OVF_CNT_CH($n)_INT interrupt
        OVF_CNT_CH0 OFFSET(10) NUMBITS(1) [],
        OVF_CNT_CH1 OFFSET(11) NUMBITS(1) [],
        OVF_CNT_CH2 OFFSET(12) NUMBITS(1) [],
        OVF_CNT_CH3 OFFSET(13) NUMBITS(1) [],
        OVF_CNT_CH4 OFFSET(14) NUMBITS(1) [],
        OVF_CNT_CH5 OFFSET(15) NUMBITS(1) [],
    ],
    LEDC_DATE [
        //version control register
        LEDC_DATE OFFSET(0) NUMBITS(32) [],
    ]
];

struct LedPwm {
    registers: StaticRef<LedPwmRegisters>,
}

impl LedPwm {
    pub const fn new() -> Self {
        LedPwm {
            registers: LED_PWM_BASE,
        }
    }

    pub fn use_apb_clock_source(&self) {
        self.registers
            .ledc_conf
            .modify(LEDC_CONF::APB_CLK_SEL::APB_CLK);
    }

    pub fn use_fosc_clk_source(&self) {
        self.registers
            .ledc_conf
            .modify(LEDC_CONF::APB_CLK_SEL::FOSC_CLK);
    }

    pub fn use_xtal_clk_source(&self) {
        self.registers
            .ledc_conf
            .modify(LEDC_CONF::APB_CLK_SEL::XTAL_CLK);
    }

    pub fn configure_clk_div_timer(
        &self,
        timer_number: u32,
        //actual max size of 'a' is 10bits and 'b' is 8bits, but declared as u32 to avoid type mismatch
        //could use u16 and u8 respectively and use 'as u32' in context; worth it ?
        a: u32,
        b: u32,
    ) -> Result<(), ErrorCode> {
        //^is the Result needed ?
        if a >= 1024 && b >= 256 {
            //implemented as to fit in the bitfield
            //SIZE or INVAL error ?
            Err(ErrorCode::SIZE)
        } else {
            match timer_number {
                0 => {
                    self.registers
                        .ledc_timer0_conf
                        .modify(LEDC_TIMER_CONF::CLK_DIV_TIMER.val((a << 2) + b));
                    self.registers
                        .ledc_timer0_conf
                        .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
                    Ok(())
                }
                1 => {
                    self.registers
                        .ledc_timer1_conf
                        .modify(LEDC_TIMER_CONF::CLK_DIV_TIMER.val((a << 2) + b));
                    self.registers
                        .ledc_timer1_conf
                        .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
                    Ok(())
                }
                2 => {
                    self.registers
                        .ledc_timer2_conf
                        .modify(LEDC_TIMER_CONF::CLK_DIV_TIMER.val((a << 2) + b));
                    self.registers
                        .ledc_timer2_conf
                        .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
                    Ok(())
                }
                3 => {
                    self.registers
                        .ledc_timer3_conf
                        .modify(LEDC_TIMER_CONF::CLK_DIV_TIMER.val((a << 2) + b));
                    self.registers
                        .ledc_timer3_conf
                        .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
                    Ok(())
                }
                _ => Err(ErrorCode::INVAL),
            }
        }
    }
}
