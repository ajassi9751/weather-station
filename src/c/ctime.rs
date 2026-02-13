use std::ffi::CStr;

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
        let result_ptr =  c::get_c_time();
        if result_ptr.is_null() {
            String::from("Invalid Time")
        }
        else {
            CStr::from_ptr(result_ptr).to_string_lossy().into_owned()
        }
    }
}