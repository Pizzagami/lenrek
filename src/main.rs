#![no_std]
#![no_main]
#![feature(naked_functions)]

mod vga;
mod utils;
mod gdt;
mod shell;
mod idt;
mod keyboard;
mod asm;

use vga::Colors;

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

    gdt::init();
    idt::init();

    cli!();

    vga::reset_screen();
    utils::print_header();
    vga::set_color(Colors::White);
    println!();

    shell::print_prompt();

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
