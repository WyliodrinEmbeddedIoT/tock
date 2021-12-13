use kernel::debug;
use kernel::hil::gpio::Pin;
use kernel::hil::led::LedRGB;
use kernel::hil::time::{Ticks, Time};




pub const TICKS_PER_US: u32 = 20; // freq/1000_000

pub const T0H:u32 = 4;
pub const T0L:u32 = 21;

pub const T1H:u32 = 16;
pub const T1L:u32 = 9;





pub struct WS2812B<'a, L: kernel::hil::gpio::Configure + kernel::hil::gpio::Output, T: kernel::hil::time::Time > {
    pin: &'a L,
    time: &'a T,
}

impl<'a, L: kernel::hil::gpio::Configure + kernel::hil::gpio::Output, T: kernel::hil::time::Time> WS2812B<'a, L, T> {
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
    fn delay_ticks(&self, ticks:u32) {
        let now: u32 = self.time.now().into_u32();
        let until:u32 = now + ticks;
        while self.time.now().into_u32()< until {}
    }

    #[inline(always)]
    fn delay_nop(&self, mut num_nops:u32) {
        while num_nops>0 {
            rv32i::support::nop();
            num_nops-=1; //?
        }
    }

    #[inline(always)]
    fn delay_ns(&self, ns:u32) {
        let now: u32 = self.time.now().into_u32();
        //let until:u32 = now + ns/50;
        let until:u32 = now + ns/10;
        while self.time.now().into_u32()< until {}
    }

    #[inline(always)]
    fn delay_us(&self, us:u32) {
        let now: u32 = self.time.now().into_u32();
        let until:u32 = now + us*TICKS_PER_US;
        while self.time.now().into_u32()< until {}
    }


    #[inline(always)]
    fn send_one(&self){
        self.pin.set();
        self.delay_nop(T1H);
        self.pin.clear();
        self.delay_nop(T1L);
    }

    #[inline(always)]
    fn send_zero(&self){
        self.pin.set();
        //self.delay_ticks(T0H);
        self.delay_nop(T0H);
        self.pin.clear();
        self.delay_nop(T0L);
    }

    pub fn show_color(&self, col:u32){
        let val_first_bit_and = 0b100000000000000000000000;
        let mut j = 0;
        while  j < 30{
            self.pin.clear();
            //self.delay_us(1000);
            self.delay_nop(1000);
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




    pub fn show_colors(&self){
        let green:u32 = 0b10010110_00000000_00000000;
        let red:u32 = 0b00000000_10010110_00000000;
        let blue:u32 = 0b00000000_00000000_00010110;
        let white:u32 = 0b1111_1111__1111_1111__1111_1111;
        let black:u32 = 0b00000000_00000000_00000000;

        let colors:[u32;5] = [green,red,blue,white,black];

        let mut i = 0;
        loop{
            if i == 5 { i=0;}
            self.show_color(colors[i]);
            self.delay_us(1000_000);
            i+=1;
        }

    }

    pub fn show_shades(&self,col:u8){
        let mut rgb: [u32; 3] = [0, 0, 0];
        let mut j = 0;
        while j<100 {
            let mut i = 0;
            while i < 255 {
                rgb[1] = i;
                let green = rgb[0]<<16;
                let red = rgb[1] << 8;
                let blue = rgb[2];
                let color = green + red + blue;
                self.show_color(color);
                self.delay_us(1000);
                i += 1;
            }
            j+=1;
        }

    }


    pub fn show_rainbow(&self, count:u32){
        let mut rgb: [u32; 3] = [0, 0, 0];
        let n = 30; //brightness
        let d = 100;
        let mut j = 0;
        while j< count {
            let mut i = 0;
            while i < n {
                rgb[0] = i;
                rgb[1] = n - i; //red
                let green = rgb[0]<<16;
                let red = rgb[1] << 8;
                let blue = rgb[2];
                let color = green + red + blue;
                self.show_color(color);
                self.delay_us(d);
                i += 1;
            }
            i = 0;
            while i < n {
                rgb[2] = i;
                rgb[0] = n - i;
                let green = rgb[0]<<16;
                let red = rgb[1] << 8;
                let blue = rgb[2];
                let color = green + red + blue;
                self.show_color(color);
                self.delay_us(1000);
                i += 1;
            }

            let mut i = 0;
            while i < n {
                rgb[0] = i;
                rgb[2] = n - i;
                let green = rgb[0]<<16;
                let red = rgb[1] << 8;
                let blue = rgb[2];
                let color = green + red + blue;
                self.show_color(color);
                self.delay_us(1000);
                i += 1;
            }
            i = 0;
            while i < n {
                rgb[1] = i;
                rgb[0] = n - i;
                let green = rgb[0]<<16;
                let red = rgb[1] << 8;
                let blue = rgb[2];
                let color = green + red + blue;
                self.show_color(color);
                self.delay_us(1000);
                i += 1;
            }
            j+=1;
        }
    }
}

impl <'a, L:kernel::hil::gpio::Configure + kernel::hil::gpio::Output, T:kernel::hil::time::Time> LedRGB for WS2812B<'a, L, T> {
    /// Initialize the LED. Must be called before the LED is used.
    fn init(&self){
        self.pin.clear();
        self.pin.make_output();
        self.pin.clear();
    }

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







