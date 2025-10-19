#![no_std]
macros::rp235x_libInit!();
macros::mod_flat!(clocks, pins);

// External high-speed crystal on the pico board is 12Mhz
pub(crate) const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// Zero-cost Board - no Option wrappers!
pub struct Board<C: ClockFeature, P: PinFeature> {
    pub clocks: C::ClockType,
    pub delay: C::DelayType,
    pub pins: P::PinType,
}

impl<C: ClockFeature, P: PinFeature> Board<C, P> {
    pub fn new() -> Self {
        let (clocks, delay) = C::init();
        let pins = P::init();

        Self {
            clocks,
            delay,
            pins,
        }
    }
}
