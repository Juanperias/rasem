# Rasem

## (ASSEMBLY + RUST)

rasem is a **cross compiler** that allows you to make your operating system using assembler and rust, the **bootloader** and other parts can be made in assembler and the kernel can be made in rust.

## Docs

In the new version of rasem V2, you have a cli that improves the development experience.

### Creating a project

to create a project you run

```bash
rasem new <project-name>
```

### Build a project

to compile your project you just do

to see the commands that are running you use

```bash
rasem build --verbose
rasem build -v
```

to do build and then run the os is

```bash
rasem build --run
rasem build -r
```

## How to install

Installing rasem is easy (if you use arch linux) you just need to run the install-arch script.

At some point I will create a script for installation on debian.

For now you can install it manually.

Requirements for manual installation:

1.  nasm
2.  rust
3.  clang
4.  i386-elf-ld in case in [debian can be easily installed](https://github.com/mell-o-tron/MellOs/blob/main/A_Setup/setup-gcc-debian.sh)
5.  git

after you have everything installed you should run the following commands

```bash
git clone https://github.com/Juanperias/rasem.git
cd rasem
cargo install rasem --path .
```
