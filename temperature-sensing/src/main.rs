#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    clock::CpuClock,
    delay::Delay,
    main,
};
use libm::log;
use log::info;

const BETA: f64 = 3_950_f64; // should match the Beta Coefficient of the thermistor

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Common Setup Code
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();
    // End Common Setup Code

    let analog_pin = peripherals.GPIO4;
    let mut adc_config = AdcConfig::new();
    let mut pin = adc_config.enable_pin(analog_pin, Attenuation::_11dB);

    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    loop {
        let reading: f64 = nb::block!(adc.read_oneshot(&mut pin)).unwrap() as f64;
        let celsius = 1.0 / (log(1.0 / (4_095.0 / reading - 1.0)) / BETA + 1.0 / 298.15) - 273.15;
        info!("Reading: {reading}, Temp: {celsius} C");
        delay.delay_millis(1_000);
    }
}
