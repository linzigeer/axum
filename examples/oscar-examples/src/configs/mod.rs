#![allow(dead_code)]
mod db_config;
mod server_config;

use std::sync::LazyLock;

use crate::configs::db_config::DBConfig;
use anyhow::{anyhow, Context};
use serde::Deserialize;
pub use server_config::ServerConfig;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load_config().expect("Error occurred while call AppConfig::load_config()"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_config: ServerConfig,
    pub db_config: DBConfig,
}

impl AppConfig {
    pub fn load_config() -> anyhow::Result<Self> {
        config::Config::builder()
            .add_source(
                config::File::with_name("examples/oscar-examples/application.yaml").required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow!("Failed to load configuration"))?
            .try_deserialize::<AppConfig>()
            .with_context(|| anyhow!("Failed to deserialize configuration"))
    }

    pub fn get_server_config(&self) -> &ServerConfig {
        &self.server_config
    }

    pub fn get_db_config(&self) -> &DBConfig {
        &self.db_config
    }
}

pub fn get_app_config() -> &'static AppConfig {
    &CONFIG
}
