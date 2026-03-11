// This files just exists as an initial test to build infastructure to make sure rust and c were compiling and linking properly
#[cfg(not(feature = "rust_only"))]
mod c {
    use std::ffi::c_int;
    unsafe extern "C" {
        pub fn add(x: c_int, y: c_int) -> c_int;
    }
}

#[cfg(not(feature = "rust_only"))]
pub fn add(x: i32, y: i32) -> i32 {
    let result;
    unsafe {
        result = c::add(x, y);
    }
    result
}
