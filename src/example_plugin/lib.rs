use std::ffi::{c_char, CString};

const PLUGIN_NAME: &[u8] = b"Example Plugin";
const LOG_PATH: &[u8] = b"/home/dbidwell/Documents/test.log";

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
    println!(
        "Incoming log at {:#?}: --\n\tcontents: {:#?}",
        LOG_PATH, log_str
    );
}

#[no_mangle]
extern "C" fn check_log_parseable(raw_log_str: *mut c_char) -> bool {
    let log_str = unsafe { CString::from_raw(raw_log_str) }
        .into_string()
        .unwrap();

    return log_str.contains("test");
}
