use anyhow::Result;
use std::fs;

use crate::models::config::Config;

pub fn get_config() -> Result<Config> {
    let config_str = fs::read_to_string("rasem.toml")?;
    let config: Config = toml::from_str(&config_str)?;

    Ok(config)
}
