use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;
use kernel::ErrorCode;

pub const LED_PWM_BASE: StaticRef<LedPwmRegisters> =
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
    pub LedPwmRegisters { //<- possible naming or "LedcRegisters"
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
    //x used for timers, value between [0, 3]
    //n used for pwm generators, value between [0, 5]

    //naming suggestions are appreciated
    LEDC_CH_CONF0 [
        //used to select one of the timers on channel n
        //0: select Timer0
        //1: select Timer1
        //2: select Timer2
        //3: select Timer3
        TIMER_SEL_CH OFFSET(0) NUMBITS(2) [],
        //set to enable signal output on channel n
        SIG_OUT_EN_CH OFFSET(2) NUMBITS(1) [],
        //controls the output value when channel n is inactive(LEDC_SIG_OUT_EN_CHn == 0)
        IDLE_LV_CH OFFSET(3) NUMBITS(1) [],
        //used to update the listed fields below for channel n; is automatically cleared by hardware
        //HPOINT_CHn, DUTY_START_CHn, SIG_OUT_EN_CHn, TIMER_SEL_CHn, DUTY_NUM_CHn, DUTY_CYCLE_CHn,
        //DUTY_SCALE_CHn, DUTY_INC_CHn and OVF_CNT_EN_CHn
        PARA_UP_CH OFFSET(4) NUMBITS(1) [],
        //configures maximum times of overflow - 1
        //LEDC_OVF_CNT_CHn_INT will be triggered when channel n overflows for (LEDC_OVF_NUM_CHn + 1) times
        OVF_NUM_CH OFFSET(5) NUMBITS(10) [],
        //counts the number of times when the timer selected by channel n overflows
        OVF_CNT_EN_CH OFFSET(15) NUMBITS(1) [],
        //set to reset timer-overflow counter of channel n
        OVF_CNT_RESET_CH OFFSET(16) NUMBITS(1) [],
    ],
    LEDC_CH_CONF1 [
        //configures the step size of the duty cucle change during fading
        DUTY_SCALE_CH OFFSET(0) NUMBITS(10) [],
        //sets the number of counter overflow cycles for every Lpointn increment/decrement
        DUTY_CYCLE_CH OFFSET(10) NUMBITS(10) [],
        //controls the number of times the duty cycle will be incremented/decremented
        DUTY_NUM_CH OFFSET(20) NUMBITS(10) [],
        //determines the monotony of the duty cycle of the output signal on channel n
        //0: Decreases
        //1: Increases
        DUTY_INC_CH OFFSET(30) NUMBITS(1) [],
        //set to enable duty cycle fading on channel n
        DUTY_START_CH OFFSET(31) NUMBITS(1) [],
    ],
    LEDC_CONF [
        //used to select the clock source for all 4 timers
        //01: APB_CLK
        //10: FOSC_CLK
        //11: XTAL_CLK
        APB_CLK_SEL OFFSET(0) NUMBITS(2) [],
        //used to control clock
        //0: support clock only when application writes registers
        //1: force clock on register
        CLK_EN OFFSET(31) NUMBITS(1) [],
    ],
    LEDC_CH_HPOINT [
        //signal output value changes to high when the selected timer
        //has reached the value in this field
        HPOINT_CH OFFSET(0) NUMBITS(14) [],
    ],
    LEDC_CH_DUTY [
        //signal output value changes to low when the selected timer
        //has reached the value specified in this field
        DUTY_CH OFFSET(0) NUMBITS(19) [],
    ],
    LEDC_CH_DUTY_R [
        DUTY_R_CH OFFSET(0) NUMBITS(19) [],
    ],
    LEDC_TIMER_CONF [
        //used to control range of counter of timer x
        TIMER_DUTY_RES OFFSET(0) NUMBITS(4) [],
        //used to configure the divisor for the divider of timer x
        CLK_DIV_TIMER OFFSET(4) NUMBITS(18) [],
        //used to suspend the counter of timer x
        TIMER_PAUSE OFFSET(22) NUMBITS(1) [],
        //used to reset timer x; counter will show 0 after reset
        TIMER_RST OFFSET(23) NUMBITS(1) [],
        //set to update LEDC_CLK_DIV_TIMERx and LEDC_TIMERx_DUTY_RES
        TIMER_PARA_UP OFFSET(25) NUMBITS(1) [],
    ],
    LEDC_TIMER_VALUE [
        //stores the current counter value of timer x
        TIMER_CNT OFFSET(0) NUMBITS(14) [],
    ],
    LEDC_INT [
        //RAW:
        //triggered when timer x has reached its maximum counter value
        //ST:
        //masked interrupt status bit for the LEDC_TIMERx_OVF_INT interrupt
        //when LEDC_TIMERx_OVF_INT_ENA is set to 1
        //ENA:
        //set to enable LEDC_TIMERx_OVF_INT interrupt
        //CLR:
        //set to clear LEDC_TIMERx_OVF_INT interrupt
        TIMER0_OVF OFFSET(0) NUMBITS(1) [],
        TIMER1_OVF OFFSET(1) NUMBITS(1) [],
        TIMER2_OVF OFFSET(2) NUMBITS(1) [],
        TIMER3_OVF OFFSET(3) NUMBITS(1) [],
        //RAW:
        //interrupt raw bit for channel n
        //triggered when the gradual change of duty has finished
        //ST:
        //masked interrupt status bit for LEDC_DUTY_CHNG_END_CHn_INT interrupt
        //when LEDC_DUTY_CHNG_CHn_INT_ENA is set to 1
        //ENA:
        //set to enable LEDC_DUTY_CHNG_END_CHn_INT interrupt
        //CLR:
        //set to clear LEDC_DUTY_CHNG_END_CHn_INT interrupt
        DUTY_CHNG_END_CH0 OFFSET(4) NUMBITS(1) [],
        DUTY_CHNG_END_CH1 OFFSET(5) NUMBITS(1) [],
        DUTY_CHNG_END_CH2 OFFSET(6) NUMBITS(1) [],
        DUTY_CHNG_END_CH3 OFFSET(7) NUMBITS(1) [],
        DUTY_CHNG_END_CH4 OFFSET(8) NUMBITS(1) [],
        DUTY_CHNG_END_CH5 OFFSET(9) NUMBITS(1) [],
        //RAW:
        //interrupt raw bit for channel n
        //triggered when overflow counter has reached value specified by LEDC_OVF_NUM_CHn
        //ST:
        //masked interrupt status bit for LEDC_DUTY_CNT_CHn_INT interrupt
        //when LEDC_OVF_CNT_CHn_INT_ENA is set to 1
        //ENA:
        //set to enable LEDC_OVF_CHn_INT interrupt
        //CLR:
        //set to clear LEDC_OVF_CNT_CHn_INT interrupt
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

#[derive(Copy, Clone)]
pub enum Timer {
    Timer0 = 0,
    Timer1 = 1,
    Timer2 = 2,
    Timer3 = 3,
}

#[derive(Copy, Clone)]
pub enum Pwm {
    Pwm0 = 0,
    Pwm1 = 1,
    Pwm2 = 2,
    Pwm3 = 3,
    Pwm4 = 4,
    Pwm5 = 5,
}

pub enum ClockSource {
    Apb = 1,
    Fosc = 2,
    Xtal = 3,
}

pub struct LedPwm {
    registers: StaticRef<LedPwmRegisters>,
    //maybe add an array with the current specs for each timer and pwm ?
}

impl LedPwm {
    pub const fn new() -> Self {
        LedPwm {
            registers: LED_PWM_BASE,
        }
    }

    pub fn configure_clk_source(&self, clock: ClockSource) {
        self.registers
            .ledc_conf
            .modify(LEDC_CONF::APB_CLK_SEL.val(clock as u32));
    }

    //are the below Results needed ? panic or assert instead ?

    //LEDC_CLK_DIV_TIMERx = A + B / 256
    //A corresponds  to the most significant 10bits of LEDC_CLK_DIV_TIMERx
    //B corresponds  to the least significant 8bits of LEDC_CLK_DIV_TIMERx
    pub fn set_clock_diviser_value(&self, timer: Timer, a: u32, b: u32) -> Result<(), ErrorCode> {
        if a >= 1024 && b >= 256 {
            Err(ErrorCode::SIZE)
        } else {
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::CLK_DIV_TIMER.val((a << 2) + b));
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
            Ok(())
        }
    }

    //The counter for each timer counts up to 2 ^ (LEDC_TIMERx_DUTY_RES) − 1,
    //overflows and begins counting from 0 again

    //the new configuration takes effect upon next overflow;
    //implement the wait here or make the user somehow wait for the interrupt to be triggered ?
    //see if the Result is needed further
    pub fn configure_counter_range(&self, timer: Timer, exponent: u32) -> Result<(), ErrorCode> {
        if exponent >= 16 {
            Err(ErrorCode::SIZE)
        } else {
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::TIMER_DUTY_RES.val(exponent));
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
            Ok(())
        }
    }

    pub fn configure_counter_overflow(
        &self,
        pwm: Pwm,
        timer: Timer,
        counter_overflows: u32,
        timer_range_exponent: u32,
    ) -> Result<(), ErrorCode> {
        if counter_overflows >= 1024 {
            Err(ErrorCode::SIZE)
        } else {
            //set the given timer as timer for the given pwm
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::TIMER_SEL_CH.val(timer as u32));

            //enable the overflow counter
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::OVF_CNT_EN_CH::SET);

            //set the number of counter overflows to generate an interrupt - 1
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::OVF_NUM_CH.val(counter_overflows));

            //mask for indexing the counter overflow field for a specific channel
            let mask: u32 = 0b0000_0000_0000_0000_0000_0100_0000_0000 << (pwm as usize);
            //enable the counter overflow
            self.registers
                .ledc_int_ena
                .set(self.registers.ledc_int_ena.get() | mask);

            if let Err(e) = self.configure_counter_range(timer, timer_range_exponent) {
                return Err(e);
            };

            //update the new configuration
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
            Ok(())
        }
    }

    pub fn set_pwm_timer(&self, pwm: Pwm, timer: Timer) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::TIMER_SEL_CH.val(timer as u32));
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    //if Timerx_cnt == high_point => sig_outn = 1
    //if Timerx_cnt == lowpoint => sig_outn = 0
    pub fn configure_fixed_duty_cycle(
        &self,
        pwm: Pwm,
        high_point: u32,
        low_point: u32,
    ) -> Result<(), ErrorCode> {
        if high_point >= 16384 || low_point >= 32768 {
            Err(ErrorCode::SIZE)
        } else if high_point > low_point {
            Err(ErrorCode::INVAL)
        } else {
            self.registers.ledc_ch[pwm as usize]
                .hpoint
                .modify(LEDC_CH_HPOINT::HPOINT_CH.val(high_point));
            self.registers.ledc_ch[pwm as usize]
                .duty
                .modify(LEDC_CH_DUTY::DUTY_CH.val(low_point << 4));
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
            Ok(())
        }
    }

    pub fn enable_signal_output(&self, pwm: Pwm) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::SIG_OUT_EN_CH::SET);

        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn disable_signal_output(&self, pwm: Pwm, idle_level: bool) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::SIG_OUT_EN_CH::CLEAR);
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(if idle_level {
                LEDC_CH_CONF0::IDLE_LV_CH::SET
            } else {
                LEDC_CH_CONF0::IDLE_LV_CH::CLEAR
            });
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn set_special_cycles(&self, pwm: Pwm, cycles_number: u32) -> Result<(), ErrorCode> {
        if cycles_number >= 16 {
            Err(ErrorCode::SIZE)
        } else {
            self.registers.ledc_ch[pwm as usize]
                .duty
                .modify(LEDC_CH_DUTY::DUTY_CH.val(cycles_number));
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
            Ok(())
        }
    }

    pub fn enable_duty_cycle_fading(&self, pwm: Pwm) {
        self.registers.ledc_ch[pwm as usize]
            .conf1
            .modify(LEDC_CH_CONF1::DUTY_START_CH::SET);

        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn disable_duty_cycle_fading(&self, pwm: Pwm) {
        self.registers.ledc_ch[pwm as usize]
            .conf1
            .modify(LEDC_CH_CONF1::DUTY_START_CH::CLEAR);

        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn configure_fading_duty_cycle(
        &self,
        pwm: Pwm,
        high_point: u32,
        low_point: u32,
        counter_overflow_cycles: u32,
        step: u32,
        steps_number: u32,
        monotony: i8,
    ) -> Result<(), ErrorCode> {
        if counter_overflow_cycles >= 1024 || step >= 1024 || steps_number >= 1024 {
            Err(ErrorCode::SIZE)
        } else {
            //set the high and low point of the soon-to-be fading duty cycle
            if let Err(e) = self.configure_fixed_duty_cycle(pwm, high_point, low_point) {
                return Err(e);
            };

            //Lpointn will be incremented/decremented after DUTY_CYCLE_CHn counter overflows
            self.registers.ledc_ch[pwm as usize]
                .conf1
                .modify(LEDC_CH_CONF1::DUTY_CYCLE_CH.val(counter_overflow_cycles));

            self.registers.ledc_ch[pwm as usize]
                .conf1
                .modify(if monotony > 0 {
                    LEDC_CH_CONF1::DUTY_INC_CH::SET
                } else {
                    LEDC_CH_CONF1::DUTY_INC_CH::CLEAR
                });

            self.enable_duty_cycle_fading(pwm);

            self.registers.ledc_ch[pwm as usize]
                .conf1
                .modify(LEDC_CH_CONF1::DUTY_SCALE_CH.val(step));

            self.registers.ledc_ch[pwm as usize]
                .conf1
                .modify(LEDC_CH_CONF1::DUTY_NUM_CH.val(steps_number));

            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
            Ok(())
        }
    }
}
