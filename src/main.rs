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
extern crate panic_abort;

extern crate arrayvec;

extern crate api;

use api::Message;

use heapless::ring_buffer::{RingBuffer, Consumer, Producer};
use api::data::{Reading, ClientHostMessage};


// use stm32f103xx_hal::flash::FlashExt;
use stm32f103xx_hal::prelude::*;
use stm32f103xx_hal::time;
use stm32f103xx_hal::timer;
use stm32f103xx_hal::mono_timer;
use stm32f103xx_hal::serial;
use stm32f103xx_hal::gpio::{self, gpioa, gpioc};
use embedded_hal_time::{Millisecond, RealCountDown, Stopwatch};
use stm32f103xx::USART2 as HwUSART2;
use stm32f103xx::EXTI;
use stm32f103xx::TIM2 as HwTIM2;
use stm32f103xx::TIM3 as HwTIM3;
use stm32f103xx::TIM4 as HwTIM4;


use rtfm::{app, Threshold, Resource};

#[macro_use]
mod macros;
mod channels;
// mod stopwatch;

const BUFFER_SIZE: usize = 200;

// Transmission timeout
const CURRENT_TIME_SEND_RATE: Millisecond = Millisecond(10);

static mut _RB: RingBuffer<Reading, [Reading; BUFFER_SIZE]> = RingBuffer::new();

app! {
    device: stm32f103xx,

    resources: {
        static CONSUMER: Consumer<'static, Reading, [Reading; BUFFER_SIZE]>;
        static PRODUCER: Producer<'static, Reading, [Reading; BUFFER_SIZE]>;
        static MONO_TIMER: mono_timer::MonoTimer32bit<HwTIM3, HwTIM4>;
        static TX: serial::Tx<HwUSART2>;
        static RX: serial::Rx<HwUSART2>;
        static PIN1: gpioa::PA8<gpio::Input<gpio::Floating>>;
        static PIN2: gpioa::PA9<gpio::Input<gpio::Floating>>;
        static EXTI: EXTI;
        static OUTPUT_PIN: gpioc::PC13<gpio::Output<gpio::PushPull>>;
        static FREQUENCY: time::Hertz;
        static TIMER2: timer::Timer<HwTIM2>;
    },

    idle: {
        resources: [CONSUMER, TX, OUTPUT_PIN]
    },

    tasks: {
        EXTI9_5: {
            path: on_pin1,
            resources: [PRODUCER, MONO_TIMER, PIN1, PIN2, EXTI],
            priority: 3,
        },
        USART2: {
            path: on_rx,
            resources: [EXTI, RX, TX, FREQUENCY],
            priority: 2
        },
        TIM2: {
            path: on_timer,
            resources: [TX, MONO_TIMER, TIMER2],
            priority: 1,
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

    // Setup the timer to send regular updates about the current time
    let mut timer2 = timer::Timer::tim2(p.device.TIM2, time::Hertz(1), clocks, &mut rcc.apb1);
    timer2.listen(timer::Event::Update);
    timer2.start_real(CURRENT_TIME_SEND_RATE);

    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3.into_floating_input(&mut gpioa.crl);
    let mut serial = serial::Serial::usart2(
        p.device.USART2,
        (tx, rx),
        &mut afio.mapr,
        115200.bps(),
        clocks,
        &mut rcc.apb1
    );
    serial.listen(serial::Event::Rxne);
    let (tx, rx) = serial.split();

    let mono_timer = mono_timer::MonoTimer32bit::tim34(
        p.device.TIM3,
        p.device.TIM4,
        clocks,
        &mut rcc.apb1
    );
    let frequency = mono_timer.frequency();


    // Configure pin a8 as a floating input
    let pin1 = gpioa.pa8.into_floating_input(&mut gpioa.crh);
    // Configure pin a9 as a floating input
    let pin2 = gpioa.pa9.into_floating_input(&mut gpioa.crh);

    channels::enable_channel(&p.device.EXTI, 0).map_err(|_e| panic!());
    channels::enable_channel(&p.device.EXTI, 1).map_err(|_e| panic!());

    let (producer, consumer) = unsafe{_RB.split()};

    let mut output_pin = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    output_pin.set_high();

    init::LateResources {
        CONSUMER: consumer,
        PRODUCER: producer,
        MONO_TIMER: mono_timer,
        TX: tx,
        RX: rx,
        PIN1: pin1,
        PIN2: pin2,
        EXTI: p.device.EXTI,
        OUTPUT_PIN: output_pin,
        FREQUENCY: frequency,
        TIMER2: timer2
    }
}

fn idle(t: &mut Threshold, mut r: idle::Resources) -> ! {
    loop {
        match r.CONSUMER.dequeue() {
            Some(reading) => {
                r.OUTPUT_PIN.set_low();
                let mut buffer = [0; 10];
                let message = ClientHostMessage::Reading(reading);
                let byte_amount = message.encode(&mut buffer).unwrap();
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
    let time = r.MONO_TIMER.ticks_passed();

    let reading = Reading::new(time, r.PIN1.is_high(), r.PIN2.is_high());
    // TODO: Error handling
    r.PRODUCER.enqueue(reading).unwrap();
}


fn on_rx(t: &mut Threshold, mut r: USART2::Resources) {
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


fn on_timer(t: &mut Threshold, mut r: TIM2::Resources) {
    // Reset the counter
    r.TIMER2.wait();
    let time = r.MONO_TIMER.claim(t, |mono_timer, _t| mono_timer.ticks_passed());
    //let time = 0x1234567e;
    send_client_host_message!(
        &ClientHostMessage::CurrentTime(time),
        10,
        r.TX,
        t
    );
}
