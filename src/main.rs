#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点
mod vga_buffer;
use core::panic::PanicInfo;

use crate::vga_buffer::*;
use crate::Color::*;
use x86::io::*;
use pc_keyboard::*;

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    println!("Oh,No!(T_T)");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.lock().draw_fill(new_color(Yellow,Red));
    WRITER.lock().draw_point_char(1,1,new_color(Yellow,Red),b'*');
    loop {}
}