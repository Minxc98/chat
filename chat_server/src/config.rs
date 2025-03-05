use serde::{Serialize, Deserialize};
use anyhow::{bail, Result};
use std::fs::File;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("chat_server/app.yml"),
            File::open("/etc/config/app.yml")
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