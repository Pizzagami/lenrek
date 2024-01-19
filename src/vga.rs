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
            let color = color_array[color_index];
            self.color = color;
            self.print_char(byte)
        }
    }

}