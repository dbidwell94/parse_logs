use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use thiserror::Error;

const DEFAULT_CONFIG_LOCATION: &str = "/etc/parse_logs/config.yaml";

#[derive(Error, Debug, Clone)]
enum ConfigError {
    #[error("The config file already exists at {0} ")]
    FileExists(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    plugins: Vec<PluginInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub plugin_location: String,
}

impl Config {
    pub fn get_plugins(&self) -> &Vec<PluginInfo> {
        return &self.plugins;
    }
}

pub fn generate_default_config(config_file_location_option: Option<&str>) -> anyhow::Result<()> {
    let config_file_location = config_file_location_option.unwrap_or(DEFAULT_CONFIG_LOCATION);
    let config = Config {
        plugins: vec![PluginInfo {
            plugin_location: String::from("/etc/parse_logs/example_library.so"),
        }],
    };

    let path = Path::new(config_file_location);
    if path.exists() {
        return Err(anyhow!(ConfigError::FileExists(
            config_file_location.to_owned()
        )));
    }

    let to_write = serde_yaml::to_string(&config)?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_file_location)?;

    let mut writer = BufWriter::new(file);
    writer.write(to_write.as_bytes())?;

    Ok(())
}

pub fn get_or_create_config(config_file_location_option: Option<&str>) -> anyhow::Result<Config> {
    let config_file_location = config_file_location_option.unwrap_or(DEFAULT_CONFIG_LOCATION);

    if !Path::new(config_file_location).exists() {
        generate_default_config(Some(config_file_location))?;
    };
    let file = OpenOptions::new().read(true).open(config_file_location)?;
    let config: Config = serde_yaml::from_reader(BufReader::new(file))?;

    Ok(config)
}
