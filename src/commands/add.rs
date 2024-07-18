use anyhow::{anyhow, Result};

use crate::utils::get_config::get_config;
use crate::utils::write_config::write_config;

pub fn add_command(module: String) -> Result<()> {
    let mut config = get_config()?;

    if config.modules.iter().any(|element| element == &module) {
        return Err(anyhow!("This module already exists"));
    }

    config.modules.push(module);

    write_config(config)?;

    println!("Your module is added!");

    Ok(())
}
