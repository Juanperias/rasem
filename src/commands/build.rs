use crate::utils::exec::exec;
use anyhow::Result;
use std::path::Path;

pub fn build_command(verbose: bool, run: bool) -> Result<()> {
    let kernel_build = Path::new("kernel_build/");

    if kernel_build.exists() {
        exec("rm -r kernel_build")?;
    }

    let commands = vec![
        "mkdir kernel_build",
        "cargo rustc -- --emit=asm -o kernel_build/base.s -g -C link-args=-no-pie -C relocation-model=static",
        "nasm asm/boot.asm -f bin -o kernel_build/boot.bin",
        "nasm asm/entry.asm -f elf -o kernel_build/entry.o",
        "nasm asm/zeroes.asm -f bin -o kernel_build/zeroes.bin",
        "cat kernel_build/base*.s > kernel_build/base.s",
        "clang  -m32 -c -o kernel_build/base.o kernel_build/base.s",
        "/usr/local/i386elfgcc/bin/i386-elf-ld --no-pie -o kernel_build/kernel.bin -Ttext 0x1000 kernel_build/entry.o kernel_build/base.o --oformat binary",
        "cat kernel_build/boot.bin kernel_build/kernel.bin kernel_build/zeroes.bin > os.bin",
    ];

    println!("Building your project");
    for command in commands {
        exec(command)?;
        if verbose {
            println!("RUNNING {}", command);
        }
    }
    println!("Finish, your os is in os.bin");

    if run {
        exec("qemu-system-x86_64 -drive format=raw,file=\"os.bin\",index=0,if=floppy, -m 128M")?;
    }

    Ok(())
}
