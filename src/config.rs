use std::{fs, error::Error};
use serde_derive::Deserialize;
use directories::UserDirs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keyboard: KeyboardConfig,
}

#[derive(Debug, Deserialize)]
pub struct KeyboardConfig {
    pub char_delay: u64,
    pub line_delay: u64,
}

pub fn load_config_file() -> Result<Config, Box<dyn Error>> {
    let user_dirs = UserDirs::new().ok_or("Could not get user directory")?;
    let mut cfg = user_dirs.home_dir().to_path_buf();
    cfg.push(".config/teletype/config.toml");
    let buf = fs::read_to_string(cfg)?;
    let config: Config = toml::from_str(&buf)?;
    Ok(config)
}
