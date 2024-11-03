use crate::debug::DEBUG;
use crate::exceptions::interrupts;
use crate::tools::video_graphics_array::{WriteMode, WRITER};
use core::fmt;

#[macro_export]
macro_rules! printt {
	($($arg:tt)*) => ($crate::macros::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printtln {
	() => (print!("\n"));
	($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_top {
	($($arg:tt)*) => ($crate::macros::print_top(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_serial {
	($($arg:tt)*) => {
		$crate::macros::print_serial(format_args!($($arg)*))
	};

}

macro_rules! println_serial {
	() => (print_serial!("\n"));
	($($arg:tt)*) => (print_serial!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        let level_str = $level.as_str();
        $crate::macros::print_serial(format_args!("{}", level_str));
        $crate::macros::print_serial(format_args!(": {}\n", format_args!($($arg)*)));
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
					"add eax, 36", // Adjust for 'pushad' and possibly other pushed registers
					"push eax", // Push stack frame pointer

					// Call the actual interrupt handler
					"call {}",

					// Restore all general-purpose registers
					"pop eax", // Clean up the stack
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
                    // Set up stack frame
                    "push ebp",
                    "mov ebp, esp",
                    
                    // Save all general-purpose registers
                    "pushad",

					// Retrieve error code
					"mov edx, [esp + 36]",

					// Calculate the correct stack frame pointer
                    "lea eax, [esp + 40]", // Adjust for 'pushad' and error code
					"push edx", // Push error code
					"push eax", // Push stack frame pointer

                    // Call the actual interrupt handler
                    "call {}",

					"pop eax", // Clean up the stack
					"pop edx", // Clean the error code

					// Restore all general-purpose registers
					"popad",

                    "add esp, 4", // Remove error code from stack
					
                    // Restore base pointer and return from interrupt
                    "pop ebp",
                    "iretd", // Return from interrupt in 32-bit mode
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


pub fn print_top(args: fmt::Arguments) {
	use core::fmt::Write;
	interrupts::disable();
	let mut writer = WRITER.lock();
	writer.set_mode(WriteMode::Top);
	writer.write_fmt(args).unwrap();
	interrupts::enable();
}

pub fn print_serial(args: fmt::Arguments) {
	use core::fmt::Write;
	interrupts::disable();
	let mut debug = DEBUG.lock();
	let mut writer = WRITER.lock();

	debug.write_fmt(args).expect("Printing to serial failed");
	writer.set_mode(WriteMode::Serial);
	writer
		.write_fmt(args)
		.expect("Writing to serial screen failed");
	writer.set_mode(WriteMode::Normal);
	interrupts::enable();
}
