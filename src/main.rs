#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    debug!("");
    let p = embassy_rp::init(Default::default());

    //GPIO 25 = SYS_LED on Pico...SYS_LED is wired to cyw43 on PicoW
    //let mut led = Output::new(p.PIN_25, Level::Low);
    let mut led = Output::new(p.PIN_14, Level::Low);

    loop {
        led.set_high();
        Timer::after_secs(1).await;

        led.set_low();
        Timer::after_secs(1).await;
    }
}
