use kernel::hil;
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
        IDLE_LV_CH OFFSET(3) NUMBITS(1) [
            Low = 0,
            High = 1,
        ],
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
pub enum PwmChannel {
    Pwm0 = 0,
    Pwm1 = 1,
    Pwm2 = 2,
    Pwm3 = 3,
    Pwm4 = 4,
    Pwm5 = 5,
}

pub enum ClockSource {
    Apb = 1,
    RcFast = 2,
    Xtal = 3,
}

//Duty cycles can be of two types
pub enum DutyCycle {
    Fixed(u32, u32),
    //with a payload used to store:
    // - highpoint value
    // - lowpoint value
    Fading(u32, u32, u32, i32, u32, u32),
    //with a payload used to store:
    // - highpoint value
    // - lowpoint value
    // - the number of counter overflows after which the lowpoint value will be incremented/decremented
    // - (+/-)1 value that determines whether the duty cycle output signal increases or decreases
    // - step size of the duty cycle change
    // - the number of times the duty cycle will be changed
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

    // functions that configure the timers

    pub fn set_clk_source(&self, clock: ClockSource) {
        self.registers
            .ledc_conf
            .modify(LEDC_CONF::APB_CLK_SEL.val(clock as u32));
    }

    pub fn configure_timer_divider(&self, timer: Timer, a: u32, b: u32) -> Result<(), ErrorCode> {
        if a >= 1024 || b >= 256 {
            Err(ErrorCode::SIZE)
        } else {
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::CLK_DIV_TIMER.val((a << 12) + (b << 4)));
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
            todo!("^This will cause the newly configured values to take effect upon the next overflow of the counter");
            Ok(())
        }
    }

    pub fn suspend_timer_counter(&self, timer: Timer) {
        self.registers.ledc_timer[timer as usize]
            .conf
            .modify(LEDC_TIMER_CONF::TIMER_PAUSE::SET);
    }

    pub fn resume_timer_counter(&self, timer: Timer) {
        self.registers.ledc_timer[timer as usize]
            .conf
            .modify(LEDC_TIMER_CONF::TIMER_PAUSE::CLEAR);
    }

    pub fn reset_timer_counter(&self, timer: Timer) {
        self.registers.ledc_timer[timer as usize]
            .conf
            .modify(LEDC_TIMER_CONF::TIMER_RST::SET);
        todo!("is it cleared by hardware or should i do it manually ?");
    }

    pub fn configure_timer_counter(
        &self,
        timer: Timer,
        overflow_value: u32,
    ) -> Result<(), ErrorCode> {
        if overflow_value >= 16 {
            Err(ErrorCode::SIZE)
        } else {
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::TIMER_DUTY_RES.val(overflow_value));
            self.registers.ledc_timer[timer as usize]
                .conf
                .modify(LEDC_TIMER_CONF::TIMER_PARA_UP::SET);
            todo!("This will cause the newly configured values to take effect upon the next overflow of the counter.");
            Ok(())
        }
    }

    pub fn configure_timer(
        &self,
        timer: Timer,
        a: u32,
        b: u32,
        overflow_value: u32,
    ) -> Result<(), ErrorCode> {
        if let Err(e) = self.configure_timer_divider(timer, a, b) {
            return Err(e);
        }
        if let Err(e) = self.configure_timer_counter(timer, overflow_value) {
            return Err(e);
        }
        Ok(())
    }

    // functions that configure the pwm generator

    pub fn set_pwm_timer(&self, pwm: PwmChannel, timer: Timer) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::TIMER_SEL_CH.val(timer as u32));
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
        todo!("This will cause the newly configured values to take effect upon the next overflow of the counter.");
    }

    pub fn enable_overflow_counting(&self, pwm: PwmChannel) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::OVF_CNT_EN_CH::SET);
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn disable_overflows_counting(&self, pwm: PwmChannel) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::OVF_CNT_EN_CH::CLEAR);
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn reset_timer_overflow_counter(&self, pwm: PwmChannel) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::OVF_CNT_RESET_CH::SET);
        todo!("is it cleared by hardware or should i do it manually ?");
    }

    pub fn set_overflows_to_generate_interrupt(
        &self,
        pwm: PwmChannel,
        nr: u32,
    ) -> Result<(), ErrorCode> {
        if nr >= 1024 {
            Err(ErrorCode::SIZE)
        } else {
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::OVF_NUM_CH.val(nr));
            Ok(())
        }
    }

    pub fn enable_overflows_interrupt(&self, pwm: PwmChannel) {
        let mask: u32 = 0b0000_0000_0000_0000_0000_0100_0000_0000 << (pwm as u32);
        self.registers
            .ledc_int_ena
            .set(self.registers.ledc_int_ena.get() | mask);
    }

    pub fn disable_overflows_interrupt(&self, pwm: PwmChannel) {
        let p = pwm as u32;
        let mask: u32 = (!0b0000_0000_0000_0000_0000_0100_0000_0000 << p) + (u32::pow(2, p) - 1);
        self.registers
            .ledc_int_ena
            .set(self.registers.ledc_int_ena.get() & mask);
    }

    pub fn configure_pwm_fixed_duty_cycle(
        &self,
        pwm: PwmChannel,
        high_point: u32,
        low_point: u32,
    ) -> Result<(), ErrorCode> {
        if high_point >= 16384 || low_point >= 49151 {
            Err(ErrorCode::SIZE)
        } else if high_point > low_point {
            Err(ErrorCode::INVAL)
        } else {
            self.registers.ledc_ch[pwm as usize]
                .hpoint
                .modify(LEDC_CH_HPOINT::HPOINT_CH.val(high_point));
            self.registers.ledc_ch[pwm as usize]
                .duty
                .modify(LEDC_CH_DUTY::DUTY_CH.val((low_point - high_point) << 4));
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
            todo!("This will cause the newly configured values to take effect upon the next overflow of the counter.");
            Ok(())
        }
    }

    pub fn enable_pwm_signal_output(&self, pwm: PwmChannel) {
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::SIG_OUT_EN_CH::SET);
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
        todo!("This will cause the newly configured values to take effect upon the next overflow of the counter.");
    }

    pub fn disable_pwm_signal_output(&self, pwm: PwmChannel, lvl: bool) {
        self.registers.ledc_ch[pwm as usize].conf0.modify(
            LEDC_CH_CONF0::SIG_OUT_EN_CH::CLEAR
                + if lvl {
                    LEDC_CH_CONF0::IDLE_LV_CH::Value::High.into()
                } else {
                    LEDC_CH_CONF0::IDLE_LV_CH::Value::Low.into()
                },
        );
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn dither_pwm_duty_cycles(&self, pwm: PwmChannel, nr: u32) -> Result<(), ErrorCode> {
        if nr >= 16 {
            Err(ErrorCode::SIZE)
        } else {
            self.registers.ledc_ch[pwm as usize]
                .duty
                .modify(LEDC_CH_DUTY::DUTY_CH.val(nr));
            Ok(())
        }
    }

    pub fn enable_fading_duty(&self, pwm: PwmChannel) {
        self.registers.ledc_ch[pwm as usize]
            .conf1
            .modify(LEDC_CH_CONF1::DUTY_START_CH::SET);
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn disable_fading_duty(&self, pwm: PwmChannel) {
        self.registers.ledc_ch[pwm as usize]
            .conf1
            .modify(LEDC_CH_CONF1::DUTY_START_CH::CLEAR);
        self.registers.ledc_ch[pwm as usize]
            .conf0
            .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
    }

    pub fn configure_pwm_fading_duty_cycle(
        &self,
        pwm: PwmChannel,
        high_point: u32,
        low_point: u32,
        counter_overflows: u32,
        duty_monotony: i32,
        duty_step: u32,
        steps_number: u32,
    ) -> Result<(), ErrorCode> {
        if counter_overflows >= 1024 || duty_step >= 1024 || steps_number >= 1024 {
            Err(ErrorCode::SIZE)
        } else {
            if let Err(e) = self.configure_pwm_fixed_duty_cycle(pwm, high_point, low_point) {
                return Err(e);
            }
            self.registers.ledc_ch[pwm as usize].conf1.modify(
                LEDC_CH_CONF1::DUTY_CYCLE_CH.val(counter_overflows)
                    + if duty_monotony < 0 {
                        LEDC_CH_CONF1::DUTY_INC_CH::CLEAR
                    } else {
                        LEDC_CH_CONF1::DUTY_INC_CH::SET
                    }
                    + LEDC_CH_CONF1::DUTY_SCALE_CH.val(duty_step)
                    + LEDC_CH_CONF1::DUTY_NUM_CH.val(steps_number),
            );
            self.registers.ledc_ch[pwm as usize]
                .conf0
                .modify(LEDC_CH_CONF0::PARA_UP_CH::SET);
            Ok(())
        }
    }

    pub fn configure_pwm(&self, pwm: PwmChannel, duty_cycle: DutyCycle) -> Result<(), ErrorCode> {
        match duty_cycle {
            DutyCycle::Fixed(a, b) => {
                if let Err(e) = self.configure_pwm_fixed_duty_cycle(pwm, a, b) {
                    Err(e)
                } else {
                    Ok(())
                }
            }
            DutyCycle::Fading(a, b, c, d, e, f) => {
                if let Err(e) = self.configure_pwm_fading_duty_cycle(pwm, a, b, c, d, e, f) {
                    Err(e)
                } else {
                    Ok(())
                }
            }
        }
    }

    // functions regarding interrupts

    pub fn handle_interrupt(&self) {
        //how do i signal which interrupt to handle
        todo!()
    }

    pub fn counter_overflow_interrupt_is_triggered(&self, timer: Timer) -> bool {
        let mask: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001 << (timer as usize);
        self.registers.ledc_int_raw.get() & mask != 0
    }

    pub fn counter_overflow_interrupt_st_reg(&self, timer: Timer) {
        todo!("not sure what a masked interrupt is and what to do with it")
    }

    pub fn enable_counter_overflow_interrupt(&self, timer: Timer) {
        let mask: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0001 << (timer as u32);
        self.registers
            .ledc_int_ena
            .set(self.registers.ledc_int_ena.get() | mask);
    }

    pub fn clear_counter_overflow_interrupt(&self, timer: Timer) {
        let t = timer as u32;
        let mask: u32 = (0b1111_1111_1111_1111_1111_1111_1111_1110 << t) + (u32::pow(2, t) - 1);
        self.registers
            .ledc_int_clr
            .set(self.registers.ledc_int_clr.get() & mask);
    }
}

impl hil::pwm::Pwm for LedPwm {
    type Pin = u32;

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
        // highest LEDC_CLK freq is 80MHz, given by selecting APB_CLK as timer clock source
        // when using PLL_CLK as clock source for the CPU
        80000000
    }

    fn get_maximum_duty_cycle(&self) -> usize {
        // the PWM can be set to high for the entire period of the signal,
        // resulting in a 100% duty cycle
        80000000
    }
}
