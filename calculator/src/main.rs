#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{AnyPin, Flex, Input, Pin, Pull},
    main,
};
use log::info;

const ROWS: usize = 4;
const COLS: usize = 4;
const KEYS: [[char; COLS]; ROWS] = [
    ['1', '2', '3', '+'],
    ['4', '5', '6', '-'],
    ['7', '8', '9', '*'],
    ['C', '0', '=', '/'],
];

struct Keypad<'d, const ROWS: usize, const COLS: usize> {
    keymap: [[char; COLS]; ROWS],
    col_pins: [Input<'d>; COLS],
    row_pins: [Flex<'d>; ROWS],
    previous_key: Option<char>,
}

impl<const ROWS: usize, const COLS: usize> Keypad<'_, ROWS, COLS> {
    fn new(
        keymap: [[char; COLS]; ROWS],
        row_pins: [AnyPin; COLS],
        col_pins: [AnyPin; COLS],
    ) -> Self {
        let mut row_flex: [Flex<'_>; ROWS] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut col_inputs: [Input<'_>; COLS] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        for (row_entry, row_pin) in row_flex.iter_mut().zip(row_pins.into_iter()) {
            *row_entry = Flex::new(row_pin);
            row_entry.set_as_input(Pull::Up);
        }

        for (col_entry, col_pin) in col_inputs.iter_mut().zip(col_pins.into_iter()) {
            *col_entry = Input::new(col_pin, Pull::Up);
        }

        Self {
            keymap,
            col_pins: col_inputs,
            row_pins: row_flex,
            previous_key: None,
        }
    }

    fn get_key(&mut self) -> Option<char> {
        let mut key: Option<char> = None;

        for (row, row_pin) in self.row_pins.iter_mut().enumerate() {
            row_pin.set_as_output();
            row_pin.set_low();

            for (col, col_pin) in self.col_pins.iter().enumerate() {
                if col_pin.is_low() {
                    key = Some(self.keymap[row][col]);
                    break;
                }
            }

            // Reset row_pin
            row_pin.set_as_input(Pull::Up);

            if key.is_some() {
                break;
            }
        }

        if key == self.previous_key {
            return None;
        }

        self.previous_key = key;
        key
    }
}

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let delay = Delay::new();

    let mut keypad = Keypad::new(
        KEYS,
        [
            peripherals.GPIO0.degrade(),
            peripherals.GPIO1.degrade(),
            peripherals.GPIO2.degrade(),
            peripherals.GPIO3.degrade(),
        ],
        [
            peripherals.GPIO4.degrade(),
            peripherals.GPIO5.degrade(),
            peripherals.GPIO6.degrade(),
            peripherals.GPIO7.degrade(),
        ],
    );

    let mut a: Option<i32> = None;
    let mut operator: Option<char> = None;
    let mut b: Option<i32> = None;

    loop {
        if let Some(key) = keypad.get_key() {
            match key {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let digit = key.to_digit(10).unwrap() as i32;

                    let to_update = if operator.is_some() { &mut b } else { &mut a };

                    if let Some(inner) = to_update.as_mut() {
                        *inner = *inner * 10 + digit;
                    } else {
                        *to_update = Some(digit);
                    }
                }
                'C' => {
                    a = None;
                    b = None;
                    operator = None;
                }
                current_op => {
                    if let (Some(lhs), Some(rhs), Some(op)) = (a, b, operator) {
                        let res = match op {
                            '+' => lhs + rhs,
                            '-' => lhs - rhs,
                            '*' => lhs * rhs,
                            '/' => lhs / rhs,
                            _ => {
                                info!("Invalid operation");
                                continue;
                            }
                        };

                        if current_op == '=' {
                            operator = None;
                        }
                        a = Some(res);
                        b = None;
                    }
                    if current_op != '=' {
                        operator = Some(current_op);
                    }
                }
            }
            if let Some(b) = b {
                let a = a.unwrap();
                let operator = operator.unwrap();
                info!("{a} {operator} {b}");
            } else if let Some(operator) = operator {
                let a = a.unwrap();
                info!("{a} {operator}");
            } else {
                let a = a.unwrap_or(0);
                info!("{a}");
            }
        }
        delay.delay_millis(50);
    }
}
