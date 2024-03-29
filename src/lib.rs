#![no_std]

mod vga;

#[no_mangle]
pub extern "C" fn rust_main() {
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
********************************************************************************");
//    print_gdt();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
