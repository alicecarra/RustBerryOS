use crate::bsp;

pub mod interface {
    pub use core::fmt::Write;
}

/// Return a reference to the console.
/// global console used by all printing macros.
pub fn console() -> impl interface::Write {
    bsp::console::console()
}
