#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

async fn os_delay(millis: u64) {
    Timer::after(Duration::from_millis(millis)).await;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let user_led_pin = p.PA5.degrade();
    let mut user_led = Output::new(user_led_pin, Level::Low, Speed::Low);

    loop {
        info!("high");
        user_led.set_high();
        os_delay(500).await;
        info!("low");
        user_led.set_low();
        os_delay(500).await;
    }
}
