#![no_std]
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use core::arch::asm;

unsafe extern "C" {
    unsafe static _stack: u8;
    unsafe static _heap_start: u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let stack_ptr = unsafe { &_stack as *const u8 };
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

// Rust modules
pub mod uart;