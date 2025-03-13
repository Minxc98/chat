use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub key_pair: KeyPairConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub db_url: String,
    pub port: u16,
    pub redis_url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPairConfig {
    pub private_key: String,
    pub public_key: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("chat_server/app.yml"),
            File::open("/etc/config/app.yml"),
        ) {
            (Ok(file), _) => serde_yaml::from_reader(file),
            (_, Ok(file)) => serde_yaml::from_reader(file),
            _ => {
                bail!("config file not found")
            }
        };
        Ok(ret?)
    }
}
