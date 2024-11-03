#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#[macro_use]
mod macros;
mod utils;
mod tools;
mod gdt;
mod shell;
mod idt;
mod keyboards;
mod memory;
mod asm;
mod exceptions;

use vga::Colors;
use crate::tools::debug;

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

    //interrupts::disable();
    debug::init_serial_port();
    gdt::init();
    idt::init();
    //interrupts::init();

    cli!();
    memory::physical_memory_managment::physical_memory_manager_init();
    unsafe { memory::page_directory::init_page_directory() };
    memory::page_directory::enable_paging();
    vga::reset_screen();
    utils::print_header();
    vga::set_color(Colors::White);
    println!();

    shell::print_prompt();
    memory::vmalloc::vmalloc_test();
	memory::kmalloc::kmalloc_test();

    sti!();
    loop {
        hlt!();
    }

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let arg = format_args!("");
    let message =  _info.message().unwrap_or(&arg);
    let location = _info.location().unwrap();

    vga::set_color(Colors::BrightRed);
    print!("[PANIC ");
    vga::set_color(Colors::BrightWhite);
    print!("{}", location);
    vga::set_color(Colors::BrightRed);
    print!("]: ");
    vga::set_color(Colors::BrightWhite);
    println!("{}", message);
    loop {}
}
