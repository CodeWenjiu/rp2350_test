//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

macros::rp235x_binInit!();

use boards::{
    clock_init,
    pin_io::{iic::iic_init, pin_init},
};
use embedded_hal::digital::OutputPin;

#[entry]
fn main() -> ! {
    macros::heap_init!();

    info!("Program started");

    let pin = pin_init();
    let (clock, mut delay) = clock_init();
    info!("Board initialized");

    iic_init(clock);

    let mut apin = pin.gpio25.into_push_pull_output();

    loop {
        info!("on");
        apin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off");
        apin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
