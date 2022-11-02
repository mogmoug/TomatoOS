#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点
#![feature(core_intrinsics)] //使用编译器intrinsics
mod vga_buffer;
use core::panic::PanicInfo;

use crate::vga_buffer::*;
use crate::Color::*;

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    println!("Oh,No!(T_T)");
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    //假装是一个终端(~_~)
    println!("root@COMPUTER $ {}", "ls");
    println!("Cargo.lock  Cargo.toml  LICENSE  Makefile  README.md  src  target  x86_64-tomatoos.json");
    println!("root@COMPUTER $ {}", "echo Hello TomatoOS");
    println!("Hello TomatoOS");
    println!("root@COMPUTER $ {}", "");
    WRITER.lock().draw_circle(20, 20, 4.0, new_color(Red,Yellow));
    loop {}
}