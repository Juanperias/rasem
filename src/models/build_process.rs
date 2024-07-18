use std::path::Path;

use crate::utils::exec::exec;

use anyhow::Result;

pub fn prepare_folders() -> Result<()> {
    let kernel_build = Path::new("kernel_build/");

    if kernel_build.exists() {
        std::fs::remove_dir_all(kernel_build)?;
    }

    std::fs::create_dir(kernel_build)?;

    Ok(())
}

pub fn build_rust() -> Result<()> {
    exec( "cargo rustc -- --emit=asm -o kernel_build/base.s -g -C link-args=-no-pie -C relocation-model=static")?;
    exec("cp kernel_build/base*.s kernel_build/base.s")?;

    exec("clang  -m32 -c -o kernel_build/base.o kernel_build/base.s")?;

    Ok(())
}

pub fn build_asm(modules: Vec<String>) -> Result<()> {
    exec("nasm asm/boot.asm -f bin -o kernel_build/boot.bin")?;
    exec("nasm asm/entry.asm -f elf -o kernel_build/entry.o")?;
    exec("nasm asm/zeroes.asm -f bin -o kernel_build/zeroes.bin")?;

    for module in modules {
        let module_build = format!("nasm asm/{0}.asm -f elf -o kernel_build/{0}.o", module);

        exec(module_build.as_str())?;
    }

    Ok(())
}

pub fn link(modules: Vec<String>) -> Result<()> {
    let mut base = String::from("/usr/local/i386elfgcc/bin/i386-elf-ld --no-pie -o kernel_build/kernel.bin -Ttext 0x1000 kernel_build/entry.o kernel_build/base.o ");

    for module in modules {
        let route = format!("kernel_build/{}.o ", module);

        base += &route;
    }

    base += " --oformat binary";

    exec(&base)?;

    Ok(())
}

pub fn make_os() -> Result<()> {
    exec("cat kernel_build/boot.bin kernel_build/kernel.bin kernel_build/zeroes.bin > os.bin")?;
    Ok(())
}
