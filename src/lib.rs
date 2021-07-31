//! # RTWins TUI libarry
//! `RTWins` is a Rust library designed for easy creation of visual terminal applications.
//!
//! *Future goal: make it run on non-os platforms, like bare Cortex-M3.*
//!
//! It provides basic facilities required by interactive applications such as screen and cursor management, keyboard input, keymaps, color codes.

pub mod esc;
pub mod pal;
pub mod colors;
pub mod widget;
pub mod widget_impl;

pub use widget::*;
pub use widget_impl::*;

pub const VER: &str = "0.1.0";

// -----------------------------------------------------------------------------------------------

pub fn init() {

}
