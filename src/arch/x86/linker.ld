OUTPUT_FORMAT(elf32-i386)
ENTRY(start)

SECTIONS {
    . = 1M;

    .multiboot :
    {
        /* ensure that the multiboot header is at the beginning */
       KEEP( *(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }
    .data : 
    {
	*(.data)
    }
    .bss :
    {
	*(.bss)
    }
}
