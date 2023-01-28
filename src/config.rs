use std::fs;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keyboard: KeyboardConfig,
}

#[derive(Debug, Deserialize)]
pub struct KeyboardConfig {
    pub char_delay: u64,
    pub line_delay: u64,
}

pub fn load_config_file() -> Result<Config, std::io::Error> {
    let f = "config/config.toml";
    let buf = fs::read_to_string(f)?;
    let config: Config = toml::from_str(&buf)?;
    Ok(config)
}
