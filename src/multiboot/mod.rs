use crate::{memory::kmem_managment::PMM, tools::debug::LogLevel};

const MULTIBOOT_HEADER_MAGIC: u32 = 0xe85250d6;
const MULTIBOOT_HEADER_ARCHITECTURE: u32 = 0;
const MULTIBOOT_HEADER_CHECKSUM: u32 = (0_u32)
	.wrapping_sub(MULTIBOOT_HEADER_MAGIC)
	.wrapping_sub(MULTIBOOT_HEADER_ARCHITECTURE);
const MULTIBOOT_BOOTLOADER_MAGIC: u32 = 0x36d76289;

#[used]
#[link_section = ".multiboot_header"]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
	magic: MULTIBOOT_HEADER_MAGIC,
	architecture: MULTIBOOT_HEADER_ARCHITECTURE,
	header_length: core::mem::size_of::<MultibootHeader>() as u32,
	checksum: MULTIBOOT_HEADER_CHECKSUM
		.wrapping_sub(core::mem::size_of::<MultibootHeader>() as u32),
	end_tag_type: 0,
	end_tag_flags: 0,
	end_tag_size: 8,
};

#[repr(C)]
pub struct MultibootHeader {
	magic: u32,
	architecture: u32,
	header_length: u32,
	checksum: u32,
	end_tag_type: u16,
	end_tag_flags: u16,
	end_tag_size: u32,
}

#[repr(C)]
pub struct MultibootInfo {
	total_size: u32,
	reserved: u32,
	tags: [MultibootTag; 1],
}

#[repr(C)]
pub struct MultibootTag {
	tag_type: u32,
	size: u32,
}

#[repr(C)]
pub struct MultibootTagString {
	tag_type: u32,
	size: u32,
	string: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootTagBasicMemInfo {
	tag_type: u32,
	size: u32,
	pub mem_lower: u32,
	pub mem_upper: u32,
}

#[repr(C)]
pub struct MultibootTagBootDev {
	tag_type: u32,
	size: u32,
	biosdev: u32,
	partition: u32,
	sub_partition: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MltbtMMT {
	tag_type: u32,
	pub size: u32,
	pub entry_size: u32,
	pub entry_version: u32,
	pub entries: [MltbtMME; 1],
}

#[derive(Debug)]
#[repr(C)]
pub struct MltbtMME {
	pub address: u64,
	pub len: u64,
	pub entry_type: u32,
	zero: u32,
}

const MULTIBOOT_TAG_TYPE_END: u32 = 0;

const MULTIBOOT_TAG_TYPE_CMDLINE: u32 = 1;
const MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME: u32 = 2;
const MULTIBOOT_TAG_TYPE_BASIC_MEMINFO: u32 = 4;
const MULTIBOOT_TAG_TYPE_BOOTDEV: u32 = 5;
const MULTIBOOT_TAG_TYPE_MMAP: u32 = 6;

pub fn strlen(s: *const u8) -> usize {
	let mut len = 0;
	while unsafe { *s.add(len) } != 0 {
		len += 1;
	}
	len
}

pub fn u8_to_str(pointer: *const u8) -> &'static str {
	unsafe {
		let length = strlen(pointer);
		let slice = core::slice::from_raw_parts(pointer, length);
		core::str::from_utf8(slice).unwrap()
	}
}
pub fn validate_multiboot(magic: u32, address: u32) {
	if magic != MULTIBOOT_BOOTLOADER_MAGIC {
		panic!("Invalid multiboot magic number: {:#x}", magic);
	}

	if address & 0x7 != 0 {
		panic!("Unaligned multiboot address: {:#x}", address);
	}

	log!(LogLevel::Info, "Multiboot header successfully validated");
}

pub fn read_multiboot_info(address: u32) {
	let multiboot_info: &MultibootInfo = unsafe { &*(address as *const MultibootInfo) };
	println_srl!(
		"\nGRUB: Announced MBI size: {:#x}",
		multiboot_info.total_size
	);

	let mut current_tag: *const MultibootTag = multiboot_info.tags.as_ptr();
	let mut tag: &MultibootTag = unsafe { &*current_tag };

	let mut _meminfo: Option<&MultibootTagBasicMemInfo> = None;

	while tag.tag_type != MULTIBOOT_TAG_TYPE_END {
		match tag.tag_type {
			MULTIBOOT_TAG_TYPE_CMDLINE => {
				let cmdline = unsafe { &*(current_tag as *const MultibootTagString) };
				if cmdline.string != 0 {
					println_srl!("      Command line: {}", u8_to_str(&cmdline.string));
				}
			}
			MULTIBOOT_TAG_TYPE_BOOT_LOADER_NAME => {
				let bootloader_name = unsafe { &*(current_tag as *const MultibootTagString) };
				println_srl!(
					"      Bootloader name: {}",
					u8_to_str(&bootloader_name.string)
				);
			}
			MULTIBOOT_TAG_TYPE_BASIC_MEMINFO => {
				_meminfo = Some(unsafe { &*(current_tag as *const MultibootTagBasicMemInfo) });
				println_srl!(
					"      Mem lower: {}KB, Mem upper: {}KB",
					_meminfo.unwrap().mem_lower,
					_meminfo.unwrap().mem_upper
				);
			}
			MULTIBOOT_TAG_TYPE_BOOTDEV => {
				let bootdev = unsafe { &*(current_tag as *const MultibootTagBootDev) };
				println_srl!(
					"      Boot device: {:#x}, {}, {}",
					bootdev.biosdev,
					bootdev.partition,
					bootdev.sub_partition
				);
			}
			MULTIBOOT_TAG_TYPE_MMAP => {
				let nmap_tag = unsafe { &*(current_tag as *const MltbtMMT) };

				let entries_count = (nmap_tag.size - nmap_tag.entry_size) / nmap_tag.entry_size;
				let memory_map_entries = unsafe {
					core::slice::from_raw_parts(nmap_tag.entries.as_ptr(), entries_count as usize)
				};
				PMM.lock().memory_map_tag = Some(nmap_tag);
				PMM.lock().memory_map_entries = Some(memory_map_entries);
			}
			_ => {}
		}
		current_tag = (current_tag as usize + (tag.size as usize + 7) & !7) as *const MultibootTag;
		tag = unsafe { &*current_tag };
	}
}