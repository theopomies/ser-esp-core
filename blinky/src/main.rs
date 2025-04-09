#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output},
    main,
};
use log::info;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let delay = Delay::new();

    esp_println::logger::init_logger_from_env();
    let mut led = Output::new(peripherals.GPIO1, Level::High);

    loop {
        info!("Hello world!");
        led.toggle();
        delay.delay_millis(1_000);
    }
}
