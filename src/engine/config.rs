use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config(pub Vec<LogConfig>);

impl TryFrom<std::io::Result<File>> for Config {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(file: std::io::Result<File>) -> Result<Self, Self::Error> {
        let file = file.map_err(Box::new)?;
        let reader = BufReader::new(file);
        let config =
            serde_yaml::from_reader::<BufReader<File>, Config>(reader).map_err(Box::new)?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogConfig {
    pub log_location: String,
    pub parse_regex: String,
    pub host_regex: String,
    pub title: String,
}

#[derive(Debug)]
pub struct CompiledConfig {
    pub log_location: PathBuf,
    pub parse_regex: Regex,
    pub host_regex: Regex,
    pub title: String,
}

impl std::hash::Hash for CompiledConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.log_location.hash(state);
        self.title.hash(state);
    }
}

impl PartialEq for CompiledConfig {
    fn eq(&self, other: &Self) -> bool {
        self.log_location == other.log_location && self.title == other.title
    }
}

impl TryFrom<LogConfig> for CompiledConfig {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: LogConfig) -> Result<Self, Self::Error> {
        let regex = Regex::new(&value.parse_regex).map_err(Box::new)?;
        let path = std::fs::canonicalize(Path::new(&value.log_location)).map_err(Box::new)?;
        let host_regex = Regex::new(&value.host_regex).map_err(Box::new)?;

        Ok(Self {
            host_regex,
            log_location: path,
            parse_regex: regex,
            title: value.title,
        })
    }
}
