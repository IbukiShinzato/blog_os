#![no_std]
#![no_main]

mod vga_buffer;

use crate::vga_buffer::WRITER;
use core::panic::PanicInfo;

// mangleするとユニークなシンボルになるのでno_mangle
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    WRITER.lock().init();
    println!("Hello, World{}\n", '!');

    square(10);
    cross(5);

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn square(n: usize) {
    for _ in 0..n {
        print!("-");
    }
    println!();
    for _ in 0..n / 2 {
        print!("|");
        for _ in 0..n - 2 {
            print!(" ");
        }
        print!("|");
        println!()
    }
    for _ in 0..n {
        print!("-");
    }
    println!();
}

fn cross(n: usize) {
    for i in (2..=n).rev() {
        for _ in 0..n - i {
            print!(" ");
        }
        print!("{}", '\\');
        for _ in 0..(i - 2) * 2 {
            print!(" ");
        }
        print!("/");

        println!();
    }
    for i in 2..=n {
        for _ in 0..n - i {
            print!(" ");
        }
        print!("/");
        for _ in 0..(i - 2) * 2 {
            print!(" ");
        }
        print!("{}", '\\');

        println!();
    }
}
