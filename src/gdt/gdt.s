.global gdt_flush
.global check_gdt
.text

gdt_flush:
    movl 4(%esp), %eax
    lgdt (%eax)
    ljmp $0x8, $complete_flush

# https://stackoverflow.com/questions/23978486/far-jump-in-gdt-in-bootloader

complete_flush:
    mov $0x10, %ax
    mov %ax, %ds
    mov %ax, %ss
    mov $0x0, %ax
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs
    ret

