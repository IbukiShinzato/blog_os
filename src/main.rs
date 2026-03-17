#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello!";
static BLOG_OS: &[u8] = b"Blog OS!";

// 80(文字 + 背景色) = 160Byte
// １行に160文字入る
static ENTER: isize = 160;

// mangleするとユニークなシンボルになるのでno_mangle
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
