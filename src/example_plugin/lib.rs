use std::ffi::{c_char, CString};

const PLUGIN_NAME: &[u8] = b"Example Plugin";
const LOG_PATH: &[u8] = b"/var/log/auth.log";

#[no_mangle]
extern "C" fn get_plugin_name() -> *mut c_char {
    return CString::new(PLUGIN_NAME).unwrap().into_raw();
}

#[no_mangle]
extern "C" fn get_log_path() -> *mut c_char {
    return CString::new(LOG_PATH).unwrap().into_raw();
}

#[no_mangle]
extern "C" fn parse_log_string(raw_log_str: *mut c_char) {
    let log_str = unsafe { CString::from_raw(raw_log_str) };

    panic!("Not implemented");
}

#[no_mangle]
extern "C" fn check_log_parseable(raw_log_str: *const c_char) -> bool {
    return true;
}
