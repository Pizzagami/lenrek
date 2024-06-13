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
	source ~/.cargo/env
	rustup update nightly
	rustup default nightly
	rustup target add i586-unknown-linux-gnu
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

run:
	qemu-system-i386 -cdrom $(ISO)

clean:
	rm -rf  target iso kernel os-x86.iso

re: clean all

.PHONY: all re clean run iso kernel
