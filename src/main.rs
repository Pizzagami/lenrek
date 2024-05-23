#![no_std]
#![no_main]
#![feature(naked_functions)]

mod vga;
mod gdt;

fn print_gdt() {
    gdt::print_gdt();
}

#[allow(dead_code)]
pub struct MultibootHeader {
    magic: u32,
    arch: u32,
    magic2: u32
}

#[no_mangle]
#[link_section = ".multiboot"]
pub static MULTIBOOT: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    arch: 0x0,
    magic2: -(0x1BADB002 as i32) as u32
};

#[naked]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!("
            cli
            mov esp, 0xf00000
            call main
            cli
            hlt
        ", options(noreturn));
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {

    let mut cell = vga::Cell::default();
    cell.reset_screen();
    cell.print_string("42******************************************************************************
*                                                                              *
*   #      #####  #   #  #####  #####  #   #             :::      ::::::::     *
*   #      #      ##  #  #   #  #      #  #            :+:      :+:    :+:     *
*   #      #      ##  #  #   #  #      # #           +:+ +:+         +:+       *
*   #      ###    # # #  #####  ###    ##          +#+  +:+       +#+          *
*   #      #      #  ##  ##     #      # #       +#+#+#+#+#+   +#+             *
*   #      #      #  ##  # #    #      #  #           #+#    #+#               *
*   #####  #####  #   #  #  #   #####  #   #         ###   ########.fr         *
*                                                                              *
********************************************************************************\n");
    gdt::init();
    print_gdt();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
