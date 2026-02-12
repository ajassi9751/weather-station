mod c {
    unsafe extern "C" {
        pub fn read_dht11_dat();
    }   
    unsafe extern "C" {
        pub fn setup_wiring_pi();
    }
}

pub fn read_dht11_dat () {
    unsafe {
        c::read_dht11_dat();
    }
}

pub fn setup_wiring_pi () {
    unsafe {
        c::setup_wiring_pi();
    }
}