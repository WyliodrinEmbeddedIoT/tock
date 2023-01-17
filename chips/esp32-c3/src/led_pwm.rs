use kernel::hil;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

pub const LEDC_BASE: StaticRef<LedPwmRegisters> =
    unsafe { StaticRef::new(0x6001_9000 as *const LedPwmRegisters) };

register_structs! {
    pub LedPwmChannel {
        (0x0000 => conf0: ReadWrite<u32, LEDC_CH_CONF0::Register>),
        (0x0004 => hpoint: ReadWrite<u32, LEDC_CH_HPOINT::Register>),
        (0x0008 => duty: ReadWrite<u32, LEDC_CH_DUTY::Register>),
        (0x000C => conf1: ReadWrite<u32, LEDC_CH_CONF1::Register>),
        (0x0010 => duty_r: ReadWrite<u32, LEDC_CH_DUTY_R::Register>),
        (0x0014 => @END),
    },
    pub LedPwmTimer {
        (0x0000 => conf: ReadWrite<u32, LEDC_TIMER_CONF::Register>),
        (0x0004 => value: ReadWrite<u32, LEDC_TIMER_VALUE::Register>),
        (0x0008 => @END),
    },
    pub LedPwmRegisters {
        (0x0000 => ledc_ch: [LedPwmChannel; 6]),
        (0x0078 => _reserved0),
        (0x00A0 => ledc_timer: [LedPwmTimer; 4]),
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
    // x used for timers, value between [0, 3]
    // n used for pwm generators, value between [0, 5]

    // naming suggestions are appreciated
    LEDC_CH_CONF0 [
        // used to select one of the timers on channel n
        // 0: select Timer0
        // 1: select Timer1
        // 2: select Timer2
        // 3: select Timer3
        TIMER_SEL_CH OFFSET(0) NUMBITS(2) [],
        // set to enable signal output on channel n
        SIG_OUT_EN_CH OFFSET(2) NUMBITS(1) [],
        // controls the output value when channel n is inactive(LEDC_SIG_OUT_EN_CHn == 0)
        IDLE_LV_CH OFFSET(3) NUMBITS(1) [
            Low = 0,
            High = 1,
        ],
        // used to update the listed fields below for channel n; is automatically cleared by hardware
        // HPOINT_CHn, DUTY_START_CHn, SIG_OUT_EN_CHn, TIMER_SEL_CHn, DUTY_NUM_CHn, DUTY_CYCLE_CHn,
        // DUTY_SCALE_CHn, DUTY_INC_CHn and OVF_CNT_EN_CHn
        PARA_UP_CH OFFSET(4) NUMBITS(1) [],
        // configures maximum times of overflow - 1
        // LEDC_OVF_CNT_CHn_INT will be triggered when channel n overflows for (LEDC_OVF_NUM_CHn + 1) times
        OVF_NUM_CH OFFSET(5) NUMBITS(10) [],
        // counts the number of times when the timer selected by channel n overflows
        OVF_CNT_EN_CH OFFSET(15) NUMBITS(1) [],
        // set to reset timer-overflow counter of channel n
        OVF_CNT_RESET_CH OFFSET(16) NUMBITS(1) [],
    ],
    LEDC_CH_CONF1 [
        // configures the step size of the duty cucle change during fading
        DUTY_SCALE_CH OFFSET(0) NUMBITS(10) [],
        // sets the number of counter overflow cycles for every Lpointn increment/decrement
        DUTY_CYCLE_CH OFFSET(10) NUMBITS(10) [],
        // controls the number of times the duty cycle will be incremented/decremented
        DUTY_NUM_CH OFFSET(20) NUMBITS(10) [],
        // determines the monotony of the duty cycle of the output signal on channel n
        // 0: Decreases
        // 1: Increases
        DUTY_INC_CH OFFSET(30) NUMBITS(1) [],
        // set to enable duty cycle fading on channel n
        DUTY_START_CH OFFSET(31) NUMBITS(1) [],
    ],
    LEDC_CONF [
        // used to select the clock source for all 4 timers
        // 01: APB_CLK
        // 10: RC_FAST_CLK
        // 11: XTAL_CLK
        APB_CLK_SEL OFFSET(0) NUMBITS(2) [
            APB_CLK = 0b01,
            RC_FAST_CLK = 0b10,
            XTAL_CLK = 0b11,
        ],
        // used to control clock
        // 0: support clock only when application writes registers
        // 1: force clock on register
        CLK_EN OFFSET(31) NUMBITS(1) [],
    ],
    LEDC_CH_HPOINT [
        // signal output value changes to high when the selected timer
        // has reached the value in this field
        HPOINT_CH OFFSET(0) NUMBITS(14) [],
    ],
    LEDC_CH_DUTY [
        // signal output value changes to low when the selected timer
        // has reached the value specified in this field
        DUTY_CH OFFSET(0) NUMBITS(19) [],
    ],
    LEDC_CH_DUTY_R [
        DUTY_R_CH OFFSET(0) NUMBITS(19) [],
    ],
    LEDC_TIMER_CONF [
        // used to control range of counter of timer x
        TIMER_DUTY_RES OFFSET(0) NUMBITS(4) [],
        // used to configure the divisor for the divider of timer x
        CLK_DIV_TIMER OFFSET(4) NUMBITS(18) [],
        // used to suspend the counter of timer x
        TIMER_PAUSE OFFSET(22) NUMBITS(1) [],
        // used to reset timer x; counter will show 0 after reset
        TIMER_RST OFFSET(23) NUMBITS(1) [],
        // set to update LEDC_CLK_DIV_TIMERx and LEDC_TIMERx_DUTY_RES
        TIMER_PARA_UP OFFSET(25) NUMBITS(1) [],
    ],
    LEDC_TIMER_VALUE [
        // stores the current counter value of timer x
        TIMER_CNT OFFSET(0) NUMBITS(14) [],
    ],
    LEDC_INT [
        // RAW:
        // triggered when timer x has reached its maximum counter value
        // ST:
        // masked interrupt status bit for the LEDC_TIMERx_OVF_INT interrupt
        // when LEDC_TIMERx_OVF_INT_ENA is set to 1
        // ENA:
        // set to enable LEDC_TIMERx_OVF_INT interrupt
        // CLR:
        // set to clear LEDC_TIMERx_OVF_INT interrupt
        TIMER0_OVF OFFSET(0) NUMBITS(1) [],
        TIMER1_OVF OFFSET(1) NUMBITS(1) [],
        TIMER2_OVF OFFSET(2) NUMBITS(1) [],
        TIMER3_OVF OFFSET(3) NUMBITS(1) [],
        // RAW:
        // interrupt raw bit for channel n
        // triggered when the gradual change of duty has finished
        // ST:
        // masked interrupt status bit for LEDC_DUTY_CHNG_END_CHn_INT interrupt
        // when LEDC_DUTY_CHNG_CHn_INT_ENA is set to 1
        // ENA:
        // set to enable LEDC_DUTY_CHNG_END_CHn_INT interrupt
        // CLR:
        // set to clear LEDC_DUTY_CHNG_END_CHn_INT interrupt
        DUTY_CHNG_END_CH0 OFFSET(4) NUMBITS(1) [],
        DUTY_CHNG_END_CH1 OFFSET(5) NUMBITS(1) [],
        DUTY_CHNG_END_CH2 OFFSET(6) NUMBITS(1) [],
        DUTY_CHNG_END_CH3 OFFSET(7) NUMBITS(1) [],
        DUTY_CHNG_END_CH4 OFFSET(8) NUMBITS(1) [],
        DUTY_CHNG_END_CH5 OFFSET(9) NUMBITS(1) [],
        // RAW:
        // interrupt raw bit for channel n
        // triggered when overflow counter has reached value specified by LEDC_OVF_NUM_CHn
        // ST:
        // masked interrupt status bit for LEDC_DUTY_CNT_CHn_INT interrupt
        // when LEDC_OVF_CNT_CHn_INT_ENA is set to 1
        // ENA:
        // set to enable LEDC_OVF_CHn_INT interrupt
        // CLR:
        // set to clear LEDC_OVF_CNT_CHn_INT interrupt
        OVF_CNT_CH0 OFFSET(10) NUMBITS(1) [],
        OVF_CNT_CH1 OFFSET(11) NUMBITS(1) [],
        OVF_CNT_CH2 OFFSET(12) NUMBITS(1) [],
        OVF_CNT_CH3 OFFSET(13) NUMBITS(1) [],
        OVF_CNT_CH4 OFFSET(14) NUMBITS(1) [],
        OVF_CNT_CH5 OFFSET(15) NUMBITS(1) [],
    ],
    LEDC_DATE [
        // version control register
        LEDC_DATE OFFSET(0) NUMBITS(32) [],
    ]
];

pub struct LedPwm {
    registers: StaticRef<LedPwmRegisters>,
}

impl LedPwm {
    pub const fn new() -> Self {
        LedPwm {
            registers: LEDC_BASE,
        }
    }

    pub fn handle_interrupt(&self) {
        todo!()
    }
}

impl hil::pwm::Pwm for LedPwm {
    type Pin = esp32::gpio::GpioPin<'static>;

    fn start(
        &self,
        pin: &Self::Pin,
        frequency_hz: usize,
        duty_cycle: usize,
    ) -> Result<(), ErrorCode> {
        todo!()
    }

    fn stop(&self, pin: &Self::Pin) -> Result<(), ErrorCode> {
        todo!()
    }

    fn get_maximum_frequency_hz(&self) -> usize {
        // frequency of a PWM generator output signal is given by:
        //
        // LEDC_CLKx / (LEDC_CLK_DIV_TIMERx * 2 ^ LEDC_TIMERx_DUTY_RES)
        //
        todo!()
    }

    fn get_maximum_duty_cycle(&self) -> usize {
        // the PWM can be set to high for the entire period of the signal,
        // resulting in a 100% duty cycle
        self.get_maximum_frequency_hz()
    }
}
