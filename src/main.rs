#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点
mod vga_buffer;
use core::panic::PanicInfo;

/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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
    loop {}
}