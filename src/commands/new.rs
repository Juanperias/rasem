use crate::utils::exec::exec;
use anyhow::Result;

pub fn new_command(name: String) -> Result<()> {
    let clone = format!(
        "git clone https://github.com/Juanperias/rasem-template {}",
        name
    );
    println!("Creating your project!");
    exec(clone.as_str())?;
    println!("Your project is created!");
    Ok(())
}
