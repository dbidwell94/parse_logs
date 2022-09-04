use libloading::{Library, Symbol};
use std::ffi::{c_char, CStr, CString};
use std::sync::mpsc::Receiver;
use thiserror::Error;

type GetLogPath = unsafe extern "C" fn() -> *mut c_char;
const LOG_PATH_FUNC_NAME: &[u8] = b"get_log_path";

type GetPluginName = unsafe extern "C" fn() -> *mut c_char;
const PLUGIN_NAME_FUNC_NAME: &[u8] = b"get_plugin_name";

type ParseLogString = unsafe extern "C" fn(*mut c_char);
const PLUGIN_PARSE_LOG_STRING_NAME: &[u8] = b"parse_log_string";

type CheckLogParseable = unsafe extern "C" fn(*mut c_char) -> bool;
const PLUGIN_CHECK_LOG_PARSEABLE: &[u8] = b"check_log_parseable";

#[derive(Error, Debug)]
enum PluginError {
    #[error("Unable to load plugin at path {0}")]
    PluginLoadError(String),

    #[error("Plugin located at {0} has an invalid signature")]
    InvalidPluginError(String),
}

#[derive(Debug)]
pub struct Plugin {
    plugin_name: String,
    plugin_location: String,
    plugin_library: Library,
}

impl Plugin {
    pub fn new(plugin_location: &str) -> anyhow::Result<Self> {
        let plugin_name_func: Symbol<GetPluginName>;
        let lib: Library;
        let plugin_name: String;

        unsafe {
            lib = Library::new(plugin_location)
                .map_err(|_| PluginError::PluginLoadError(String::from(plugin_location)))?;

            plugin_name_func = lib
                .get(PLUGIN_NAME_FUNC_NAME)
                .map_err(|_| PluginError::PluginLoadError(String::from(plugin_location)))?;

            plugin_name = CString::from_raw(plugin_name_func()).into_string()?;
        }

        Ok(Self {
            plugin_location: plugin_location.to_owned(),
            plugin_name,
            plugin_library: lib,
        })
    }

    pub fn get_plugin_name(&self) -> &str {
        &self.plugin_name
    }

    pub fn is_log_parseable(&self, log_line: &str) -> anyhow::Result<bool> {
        let parse_fn: Symbol<CheckLogParseable> =
            unsafe { self.plugin_library.get(PLUGIN_CHECK_LOG_PARSEABLE)? };

        let parseable = unsafe {
            let c_str = CString::new(log_line)?;
            parse_fn(c_str.into_raw())
        };

        return Ok(parseable);
    }

    pub fn get_log_path(&self) -> anyhow::Result<String> {
        let to_return: String;
        unsafe {
            let get_log_location: Symbol<GetLogPath> =
                self.plugin_library.get(LOG_PATH_FUNC_NAME)?;
            let returned_cstr = CString::from_raw(get_log_location());
            to_return = returned_cstr.into_string()?;
        };
        return Ok(to_return);
    }

    pub fn parse_log_string(&self, log_str: &str) -> anyhow::Result<()> {
        let parse_fn: Symbol<ParseLogString> =
            unsafe { self.plugin_library.get(PLUGIN_PARSE_LOG_STRING_NAME)? };
        unsafe { parse_fn(CString::new(log_str)?.into_raw()) }
        Ok(())
    }
}

pub async fn parse_plugin(plugin: Plugin, rcv: Receiver<String>) -> anyhow::Result<()> {
    let iter = rcv.iter();
    for s in iter {
        if plugin.is_log_parseable(&s)? {
            plugin.parse_log_string(&s)?;
        } else {
            println!("Not Parseable");
        }
    }

    return Ok(());
}
