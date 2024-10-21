
use core::fmt;
use core::fmt::Write;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
// You knwo what, use this rather than the println! macro in function
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// Rust provides two primary ways to define structs: tuple structs and named-field structs
// Tuple struct:: struct ColorCode (u8, i32, String);
// Named-Field:: struct ColorCode {
//     uname: int8,
//     klop: int32,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode (u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// Volatile ensures that we can’t accidentally write to it “normally”. 
// Instead, we have to use the write method now.
//Instead of a typical assignment using =, we’re now using the write method. 
//Now we can guarantee that the compiler will never optimize away this write.

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_WIDTH],
}

pub struct Writer {
    column_position: usize, //one thing about usize type is that it takes the type of the target architecture
    // so if the target archtecture is a 32bit, column_position will be a u32 type, same if its 64bit.
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte:u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                } 

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //not part of tprintable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT -1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row:usize) {
        let blank: ScreenChar = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}


//implementing rust formating macros to the write string
//function in Writer
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        ($crate::vga_buffers::_print(format_args!($($arg)*)));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}


// implementing a public stactic writer that can be used from other modules
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    });
}


// Uncomment the line below and comment the lazy_static line above.
// pub fn print_something() {
//     let mut writer = Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe {
//             &mut *(0xb8000 as *mut Buffer)
//         },
//     };

//     writer.write_byte(b'H');
//     writer.write_string("ello ");
//     writer.write_string("Wörld");
//     write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
//     writer.new_line();
//     writer.write_string("Good Job Samu!");
// }