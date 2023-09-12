#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::{AnyChannel, Channel, ExtiInput},
    gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

static SIGNAL_USER_LED: Signal<ThreadModeRawMutex, Level> = Signal::new();

async fn os_delay(millis: u64) {
    Timer::after(Duration::from_millis(millis)).await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    // button task
    let user_button_pin = p.PC13.degrade();
    let user_button_ext_irq_channel = p.EXTI13.degrade();
    spawner
        .spawn(button_task(user_button_pin, user_button_ext_irq_channel))
        .unwrap();

    // led task
    let user_led_pin = p.PA5.degrade();
    spawner.spawn(led_task(user_led_pin)).unwrap();

    // main task
    loop {
        info!("high");
        SIGNAL_USER_LED.signal(Level::High);
        os_delay(500).await;
        info!("low");
        SIGNAL_USER_LED.signal(Level::Low);
        os_delay(500).await;
    }
}

#[embassy_executor::task]
async fn led_task(user_led_pin: AnyPin) {
    let mut user_led = Output::new(user_led_pin, Level::Low, Speed::Low);

    loop {
        let user_led_level = SIGNAL_USER_LED.wait().await;
        user_led.set_level(user_led_level);
    }
}

#[embassy_executor::task]
async fn button_task(button_pin: AnyPin, irq_channel: AnyChannel) {
    let user_button = Input::new(button_pin, Pull::None);
    let mut user_button_irq = ExtiInput::new(user_button, irq_channel);

    loop {
        user_button_irq.wait_for_rising_edge().await;
        SIGNAL_USER_LED.signal(Level::High);
        os_delay(100).await;
        SIGNAL_USER_LED.signal(Level::Low);
    }
}
