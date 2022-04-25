#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
mod console;
mod log;
use crate::log::*;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello LL_OS!");
    puts_log(Color::Red, "I am a error!");
    puts_log(Color::Yellow, "I am a warning!");
    puts_log(Color::Blue, "I am a infomation!");
    puts_log(Color::Green, "I am a debug info!");
    puts_log(Color::Gray, "I am a trace info!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0)}
    });
}