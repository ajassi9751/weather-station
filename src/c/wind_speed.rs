#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Wind_speed {
    pub rotations: f64,
    pub rots_per_second: f64,
    pub windspeed: f64,
    pub windspeedMPH: f64,
}

mod c {
    unsafe extern "C" {
        pub fn print_wind_speed();
        pub fn get_wind_speed() -> Wind_speed;
    }
}

pub fn print_wind_speed() {
    unsafe {
        c::print_wind_speed();
    }
}

pub fn get_wind_speed() -> Wind_speed {
    unsafe {
        c::get_wind_speed()
    }
}