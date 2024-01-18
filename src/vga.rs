const VGA_ADDRESS: u32 = 0xB8000;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Colors {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Yellow,
    White,
    Grey,
    BrightBlue,
    BrightGreen,
    BrightCyan,
    BrightRed,
    BrightPurple,
    BrightYellow,
    BrightWhite,
}


#[repr(C)]
struct ScreenChar {
    char: u8,
    color: Colors,
}

struct Buffer {
    pix: [[ScreenChar; WIDTH]; HEIGHT]
}

pub struct Cell {
    pub col: usize,
    pub row: usize,
    pub color: Colors,
    pub buffer: &'static mut Buffer,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            col: 0,
            row: 0,
            color: Colors::White,
            buffer: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) },
        }
    }   
}

impl Cell {

    fn set_cursor_position(&mut self) {
        let mut col = self.col;
        if col >= WIDTH {
            col -= 1;
        }
        let pos = self.row * WIDTH + col;
    }

    fn clear_line(&mut self, n: usize) {
        for x in 0..WIDTH {
            self.buffer.pix[n][x].char = b' ';
            self.buffer.pix[n][x].color = Colors::White;
        }
    }

    pub fn reset_screen(&mut self) {
        self.col = 0;
        self.row = 0;
        self.color = Colors::White;
        for _ in 0..HEIGHT * WIDTH {
            self.print_char(b' ');
        }
        self.col = 0;
        self.row = 0;
    }

    pub fn print_char(&mut self, char: u8) {
        if self.col >= WIDTH && char != b'\n' {
            self.col = 0;
            self.row += 1;
        }
        if self.row >= HEIGHT {

            for i in 0..HEIGHT - 1 { 
                for j in 0..WIDTH {
                    self.buffer.pix[i][j].char = self.buffer.pix[i + 1][j].char;
                    self.buffer.pix[i][j].color = self.buffer.pix[i + 1][j].color;
                }
            }
            self.clear_line(HEIGHT - 1);
            self.row = HEIGHT -1;
        }
        if char == b'\n' {
            self.col = 0;
            self.row += 1;
        }
        else {
            self.buffer.pix[self.row][self.col].char = char;
            self.buffer.pix[self.row][self.col].color = self.color;
            self.col += 1;
        }
    }

    pub fn print_string(&mut self, str: &str) {
        for byte in str.bytes() {
            self.print_char(byte)
        }
        self.set_cursor_position();
    }


    pub fn del(&mut self) {
        if self.col == 0 {
            if self.row > 0 {
                self.row -= 1;
            }
            self.col = WIDTH - 1;
            self.buffer.pix[self.row][self.col].char = b' ';
        }
        else {
            self.col -= 1;
            self.buffer.pix[self.row][self.col].char = b' ';
        }
        self.set_cursor_position();
    }
}

use core::fmt::{self, Write};
impl fmt::Write for Cell {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        Ok(())
    }
}


use spin::Mutex;
use once_cell::unsync::Lazy;

static INTERFACE: Mutex<Lazy<Cell>> = Mutex::new(Lazy::new(|| Cell::default()));

#[macro_export]
macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    }
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {
        crate::vga::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! printdel {
    () => {
        crate::vga::_del();
    };
}

pub fn _del() {
    INTERFACE.lock().del();
}

pub(crate) fn _print(args: fmt::Arguments) {
    INTERFACE.lock().write_fmt(args).unwrap();
}