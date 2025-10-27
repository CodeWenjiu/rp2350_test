//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

macros::rp235x_binInit!();

use boards::{clock_init, pin_io::pin_init};
use dht_sensor::{DhtReading, dht11};
use embedded_hal::digital::StatefulOutputPin;

#[entry]
fn main() -> ! {
    macros::heap_init!();

    info!("Program started");

    let pin = pin_init();
    let (_clocks, mut delay) = clock_init();
    info!("Board initialized");

    let mut apin = pin.gpio25.into_push_pull_output();

    let mut dhtpin = hal::gpio::InOutPin::new(pin.gpio22);

    loop {
        info!("toggle");
        apin.toggle().unwrap();

        let measurement = dht11::Reading::read(&mut delay, &mut dhtpin);
        match measurement {
            Ok(reading) => {
                info!(
                    "Temperature: {}°C, Humidity: {}%",
                    reading.temperature, reading.relative_humidity
                );
            }
            Err(_) => {
                info!("Failed to read DHT11 sensor");
            }
        }

        delay.delay_ms(500);
    }
}
