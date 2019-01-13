#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use justix::{println, serial_println};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop{}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  serial_println!("Hello Host{}", "!");
  println!("Hello World{}", "!");
  justix::interrupts::init_idt();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

  println!("It did not crash!");
  loop {}
}
