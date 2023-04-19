use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u32,
    pub base_name: String,
    pub key_update: String,
    pub url_update: String,
}

// Пропарсим конфигурационный файл
pub fn parse() -> Result<Config, std::io::Error> {
    let path = "./config.json";
    let conf_string = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&conf_string)?;
    Result::Ok(config)
}
