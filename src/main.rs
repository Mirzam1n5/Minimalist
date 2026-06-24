#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("asm/boot.S")); 
global_asm!(include_str!("asm/trap.S"));

mod uart;

#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
			});
}
#[macro_export]
macro_rules! println
{
	() => ({
		   print!("\r\n")
		   });
	($fmt:expr) => ({
			print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut my_uart = uart::Uart::new(0x1000_0000);
    my_uart.init();

	use core::fmt::Write;
	let _ = my_uart.write_str("It is working!\r\n");
	let _ = my_uart.write_str("If you see this, uart is alive.\r\n");

	loop { }

    /* println!("This is my operating system!");
	println!("I'm so awesome. If you start typing something, I'll show you what you typed!");

    loop { } */
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[unsafe(no_mangle)]
extern "C" fn abort() -> ! {
    loop { }
}