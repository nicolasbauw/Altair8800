use directories::UserDirs;
use serde_derive::Deserialize;
use std::{error::Error, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keyboard: KeyboardConfig,
    pub memory: MemConfig,
    pub snapshot: SnapshotDir
}

#[derive(Debug, Deserialize)]
pub struct KeyboardConfig {
    pub char_delay: u64,
    pub line_delay: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MemConfig {
    pub rom: String,
    pub ram: u16,
}

#[derive(Debug, Deserialize)]
pub struct SnapshotDir {
    pub dir: String
}

pub fn load_config_file() -> Result<Config, Box<dyn Error>> {
    let user_dirs = UserDirs::new().ok_or("Could not get user directory")?;
    let mut cfg = user_dirs.home_dir().to_path_buf();
    cfg.push(".config/teletype/config.toml");
    let buf = fs::read_to_string(cfg)?;
    let config: Config = toml::from_str(&buf)?;
    Ok(config)
}
