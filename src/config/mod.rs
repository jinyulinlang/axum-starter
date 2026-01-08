use std::sync::LazyLock;

use anyhow::Context;
use config::{Environment, File, FileFormat};
use serde::Deserialize;

use crate::config::{database::DatabaseConfig, server::ServerConfig};

mod database;
pub mod server;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        config::Config::builder()
            .add_source(
                File::with_name("application")
                    .required(true)
                    .format(FileFormat::Yaml),
            )
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow::anyhow!("Failed to load config"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize config"))
    }
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
