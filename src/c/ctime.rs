use std::ffi::CStr;
// The reason to use c to get the time is because it is preetier than the rust version
// It formats it nicley as dayoftheweek month dayofmonth HH:MM:SS year
// And it is quite reliable (not to say rust isn't)

mod c {
    use std::ffi::c_char;
    unsafe extern "C" {
        pub fn get_c_time () -> *const c_char;
    }
}

pub fn get_c_time () -> String {
    // The safe wrapper that is completely unsafe
    // Maybe use an option instead
    unsafe {
        let result_ptr = c::get_c_time();
        if result_ptr.is_null() {
            String::from("Invalid Time")
        }
        else {
            CStr::from_ptr(result_ptr).to_string_lossy().into_owned()
        }
    }
}