use crate::shell::accessflags::{
	KERNEL_CODE_SEGMENT, KERNEL_DATA_SEGMENT, KERNEL_STACK_SEGMENT, MAX_SEGMENT_SIZE, NO_OFFSET,
	NULL_SEGMENT, SEGMENT_FLAGS, USER_CODE_SEGMENT, USER_DATA_SEGMENT, USER_STACK_SEGMENT,
};
use crate::tools::debug::LogLevel;
use core::arch::asm;

#[repr(C, packed)]
pub struct GdtEntry {
	limit_low: u16,
	base_low: u16,
	base_middle: u8,
	access: u8,
	flags: u8,
	base_high: u8,
}

macro_rules! create_gdt_entry {
	($limit:expr, $base:expr, $access:expr, $flags:expr, $name:expr) => {
		GdtEntry {
			limit_low: ($limit & 0xffff) as u16,
			base_low: ($base & 0xffff) as u16,
			base_middle: (($base >> 16) & 0xff) as u8,
			access: $access,
			flags: ($flags & 0xf0) | ((($limit >> 16) & 0x0f) as u8),
			base_high: (($base >> 24) & 0xff) as u8,
		}
	};
}

#[link_section = ".gdt"]
static LOW_GDT: [GdtEntry; 7] = [
	create_gdt_entry!(0, 0, NULL_SEGMENT, 0, "NULL segment"),
	create_gdt_entry!(
		MAX_SEGMENT_SIZE,
		NO_OFFSET,
		KERNEL_CODE_SEGMENT,
		SEGMENT_FLAGS,
		"Kernel code segment"
	),
	create_gdt_entry!(
		MAX_SEGMENT_SIZE,
		NO_OFFSET,
		KERNEL_DATA_SEGMENT,
		SEGMENT_FLAGS,
		"Kernel data segment"
	),
	create_gdt_entry!(
		MAX_SEGMENT_SIZE,
		NO_OFFSET,
		KERNEL_STACK_SEGMENT,
		SEGMENT_FLAGS,
		"Kernel stack segment"
	),
	create_gdt_entry!(
		MAX_SEGMENT_SIZE,
		NO_OFFSET,
		USER_CODE_SEGMENT,
		SEGMENT_FLAGS,
		"User code segment"
	),
	create_gdt_entry!(
		MAX_SEGMENT_SIZE,
		NO_OFFSET,
		USER_DATA_SEGMENT,
		SEGMENT_FLAGS,
		"User data segment"
	),
	create_gdt_entry!(
		MAX_SEGMENT_SIZE,
		NO_OFFSET,
		USER_STACK_SEGMENT,
		SEGMENT_FLAGS,
		"User stack segment"
	),
];

pub static mut GDT: *mut [GdtEntry; 7] = core::ptr::null_mut();
#[repr(C, packed)]
pub struct GdtRegister {
	size: u16, 
	offset: u32,
}

fn load_gdt() {
	unsafe {
		let gdt_register = GdtRegister {
			size: (core::mem::size_of::<[GdtEntry; 7]>() - 1) as u16,
			offset: GDT as u32,
		};

		asm!("lgdt [{}]", in(reg) &gdt_register, options(readonly, nostack, preserves_flags));
	}
}


fn load_data_segments() {
	unsafe {
		asm!(
			"mov ax, 0x10", // Kernel data segments
			"mov ds, ax",
			"mov es, ax",
			"mov fs, ax",
			"mov gs, ax",
			options(nostack, preserves_flags)
		);
	}
}


fn load_stack_segment() {
	unsafe {
		asm!(
			"mov ax, 0x18", // Kernel stack segment
			"mov ss, ax",
			options(nostack, preserves_flags)
		);
	}
}

fn load_code_segment() {
	unsafe {
		asm!(
			"push 0x08", // Kernel code segment
			"lea eax, [1f]",
			"push eax",
			"retf",
			"1:",
			options(nostack, preserves_flags)
		);
	}
}

pub fn init() {
	unsafe {
		GDT = (&LOW_GDT as *const _ as usize + 0xC0000000) as *mut [GdtEntry; 7];
	}
	load_gdt();
	log!(
		LogLevel::Info,
		"GDT successfully loaded at 0x{:08x}",
		unsafe { GDT as *const _ as usize }
	);
	load_data_segments();
	load_stack_segment();
	load_code_segment();
	log!(
		LogLevel::Info,
		"Kernel data, stack and code segments successfully loaded"
	);
}