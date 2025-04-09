#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    clock::CpuClock,
    delay::Delay,
    main,
};
use log::info;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Common Setup Code
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();
    // End Common Setup Code

    let analog_pin = peripherals.GPIO0;
    let mut adc_config = AdcConfig::new();
    let mut pin = adc_config.enable_pin(analog_pin, Attenuation::_11dB);

    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    loop {
        let reading = nb::block!(adc.read_oneshot(&mut pin)).unwrap();
        info!("Reading: {reading}");
        delay.delay_millis(1_000);
    }
}
