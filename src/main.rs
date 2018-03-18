#![feature(proc_macro)]
#![no_std]

#[macro_use(block)]
extern crate nb;

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;
extern crate stm32f103xx_hal;
extern crate embedded_hal;
extern crate embedded_hal_time;

extern crate arrayvec;

mod data;

use arrayvec::ArrayVec;
use data::PinData;

// use stm32f103xx_hal::flash::FlashExt;
use stm32f103xx_hal::prelude::*;
use stm32f103xx_hal::time;
use stm32f103xx_hal::timer;
use stm32f103xx_hal::serial;
use embedded_hal::serial::Write;
use embedded_hal_time::{Millisecond, RealCountDown};
use stm32f103xx::USART1;
use stm32f103xx::TIM2 as HwTIM2;

use rtfm::{app, Threshold};

const BUFFER_SIZE: usize = 200;

// Transmission timeout
const TIMEOUT: Millisecond = Millisecond(500);


app! {
    device: stm32f103xx,

    resources: {
        static BUFFER1: ArrayVec<[PinData; BUFFER_SIZE]>;
        static BUFFER2: ArrayVec<[PinData; BUFFER_SIZE]>;
        static START_TIME: time::Instant;
        static TIMER_FREQ: time::Hertz;
        static TX: serial::Tx<USART1>;
        static COUNTDOWN: timer::Timer<HwTIM2>;
    },

    // Both SYS_TICK and TIM2 have access to the `COUNTER` data
    tasks: {
        TIM2: {
            path: sender,
            resources: [BUFFER1, BUFFER2, START_TIME, TIMER_FREQ, TX, COUNTDOWN]
        },
        EXTI0: {
            path: onpin1,
            resources: [BUFFER1, START_TIME, COUNTDOWN]
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut rcc = p.device.RCC.constrain();
    let mut flash = p.device.FLASH.constrain();
    let gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut countdown = timer::Timer::tim2(p.device.TIM2, time::Hertz(1), clocks, &mut rcc.apb1);
    countdown.listen(timer::Event::Update);
    // Pause the countdown and only resume it once bytes are received
    countdown.start_real(TIMEOUT);

    let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let rx = gpiob.pb7.into_floating_input(&mut gpiob.crl);
    let serial = serial::Serial::usart1(
        p.device.USART1,
        (tx, rx),
        &mut afio.mapr,
        115200.bps(),
        clocks,
        &mut rcc.apb2
    );
    let (tx, _) = serial.split();

    let timer = time::MonoTimer::new(p.core.DWT, clocks);
    let start_time = timer.now();
    let frequency = timer.frequency();


    let buffer1 = ArrayVec::new();
    let buffer2 = ArrayVec::new();

    // Setup pins as inputs
    // Setup exti0 to trigger on rising and falling edge
    // Enable exti1 line 0
    // p.device.EXTI.imr.modify(|_r, w| w.mr0().set_bit());
    // // Enable both rising edge and falling edge trigger
    // p.device.EXTI.rtsr.modify(|_r, w| w.tr0().set_bit());
    // p.device.EXTI.ftsr.modify(|_r, w| w.tr0().set_bit());
    //
    init::LateResources {
        BUFFER1: buffer1,
        BUFFER2: buffer2,
        START_TIME: start_time,
        TIMER_FREQ: frequency,
        TX: tx,
        COUNTDOWN: countdown,
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn sender(_t: &mut Threshold, mut r: TIM2::Resources) {
    r.COUNTDOWN.start_real(TIMEOUT);
    r.TX.write(b'a');
}

fn onpin1(_t: &mut Threshold, mut r: EXTI0::Resources) {
    // Read the time
    let time = r.START_TIME.elapsed();
    // Figure out if this was a rising or falling edge
    // Reset interrupt flag
    unimplemented!()
}
