#[macro_export]
macro_rules! rp235x_preclude {
    [ ] => {
        use defmt::*;
        use defmt_rtt as _;
        use panic_probe as _;

        use rp235x_hal as hal;
    };
}

#[macro_export]
macro_rules! rp235x_libInit {
    [ ] => {
        $crate::rp235x_preclude!();
    }
}

#[macro_export]
macro_rules! rp235x_binInit {
    [ ] => {
        $crate::rp235x_preclude!();

        // Provide an alias for our BSP so we can switch targets quickly.
        // Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
        // use some_bsp;

        /// Tell the Boot ROM about our application
        #[unsafe(link_section = ".start_block")]
        #[used]
        pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

        /// Program metadata for `picotool info`
        #[unsafe(link_section = ".bi_entries")]
        #[used]
        pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
            hal::binary_info::rp_cargo_bin_name!(),
            hal::binary_info::rp_cargo_version!(),
            hal::binary_info::rp_program_description!(c"RP2350 Template"),
            hal::binary_info::rp_cargo_homepage_url!(),
            hal::binary_info::rp_program_build_attribute!(),
        ];

        use hal::entry;
    };
}
