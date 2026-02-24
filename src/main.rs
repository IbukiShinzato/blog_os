#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello!";
static BLOG_OS: &[u8] = b"Blog OS!";

// １行に160文字入る
static ENTER: isize = 160;

// mangleするとユニークなシンボルになるのでno_mangle
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // 1バイト目が文字、2バイト目が背景の色
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xd;
        }
    }

    for (i, &byte) in BLOG_OS.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2 + ENTER) = byte;
            *vga_buffer.offset(i as isize * 2 + 1 + ENTER) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
