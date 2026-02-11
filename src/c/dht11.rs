mod c {
     unsafe extern "C" {
        pub fn read_dht11_dat();
    }   
}

pub fn read_dht11_dat () {
    unsafe {
        c::read_dht11_dat();
    }
}