arch ?= x86
KERNEL = target/x86/debug/kernel
GRUB_CFG = src/arch/x86/grub.cfg

ISO = build/os-$(arch).iso

all: kernel iso

kernel:
	cargo build
	cp $(KERNEL) .

iso: kernel
	mkdir -pv iso/boot/grub
	cp kernel iso/boot
	cp $(GRUB_CFG) iso/boot/grub
	grub-mkrescue -o $(ISO) iso

run:
	qemu-system-i386 -cdrom $(ISO)

clean:
	rm -rf  target

re: clean all

.PHONY: all re clean run iso kernel
