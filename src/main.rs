#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// mangleするとユニークなシンボルになるのでno_mangle
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", '!');
    panic!("Some panic message");
    // loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
