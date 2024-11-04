HIGHER_HALF_OFFSET = 0xC0000000;
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

    . = HIGHER_HALF_OFFSET + 1M;
    _kernel_start = .;

    .text ALIGN(4K) : AT(ADDR(.text) - HIGHER_HALF_OFFSET) { *(.text .text.*) } : text
    .rodata ALIGN(4K) : AT(ADDR(.rodata) - HIGHER_HALF_OFFSET) { *(.rodata .rodata.*) } : rodata
    .data ALIGN(4K) : AT(ADDR(.data) - HIGHER_HALF_OFFSET) { *(.data .data.*) } : data
    .bss ALIGN(4K) (NOLOAD) : AT(ADDR(.bss) - HIGHER_HALF_OFFSET) { *(.bss .bss.*) *(COMMON) *(.bootstrap_stack) } : bss

    . = ALIGN(4K);
    _kernel_end = .;
}
