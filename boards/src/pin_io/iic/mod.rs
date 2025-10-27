use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use rp235x_hal::gpio::{FunctionI2C, Pin};
use ssd1306::{
    I2CDisplayInterface, Ssd1306, mode::DisplayConfig, prelude::DisplayRotation,
    size::DisplaySize128x64,
};

use crate::pin_io::pin_init;
use hal::fugit::RateExtU32;

macros::rp235x_libInit!();

pub fn iic_init(clock: hal::clocks::ClocksManager) {
    let mut pac = unsafe { hal::pac::Peripherals::steal() };

    let gpio = pin_init();

    let sda_pin: Pin<_, FunctionI2C, _> = gpio.gpio18.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = gpio.gpio19.reconfigure();

    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut pac.RESETS,
        &clock.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);

    let im = Image::new(&raw, Point::new(32, 0));

    im.draw(&mut display).unwrap();

    display.flush().unwrap();
}
