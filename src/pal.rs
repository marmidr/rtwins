//! # RTWins Widget

/// Platform Abstraction Layer for your project
#[allow(unused_variables)]
pub trait Pal {
    /// Write character multiple times
    fn write_char_n(&mut self, c: char, repeat: i16) {}

    /// Write single character
    #[inline]
    fn write_char(&mut self, c: char) {
        self.write_char_n(c, 1);
    }

    /// Write string multiple times
    fn write_str_n(&mut self, s: &str, repeat: i16) {}

    /// Write single string
    #[inline]
    fn write_str(&mut self, s: &str) {
        self.write_str_n(s, 1);
    }

    /// Flush buffer to the terminal (depends on implementation)
    fn flush_buff(&mut self) {}

    /// Tell the PAL that we are writing logs
    fn mark_logging(&mut self, active: bool) {}

    /// Sleep for `ms` milliseconds
    fn sleep(&self, ms: u16) {}

    /// Get current timestamp (since program start), in milliseconds
    fn get_timestamp_ms(&self) -> u32 {
        0
    }

    /// Get difference between current and previous timestamp, in milliseconds
    fn get_timespan_ms(&self, prev_timestamp: u32) -> u32 {
        0
    }
}

/// Empty PAL
#[derive(Default)]
pub struct PalStub {}

impl Pal for PalStub {}
