use crate::exceptions::interrupts::{self, TICKS};
use crate::shell::history::HISTORY;
use crate::shell::prints::PrintSM;
use crate::shell::prints::{help, print_stack, print_unknown_command};
use crate::tools::debug::LogLevel;
use crate::tools::io::{outb, outw};
use crate::tools::librs::hlt;
use crate::tools::librs::{get_rtc_date, get_rtc_time};
use crate::tools::vga::WRITER;
use core::sync::atomic::Ordering;

pub const MAX_LINE_LENGTH: usize = 76;
pub const MAX_HISTORY_LINES: usize = 16;


pub fn clear() {
	interrupts::disable();
	WRITER.lock().clear_screen();
	interrupts::enable();
}


fn echo(line: &str) {
	let message: &str = &line["echo".len()..];
	if message.starts_with(" ") && message.len() > 1 {
		println!("{}", message[1..].trim());
	} else {
		println!("echo: missing argument");
	}
}

fn time() {
	let (hours, minutes, seconds) = get_rtc_time();
	println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}


fn date() {
	let (hours, minutes, seconds) = get_rtc_time();
	let (year, month, day) = get_rtc_date();

	let full_year = 2000 + year as u16;

	println!(
		"{:02}/{:02}/{:02} {:02}:{:02}:{:02}",
		day, month, full_year, hours, minutes, seconds
	);
}


fn reboot() {
	unsafe {
		outb(0x64, 0xfe);
	}
}

fn shutdown() {
	unsafe {
		outw(0x604, 0x2000);
	}
}

fn uname() {
	println!(
		"{} {} {} {} {} {}",
		"kfs 3",
		"0.0.1-kfs-i386",
		"Lenrek",
		"i386",
		"Kernel",
		"B&P"
	);
}

fn cmd_mode() {
	let cr0: usize;
	unsafe {
		core::arch::asm!("mov {}, cr0", out(reg) cr0, options(nostack, preserves_flags));
	}

	let mode = if cr0 & 1 == 0 { "real" } else { "protected" };
	log!(
		LogLevel::Info,
		"System is running in {} mode. CR0: {:b}",
		mode,
		cr0
	);
	println!("System is running in {} mode.", mode);
	describe_cr0(cr0);
}

fn describe_cr0(cr0: usize) {
	log!(LogLevel::Info, "CR0 Register: 0b{:032b}", cr0);
	println!("CR0 Register: 0b{:032b}", cr0);

	let flags = [
		("PE (Protection Enable)", cr0 & (1 << 0) != 0),
		("MP (Monitor Co-processor)", cr0 & (1 << 1) != 0),
		("EM (Emulation)", cr0 & (1 << 2) != 0),
		("TS (Task Switched)", cr0 & (1 << 3) != 0),
		("ET (Extension Type)", cr0 & (1 << 4) != 0),
		("NE (Numeric Error)", cr0 & (1 << 5) != 0),
		("WP (Write Protect)", cr0 & (1 << 16) != 0),
		("AM (Alignment Mask)", cr0 & (1 << 18) != 0),
		("NW (Not Write-through)", cr0 & (1 << 29) != 0),
		("CD (Cache Disable)", cr0 & (1 << 30) != 0),
		("PG (Paging)", cr0 & (1 << 31) != 0),
	];

	for (name, active) in flags.iter() {
		if *active {
			log!(LogLevel::Info, "{}: Activated", name);
			println!("{}: Activated", name);
		}
	}
}

fn cpu_info() {
	let mut cpu_vendor = [0u8; 12];
	let mut cpu_brand = [0u8; 48];
	let mut eax: usize = 0;
	let mut ebx: usize = 0;
	let mut ecx: usize = 0;
	let mut edx: usize = 0;

	get_cpuid(0, &mut eax, &mut ebx, &mut ecx, &mut edx);
	cpu_vendor[0..4].copy_from_slice(&ebx.to_ne_bytes());
	cpu_vendor[4..8].copy_from_slice(&edx.to_ne_bytes());
	cpu_vendor[8..12].copy_from_slice(&ecx.to_ne_bytes());

	for i in 0x80000002..=0x80000004 {
		get_cpuid(i, &mut eax, &mut ebx, &mut ecx, &mut edx);
		let off = (i - 0x80000002) * 16;
		cpu_brand[off..off + 4].copy_from_slice(&eax.to_ne_bytes());
		cpu_brand[off + 4..off + 8].copy_from_slice(&ebx.to_ne_bytes());
		cpu_brand[off + 8..off + 12].copy_from_slice(&ecx.to_ne_bytes());
		cpu_brand[off + 12..off + 16].copy_from_slice(&edx.to_ne_bytes());
	}

	let cpu_vendor_str = core::str::from_utf8(&cpu_vendor).unwrap_or("Unknown");
	let cpu_brand_str = core::str::from_utf8(&cpu_brand).unwrap_or("Unknown");

	println!("CPU Vendor: {}", cpu_vendor_str);
	println!("CPU Brand: {}", cpu_brand_str);
}

fn get_cpuid(info_type: usize, eax: &mut usize, ebx: &mut usize, ecx: &mut usize, edx: &mut usize) {
	unsafe {
		core::arch::asm!(
			"cpuid",
			in("eax") info_type,
			lateout("eax") *eax,
			lateout("ebx") *ebx,
			lateout("ecx") *ecx,
			lateout("edx") *edx,
			options(nostack, nomem, preserves_flags)
		);
	}
}

fn show_uptime() {
	let uptime_seconds = TICKS.load(Ordering::SeqCst) / 18;

	let hours = (uptime_seconds % 86400) / 3600;
	let minutes = (uptime_seconds % 3600) / 60;
	let seconds = uptime_seconds % 60;

	println!(
		"System uptime: {:02}h:{:02}m:{:02}s",
		hours, minutes, seconds
	);
}

pub fn trigger_syscall(syscall_number: u32, arg1: u32, arg2: u32, arg3: u32) {
	use crate::exceptions::syscalls::GeneralRegs;
	let mut regs = GeneralRegs {
		eax: syscall_number,
		ebx: arg1,
		ecx: arg2,
		edx: arg3,
		esi: 0,
		edi: 0,
		ebp: 0,
	};

	crate::exceptions::syscalls::syscall(&mut regs);
}

fn test_syscall(line: &str) {
	let mut parts = [""; 5]; // syscall name, syscall number, arg1, arg2, arg3
	let mut part_index = 0;

	for word in line.split_whitespace() {
		parts[part_index] = word;
		part_index += 1;
		if part_index >= parts.len() {
			break;
		}
	}

	if part_index < 3 {
		println!("Usage: test_syscall <syscall_number> <arg1> <arg2> <arg3>");
		return;
	}

	let syscall_number = parts[1].parse::<u32>().unwrap_or(0);
	let arg1 = parts[2].parse::<u32>().unwrap_or(0);
	let arg2 = parts[3].parse::<u32>().unwrap_or(0);
	let arg3 = parts[4].parse::<u32>().unwrap_or(0);

	trigger_syscall(syscall_number, arg1, arg2, arg3);
}

pub fn readline(raw_line: &str) {
	let line = raw_line.trim();
	if line.is_empty() {
		return;
	}
	HISTORY.lock().add(raw_line);

	match line {
		"help" | "man" => help(),
		"clear" => clear(),
		"time" => time(),
		"reboot" => reboot(),
		"halt" => hlt(),
		"shutdown" => shutdown(),
		"history" => HISTORY.lock().print(),
		"date" => date(),
		"uname" => uname(),
		"uptime" => show_uptime(),
		"cpu" => cpu_info(),
		"mode" => cmd_mode(),
		_ => handle_special_commands(line),
	}
}

fn handle_special_commands(line: &str) {
	if line.starts_with("echo") {
		echo(line);
	} else if line.starts_with("stack") {
		print_stack(line, PrintSM::Vga);
	} else if line.starts_with("hexdump") {
		print_stack(line, PrintSM::Srl);
	} else if line.starts_with("test_syscall") {
		test_syscall(line);
	} else {
		print_unknown_command(line);
	}
}
