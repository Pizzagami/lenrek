use crate::shell::builtins::readline;
use crate::tools::vga::{VGA_COLUMNS, WRITER};
use lazy_static::lazy_static;
use spin::Mutex;

pub static PROMPT_STRING: &str = "$> ";
pub static PROMPT_LENGTH: usize = PROMPT_STRING.len();

lazy_static! {
	pub static ref PROMPT: Mutex<Prompt> = Mutex::new(Prompt {
		buffer: [0; VGA_COLUMNS],
		length: 0,
	});
}

pub struct Prompt {
	buffer: [u8; VGA_COLUMNS],
	pub length: usize,
}

impl Prompt {
	pub fn insert_string(&mut self, s: &str) {
		for c in s.bytes() {
			self.insert_char(c, false);
		}
	}

	pub fn insert_char(&mut self, c: u8, insert: bool) {
		if c == b'\n' {
			println!();
			if self.length < PROMPT_LENGTH {
				self.init();
				return;
			}
			readline(core::str::from_utf8(&self.buffer[PROMPT_LENGTH..self.length]).unwrap());
			self.init();
			return;
		}

		if self.length == VGA_COLUMNS - 1 {
			return;
		}

		let column_position = WRITER.lock().column_position;
		if !insert {
			for i in (column_position..self.length).rev() {
				self.buffer[i + 1] = self.buffer[i];
			}
		}

		if !insert || column_position == self.length {
			self.length += 1;
		}

		self.buffer[column_position] = c;
		self.update_line();
		WRITER.lock().move_cursor(1);
	}

	pub fn remove_char(&mut self) {
		let column_position = WRITER.lock().column_position - 1;
		for i in column_position..self.length {
			self.buffer[i] = self.buffer[i + 1];
		}

		self.length -= 1;
		self.update_line();
		WRITER.lock().move_cursor(-1);
	}

	pub fn clear(&mut self) {
		for i in 0..self.length {
			self.buffer[i] = 0;
		}
		self.length = 0;
	}

	pub fn update_line(&mut self) {
		let buffer_as_str = core::str::from_utf8(&self.buffer[..self.length]).unwrap();
		WRITER.lock().update_line(buffer_as_str);
	}

	fn init(&mut self) {
		self.clear();
		WRITER.lock().column_position = 0;
		self.insert_string(PROMPT_STRING);
	}
}

pub fn left_arrow() {
	if WRITER.lock().column_position > PROMPT_LENGTH {
		WRITER.lock().move_cursor(-1);
	}
}

pub fn right_arrow() {
	if WRITER.lock().column_position < PROMPT.lock().length {
		WRITER.lock().move_cursor(1);
	}
}

pub fn backspace() {
	if WRITER.lock().column_position > PROMPT_LENGTH {
		PROMPT.lock().remove_char();
	}
}

pub fn delete() {
	if WRITER.lock().column_position < PROMPT.lock().length {
		WRITER.lock().move_cursor(1);
		PROMPT.lock().remove_char();
	}
}

pub fn home() {
	let diff: i8 = (WRITER.lock().column_position - PROMPT_LENGTH) as i8;
	if diff > 0 {
		WRITER.lock().move_cursor(-diff);
	}
}

pub fn end() {
	let diff: i8 = (PROMPT.lock().length - WRITER.lock().column_position) as i8;
	if diff > 0 {
		WRITER.lock().move_cursor(diff);
	}
}

pub fn tab() {
	if WRITER.lock().column_position < VGA_COLUMNS - 4 {
		PROMPT.lock().insert_string("    ");
	}
}

pub fn enter() {
	PROMPT.lock().insert_char(b'\n', false);
}

pub fn init() {
	print!("");
	PROMPT.lock().init();
}
