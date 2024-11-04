use crate::memory::kmem_managment::PMM;
use bitflags::bitflags;
use crate::print_serial;

bitflags! {
	pub struct FlagTablePages: u32 {
		const PRESENT = 0b1;
		const WRITABLE = 0b10;
		const USER = 0b100;
		const PWT = 0b1000;
		const PCD = 0b1_0000;
		const ACCESSED = 0b10_0000;
		const DIRTY = 0b100_0000;
		const PAT = 0b1000_0000;
		const CPU_GLOBAL = 0b1_0000_0000;
		const AVAILABLE = 0b1110_0000_0000;
		const FRAME = 0xFFFFF000;
	}
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct PageTableEntry {
	pub value: u32,
}

impl PageTableEntry {
	pub fn new() -> Self {
		PageTableEntry { value: 0 }
	}

	pub fn set_frame_address(&mut self, frame_address: u32, flags: FlagTablePages) {
		self.value = frame_address | flags.bits();
	}

	pub fn set_flags(&mut self, flags: FlagTablePages) {
		self.value = (self.value & FlagTablePages::FRAME.bits()) | flags.bits();
	}

	pub fn alloc_new(&mut self) {
		print_serial!("Allocating new frame for page table entry...");
		let frame = PMM
			.lock()
			.allocate_frame()
			.map_err(|_| "Failed to allocate frame for page table entry");

		print_serial!("Frame allocated at {:?}\n", frame.unwrap());
		self.set_frame_address(
			frame.unwrap(),
			FlagTablePages::PRESENT | FlagTablePages::WRITABLE,
		);
		print_serial!("Frame allocated at {:?}\n", frame.unwrap());
	}

	pub fn frame(&self) -> u32 {
		self.value & FlagTablePages::FRAME.bits()
	}

	pub fn value(&self) -> u32 {
		self.value
	}
}
