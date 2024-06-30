export PATH=$PATH:/usr/local/i386elfgcc/bin
make clean
make build
qemu-system-x86_64 -drive format=raw,file="binaries/rasem.bin",index=0,if=floppy, -m 128M
