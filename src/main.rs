#![feature(asm_const)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic;

unsafe fn kernel_init() -> ! {
    panic!()
}
