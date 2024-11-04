use crate::exceptions::interrupts;
use crate::shell::builtins::clear;
use crate::gdt::GDT;
use crate::idt::IDT;
use crate::tools::librs::hexdump;
use crate::tools::prompt;
use crate::tools::vga::WRITER;

pub fn print_unknown_command(line: &str) {
	let len = line.len().min(50);
	println!("Unknown command: {}", line[0..len].trim());
}

pub fn print_welcome_message() {
	clear();
	println!("                                     :---------:    .---------:---------- ");
	println!("                                   :#@@@@@@@@%=     +@@@@@@@#::@@@@@@@@@@.");
	println!("                                 :#@@@@@@@@%=       +@@@@@%:  :@@@@@@@@@@.");
	println!("                               :#@@@@@@@@%=         +@@@%-    :@@@@@@@@@@.");
	println!("                             :#@@@@@@@@@=           +@%-      :@@@@@@@@@@.");
	println!("                           :#@@@@@@@@@=             =-        -@@@@@@@@@@ ");
	println!("                         :#@@@@@@@@@=                        +@@@@@@@@@*. ");
	println!("                       :#@@@@@@@@@=                        +@@@@@@@@@*.   ");
	println!("                     :#@@@@@@@@@=                        +@@@@@@@@@*.     ");
	println!("                   :#@@@@@@@@@=                        +@@@@@@@@@*.       ");
	println!("                 :#@@@@@@@@@=                        +@@@@@@@@@+.         ");
	println!("                 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    +@@@@@@@@@#        :#.");
	println!("                 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    +@@@@@@@@@#      :#@@.");
	println!("                 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    +@@@@@@@@@#    :#@@@@.");
	println!("                 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    +@@@@@@@@@#  :#@@@@@@.");
	println!("                 ....................=@@@@@@@@@@    +@@@@@@@@@#:#@@@@@@@@.");
	println!("                                     -@@@@@@@@@@     .................... ");
	println!("                                     -@@@@@@@@@@     by                   ");
	println!("                                     -@@@@@@@@@@       selgrabl           ");
	println!("                                     -@@@@@@@@@@       braimbau           ");
	println!("                                     .----------                          ");
	println!("");
	println!("                       Welcome to KFS! Type 'help' for a list of commands!");
	prompt::init();
}

#[derive(Copy, Clone)]
pub enum PrintStackMode {
	Vga,
	Serial,
}

pub fn print_stack(line: &str, mode: PrintStackMode) {
	let trimmed_line = match mode {
		PrintStackMode::Vga => line["stack".len()..].trim(),
		PrintStackMode::Serial => line["hexdump".len()..].trim(),
	};

	let args = &trimmed_line;

	let mut parts = args.split_whitespace();

	let address = match parts.next() {
		Some("esp") => {
			let esp: usize;
			unsafe {
				core::arch::asm!("mov {}, esp", out(reg) esp);
			}
			esp
		}
		Some("gdt") => unsafe {
			GDT as usize
		}
		Some("idt") => {
			let offset: usize;
			unsafe {
				offset = IDT as usize;
			}
			offset
		}
		Some("cr3") => {
			let cr3: usize;
			unsafe {
				core::arch::asm!("mov {}, cr3", out(reg) cr3);
			}
			println_serial!("cr3: {:x}", cr3);
						let esp: usize;
			unsafe {
				core::arch::asm!("mov {}, esp", out(reg) esp);
			}
			esp
		}
		Some(addr_str) => usize::from_str_radix(addr_str, 16).unwrap_or(0),
		None => 0,
	};

	let num_bytes = parts
		.next()
		.and_then(|arg| arg.parse::<usize>().ok())
		.unwrap_or(256);

	hexdump(address, num_bytes, mode);
}

pub fn print_help_line(command: &str, description: &str) {
	print!("  {:21}", command);
	printraw("Z");
	print!("  {:52}", description);
	if command == "shutdown | reboot" {
		printraw("Z");
	} else if command != "F11 | F12" {
		printraw("ZZ");
	}
}


pub fn help() {
	clear();
	printraw("immmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm[Z");
	print!(" Available commands                                                           ");
	printraw("ZlmmmmmmmmmmmmmmmmmmmmmmmkmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmYZ");
	print_help_line("echo", "display a line of text");
	print_help_line("clear", "clear the screen");
	print_help_line("stack ", "print the stack");
	print_help_line("hexdump", "print to the serial COM a hexdump of memory");
	print_help_line(
		"date | time | uptime",
		"display the current date | time | uptime",
	);
	print_help_line("cpu", "display the CPU information");
	print_help_line("mode", "display the current system mode");
	print_help_line("uname", "print system information");
	print_help_line("halt", "halt the system");
	print_help_line("shutdown | reboot", "shutdown | reboot the system");
	printraw("lmmmmmmmmmmmmmmmmmmmmmmmnmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmYZ");
	print_help_line("F1 -> F5", "change between screens");
	print_help_line("F9", "display welcome message");
	print_help_line("F10", "change keyboard layout");
	print_help_line("F11 | F12", "switch text | background color");

	printraw("ZlmmmmmmmmmmmmmmmmmmmmmmmjmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmYZ");
	print!(
		" Type 'history' to view command history           {} {} navigate history        ",
		0x1e as char, 0x1f as char
	);
	printraw("Zhmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\\");
	println!("");
}

pub fn printraw(string: &str) {
	interrupts::disable();
	WRITER.lock().write_string_raw(string);
	interrupts::enable();
}
