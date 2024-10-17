use crate::asm;

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
    buffer: &'static mut Buffer,
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

    fn enable_cursor(&mut self) {
        // We call `out 0x3d4, 0xa` so that on the `out 0x3d5, 0xe` the cursor will change it's look
        asm::outb(0x3d4, 0xa);
        // bits 0-4 control the cursor shape (0x0-0xf range), we chose 0xe because it looks cool
        asm::outb(0x3d5, 0xe);
    }
    fn disable_cursor(&mut self) {
        // We call `out 0x3d4, 0xa` so that on the `out 0x3d5, 0x10` the cursor will disapear
        asm::outb(0x3d4, 0xa);
        // bit 5 disables the cursor (0xf or 1 << 4)
        asm::outb(0x3d5, 0xf);
    }

    fn set_cursor_position(&mut self) {
        // pos of the cursor is calculated the same way character are placed on the screen
        // pos should (and must) be in the range (0-WIDTH*HEIGHT-1)
        let mut col = self.col;
        if col >= WIDTH {
            col -= 1;
        }
        let pos = self.row * WIDTH + col;

        // say we are going to put the lower bits (0-7)
        asm::outb(0x3D4, 0x0F);
        // put the lower 8 bits
        asm::outb(0x3D5, (pos & 0xff).try_into().unwrap());
        // say we are going to put the upper bits (8-15)
        asm::outb(0x3D4, 0x0E);
        // put the upper 8 bits
        asm::outb(0x3D5, ((pos >> 8) & 0xff).try_into().unwrap());
    }


    fn clear_line(&mut self, n: usize) {
        for col in 0..WIDTH {
            self.buffer.pix[n][col].char = b' ';
            self.buffer.pix[n][col].color = Colors::White;
        }
    }

    pub fn reset_screen(&mut self) {
        self.disable_cursor();
        self.enable_cursor();
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
        let color_array: [Colors; 14] = [
            Colors::BrightRed,
            Colors::BrightRed,
            Colors::BrightYellow,
            Colors::BrightYellow,
            Colors::BrightGreen,
            Colors::BrightGreen,
            Colors::BrightCyan,
            Colors::BrightCyan,
            Colors::BrightBlue,
            Colors::BrightBlue,
            Colors::Purple,
            Colors::Purple,
            Colors::BrightPurple,
            Colors::BrightPurple,
        ];
        for byte in str.bytes() {
            let color_index = (self.col + self.row) % color_array.len();
            let color = color_array[color_indecol];
            self.color = color;
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

static CELL: Mutex<Lazy<Cell>> = Mutex::new(Lazy::new(|| Cell::default()));

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
        crate::CELL::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! printdel {
    () => {
        crate::CELL::_del();
    };
}

pub fn _del() {
    CELL.lock().del();
}


pub(crate) fn _print(args: fmt::Arguments) {
    CELL.lock().write_fmt(args).unwrap();
}

pub fn set_color(color: Colors) {
    CELL.lock().color = color;
}

pub fn color_str_to_color(s: &[u8]) -> Option<Colors> { // maybe to remove
    match s {
        [b'b', b'l', b'a', b'c', b'k'] => { return Some(Colors::Black) }
        [b'b', b'l', b'u', b'e'] => { return Some(Colors::Blue) }
        [b'g', b'r', b'e', b'e', b'n'] => { return Some(Colors::Green) }
        [b'c', b'y', b'a', b'n']  => { return Some(Colors::Cyan) }
        [b'r', b'e', b'd'] => { return Some(Colors::Red) }
        [b'p', b'u', b'r', b'p', b'l', b'e'] => { return Some(Colors::Purple) }
        [b'y', b'e', b'l', b'l', b'o', b'w'] => { return Some(Colors::Yellow) }
        [b'w', b'h', b'i', b't', b'e'] => { return Some(Colors::White) }
        [b'g', b'r', b'e', b'y'] => { return Some(Colors::Grey) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'b', b'l', b'u', b'e'] => { return Some(Colors::BrightBlue) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'g', b'r', b'e', b'e', b'n'] => { return Some(Colors::BrightGreen) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'c', b'y', b'a', b'n'] => { return Some(Colors::BrightCyan) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'r', b'e', b'd'] => { return Some(Colors::BrightRed) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'p', b'u', b'r', b'p', b'l', b'e'] => { return Some(Colors::BrightPurple) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'y', b'e', b'l', b'l', b'o', b'w'] => { return Some(Colors::BrightYellow) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'w', b'h', b'i', b't', b'e'] => { return Some(Colors::BrightWhite) }
        _ => { return None }
    }
}

pub fn get_color() -> Colors {
    return CELL.lock().color;
}

pub fn reset_screen() {
    CELL.lock().reset_screen()
}
