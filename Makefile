RUSTC := rustup run nightly-2024-06-12 rustc
CARGO := rustup run nightly-2024-06-12 cargo

arch ?= x86
KERNEL = target/x86/debug/kernel
GRUB_CFG = src/arch/x86/grub.cfg

ISO = os-$(arch).iso

all: kernel iso

kernel:
	cargo build
	cp $(KERNEL) .

iso: kernel
	mkdir -pv iso/boot/grub
	cp kernel iso/boot
	cp $(GRUB_CFG) iso/boot/grub
	grub-file --is-x86-multiboot iso/boot/kernel
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
	rm -rf  target iso kernel os-x86.iso

re: clean all

.PHONY: all re clean run iso kernel
