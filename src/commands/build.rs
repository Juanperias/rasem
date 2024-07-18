use crate::{
    models::build_process,
    utils::{exec::exec, get_config::get_config},
};
use anyhow::Result;

pub fn build_command(run: bool) -> Result<()> {
    let config = get_config()?;

    // Stage 1
    build_process::prepare_folders()?;

    // Stage 2
    build_process::build_rust()?;

    // Stage 3
    build_process::build_asm(config.modules.clone())?;

    // Stage 4
    build_process::link(config.modules.clone())?;

    // Stage 5
    build_process::make_os()?;

    if run {
        exec("qemu-system-x86_64 -drive format=raw,file=\"os.bin\",index=0,if=floppy, -m 128M")?;
    }

    Ok(())
}
