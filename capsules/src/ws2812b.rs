use core::time::Duration;
use kernel::debug;
use kernel::hil::gpio::Pin;
use kernel::hil::led::LedRGB;
use kernel::hil::time::SynchronousTime;



pub const TICKS_PER_US: u32 = 20; // freq/1000_000

pub const T0H:u32 = 4*5;
pub const T0L:u32 = 21*5;

pub const T1H:u32 = 16*5;
pub const T1L:u32 = 9*5;


pub struct WS2812B<'a, L: kernel::hil::gpio::Configure + kernel::hil::gpio::Output, T: kernel::hil::time::SynchronousTime > {
    pin: &'a L,
    time: &'a T,
}

impl<'a, L: kernel::hil::gpio::Configure + kernel::hil::gpio::Output, T: kernel::hil::time::SynchronousTime> WS2812B<'a, L, T> {
    pub fn new(
        pin: &'a L,
        time: &'a T

    ) -> Self {
        Self {
            pin,
            time,
        }
    }

    #[inline(always)]
    fn send_one(&self){
        self.pin.set();
        self.time.sleep(Duration::from_nanos(T1H as u64));
        self.pin.clear();
        self.time.sleep(Duration::from_nanos(T1L as u64));
    }

    #[inline(always)]
    fn send_zero(&self){
        self.pin.set();
        self.time.sleep(Duration::from_nanos(T0H as u64));
        self.pin.clear();
        self.time.sleep(Duration::from_nanos(T0L as u64));
    }

    fn show_color(&self, col:u32){
        let val_first_bit_and = 0b1000_0000__0000_0000__0000_0000;
        let mut j = 0;
        while  j < 30{
            self.pin.clear();
            self.time.sleep(Duration::from_nanos(100));
            let mut i = 0;
            let mut send = col;
            while i < 24 {
                if (send & val_first_bit_and) > 0 {
                    self.send_one();
                } else {
                    self.send_zero();
                }
            send = send << 1;
                i += 1;
            }
            self.pin.clear();
            j += 1;
        }
    }
}

impl <'a, L:kernel::hil::gpio::Configure + kernel::hil::gpio::Output, T:kernel::hil::time::SynchronousTime> LedRGB for WS2812B<'a, L, T> {
    /// Initialize the LED. Must be called before the LED is used.
    fn init(&self){
        self.pin.clear();
        self.pin.make_output();
        self.pin.clear();
    }

    /// Turn LED on with specified color.
    fn on(&self, red:u8, green:u8, blue:u8){
        let g:u32 = (green as u32)  << 16 as u32;
        let r:u32 = (red as u32) << 8  as u32;
        let b:u32 = blue as u32;
        let color:u32 = g + r + b;
        self.show_color(color);
    }

    /// Turn the LED off.
    fn off(&self){
        self.show_color(0);
    }
}







