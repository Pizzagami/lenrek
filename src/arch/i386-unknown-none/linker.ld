HH_OFF = 0xC0000000;
ENTRY(start)

PHDRS {
    idt PT_LOAD FLAGS(4);
    gdt PT_LOAD FLAGS(4);
    boot PT_LOAD FLAGS(5);
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);
    bss PT_LOAD FLAGS(6);
}

SECTIONS {
    .idt 0x00000000 : { *(.idt) . = ALIGN(0x800); } : idt
    .gdt 0x00000800 : { *(.gdt) . = ALIGN(4); } : gdt

    .boot 0x1000 : {
        _multi_boot_header = .;
        KEEP(*(.multiboot_header))
    } : boot

    . = HH_OFF + 1M;
    _kernel_start = .;

    .text ALIGN(4K) : AT(ADDR(.text) - HH_OFF) { *(.text .text.*) } : text
    .rodata ALIGN(4K) : AT(ADDR(.rodata) - HH_OFF) { *(.rodata .rodata.*) } : rodata
    .data ALIGN(4K) : AT(ADDR(.data) - HH_OFF) { *(.data .data.*) } : data
    .bss ALIGN(4K) (NOLOAD) : AT(ADDR(.bss) - HH_OFF) { *(.bss .bss.*) *(COMMON) *(.bootstrap_stack) } : bss

    . = ALIGN(4K);
    _kernel_end = .;
}
