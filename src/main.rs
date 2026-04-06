#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

global_asm!(include_str!("asm/boot.S")); 
global_asm!(include_str!("asm/trap.S"));

mod uart;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut my_uart = uart::Uart::new(0x1000_0000);
    my_uart.init();

    use core::fmt::Write;
    let _ = write!(my_uart, "Hello from Rust kmain!\n");

    loop { }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[unsafe(no_mangle)]
extern "C" fn abort() -> ! {
    loop { }
}