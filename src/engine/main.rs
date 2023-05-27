mod config;
mod man;
use anyhow::{anyhow, Result as AnyhowResult};
use clap::Parser;
use config::{CompiledConfig, Config};
use man::Args;
use notify::event::ModifyKind;
use notify::{RecommendedWatcher, Watcher};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Seek};
use std::path::{Path, PathBuf};

pub fn parse_err(e: Box<dyn std::error::Error + Send + Sync>) -> anyhow::Error {
    anyhow!(e)
}

struct Engine {
    config: HashMap<PathBuf, (CompiledConfig, usize)>,
    buffer: [u8; 10 * 1000],
}

impl Engine {
    pub fn new(config: Config) -> AnyhowResult<Self> {
        let mut config_map: HashMap<PathBuf, (CompiledConfig, usize)> = HashMap::new();
        for conf in config.0 {
            let cc = CompiledConfig::try_from(conf).map_err(parse_err)?;

            let oo = OpenOptions::new()
                .read(true)
                .open(&cc.log_location)
                .map_err(|e| anyhow!(e))?;

            let file_size = oo.metadata().map_err(|e| anyhow!(e))?.len();

            config_map.insert(cc.log_location.clone(), (cc, file_size as usize));
        }

        let to_return = Self {
            config: config_map,
            buffer: [0u8; 10 * 1000],
        };

        return Ok(to_return);
    }

    pub async fn begin_watching(&mut self) -> AnyhowResult<()> {
        let (sx, mut rx) = tokio::sync::mpsc::channel(100);
        if self.config.len() < 1 {
            return Ok(());
        }
        let mut watcher = RecommendedWatcher::new(
            move |e| {
                let _ = sx.blocking_send(e);
            },
            notify::Config::default(),
        )
        .map_err(|e| anyhow!(e))?;

        for (_, (config, _)) in &self.config {
            watcher
                .watch(
                    Path::new(&config.log_location),
                    notify::RecursiveMode::NonRecursive,
                )
                .map_err(|e| anyhow!(e))?;
        }

        while let Some(evt) = rx.recv().await {
            let evt = evt.map_err(|e| anyhow!(e))?;
            for path in &evt.paths {
                if let Some((found_config, location)) = self.config.get_mut(path) {
                    if let Err(e) =
                        Self::parse_log_event(evt, found_config, location, &mut self.buffer).await
                    {
                        println!("Error: {e}");
                    }
                    break;
                }
            }
        }

        Ok(())
    }

    async fn parse_log_event(
        evt: notify::Event,
        config: &CompiledConfig,
        previous_pos: &mut usize,
        buffer: &mut [u8],
    ) -> AnyhowResult<()> {
        if let notify::EventKind::Remove(_) = evt.kind {
            return Ok(());
        }
        let oo = OpenOptions::new()
            .read(true)
            .open(&config.log_location)
            .map_err(|e| anyhow!(e))?;
        let mut reader = BufReader::new(oo);

        if let notify::EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any)) =
            evt.kind
        {
            reader
                .seek(std::io::SeekFrom::Start(*previous_pos as u64))
                .map_err(|e| anyhow!(e))?;

            let amt = reader.read(buffer).map_err(|e| anyhow!(e))?;

            *previous_pos += amt;
            let buff_slice = &buffer[..amt];
            let str = std::str::from_utf8(buff_slice).map_err(|e| anyhow!(e))?;

            // if config.parse_regex.is_match(&str) {
            //     println!("Found matching log for {0:?}", config.title);
            // }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let oo = OpenOptions::new().read(true).open(args.config_path);
    let config = Config::try_from(oo).map_err(parse_err)?;

    let mut engine = Engine::new(config)?;
    if !args.test_config {
        engine.begin_watching().await?;
    }

    return Ok(());
}
