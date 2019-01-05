#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

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

  unsafe { exit_qemu(); }

  loop {}
}
