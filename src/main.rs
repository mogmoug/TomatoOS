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
    loop {
        unsafe{
            match ScancodeSet1::map_scancode(inb(0x60)) {
                Ok(key) => match key {
                    KeyCode::AltLeft => todo!(),
                    KeyCode::AltRight => todo!(),
                    KeyCode::ArrowDown => todo!(),
                    KeyCode::ArrowLeft => todo!(),
                    KeyCode::ArrowRight => todo!(),
                    KeyCode::ArrowUp => todo!(),
                    KeyCode::BackSlash => todo!(),
                    KeyCode::Backspace => todo!(),
                    KeyCode::BackTick => todo!(),
                    KeyCode::BracketSquareLeft => todo!(),
                    KeyCode::BracketSquareRight => todo!(),
                    KeyCode::Break => todo!(),
                    KeyCode::CapsLock => todo!(),
                    KeyCode::Comma => todo!(),
                    KeyCode::ControlLeft => todo!(),
                    KeyCode::ControlRight => todo!(),
                    KeyCode::Delete => todo!(),
                    KeyCode::End => todo!(),
                    KeyCode::Enter => todo!(),
                    KeyCode::Escape => todo!(),
                    KeyCode::Equals => todo!(),
                    KeyCode::F1 => todo!(),
                    KeyCode::F2 => todo!(),
                    KeyCode::F3 => todo!(),
                    KeyCode::F4 => todo!(),
                    KeyCode::F5 => todo!(),
                    KeyCode::F6 => todo!(),
                    KeyCode::F7 => todo!(),
                    KeyCode::F8 => todo!(),
                    KeyCode::F9 => todo!(),
                    KeyCode::F10 => todo!(),
                    KeyCode::F11 => todo!(),
                    KeyCode::F12 => todo!(),
                    KeyCode::Fullstop => todo!(),
                    KeyCode::Home => todo!(),
                    KeyCode::Insert => todo!(),
                    KeyCode::Key1 => todo!(),
                    KeyCode::Key2 => todo!(),
                    KeyCode::Key3 => todo!(),
                    KeyCode::Key4 => todo!(),
                    KeyCode::Key5 => todo!(),
                    KeyCode::Key6 => todo!(),
                    KeyCode::Key7 => todo!(),
                    KeyCode::Key8 => todo!(),
                    KeyCode::Key9 => todo!(),
                    KeyCode::Key0 => todo!(),
                    KeyCode::Menus => todo!(),
                    KeyCode::Minus => todo!(),
                    KeyCode::Numpad0 => todo!(),
                    KeyCode::Numpad1 => todo!(),
                    KeyCode::Numpad2 => todo!(),
                    KeyCode::Numpad3 => todo!(),
                    KeyCode::Numpad4 => todo!(),
                    KeyCode::Numpad5 => todo!(),
                    KeyCode::Numpad6 => todo!(),
                    KeyCode::Numpad7 => todo!(),
                    KeyCode::Numpad8 => todo!(),
                    KeyCode::Numpad9 => todo!(),
                    KeyCode::NumpadEnter => todo!(),
                    KeyCode::NumpadLock => todo!(),
                    KeyCode::NumpadSlash => todo!(),
                    KeyCode::NumpadStar => todo!(),
                    KeyCode::NumpadMinus => todo!(),
                    KeyCode::NumpadPeriod => todo!(),
                    KeyCode::NumpadPlus => todo!(),
                    KeyCode::PageDown => todo!(),
                    KeyCode::PageUp => todo!(),
                    KeyCode::PauseBreak => todo!(),
                    KeyCode::PrintScreen => todo!(),
                    KeyCode::ScrollLock => todo!(),
                    KeyCode::SemiColon => todo!(),
                    KeyCode::ShiftLeft => todo!(),
                    KeyCode::ShiftRight => todo!(),
                    KeyCode::Slash => todo!(),
                    KeyCode::Spacebar => todo!(),
                    KeyCode::SysReq => todo!(),
                    KeyCode::Tab => todo!(),
                    KeyCode::Quote => todo!(),
                    KeyCode::WindowsLeft => todo!(),
                    KeyCode::WindowsRight => todo!(),
                    KeyCode::A => todo!(),
                    KeyCode::B => todo!(),
                    KeyCode::C => todo!(),
                    KeyCode::D => todo!(),
                    KeyCode::E => todo!(),
                    KeyCode::F => todo!(),
                    KeyCode::G => todo!(),
                    KeyCode::H => todo!(),
                    KeyCode::I => todo!(),
                    KeyCode::J => todo!(),
                    KeyCode::K => todo!(),
                    KeyCode::L => todo!(),
                    KeyCode::M => todo!(),
                    KeyCode::N => todo!(),
                    KeyCode::O => todo!(),
                    KeyCode::P => todo!(),
                    KeyCode::Q => todo!(),
                    KeyCode::R => todo!(),
                    KeyCode::S => todo!(),
                    KeyCode::T => todo!(),
                    KeyCode::U => todo!(),
                    KeyCode::V => todo!(),
                    KeyCode::W => todo!(),
                    KeyCode::X => todo!(),
                    KeyCode::Y => todo!(),
                    KeyCode::Z => todo!(),
                    KeyCode::HashTilde => todo!(),
                    KeyCode::PrevTrack => todo!(),
                    KeyCode::NextTrack => todo!(),
                    KeyCode::Mute => todo!(),
                    KeyCode::Calculator => todo!(),
                    KeyCode::Play => todo!(),
                    KeyCode::Stop => todo!(),
                    KeyCode::VolumeDown => todo!(),
                    KeyCode::VolumeUp => todo!(),
                    KeyCode::WWWHome => todo!(),
                    KeyCode::PowerOnTestOk => todo!(),
                    KeyCode::Oem102 => todo!(),
                    KeyCode::PrintScreen2 => todo!(),
                    KeyCode::TooManyKeys => todo!(),
                    _ => todo!()
                },
                Err(err) => {
                    match err {
                        Error::BadStartBit => println!("Keyboard input error:Bad Start Bit"),
                        Error::BadStopBit => println!("Keyboard input error:Bad Stop Bit"),
                        Error::ParityError => println!("Keyboard input error:Parity Error"),
                        Error::UnknownKeyCode => println!("Keyboard input error:Unknown Key Code"),
                        Error::InvalidState => println!("Keyboard input Error:Invalid State"),
                    }
                },
            }
        }
    }
}