/// Settings for the crate
use std::lazy::SyncLazy;

use crate::prelude::*;
use config::{Config, File};
use serde_derive::Deserialize;

/// AdventOfCode settings
#[derive(Debug, Deserialize)]
#[serde(default)]
pub(crate) struct AdventOfCode {
    pub cache_time: u64,
    pub session_secret: String,
}

impl std::default::Default for AdventOfCode {
    fn default() -> Self {
        Self {
            cache_time: 3600,
            session_secret: "".into(),
        }
    }
}

/// Logging level setting
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Logging {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::default::Default for Logging {
    fn default() -> Self {
        Self::Info
    }
}

/// Convert configured logging to the simplelog::LevelFilter implementation
impl Into<log::LevelFilter> for Logging {
    fn into(self) -> log::LevelFilter {
        match self {
            Logging::Debug => log::LevelFilter::Debug,
            Logging::Info => log::LevelFilter::Info,
            Logging::Warn => log::LevelFilter::Warn,
            Logging::Error => log::LevelFilter::Error,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Settings {
    pub(crate) advent_of_code: AdventOfCode,
    pub(crate) logging: Logging,
}

static SETTINGS: SyncLazy<Settings> = SyncLazy::new(|| {
    Settings::load()
        .context("Static loading of configuration")
        .expect("could not load configuration")
});

impl Settings {
    pub fn get() -> &'static SyncLazy<Settings> {
        &SETTINGS
    }

    fn load() -> Result<Self> {
        let config = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(
                File::from(
                    dirs::home_dir()
                        .context("Locating home directory")?
                        .join(".config")
                        .join("puzzling.toml"),
                )
                .required(false),
            )
            // Then merge in the "workspace" configuration file
            .add_source(File::with_name("puzzling"))
            .build()
            .context("Loading workspace config file (puzzling.toml)")?;

        config.try_deserialize().context("Deserializing settings")
    }
}
