use crate::shell::prints::PrintStackMode;
use crate::shell::{builtins::MAX_LINE_LENGTH, history::Line};
use crate::tools::io::{inb, outb};
use core::arch::asm;
use crate::print_serial;

const CMOS_ADDRESS: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

pub fn array_cmp(a: &Line, b: &Line) -> bool {
	a.iter().zip(b.iter()).all(|(&x, &y)| x == y)
}

pub fn array_to_str(arr: &Line) -> &str {
	let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
	core::str::from_utf8(&arr[..len]).unwrap_or_default()
}

pub fn str_to_array(s: &str) -> Line {
	let mut line = [0; MAX_LINE_LENGTH];
	for (i, c) in s.bytes().enumerate() {
		line[i] = c;
	}
	line
}

pub fn bcd_to_binary(bcd: u8) -> u8 {
	((bcd & 0xf0) >> 4) * 10 + (bcd & 0x0f)
}

pub fn read_cmos(register: u8) -> u8 {
	unsafe {
		outb(CMOS_ADDRESS, register);
		inb(CMOS_DATA)
	}
}

pub fn get_rtc_time() -> (u8, u8, u8) {
	let seconds = bcd_to_binary(read_cmos(0x00));
	let minutes = bcd_to_binary(read_cmos(0x02));
	let hours = bcd_to_binary(read_cmos(0x04));

	(hours, minutes, seconds)
}

pub fn get_rtc_date() -> (u8, u8, u8) {
	let year = bcd_to_binary(read_cmos(0x09));
	let month = bcd_to_binary(read_cmos(0x08));
	let day = bcd_to_binary(read_cmos(0x07));

	(year, month, day)
}

#[inline]
pub fn hlt() {
	unsafe {
		asm!("hlt", options(nomem, nostack, preserves_flags));
	}
}

pub fn hexdump(mut address: usize, limit: usize, mode: PrintStackMode) {
	if limit == 0 {
		return;
	}

	let bytes = unsafe { core::slice::from_raw_parts(address as *const u8, limit) };

	for (i, &byte) in bytes.iter().enumerate() {
		if i % 16 == 0 {
			if i != 0 {
				print_hex_line(address - 16, 16, mode);
				match mode {
					PrintStackMode::Vga => println!(""),
					PrintStackMode::Serial => println_serial!(""),
				}
			}
			match mode {
				PrintStackMode::Vga => print!("{:08x} ", address),
				PrintStackMode::Serial => print_serial!("{:08x} ", address),
			}
		}

		if i % 8 == 0 {
			match mode {
				PrintStackMode::Vga => print!(" "),
				PrintStackMode::Serial => print_serial!(" "),
			}
		}

		match mode {
			PrintStackMode::Vga => print!("{:02x} ", byte),
			PrintStackMode::Serial => print_serial!("{:02x} ", byte),
		}
		address += 1;
	}

	let remaining = limit % 16;
	if remaining > 0 {
		let padding = 16 - remaining;
		for _ in 0..padding {
			match mode {
				PrintStackMode::Vga => print!("   "),
				PrintStackMode::Serial => print_serial!("   "),
			}
		}
		if padding > 7 {
			match mode {
				PrintStackMode::Vga => print!(" "),
				PrintStackMode::Serial => print_serial!(" "),
			}
		}
		print_hex_line(address - remaining, remaining, mode);
	} else {
		print_hex_line(address - 16, 16, mode);
	}

	match mode {
		PrintStackMode::Vga => println!(""),
		PrintStackMode::Serial => println_serial!(""),
	}
}

fn print_hex_line(address: usize, count: usize, mode: PrintStackMode) {
	let bytes = unsafe { core::slice::from_raw_parts(address as *const u8, count) };

	match mode {
		PrintStackMode::Vga => print!(" "),
		PrintStackMode::Serial => print_serial!(" "),
	}

	for i in 0..count {
		let ch = if bytes[i] >= 0x20 && bytes[i] <= 0x7e {
			bytes[i] as char
		} else {
			'.'
		};
		match mode {
			PrintStackMode::Vga => print!("{}", ch),
			PrintStackMode::Serial => print_serial!("{}", ch),
		}
	}
}
