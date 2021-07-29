//! RTWins TUI libarry

pub mod esc_codes;
pub mod colors;
pub mod widget;

// shortcut to esc::
pub use esc_codes::*;
pub use widget::*;

pub const VER: &str = "0.1.0";

// -----------------------------------------------------------------------------------------------

pub fn init() {

}
