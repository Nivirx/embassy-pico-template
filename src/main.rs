//! This example test the RP Pico W on board LED.
//!
//! It does not work with the RP Pico board. See blinky.rs.

#![no_std]
#![no_main]

use core::str;

use cyw43::ScanOptions;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const WIFI_FIRMWARE_BASE: u32 = 0x1010_0000;
const BT_FIRMWARE_BASE: u32 = 0x1014_0000;
const CLM_FIRMWARE_BASE: u32 = 0x1014_4000;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    //let fw = include_bytes!("../include/cyw43-firmware/43439A0.bin");
    //let clm = include_bytes!("../include/cyw43-firmware/43439A0_clm.bin");
    let fw = unsafe { core::slice::from_raw_parts(WIFI_FIRMWARE_BASE as *const u8, 230321) };
    let clm = unsafe { core::slice::from_raw_parts(CLM_FIRMWARE_BASE as *const u8, 4752) };

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // turn on LED to indicate cyw43 is loaded
    info!("SYS_LED on");
    control.gpio_set(0, true).await;

    // Scope wifi_scan so we can change gpio's later
    {
        let mut wifi_scan = control.scan(ScanOptions::default()).await;
        while let Some(bss) = wifi_scan.next().await {
            if let Ok(ssid_str) = str::from_utf8(&bss.ssid) {
                info!(
                    "Scanned {} == {:x}\nRSSI: {}\tPHY_NOISE: {}\tSNR: {}",
                    ssid_str, bss.bssid, bss.rssi, bss.phy_noise, bss.snr
                );
            }
        }
    }

    info!("SYS_LED off");
    control.gpio_set(0, false).await;

    let loop_delay = Duration::from_secs(10);
    let blink_delay = Duration::from_millis(125);
    loop {
        info!("All done - Waiting in loop!");
        for _ in 0..4 {
            control.gpio_set(0, true).await;
            Timer::after(blink_delay).await;
            control.gpio_set(0, false).await;
            Timer::after(blink_delay).await;
        }
        Timer::after(loop_delay).await;
    }
}
