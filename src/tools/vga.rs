use crate::exceptions::interrupts;
use crate::tools::io::outb;
use crate::tools::prompt;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const NUM_SCREENS: usize = 5;
const SERIAL_SCREEN: usize = 4;
const VGA_BUFFER_SIZE: usize = VGA_COLUMNS * VGA_ROWS;

const VGA_BUFF_ADDRR: usize = 0xc00b8000;
pub const VGA_COLUMNS: usize = 80;
const VGA_ROWS: usize = 25;
pub const VGA_LAST_LINE: usize = VGA_ROWS - 1;

const VGA_CTRL_REGISTER: u16 = 0x3d4;
const VGA_DATA_REGISTER: u16 = 0x3d5;

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		row_position: 0,
		color: Color::new(ColorCode::Red, ColorCode::Yellow),
		buffer: unsafe { &mut *(VGA_BUFF_ADDRR as *mut VgaBuffer) },
		screen: [
			ScreenState {
				column_position: 0,
				row_position: 0,
				color: Color::new(ColorCode::Green, ColorCode::Black),
				buffer: [0; VGA_BUFFER_SIZE],
			},
			ScreenState {
				column_position: 0,
				row_position: 0,
				color: Color::new(ColorCode::Yellow, ColorCode::Brown),
				buffer: [0; VGA_BUFFER_SIZE],
			},
			ScreenState {
				column_position: 0,
				row_position: 0,
				color: Color::new(ColorCode::Black, ColorCode::LightCyan),
				buffer: [0; VGA_BUFFER_SIZE],
			},
			ScreenState {
				column_position: 0,
				row_position: 0,
				color: Color::new(ColorCode::Yellow, ColorCode::Red),
				buffer: [0; VGA_BUFFER_SIZE],
			},
			ScreenState {
				column_position: 0,
				row_position: 0,
				color: Color::new(ColorCode::LightGray, ColorCode::Black),
				buffer: [0; VGA_BUFFER_SIZE],
			},
		],
		current_display: 0,
		mode: WriteMode::Normal,
	});
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorCode {
	Black = 0x0,
	Blue = 0x1,
	Green = 0x2,
	Cyan = 0x3,
	Red = 0x4,
	Magenta = 0x5,
	Brown = 0x6,
	LightGray = 0x7,
	DarkGray = 0x8,
	LightBlue = 0x9,
	LightGreen = 0xa,
	LightCyan = 0xb,
	LightRed = 0xc,
	Pink = 0xd,
	Yellow = 0xe,
	White = 0xf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Color(u8);

impl Color {
	fn new(foreground: ColorCode, background: ColorCode) -> Color {
		Color(((background as u8) << 4) | (foreground as u8))
	}

	fn increase_foreground(&mut self) {
		let foreground = self.0 & 0x0f;
		self.0 = ((foreground + 0x01) % 0x0f) + (self.0 & 0xf0);
	}

	fn increase_background(&mut self) {
		let background = self.0 & 0xf0;
		self.0 = ((background + 0x10) % 0xf0) + (self.0 & 0x0f);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color: Color,
}

#[repr(transparent)]
struct VgaBuffer {
	chars: [[ScreenChar; VGA_COLUMNS]; VGA_ROWS],
}

impl VgaBuffer {
	fn read(&self, row: usize, column: usize) -> ScreenChar {
		self.chars[row][column]
	}

	fn write(&mut self, character: ScreenChar, row: usize, column: usize) {
		self.chars[row][column] = character;
	}
}

struct ScreenState {
	column_position: usize,
	row_position: usize,
	color: Color,
	buffer: [u8; VGA_BUFFER_SIZE],
}

pub struct Writer {
	pub column_position: usize,
	pub row_position: usize,
	color: Color,
	buffer: &'static mut VgaBuffer,
	screen: [ScreenState; NUM_SCREENS],
	pub current_display: usize,
	mode: WriteMode,
}

pub enum WriteMode {
	Normal,
	Srl,
}

impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match self.mode {
			WriteMode::Normal => self.write_byte_normal(byte),
			WriteMode::Srl => self.write_byte_srl(byte),
		}
	}

	fn write_byte_normal(&mut self, byte: u8) {
		if self.column_position == VGA_COLUMNS {
			self.new_line();
		}
		match byte {
			b'\n' => self.new_line(),
			byte => {
				self.buffer.write(
					ScreenChar {
						ascii_character: byte,
						color: self.color,
					},
					VGA_LAST_LINE,
					self.column_position,
				);

				self.column_position += 1;
			}
		}
	}

	fn write_byte_srl(&mut self, byte: u8) {
		if self.screen[SERIAL_SCREEN].column_position >= VGA_COLUMNS {
			self.new_line_srl();
		}

		match byte {
			b'\n' => self.new_line_srl(),
			byte => {
				self.screen[SERIAL_SCREEN].buffer[self.screen[SERIAL_SCREEN].row_position
					* VGA_COLUMNS + self.screen[SERIAL_SCREEN]
					.column_position] = byte;
				self.screen[SERIAL_SCREEN].column_position += 1;
			}
		}
	}

	fn new_line_srl(&mut self) {
		let srl_screen = &mut self.screen[SERIAL_SCREEN];
		srl_screen.column_position = 0;
		if srl_screen.row_position < VGA_ROWS - 1 {
			srl_screen.row_position += 1;
		} else {
			self.scroll_screen_srl();
		}
	}

	fn scroll_screen_srl(&mut self) {
		let srl_screen = &mut self.screen[SERIAL_SCREEN];

		for row in 1..VGA_ROWS {
			for col in 0..VGA_COLUMNS {
				let character = srl_screen.buffer[row * VGA_COLUMNS + col];
				srl_screen.buffer[(row - 1) * VGA_COLUMNS + col] = character;
			}
		}

		self.clear_line_srl(VGA_LAST_LINE);
	}

	fn clear_line_srl(&mut self, row: usize) {
		for col in 0..VGA_COLUMNS {
			self.screen[SERIAL_SCREEN].buffer[row * VGA_COLUMNS + col] = b' ';
		}
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			self.write_byte(convert_to_cp437(byte));
		}
		self.update_cursor(VGA_LAST_LINE, self.column_position);
	}

	pub fn write_string_raw(&mut self, s: &str) {
		let shift: u8 = 96;
		for byte in s.bytes() {
			self.write_byte(byte + shift);
		}
		self.update_cursor(VGA_LAST_LINE, self.column_position);
	}

	pub fn update_line(&mut self, s: &str) {
		let cursor = self.column_position;
		self.clear_row(VGA_LAST_LINE);
		self.write_string(s);
		self.column_position = cursor;
	}

	fn new_line(&mut self) {
		for row in 1..VGA_ROWS {
			for column in 0..VGA_COLUMNS {
				let character = self.buffer.read(row, column);
				self.buffer.write(character, row - 1, column);
			}
		}
		self.clear_row(VGA_LAST_LINE);
	}

	fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar {
			ascii_character: b' ',
			color: self.color,
		};
		for column in 0..VGA_COLUMNS {
			self.buffer.write(blank, row, column);
		}
		self.column_position = 0;
	}

	pub fn clear_screen(&mut self) {
		for row in 0..VGA_ROWS {
			self.clear_row(row);
		}
		self.update_cursor(VGA_LAST_LINE, self.column_position);
	}

	pub fn hide_cursor(&self) {
		unsafe {
			outb(VGA_CTRL_REGISTER, 0x0a);
			outb(VGA_DATA_REGISTER, 0x20);
		}
	}

	pub fn show_cursor(&self) {
		unsafe {
			outb(VGA_CTRL_REGISTER, 0x0a);
			outb(VGA_DATA_REGISTER, 0x0e);
		}
	}

	pub fn update_cursor(&mut self, row: usize, column: usize) {
		if self.current_display == SERIAL_SCREEN {
			return;
		}

		let position: u16 = (row * VGA_COLUMNS + column) as u16;

		unsafe {
			outb(VGA_CTRL_REGISTER, 0x0f);
			outb(VGA_DATA_REGISTER, (position & 0xff) as u8);
			outb(VGA_CTRL_REGISTER, 0x0e);
			outb(VGA_DATA_REGISTER, ((position >> 8) & 0xff) as u8);
		}
	}

	pub fn move_cursor(&mut self, i: i8) {
		if i < 0 {
			self.column_position -= i.abs() as usize;
		} else if i > 0 {
			self.column_position += i as usize;
		}
		self.update_cursor(VGA_LAST_LINE, self.column_position);
	}

	fn backup_display(&mut self) {
		self.screen[self.current_display].column_position = self.column_position;
		self.screen[self.current_display].color = self.color;
		for row in 0..VGA_ROWS - 1 {
			for column in 0..VGA_COLUMNS {
				self.screen[self.current_display].buffer[row * VGA_COLUMNS + column] =
					self.buffer.read(row, column).ascii_character;
			}
		}
	}

	fn restore_display(&mut self, display: usize) {
		self.column_position = self.screen[display].column_position;
		self.color = self.screen[display].color;
		for row in 0..VGA_ROWS - 1 {
			for column in 0..VGA_COLUMNS {
				self.buffer.write(
					ScreenChar {
						ascii_character: self.screen[display].buffer[row * VGA_COLUMNS + column],
						color: self.color,
					},
					row,
					column,
				);
			}
		}
	}

	fn update_display(&mut self) {
		for row in 0..VGA_ROWS {
			for column in 0..VGA_COLUMNS {
				self.buffer.write(
					ScreenChar {
						ascii_character: self.buffer.read(row, column).ascii_character,
						color: self.color,
					},
					row,
					column,
				);
			}
		}
	}

	pub fn set_mode(&mut self, mode: WriteMode) {
		self.mode = mode;
	}
}

pub fn change_display(display: usize) {
	if WRITER.lock().current_display == display {
		return;
	}
	WRITER.lock().backup_display();
	WRITER.lock().restore_display(display);
	WRITER.lock().current_display = display;
	if display != SERIAL_SCREEN {
		prompt::init();
	} else {
		WRITER.lock().clear_row(VGA_LAST_LINE);
	}
}

pub fn change_color(foreground: bool) {
	interrupts::disable();
	if foreground {
		WRITER.lock().color.increase_foreground();
	} else {
		WRITER.lock().color.increase_background();
	}
	WRITER.lock().update_display();
	interrupts::enable();
}

fn convert_to_cp437(byte: u8) -> u8 {
	match byte {
		0x01 => 0x80,
		0x02 => 0x81,
		0x03 => 0x82,
		0x04 => 0x83,
		0x05 => 0x84,
		0x06 => 0x85,
		0x07 => 0x87,
		0x08 => 0x88,
		0x09 => 0x89,
		0x0b => 0x8a,
		0x0c => 0x8b,
		0x0d => 0x8c,
		0x0e => 0x8e,
		0x0f => 0x90,
		0x10 => 0x93,
		0x11 => 0x94,
		0x12 => 0x96,
		0x13 => 0x97,
		0x14 => 0x99,
		0x15 => 0x9a,
		0x16 => 0x9c,
		0x17 => 0xe6,
		0x18 => 0xf8,
		0x19 => 0xfd,
		0x1a => 0x15,
		_ => byte,
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}
