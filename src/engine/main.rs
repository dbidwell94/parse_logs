mod config;
mod plugin;

use crate::plugin::parse_plugin;
use crate::{config::Config, plugin::Plugin};
use anyhow::Result as AnyhowResult;
use notify::{
    event::Event, INotifyWatcher, RecommendedWatcher, RecursiveMode, Result as NotifyResult,
    Watcher,
};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc::Receiver as StdReceiver;
use tokio::io::{AsyncSeekExt, BufReader, BufWriter};
use tokio::sync::mpsc::{Receiver as TokioReceiver, Sender as TokioSender};
use tokio::task::JoinHandle;

struct Engine {
    plugins: HashMap<String, Option<Plugin>>,
    log_senders: HashMap<String, Option<TokioSender<String>>>,
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        let mut to_return = Self {
            plugins: HashMap::new(),
            config: config.clone(),
            log_senders: HashMap::new(),
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

    pub async fn start_parse(mut self) -> anyhow::Result<()> {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for (_, mut value_option) in self.plugins {
            let value = value_option.take();
            if let Some(plugin) = value {
                let (sx, rx): (TokioSender<String>, TokioReceiver<String>) =
                    tokio::sync::mpsc::channel(100);
                self.log_senders.insert(plugin.get_log_path()?, Some(sx));
                handles.push(tokio::spawn(async move {
                    parse_plugin(plugin, rx).await;
                }));
            }
        }
        let mut send_threads: Vec<JoinHandle<anyhow::Result<()>>> = Vec::new();
        for (log_path, mut sender_option) in self.log_senders {
            let sender_option = sender_option.take();
            if let Some(sx) = sender_option {
                send_threads.push(tokio::spawn(async move {
                    let (rx, mut watcher) = create_watcher()?;
                    watcher.watch(Path::new(&log_path), RecursiveMode::NonRecursive)?;
                    let mut file = OpenOptions::new().read(true).open(&log_path)?;
                    let mut previous_size = file.metadata()?.len();
                    file.seek(SeekFrom::Start(previous_size))?;
                    let mut buffer = [0u8; 1024 * 1024];
                    loop {
                        for res in rx.iter() {
                            match res {
                                Ok(output) => match output.kind {
                                    notify::EventKind::Modify(ty) => match ty {
                                        notify::event::ModifyKind::Data(_) => {
                                            let current_size = file.metadata()?.len();
                                            let delta_size =
                                                (current_size as i64) - (previous_size as i64);
                                            if delta_size < 1 {
                                                continue;
                                            }
                                            let read_bytes = file.read(&mut buffer)? as i64;
                                            if read_bytes != delta_size {
                                                panic!("Something went wrong, read wrong amount of bytes");
                                            }
                                            let string = std::str::from_utf8(&buffer[0..(read_bytes as usize)])?;
                                            previous_size = current_size;
                                            file.seek(SeekFrom::Start(previous_size))?;
                                            println!(
                                                "Data changed at {} changed.\n\tdelta: {}\n\tcontents: {}",
                                                log_path, delta_size, string
                                            );
                                        }
                                        _ => {
                                        }
                                    },
                                    notify::EventKind::Remove(rm) => match rm {
                                        notify::event::RemoveKind::File => {}
                                        _ => {}
                                    },
                                    _ => {
                                    }
                                },
                                Err(e) => {
                                    println!("Error!: {:?}", e);
                                }
                            }
                        }
                    }

                    return Ok(());
                }));
            } else {
            }
        }

        for thread in send_threads {
            thread.await??;
        }

        Ok(())
    }
}

impl Default for Engine {
    fn default() -> Self {
        let config = config::get_or_create_config(None).expect("Unable to get or create config");
        let plugins = HashMap::<String, Option<Plugin>>::new();

        let mut to_return = Self {
            plugins,
            config,
            log_senders: HashMap::new(),
        };
        for plugin_info in to_return.config.get_plugins().clone() {
            to_return
                .add_plugin(&plugin_info.plugin_location)
                .expect("Unable to load plugin");
        }

        return to_return;
    }
}

type CreateWatcherReturn = (StdReceiver<NotifyResult<Event>>, INotifyWatcher);

fn create_watcher() -> AnyhowResult<CreateWatcherReturn> {
    let (tx, rx) = std::sync::mpsc::channel();

    let watcher = RecommendedWatcher::new(tx, notify::Config::default())?;

    Ok((rx, watcher))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let engine = Engine::new(config::get_or_create_config(Some("./config-devel.yaml"))?);
    engine.start_parse().await?;

    return Ok(());
}
