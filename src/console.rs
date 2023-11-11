use crate::synchronization::{self, NullLock};

mod null_console;

pub mod interface {
    use core::fmt;

    pub trait Write {
        fn write_char(&self, c: char);

        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        fn flush(&self);
    }

    pub trait Read {
        /// Read a single character.
        fn read_char(&self) -> char {
            ' '
        }

        /// Clear RX buffers, if any.
        fn clear_rx(&self);
    }

    /// Console statistics.
    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
        fn chars_read(&self) -> usize {
            0
        }
    }

    pub trait All: Write + Read + Statistics {}
}

static CUR_CONSOLE: NullLock<&'static (dyn interface::All + Sync)> =
    NullLock::new(&null_console::NULL_CONSOLE);

use synchronization::interface::Mutex;

/// Register a new console.
pub fn register_console(new_console: &'static (dyn interface::All + Sync)) {
    CUR_CONSOLE.lock(|con| *con = new_console);
}

/// Return a reference to the currently registered console.
/// global console used by all printing macros.
pub fn console() -> &'static dyn interface::All {
    CUR_CONSOLE.lock(|con| *con)
}
