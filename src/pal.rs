//! # RTWins Widget

/// Platform Abstraction Layer for your project
pub trait Pal {
    /// Write character multiple times
    fn write_char_n(&mut self, c: char, repeat: i16);

    /// Write single character
    fn write_char(&mut self, c: char) { self.write_char_n(c, 1); }

    /// Write string multiple times
    fn write_str_n(&mut self, s: &str, repeat: i16);

    /// Write single string
    fn write_str(&mut self, s: &str) { self.write_str_n(s, 1); }

    /// Flush buffer to the terminal (depends on implementation)
    fn flush_buff(&mut self);

    /// Tell the PAL that we are writing logs
    fn mark_logging(&mut self, active: bool);

    /// Sleep for `ms` milliseconds
    fn sleep(&mut self, ms: u16);

    /// Get current timestamp in milliseconds
    fn get_time_stamp(&mut self) -> u32;

    /// Get difference between current and previous timestamp, in milliseconds
    fn get_time_diff(&mut self, prev_timestamp: u32) -> u32;

    /// Returns formated time, used for logs timestamp
    fn get_time_str(&mut self) -> String;
}
