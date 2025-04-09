#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Event, Input, Io, Pull},
    handler,
    interrupt::InterruptConfigurable,
    main,
};
use log::info;

static PRESS_COUNT: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));
static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(gpio);

    let _delay = Delay::new();

    esp_println::logger::init_logger_from_env();
    let mut button = Input::new(peripherals.GPIO0, Pull::Up);

    critical_section::with(|cs| {
        button.listen(Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button);
    });

    loop {
        critical_section::with(|cs| {
            let n = PRESS_COUNT.borrow_ref(cs);
            info!("Button Pressed {n} times.");
        });
    }
}

#[handler]
fn gpio() {
    critical_section::with(|cs| {
        let mut button = BUTTON.borrow_ref_mut(cs);
        let Some(button) = button.as_mut() else {
            return;
        };

        if button.is_interrupt_set() {
            info!("Button pressed");
            *PRESS_COUNT.borrow_ref_mut(cs) += 1;
            button.clear_interrupt();
        }
    });
}
