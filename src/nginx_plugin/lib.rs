use std::ffi::{c_char, CString};
use std::str;

mod nginx;
use nginx::NGINX_LOG_REGEX;

const PLUGIN_NAME: &[u8] = b"Nginx Plugin";
const LOG_PATH: &[u8] = b"/var/log/nginx/access.log";

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
    let log_str = unsafe { CString::from_raw(raw_log_str) }
        .into_string()
        .unwrap();
    println!(
        "Incoming log at {}: --\n\tcontents: {}",
        str::from_utf8(LOG_PATH).unwrap(),
        log_str
    );
}

#[no_mangle]
extern "C" fn check_log_parseable(raw_log_str: *mut c_char) -> bool {
    let log_str = unsafe { CString::from_raw(raw_log_str) }
        .into_string()
        .unwrap();

    println!("{}", &log_str);

    return NGINX_LOG_REGEX.is_match(&log_str.trim());
}
