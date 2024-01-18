#![feature(lang_items)]
#![no_std]

mod vga;

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut cell = vga::Cell::default();
    cell.reset_screen();
    cell.print_string("42!");
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
