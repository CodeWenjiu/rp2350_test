macros::rp235x_libInit!();
macros::mod_pub!(iic);

pub fn pin_init() -> hal::gpio::Pins {
    let mut pac = unsafe { hal::pac::Peripherals::steal() };
    let sio = hal::Sio::new(pac.SIO);

    hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    )
}
