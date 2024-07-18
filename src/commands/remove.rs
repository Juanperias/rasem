use anyhow::Result;

use crate::utils::get_config::get_config;
use crate::utils::write_config::write_config;

pub fn remove_command(module: String) -> Result<()> {
    let mut config = get_config()?;

    if let Some(position) = config.modules.iter().position(|el| *el == module) {
        config.modules.remove(position);
        println!("Module is deleted");
    } else {
        println!("Module {} not exist", module);
    }

    write_config(config)?;

    Ok(())
}
