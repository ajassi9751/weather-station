mod c {
    use std::ffi::c_int;
    unsafe extern "C" {
        pub fn add (x:c_int, y:c_int) -> c_int;
    }
}

pub fn add (x:i32, y:i32) -> i32 {
    let result;
    unsafe {
        result = c::add(x,y);
    }
    result
}