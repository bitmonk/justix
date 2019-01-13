use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use crate::println;

pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
