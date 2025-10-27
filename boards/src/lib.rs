#![no_std]
macros::rp235x_libInit!();
macros::mod_flat!(clocks);
macros::mod_pub!(pin_io);

// External high-speed crystal on the pico board is 12Mhz
pub(crate) const XTAL_FREQ_HZ: u32 = 12_000_000u32;
