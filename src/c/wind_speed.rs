use std::ffi::c_double;
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Wind_speed {
    pub rotations: c_double,
    pub rots_per_second: c_double,
    pub windspeed: c_double,
    pub windspeedMPH: c_double,
}

mod c {
    use crate::c::wind_speed::Wind_speed;
    unsafe extern "C" {
        pub fn print_wind_speed();
        pub fn get_wind_speed(data: *mut Wind_speed);
    }
}

pub fn print_wind_speed() {
    unsafe {
        c::print_wind_speed();
    }
}

pub fn get_wind_speed() -> Wind_speed {
    let mut data: Wind_speed = Wind_speed {
        rotations: 0.0,
        rots_per_second: 0.0,
        windspeed: 0.0,
        windspeedMPH: 0.0
    };
    unsafe { c::get_wind_speed(&mut data); }
    data
}
