#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Colour {
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

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

fn write_position(framebuffer: *mut u8, row: usize, column: usize, str: &str, colour: Colour) {
    if row > BUFFER_HEIGHT - 1 {
        //TODO: implement scroll
        panic!("Row should be from 0 to 24")
    }

    let initial_position = (row * BUFFER_WIDTH) + column;

    for (i, &byte) in str.as_bytes().iter().enumerate() {
        let count = i + initial_position;
        unsafe {
            *framebuffer.offset(count as isize * 2) = byte;
            *framebuffer.offset(count as isize * 2 + 1) = colour as u8;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    write_position(framebuffer, 1, 3, "Test string", Colour::White);

    write_position(framebuffer, 20, 3, "Test string again", Colour::LightGreen);

    write_position(
        framebuffer,
        21,
        7,
        "Test string again and again",
        Colour::Cyan,
    );

    loop {}
}
