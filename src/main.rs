#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
    println!("Hello from Rust!");
    panic!("Stopping here.")
}
