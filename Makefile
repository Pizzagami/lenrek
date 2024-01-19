arch ?= x86
rust_os := target/$(arch)/debug/liblenrek.a
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
asm_src_files := $(wildcard src/arch/$(arch)/*.asm)
asm_obj_files := $(patsubst src/arch/$(arch)/%.asm, \
    build/arch/$(arch)/%.o, $(asm_src_files))


all: $(kernel)

clean:
	rm -r build

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
	sudo apt install -y grub-common grub-pc-bin binutils xorriso mtools qemu-system
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.sh | sh
	rustup update nightly
	rustup default nightly
	rustup target add i686-unknown-linux-gnu
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

$(kernel): kernel $(asm_obj_files) $(linker_script)
	ld -m elf_i386 -T $(linker_script) -o $(kernel) $(asm_obj_files) $(rust_os)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -f elf32 $< -o $@

kernel:
	cargo build


.PHONY: all clean run iso kernel
