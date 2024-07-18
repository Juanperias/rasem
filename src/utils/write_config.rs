use anyhow::Result;
use std::{fs::File, io::Write};

use crate::models::config::Config;

pub fn write_config(config: Config) -> Result<()> {
    let config_str = toml::to_string(&config)?;
    let mut file = File::create("rasem.toml")?;

    file.write_all(config_str.as_bytes())?;

    Ok(())
}
