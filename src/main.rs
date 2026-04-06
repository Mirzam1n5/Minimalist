#![no_std]
#![no_main]

use minimalist::kmain; 

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    kmain();
}