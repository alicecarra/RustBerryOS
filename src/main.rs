#![feature(asm_const)]
#![allow(clippy::upper_case_acronyms)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![feature(trait_alias)]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod driver;
mod panic;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }

    driver::driver_manager().init_drivers();
    // println! is usable from here on.
    // Transition from unsafe to safe.
    kernel_main()
}

fn kernel_main() -> ! {
    use console::console;

    println!("Loading Kernel via UART");
    println!("{:^37}", bsp::board_name());
    println!();
    println!("[ML] Requesting binary");
    console().flush();

    // Discard any spurious received characters before starting with the loader protocol.
    console().clear_rx();

    // Notify to send the binary.
    for _ in 0..3 {
        console().write_char(3 as char);
    }

    let mut size: u32 = u32::from(console().read_char() as u8);
    size |= u32::from(console().read_char() as u8) << 8;
    size |= u32::from(console().read_char() as u8) << 16;
    size |= u32::from(console().read_char() as u8) << 24;

    console().write_char('O');
    console().write_char('K');

    let kernel_addr: *mut u8 = bsp::memory::board_default_load_addr() as *mut u8;
    unsafe {
        for i in 0..size {
            core::ptr::write_volatile(kernel_addr.offset(i as isize), console().read_char() as u8)
        }
    }

    println!("[Loader] Loaded! Executing the payload now\n");
    console().flush();

    // Use black magic to create a function pointer.
    let kernel: fn() -> ! = unsafe { core::mem::transmute(kernel_addr) };

    // Jump to loaded kernel!
    kernel()
}
