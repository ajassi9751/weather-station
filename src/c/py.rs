mod c {
    unsafe extern "C" {
        pub fn print_temp_humidity();
    }
}

pub fn print_temp_humidity() {
    unsafe {
        c::print_temp_humidity();
    }
}