use crate::console;

pub fn console() -> &'static dyn console::interface::All {
    &super::driver::PL011_UART
}
