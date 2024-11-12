use crate::tools::io::{inb, outb};

const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTP: u8 = 0x20;
const MODE_8086: u8 = 0x01;

const PIC1_COMMAND: u8 = 0x20;
const PIC1_DATA: u8 = 0x21;
const PIC2_COMMAND: u8 = 0xa0;
const PIC2_DATA: u8 = 0xa1;

const WAIT_PORT: u8 = 0x80;

struct Pic {
	off: u8,
	command: u8,
	data: u8,
}

impl Pic {
	fn handles_intp(&self, interrupt_id: u8) -> bool {
		self.off <= interrupt_id && interrupt_id < self.off + 8
	}

	unsafe fn end_of_intp(&mut self) {
		outb(self.command as u16, CMD_END_OF_INTP);
	}

	unsafe fn read_mask(&mut self) -> u8 {
		inb(self.data as u16)
	}

	unsafe fn write_mask(&mut self, mask: u8) {
		outb(self.data as u16, mask);
	}
}

pub struct ChainedPics {
	pics: [Pic; 2],
}

impl ChainedPics {
	pub const unsafe fn new(off1: u8, off2: u8) -> ChainedPics {
		ChainedPics {
			pics: [
				Pic {
					off: off1,
					command: PIC1_COMMAND,
					data: PIC1_DATA,
				},
				Pic {
					off: off2,
					command: PIC2_COMMAND,
					data: PIC2_DATA,
				},
			],
		}
	}

	pub const unsafe fn new_contiguous(primary_off: u8) -> ChainedPics {
		Self::new(primary_off, primary_off + 8)
	}

	pub unsafe fn initialize(&mut self) {
		let wait = || outb(WAIT_PORT as u16, 0);

		let saved_masks = self.read_masks();

		outb(self.pics[0].command as u16, CMD_INIT);
		wait();
		outb(self.pics[1].command as u16, CMD_INIT);
		wait();

		outb(self.pics[0].data as u16, self.pics[0].off);
		wait();
		outb(self.pics[1].data as u16, self.pics[1].off);
		wait();

		outb(self.pics[0].data as u16, 0x04);
		wait();
		outb(self.pics[1].data as u16, 0x02);
		wait();

		outb(self.pics[0].data as u16, MODE_8086);
		wait();
		outb(self.pics[1].data as u16, MODE_8086);
		wait();

		self.write_masks(saved_masks[0], saved_masks[1])
	}

	pub unsafe fn read_masks(&mut self) -> [u8; 2] {
		[self.pics[0].read_mask(), self.pics[1].read_mask()]
	}

	pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) {
		self.pics[0].write_mask(mask1);
		self.pics[1].write_mask(mask2);
	}

	pub fn handles_intp(&self, interrupt_id: u8) -> bool {
		self.pics.iter().any(|p| p.handles_intp(interrupt_id))
	}

	pub unsafe fn notify_end_of_intp(&mut self, interrupt_id: u8) {
		if self.handles_intp(interrupt_id) {
			if self.pics[1].handles_intp(interrupt_id) {
				self.pics[1].end_of_intp();
			}
			self.pics[0].end_of_intp();
		}
	}
}
