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

also to compile the assembly provided by rust you must have clang installed.

<code>sudo pacman -S clang</code>

now in this version the qemu command to create the vm is already included in run.sh

In case everything went well you will see a blue exclamation mark, if there was an error open an issue on github and I will help you with your error.
