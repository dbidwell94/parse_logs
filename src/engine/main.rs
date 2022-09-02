mod config;
mod plugin;

use crate::{config::Config, plugin::Plugin};
use std::collections::HashMap;
use tokio::task::JoinHandle;

struct Engine {
    plugins: HashMap<String, Option<Plugin>>,
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

    pub fn add_plugin(&mut self, location: &str) -> anyhow::Result<()> {
        let plugin = Plugin::new(location)?;
        let plugin_name = plugin.get_plugin_name();
        self.plugins.insert(plugin_name.to_owned(), Some(plugin));
        Ok(())
    }

    pub async fn start_parse(self) -> anyhow::Result<()> {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for (_, mut value_option) in self.plugins {
            let value = value_option.take();
            if let Some(plugin) = value {
                handles.push(tokio::spawn(async move {}));
            }
        }
        for handle in handles {
            handle.await?;
        }
        Ok(())
    }
}

impl Default for Engine {
    fn default() -> Self {
        let config = config::get_or_create_config(None).expect("Unable to get or create config");
        let plugins = HashMap::<String, Option<Plugin>>::new();

        let mut to_return = Self { plugins, config };
        for plugin_info in to_return.config.get_plugins().clone() {
            to_return
                .add_plugin(&plugin_info.plugin_location)
                .expect("Unable to load plugin");
        }

        return to_return;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let engine = Engine::new(config::get_or_create_config(Some("./config-devel.yaml"))?);
    engine.start_parse().await?;

    return Ok(());
}
