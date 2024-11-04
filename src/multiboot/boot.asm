extern _start
extern _kernel_start
extern _kernel_end
extern _paging_end

section .multiboot_header
align 4
dd 0xE85250D6
dd 0
dd - (0xE85250D6 + 0)

section .bootstrap_stack
align 16
stack_bottom:
times 32768 db 0
stack_top:

section .bss
align 4096
boot_page_directory:
    resb 4096
boot_page_table:
    resb 4096 * 2

section .boot
global start
start:
    cli
    mov edi, boot_page_table - 0xC0000000
    mov esi, 0
    mov ecx, 2048

.loop_start:
    cmp esi, 0
    jl .skip_mapping
    cmp esi, 0xC0800000 - 0xC0000000
    jge .end_mapping
    mov edx, esi
    or edx, 0x003
    mov [edi], edx

.skip_mapping:
    add esi, 4096
    add edi, 4
    loop .loop_start

.end_mapping:
    mov dword [boot_page_directory - 0xC0000000], boot_page_table - 0xC0000000 + 0x003
    mov dword [boot_page_directory - 0xC0000000 + 4], boot_page_table - 0xC0000000 + 0x1003
    mov dword [boot_page_directory - 0xC0000000 + 768 * 4], boot_page_table - 0xC0000000 + 0x003
    mov dword [boot_page_directory - 0xC0000000 + 769 * 4], boot_page_table - 0xC0000000 + 0x1003
    mov ecx, boot_page_directory - 0xC0000000
    mov cr3, ecx
    mov ecx, cr0
    or ecx, 0x80010000
    mov cr0, ecx
    lea ecx, [rel .higher_half]
    jmp ecx

section .text

.higher_half:
    mov dword [boot_page_directory], 0
    mov ecx, cr3
    mov cr3, ecx
    mov esp, stack_top
    push ebx
    push eax
    call _start
