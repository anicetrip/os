// os/src/main.rs
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use] //一定在最前面否则会报错找不到
mod console;
mod sbi;
mod lang_items;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));



fn main() {
    
}


// fn main() {
//     // println!("Hello, world!");
// }


// #[no_mangle]
// pub fn rust_main() -> ! {
//     clear_bss();
//     console_putchar(44);
//     panic!("Shutdown machine!");
// }



// #[no_mangle]
// pub fn rust_main() -> ! {
//     clear_bss();
//     println!("Hello, world!");
//     panic!("Shutdown machine!");
// }

fn clear_bss() {
    extern "C" { 
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}



