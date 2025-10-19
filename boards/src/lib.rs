#![no_std]

use rp235x_hal::{Clock, clocks::init_clocks_and_plls};

macros::rp235x_libInit!();

#[unsafe(no_mangle)]
static mut BOARD_PERIPHERALS: bool = false;

pub struct Board {
    pub clocks: hal::clocks::ClocksManager,
    pub delay: cortex_m::delay::Delay,

    pub pins: hal::gpio::Pins,
}

impl Board {
    #[inline]
    pub fn take() -> Option<Self> {
        if unsafe { BOARD_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    #[inline]
    unsafe fn steal() -> Self {
        unsafe {
            BOARD_PERIPHERALS = true;
        }
        let mut pac = hal::pac::Peripherals::take().unwrap();
        let core = cortex_m::Peripherals::take().unwrap();
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
        let sio = hal::Sio::new(pac.SIO);

        // External high-speed crystal on the pico board is 12Mhz
        let external_xtal_freq_hz = 12_000_000u32;
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        let pins = hal::gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        Self {
            clocks,
            delay,
            pins,
        }
    }
}
