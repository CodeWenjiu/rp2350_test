macros::rp235x_libInit!();

pub trait PinFeature {
    type PinType;

    fn init() -> Self::PinType;
}

// Enabled features - provide actual hardware types
pub struct PinEnabled;

impl PinFeature for PinEnabled {
    type PinType = hal::gpio::Pins;

    fn init() -> Self::PinType {
        let mut pac = unsafe { hal::pac::Peripherals::steal() };
        let sio = hal::Sio::new(pac.SIO);

        hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        )
    }
}

// Disabled features - use zero-sized types
pub struct PinDisabled;

impl PinFeature for PinDisabled {
    type PinType = ();

    fn init() -> Self::PinType {
        ()
    }
}
