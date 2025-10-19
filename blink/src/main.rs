//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use boards::{Board, ClockEnabled, PinEnabled};
use embedded_hal::digital::OutputPin;

macros::rp235x_binInit!();

#[entry]
fn main() -> ! {
    info!("Program started");

    let board = Board::<ClockEnabled, PinEnabled>::new();
    info!("Board initialized");

    let mut apin = board.pins.gpio25.into_push_pull_output();

    let mut delay = board.delay;

    loop {
        info!("on");
        apin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off");
        apin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
