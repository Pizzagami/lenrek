use atoi::FromRadix10;

use crate::{vga::{set_color, Colors}, print, println};

pub fn print_header() {
    let ft_color = Colors::BrightYellow;
    let border_color = Colors::BrightCyan;
    let name_color = Colors::BrightGreen;
    let filename_color = Colors::BrightPurple;

    set_color(border_color);
    println!("/* ************************************************************************** */");
    println!("/*                                                                            */");
    print!("/*                                                        ");
    set_color(ft_color);
    print!(":::      ::::::::");
    set_color(border_color);
    println!("   */");
    print!("/*   ");
    set_color(filename_color);
    print!("main.rs                                            ");
    set_color(ft_color);
    print!(":+:      :+:    :+:");
    set_color(border_color);
    println!("   */");
    print!("/*                                         ,          ");
    set_color(ft_color);
    print!("+:+ +:+         +:+");
    set_color(border_color);
    println!("     */");
    print!("/*   By: ");
    set_color(name_color);
    print!("Pizz");
    set_color(border_color);
    print!(" and ");
    set_color(name_color);
    print!("Billy");
    set_color(border_color);
    print!("                           ");
    set_color(ft_color);
    print!("+#+  +:+       +#+");
    set_color(border_color);
    println!("        */");
    print!("/*                                                ");
    set_color(ft_color);
    print!("+#+#+#+#+#+   +#+");
    set_color(border_color);
    println!("           */");
    print!("/*   Created: 2023/11/14 15:09:19 by ");
    set_color(name_color);
    print!("Billy             ");
    set_color(ft_color);
    print!("#+#    #+#");
    set_color(border_color);
    println!("             */");
    print!("/*   Updated: 2023/11/14 01:24:22 by ");
    set_color(name_color);
    print!("Pizz           ");
    set_color(ft_color);
    print!("###   ########.fr");
    set_color(border_color);
    println!("       */");
    println!("/*                                                                            */");
    println!("/* ************************************************************************** */");
}

#[allow(dead_code)]
pub fn test_color() {
    set_color(Colors::Black);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Blue);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Green);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Cyan);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Red);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Purple);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Yellow);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::White);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Grey);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightBlue);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightGreen);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightCyan);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightRed);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightPurple);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightYellow);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightWhite);
    println!("Billy ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
}

/// Return the parsed integer and remaining slice if successful.
pub fn atoi_with_rest<I: FromRadix10>(text: &[u8]) -> Option<(&[u8], I)> {
    match I::from_radix_10(text) {
        (_, 0) => None,
        (n, used) => Some((&text[used..], n)),
    }
}

pub fn get_kernel_address<T>(address: usize) -> *const T
{
    return address as *const T;
}
