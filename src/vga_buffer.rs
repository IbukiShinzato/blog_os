#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// 各要素をu8(1Byte)に指定
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Colorと同じでu8(1Byte)
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    // 上位4bitを背景にして、下位4bitを前景
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// ここに文字を入れて出力する
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// 画面を書き出す為の型
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // 改行
            b'\n' => self.new_line(),
            byte => {
                // 右端まで到達したら改行
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // 常に一番下の行にカーソルを持っていく
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // Bufferに直接ScreenChar構造体を入れ込む
                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        unimplemented!()
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 出力可能なASCII(Space ~ Tilde)
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 出力不可能なASCII => ☻を出力
                _ => self.write_byte(0x02),
            }
        }
    }
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightCyan, Color::Black),
        //  VGAテキストバッファへのメモリマップドI/O (MMIO) アドレス
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
