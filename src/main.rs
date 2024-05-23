#![no_std]
#![no_main]


mod vga;
mod gdt;

fn print_gdt() {
    gdt::print_gdt();
}

#[no_mangle]
pub extern "C" fn main() {

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
