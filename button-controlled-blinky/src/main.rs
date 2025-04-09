#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, Level, Output, Pull},
    main,
};

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let _delay = Delay::new();
    let mut delay_value = 1_000_000_u32;

    esp_println::logger::init_logger_from_env();
    let mut led = Output::new(peripherals.GPIO4, Level::High);
    let button = Input::new(peripherals.GPIO0, Pull::Up);

    loop {
        for _ in 1..delay_value {
            if button.is_low() {
                delay_value = delay_value.saturating_sub(250_000_u32);
                if delay_value < 250_000_u32 {
                    delay_value = 1_000_000_u32;
                }
            }
        }

        led.toggle();
    }
}
