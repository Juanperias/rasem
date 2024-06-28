RUST = cargo
RUST_FLAGS = rustc -- --emit=obj -g -C link-args=-no-pie -C relocation-model=static
ASM = nasm


KERNEL_BUILD = target/rasem_x86/debug/deps
KERNEL_TARGET = base
FINAL_KERNEL = kernel
BOOTLOADER = boot
KERNEL_ENTRY = entry

ZEROES = zeroes
LD = i386-elf-ld
ASM_BIN_FLAGS := -f bin -o
ASM_ELF_FLAGS := -f elf -o

DIST_FOLDER = binaries
OS_TARGET = rasem
CREATE_FOLDER = mkdir
COPY_FOLDER = cp
DELETE_FOLDER := rm -r


build:
	$(CREATE_FOLDER) $(DIST_FOLDER)

	$(RUST) $(RUST_FLAGS)

	$(COPY_FOLDER) $(KERNEL_BUILD)/*.o $(DIST_FOLDER)/$(KERNEL_TARGET).o
	
	$(ASM) $(BOOTLOADER).asm $(ASM_BIN_FLAGS) $(DIST_FOLDER)/$(BOOTLOADER).bin 
	$(ASM) $(KERNEL_ENTRY).asm $(ASM_ELF_FLAGS) $(DIST_FOLDER)/$(KERNEL_ENTRY).o
	$(ASM) $(ZEROES).asm $(ASM_BIN_FLAGS) $(DIST_FOLDER)/$(ZEROES).bin

	$(LD) -o $(DIST_FOLDER)/$(FINAL_KERNEL).bin -Ttext 0x1000 $(DIST_FOLDER)/$(KERNEL_ENTRY).o $(DIST_FOLDER)/$(KERNEL_TARGET).o --oformat binary
	cat $(DIST_FOLDER)/$(BOOTLOADER).bin $(DIST_FOLDER)/$(FINAL_KERNEL).bin $(DIST_FOLDER)/$(ZEROES).bin > $(DIST_FOLDER)/$(OS_TARGET).bin
clean:
	$(DELETE_FOLDER) $(DIST_FOLDER)
