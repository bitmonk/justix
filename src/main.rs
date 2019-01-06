#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use justix::println;
use justix::vga_buffer;
use justix::serial_println;
use justix::exit_qemu;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop{}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  vga_buffer::print_something();
  serial_println!("Hello Host{}", "!");
  println!("Hello World{}", "!");

  loop {}
}
