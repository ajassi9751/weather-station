use std::ffi::c_double;
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Wind_data {
    pub rotations: c_double,
    pub rots_per_second: c_double,
    pub windspeed: c_double,
    pub windspeedMPH: c_double,
}

mod c {
    use crate::c::wind_speed::Wind_data;
    unsafe extern "C" {
        pub fn print_wind_speed();
        #[no_mangle]
        pub fn get_wind_speed(data: *mut Wind_data);
    }
}

pub fn print_wind_speed() {
    unsafe {
        c::print_wind_speed();
    }
}

pub fn get_wind_speed() -> Wind_data {
    let mut data: Wind_data = Wind_data {
        rotations: 0.0,
        rots_per_second: 0.0,
        windspeed: 0.0,
        windspeedMPH: 0.0
    };
    unsafe { c::get_wind_speed(&mut data); }
    data
}
