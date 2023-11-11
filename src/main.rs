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

    println!(
        "[0] {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("[1] Booting on: {}", bsp::board_name());

    println!("[2] Drivers loaded:");
    driver::driver_manager().enumerate();

    println!("[3] Chars written: {}", console().chars_written());
    println!("[4] Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}
