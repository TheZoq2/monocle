#![feature(proc_macro)]
#![no_std]

#[macro_use(block)]
extern crate nb;

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;
extern crate stm32f103xx_hal;
extern crate embedded_hal;
extern crate embedded_hal_time;
extern crate heapless;

extern crate arrayvec;

extern crate api;

use heapless::ring_buffer::{RingBuffer, Consumer, Producer};
use api::data::Reading;


// use stm32f103xx_hal::flash::FlashExt;
use stm32f103xx_hal::prelude::*;
use stm32f103xx_hal::time;
use stm32f103xx_hal::timer;
use stm32f103xx_hal::serial;
use stm32f103xx_hal::gpio::{self, gpioa};
use embedded_hal_time::{Millisecond, RealCountDown};
use stm32f103xx::USART1;
use stm32f103xx::TIM2 as HwTIM2;
use stm32f103xx::EXTI;

use rtfm::{app, Threshold};

const BUFFER_SIZE: usize = 200;

// Transmission timeout
const TIMEOUT: Millisecond = Millisecond(500);

static mut _RB: RingBuffer<Reading, [Reading; BUFFER_SIZE]> = RingBuffer::new();

app! {
    device: stm32f103xx,

    resources: {
        static CONSUMER: Consumer<'static, Reading, [Reading; BUFFER_SIZE]>;
        static PRODUCER: Producer<'static, Reading, [Reading; BUFFER_SIZE]>;
        static START_TIME: time::Instant;
        static TX: serial::Tx<USART1>;
        static COUNTDOWN: timer::Timer<HwTIM2>;
        static PIN1: gpioa::PA8<gpio::Input<gpio::Floating>>;
        static EXTI: EXTI;
    },

    idle: {
        resources: [CONSUMER, TX]
    },

    // Both SYS_TICK and TIM2 have access to the `COUNTER` data
    tasks: {
        EXTI9_5: {
            path: on_pin1,
            resources: [PRODUCER, START_TIME, COUNTDOWN, PIN1, EXTI]
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut rcc = p.device.RCC.constrain();
    let mut flash = p.device.FLASH.constrain();
    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
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


    // Configure pin a8 as a floating input
    let pin1 = gpioa.pa8.into_floating_input(&mut gpioa.crh);
    // Mask exti8
    p.device.EXTI.imr.modify(|_r, w| w.mr8().set_bit());
    // Trigger exti8 for both rising and falling edge
    p.device.EXTI.rtsr.modify(|_r, w| w.tr8().set_bit());
    p.device.EXTI.ftsr.modify(|_r, w| w.tr8().set_bit());

    let (producer, consumer) = unsafe{_RB.split()};

    init::LateResources {
        CONSUMER: consumer,
        PRODUCER: producer,
        START_TIME: start_time,
        TX: tx,
        COUNTDOWN: countdown,
        PIN1: pin1,
        EXTI: p.device.EXTI
    }
}

fn idle(_t: &mut Threshold, mut r: idle::Resources) -> ! {
    loop {
        match r.CONSUMER.dequeue() {
            Some(reading) => for byte in reading.encode().iter() {
                block!(r.TX.write(*byte)).unwrap()
            }
            None => {
                rtfm::wfi();
            }
        }
    }
}

fn on_pin1(_t: &mut Threshold, mut r: EXTI9_5::Resources) {
    // Read the time
    let time = r.START_TIME.elapsed();
    // Reset interrupt flag
    r.EXTI.pr.modify(|_r, w| w.pr8().set_bit());

    let reading = Reading::new(time, r.PIN1.is_high(), true);
    // TODO: Error handling
    r.PRODUCER.enqueue(reading);
}
