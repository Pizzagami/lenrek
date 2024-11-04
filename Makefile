RUSTC := rustup run nightly-2024-06-12 rustc
CARGO := rustup run nightly-2024-06-12 cargo

arch ?= i386-unknown-none
RELEASE = target/i386-unknown-none/debug/release
KERNEL = target/i386-unknown-none/release/libkernel.a
GRUB_CFG = src/arch/i386-unknown-none/grub.cfg

ISO = os-$(arch).iso

all: kernel iso

kernel:
	cargo build --release
	cp $(KERNEL) .

iso: kernel
	mkdir -pv iso/boot/grub
	cp libkernel.a iso/boot
	nasm -f elf32 src/multiboot/boot.asm -o iso/boot/boot.o
	ld -m elf_i386 -n -o iso/boot/kernel.bin -T src/arch/$(arch)/linker.ld iso/boot/boot.o iso/boot/libkernel.a
	cp $(GRUB_CFG) iso/boot/grub
	grub-mkrescue -o $(ISO) iso

install:
	sudo apt install -y nasm curl gcc grub-common grub-pc-bin binutils xorriso mtools qemu-system
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	. ~/.cargo/env
	rustup install nightly-2024-06-12
	rustup default nightly-2024-06-12
	rustup target add i586-unknown-linux-gnu
	rustup component add rust-src --toolchain nightly-2024-06-12-x86_64-unknown-linux-gnu

run:
	LD_PRELOAD=/lib/x86_64-linux-gnu/libpthread.so.0 /usr/bin/qemu-system-i386 -cdrom $(ISO)

clean:
	rm -rf  target iso kernel os-$(arch).iso libkernel.a
	cargo clean

re: clean all

.PHONY: all re clean run iso kernel
