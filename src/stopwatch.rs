use stm32f103xx;
use stm32f103xx::TIM3;

struct Stopwatch {
    tim3: TIM3,
    overflow_counter: u16,
}

impl Stopwatch {
    pub fn new(mut tim3: TIM3) -> Self{
        // Set clock source
        // Set overflow to max
        tim3.psc.write(|w| w.reset_value());
        
        // Set prescaler
        tim3.psc.write(|w| w.reset_value());

        tim3.cr1.modify(|_r, w| {
            // Set clock division to 1, direction to up and enable
            w
                .ckd().no_div()
                .dir().up()
                .cen().set_bit()
        });
        Self {
            tim3,
            overflow_counter: 0
        }
    }

    pub fn on_overflow(&mut self) {
        self.overflow_counter += 1;
    }

    pub fn now(&self) -> u32 {
        ((self.overflow_counter as u32) << 16) + (self.raw_value() as u32)
    }

    fn raw_value(&self) -> u16 {
        unimplemented!()
    }
}
