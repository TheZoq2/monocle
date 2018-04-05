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
extern crate ssmarshal;

extern crate arrayvec;

extern crate api;

use heapless::ring_buffer::{RingBuffer, Consumer, Producer};
use api::data::{Reading, ClientHostMessage};

use ssmarshal::serialize;


// use stm32f103xx_hal::flash::FlashExt;
use stm32f103xx_hal::prelude::*;
use stm32f103xx_hal::time;
use stm32f103xx_hal::timer;
use stm32f103xx_hal::serial;
use stm32f103xx_hal::gpio::{self, gpioa, gpioc};
use embedded_hal_time::{Millisecond, RealCountDown};
use stm32f103xx::USART1 as HwUSART1;
use stm32f103xx::TIM2 as HwTIM2;
use stm32f103xx::EXTI;

use rtfm::{app, Threshold, Resource};

#[macro_use]
mod macros;

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
        static TX: serial::Tx<HwUSART1>;
        static RX: serial::Rx<HwUSART1>;
        static COUNTDOWN: timer::Timer<HwTIM2>;
        static PIN1: gpioa::PA8<gpio::Input<gpio::Floating>>;
        static PIN2: gpioa::PA9<gpio::Input<gpio::Floating>>;
        static EXTI: EXTI;
        static OUTPUT_PIN: gpioc::PC13<gpio::Output<gpio::PushPull>>;
        static FREQUENCY: time::Hertz;
    },

    idle: {
        resources: [CONSUMER, TX, OUTPUT_PIN]
    },

    // Both SYS_TICK and TIM2 have access to the `COUNTER` data
    tasks: {
        EXTI9_5: {
            path: on_pin1,
            resources: [PRODUCER, START_TIME, COUNTDOWN, PIN1, PIN2, EXTI],
            priority: 2,
        },
        USART1: {
            path: on_rx,
            resources: [RX, TX, FREQUENCY],
            priority: 1
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut rcc = p.device.RCC.constrain();
    let mut flash = p.device.FLASH.constrain();
    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = p.device.GPIOC.split(&mut rcc.apb2);
    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut countdown = timer::Timer::tim2(p.device.TIM2, time::Hertz(1), clocks, &mut rcc.apb1);
    countdown.listen(timer::Event::Update);
    // Pause the countdown and only resume it once bytes are received
    countdown.start_real(TIMEOUT);

    let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let rx = gpiob.pb7.into_floating_input(&mut gpiob.crl);
    let mut serial = serial::Serial::usart1(
        p.device.USART1,
        (tx, rx),
        &mut afio.mapr,
        115200.bps(),
        clocks,
        &mut rcc.apb2
    );
    serial.listen(serial::Event::Rxne);
    let (tx, rx) = serial.split();

    let timer = time::MonoTimer::new(p.core.DWT, clocks);
    let start_time = timer.now();
    let frequency = timer.frequency();


    // Configure pin a8 as a floating input
    let pin1 = gpioa.pa8.into_floating_input(&mut gpioa.crh);
    // Configure pin a9 as a floating input
    let pin2 = gpioa.pa9.into_floating_input(&mut gpioa.crh);
    // Mask exti8 and 9
    p.device.EXTI.imr.modify(|_r, w| w.mr8().set_bit());
    p.device.EXTI.imr.modify(|_r, w| w.mr9().set_bit());
    // Trigger exti8 for both rising and falling edge
    p.device.EXTI.rtsr.modify(|_r, w| w.tr8().set_bit());
    p.device.EXTI.ftsr.modify(|_r, w| w.tr8().set_bit());
    // Trigger exti9 for both rising and falling edge
    p.device.EXTI.rtsr.modify(|_r, w| w.tr9().set_bit());
    p.device.EXTI.ftsr.modify(|_r, w| w.tr9().set_bit());

    let (producer, consumer) = unsafe{_RB.split()};

    let mut output_pin = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    output_pin.set_high();

    init::LateResources {
        CONSUMER: consumer,
        PRODUCER: producer,
        START_TIME: start_time,
        TX: tx,
        RX: rx,
        COUNTDOWN: countdown,
        PIN1: pin1,
        PIN2: pin2,
        EXTI: p.device.EXTI,
        OUTPUT_PIN: output_pin,
        FREQUENCY: frequency
    }
}

fn idle(t: &mut Threshold, mut r: idle::Resources) -> ! {
    loop {
        match r.CONSUMER.dequeue() {
            Some(reading) => {
                r.OUTPUT_PIN.set_low();
                let mut buffer = [0; 10];
                let message = ClientHostMessage::Reading(reading);
                let byte_amount = serialize(&mut buffer, &message).unwrap();
                r.OUTPUT_PIN.set_high();

                //let mut tx = r.TX.lock_mut();
                r.TX.claim_mut(t, |tx, _| {
                    for byte in buffer[..byte_amount].iter() {
                        block!(tx.write(*byte)).unwrap()
                    }
                })
            }
            None => {
                rtfm::wfi();
            }
        }
    }
}

fn on_pin1(_t: &mut Threshold, mut r: EXTI9_5::Resources) {
    // Reset interrupt flag
    r.EXTI.pr.modify(|_r, w| w.pr8().set_bit());
    // Read the time
    let time = r.START_TIME.elapsed();

    let reading = Reading::new(time, r.PIN1.is_high(), r.PIN2.is_high());
    // TODO: Error handling
    r.PRODUCER.enqueue(reading).unwrap();
}


fn on_rx(t: &mut Threshold, mut r: USART1::Resources) {
    // Read byte to reset state
    let received = r.RX.read().unwrap();

    send_client_host_message!(
        &ClientHostMessage::FrequencyHertz(r.FREQUENCY.0),
        10,
        r.TX,
        t
    );
    send_client_host_message!(
        &ClientHostMessage::Reset(1),
        10,
        r.TX,
        t
    );
    send_client_host_message!(
        &ClientHostMessage::Reset(2),
        10,
        r.TX,
        t
    );
}

