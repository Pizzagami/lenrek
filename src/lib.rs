#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#[macro_use]
mod macros;
mod tools;
mod gdt;
mod shell;
mod idt;
mod memory;
mod exceptions;
mod multiboot;

use crate::shell::prints;
use crate::tools::debug;
use crate::tools::librs::hlt;
use core::panic::PanicInfo;
use exceptions::{interrupts, keyboard::process_keyboard_input, panic::handle_panic};
use crate::memory::kmem_managment::HK_OFST;

#[no_mangle]
pub extern "C" fn _start(multiboot_magic: u32, multiboot_addr: u32) -> ! {
    init(multiboot_magic, multiboot_addr);
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {
        process_keyboard_input();
        hlt();
    }

}

fn init(multiboot_magic: u32, multiboot_addr: u32) {
    multiboot::validate_multiboot(multiboot_magic, multiboot_addr);
    interrupts::disable();
    debug::init_serial_port();
    gdt::init();
    idt::init();
    interrupts::init();
    multiboot::read_multiboot_info(multiboot_addr + HK_OFST);
    memory::kmem_managment::kmem_manager_init();
    unsafe { memory::page_directory::init_page_directory() };
    memory::page_directory::enable_paging();
    prints::print_welcome_message();
	memory::vmalloc::vmalloc_test();
	memory::kmalloc::kmalloc_test();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	handle_panic(info, None);
}
