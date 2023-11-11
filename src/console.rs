use crate::bsp;

pub mod interface {
    use core::fmt;

    pub trait Write {
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    /// Console statistics.
    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
    }

    pub trait All: Write + Statistics {}
}

/// Return a reference to the console.
/// global console used by all printing macros.
pub fn console() -> &'static dyn interface::All {
    bsp::console::console()
}
