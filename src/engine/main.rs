mod config;
mod plugin;

use crate::{config::Config, plugin::Plugin};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

struct Engine {
    plugins: HashMap<String, Plugin>,
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        let mut to_return = Self {
            plugins: HashMap::new(),
            config: config.clone(),
        };

        for plugin_info in config.get_plugins() {
            to_return
                .add_plugin(&plugin_info.plugin_location)
                .expect("Unable to load plugin");
        }

        return to_return;
    }

    pub fn get_config(&self) -> &Config {
        return &self.config;
    }

    pub fn add_plugin(&mut self, location: &str) -> anyhow::Result<()> {
        let plugin = Plugin::new(location)?;
        let plugin_name = plugin.get_plugin_name();
        self.plugins.insert(plugin_name.to_owned(), plugin);
        Ok(())
    }
}

impl Default for Engine {
    fn default() -> Self {
        let config = config::get_or_create_config(None).expect("Unable to get or create config");
        let plugins = HashMap::<String, Plugin>::new();

        let mut to_return = Self { plugins, config };
        for plugin_info in to_return.config.get_plugins().clone() {
            to_return
                .add_plugin(&plugin_info.plugin_location)
                .expect("Unable to load plugin");
        }

        return to_return;
    }
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::new(config::get_or_create_config(Some("./config-devel.yaml"))?);
    println!("{:?}", engine.plugins);

    return Ok(());
}
