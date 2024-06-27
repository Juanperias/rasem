# How to install rasem

The first thing to do is to have rust installed if you do not have it installed you can run this command

<code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs '| sh</code>

then you will need to have nasm installed to install it on arch linux you can use this command

<code>sudo pacman -S nasm</code>

also you need to have the cross compiler tool which you can install in this github
in arch linux: https://github.com/mell-o-tron/MellOs/blob/main/A_Setup/setup-gcc-arch.sh
in debian: https://github.com/mell-o-tron/MellOs/blob/main/A_Setup/setup-gcc-debian.sh

once you have this tool installed you must make a fork of the repository and clone it I used nightly for development and testing so it runs:

<code>rustup default nightly</code>

the file run.sh is the combination of make clean and make build if you want to use it run the following commands

<code>
chmod +x run.sh
./run.sh
</code>

once it has been compiled you can run the following command (you must have qemu installed on your computer)

<code>qemu-system-x86_64 -drive format=raw,file="binaries/rasem.bin",index=0,if=floppy, -m 128M</code>

In case everything went well you will see a blue exclamation mark, if there was an error open an issue on github and I will help you with your error.
