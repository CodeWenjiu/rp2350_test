macros::rp235x_libInit!();

use cortex_m::delay::Delay;
use rp235x_hal::{Clock, clocks::init_clocks_and_plls};

use crate::XTAL_FREQ_HZ;

// Feature traits - define what types are used when enabled/disabled
pub trait ClockFeature {
    type ClockType;
    type DelayType;

    fn init() -> (Self::ClockType, Self::DelayType);
}

pub struct ClockEnabled;

impl ClockFeature for ClockEnabled {
    type ClockType = hal::clocks::ClocksManager;
    type DelayType = Delay;

    fn init() -> (Self::ClockType, Self::DelayType) {
        let mut pac = unsafe { hal::pac::Peripherals::steal() };
        let core = unsafe { cortex_m::Peripherals::steal() };
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

        let clocks = init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        (clocks, delay)
    }
}

pub struct ClockDisabled;

impl ClockFeature for ClockDisabled {
    type ClockType = ();
    type DelayType = ();

    fn init() -> (Self::ClockType, Self::DelayType) {
        ((), ())
    }
}
