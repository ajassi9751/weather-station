// dht11.c doesn't really work but this was a binding to call that c file
#[cfg(not(feature = "no_pi"))]
mod c {
    unsafe extern "C" {
        pub fn read_dht11_dat();
    }
    unsafe extern "C" {
        pub fn setup_wiring_pi();
    }
}

#[cfg(not(feature = "no_pi"))]
pub fn read_dht11_dat() {
    unsafe {
        c::read_dht11_dat();
    }
}

#[cfg(not(feature = "no_pi"))]
pub fn setup_wiring_pi() {
    unsafe {
        c::setup_wiring_pi();
    }
}

