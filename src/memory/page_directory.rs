use crate::memory::{
	page_directory_entry::{PageDirectoryEntry, PageDirectoryFlags},
	page_table::PageTable,
	page_table_entry::FlagTablePages,
	kmem_managment::HK_OFST,
};
use crate::print_srl;
use core::arch::asm;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering};

use super::{page_table_entry::PageTableEntry, kmem_managment::PMM};

pub const PAGE_SIZE: usize = 4096;
pub const ENTRY_COUNT: usize = 1024;
pub const PAGE_TABLE_SIZE: usize = ENTRY_COUNT * PAGE_SIZE;

pub static mut PAGE_DIRECTORY_ADDR: u32 = 0;
pub static mut PAGE_TABLES_ADDR: u32 = 0;

pub static mut PAGE_DIRECTORY: AtomicPtr<PageDirectory> = AtomicPtr::new(null_mut());

#[repr(C, align(4096))]
pub struct PageDirectory {
	pub entries: [PageDirectoryEntry; ENTRY_COUNT],
}

impl PageDirectory {
	pub fn get_page_table(&mut self, virtual_address: u32) -> &mut PageTable {
		let index = (virtual_address >> 22) as usize;
		let addr = self.entries[index].get_page_table();
		addr
	}
}

pub fn map_address(virtual_address: *mut u8) {
	let page_directory: &mut PageDirectory =
		unsafe { &mut *PAGE_DIRECTORY.load(Ordering::Relaxed) };
	let page_table: &mut PageTable = page_directory.get_page_table(virtual_address as u32);
	let page_table_entry: &mut PageTableEntry =
		page_table.get_page_table_entry(virtual_address as u32);

	page_table_entry.alloc_new();
}

pub fn unmap_address(virtual_address: *mut u8) {
	let page_directory: &mut PageDirectory =
		unsafe { &mut *PAGE_DIRECTORY.load(Ordering::Relaxed) };
	let page_table: &mut PageTable = page_directory.get_page_table(virtual_address as u32);
	let page_table_entry: &PageTableEntry =
		page_table.get_page_table_entry(virtual_address as u32);

	PMM.lock().deallocate_frame(page_table_entry.frame());
}

pub fn enable_paging() {
	println_srl!("Enabling paging...");
	let page_directory_addr = unsafe { PAGE_DIRECTORY_ADDR - HK_OFST };
	unsafe {
		asm!("mov cr3, {}", in(reg) page_directory_addr);
		asm!("mov cr3, {}", in(reg) page_directory_addr);
	}

	print_srl!("Mapping page tables...");
	unsafe {
		let mut cr0: u32;
		asm!("mov {}, cr0", out(reg) cr0);
		cr0 |= 0x80000000; // Set the PG bit to enable paging
		asm!("mov cr0, {}", in(reg) cr0);
	}
	println_srl!("Paging enabled!");
}

pub unsafe fn init_page_directory() {
	PAGE_DIRECTORY = AtomicPtr::new(PAGE_DIRECTORY_ADDR as *mut PageDirectory);
	let page_directory = &mut *PAGE_DIRECTORY.load(Ordering::Relaxed);
	println_srl!("Page Directory __ : {:p}", page_directory);

	let mut current_page_table = PAGE_TABLES_ADDR;
	for page_directory_entry in page_directory.entries.iter_mut().enumerate() {
		if page_directory_entry.0 < 768 {
			page_directory_entry.1.set(
				current_page_table,
				PageDirectoryFlags::PRESENT
					| PageDirectoryFlags::WRITABLE
					| PageDirectoryFlags::USER,
			);
			page_directory_entry
				.1
				.get_page_table()
				.new(FlagTablePages::USER | FlagTablePages::WRITABLE);
		} else {
			page_directory_entry.1.set(
				current_page_table,
				PageDirectoryFlags::PRESENT | PageDirectoryFlags::WRITABLE,
			);
			page_directory_entry
				.1
				.get_page_table()
				.new(FlagTablePages::WRITABLE);
		}
		current_page_table += PAGE_SIZE as u32;
	}

	page_directory
		.get_page_table(HK_OFST)
		.kernel_mapping(
			0x00000000,
			FlagTablePages::PRESENT | FlagTablePages::WRITABLE,
		);
	page_directory
		.get_page_table(HK_OFST + PAGE_TABLE_SIZE as u32)
		.kernel_mapping(
			0x00000000 + PAGE_TABLE_SIZE as u32,
			FlagTablePages::PRESENT | FlagTablePages::WRITABLE,
		);
}
