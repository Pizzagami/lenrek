use crate::debug::DEBUG;
use crate::exceptions::interrupts;
use crate::tools::vga::{WriteMode, WRITER};
use core::fmt;

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::macros::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => (print!("\n"));
	($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_srl {
	($($arg:tt)*) => {
		$crate::macros::print_srl(format_args!($($arg)*))
	};

}

macro_rules! println_srl {
	() => (print_srl!("\n"));
	($($arg:tt)*) => (print_srl!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        let level_str = $level.as_str();
        $crate::macros::print_srl(format_args!("{}", level_str));
        $crate::macros::print_srl(format_args!(": {}\n", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! handler {
	($name: ident) => {{
		#[naked]
		extern "C" fn wrapper() {
			unsafe {
				asm!(
					// Set up stack frame
					"push ebp",
					"mov ebp, esp",

					// Save all general-purpose registers
					"pushad",

					// Calculate the correct stack frame pointer
					"mov eax, esp",
					"add eax, 36",
					"push eax",

					// Call the actual interrupt handler
					"call {}",

					// Restore all general-purpose registers
					"pop eax",
					"popad",

					// Restore base pointer and return from interrupt
					"pop ebp",
					"iretd",
					sym $name,
					options(noreturn)
				);
			}
		}
		wrapper as extern "C" fn()
	}};
}

#[macro_export]
macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        #[no_mangle]
        extern "C" fn wrapper() {
            unsafe {
                asm!(
                    "push ebp",
                    "mov ebp, esp",
                    "pushad",
					"mov edx, [esp + 36]",
                    "lea eax, [esp + 40]",
					"push edx",
					"push eax",
                    "call {}",
					"pop eax",
					"pop edx",
					"popad",
                    "add esp, 4",
                    "pop ebp",
                    "iretd",
                    sym $name,
                    options(noreturn)
                );
            }
        }
        wrapper as extern "C" fn()
    }};
}

pub fn print(args: fmt::Arguments) {
	use core::fmt::Write;
	interrupts::disable();
	let mut writer = WRITER.lock();
	writer.set_mode(WriteMode::Normal);
	writer.write_fmt(args).unwrap();
	interrupts::enable();
}

pub fn print_srl(args: fmt::Arguments) {
	use core::fmt::Write;
	interrupts::disable();
	let mut debug = DEBUG.lock();
	let mut writer = WRITER.lock();

	debug.write_fmt(args).expect("Printing to srl failed");
	writer.set_mode(WriteMode::Srl);
	writer
		.write_fmt(args)
		.expect("Writing to srl screen failed");
	writer.set_mode(WriteMode::Normal);
	interrupts::enable();
}
