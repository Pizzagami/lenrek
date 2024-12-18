use crate::memory::{
	page_directory::{ENTRY_COUNT, PAGE_SIZE},
	page_table_entry::{PageTableEntry, FlagTablePages},
};

#[derive(Clone, Copy, Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
	pub entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
	pub fn new(&mut self, flags: FlagTablePages) {
		self.entries = [PageTableEntry::new(); ENTRY_COUNT];
		for entry in self.entries.iter_mut() {
			entry.set_flags(flags);
		}
	}

	pub fn get_page_table_entry(&mut self, virtual_address: u32) -> &mut PageTableEntry {
		let index = (virtual_address & 0x003FF000) >> 12;
		&mut self.entries[index as usize]
	}

	pub fn kernel_mapping(&mut self, mut physical_address: u32, flags: FlagTablePages) {
		for page_table_entry in self.entries.iter_mut() {
			page_table_entry.set_frame_address(physical_address, flags);
			physical_address += PAGE_SIZE as u32;
		}
	}
}
