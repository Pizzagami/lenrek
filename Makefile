arch ?= x86
rust_os := target/$(arch)/debug/liblenrek.a
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
    build/arch/$(arch)/%.o, $(assembly_source_files))


all: $(kernel)

clean:
	rm -r target build

run: $(iso)
	qemu-system-i386 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	rm -r build/isofiles

install:
	sudo apt install -y nasm curl gcc grub-common grub-pc-bin binutils xorriso mtools qemu-system
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source ~/.cargo/env
	rustup update nightly
	rustup default nightly
	rustup target add i586-unknown-linux-gnu
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

$(kernel): kernel $(assembly_object_files) $(linker_script)
	ld -m elf_i386 -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -f elf32 $< -o $@

kernel:
	cargo build


.PHONY: all clean run iso kernel
