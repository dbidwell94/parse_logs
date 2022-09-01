use libloading::{Library, Symbol};
use std::ffi::{c_char, CStr, CString};
use thiserror::Error;

type GetLogPath = unsafe extern "C" fn() -> *mut c_char;
const LOG_PATH_FUNC_NAME: &[u8] = b"get_log_path";

type GetPluginName = unsafe extern "C" fn() -> *mut c_char;
const PLUGIN_NAME_FUNC_NAME: &[u8] = b"get_plugin_name";

type ParseLogString = unsafe extern "C" fn(*mut c_char);
const PLUGIN_PARSE_LOG_STRING_NAME: &[u8] = b"parse_log_string";

type CheckLogParseable = unsafe extern "C" fn(*const c_char) -> bool;

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
}
