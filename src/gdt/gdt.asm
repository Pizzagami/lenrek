section .text

gdt_flush:
    mov eax, [esp+4]             ; 1st argument to this function is a pointer to a GdtPtr struct containing the limit and location of the Gdt.
    lgdt [eax]                   ; Load the new Gdt
    ljmp 0x8, full_flush

full_flush:
    mov ax, 0x10
    mov ds, ax
    mov ss, ax
    xor ax, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    ret
