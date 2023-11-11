#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![feature(trait_alias)]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
    use console::console;

    println!("[0] Hello from Rust!");
    println!("[1] Chars written: {}", console().chars_written());
    println!("[2] Stopping here.");
    cpu::wait_forever()
}
