//! # RTWins TUI library
//! `RTWins` is a Rust library designed for easy creation of visual terminal applications.
//!
//! *Future goal: make it run on non-os platforms, like bare Cortex-M3.*
//!
//! It provides basic facilities required by interactive applications such as screen and cursor management, keyboard input, keymaps, color codes.

/// Library version from Cargo.toml
pub const VER: &str = env!("CARGO_PKG_VERSION");

// public modules
pub mod colors;
pub mod common;
pub mod esc;
pub mod input;
pub mod input_decoder;
#[cfg(target_os = "linux")]
pub mod input_tty;
pub mod pal;
pub mod string_ext;
pub mod utils;

// private modules
mod debug_trace;
mod terminal;
mod widget_def;
mod widget_draw;
mod widget_impl;

// import common definition into library's namespace
pub use crate::common::*;
pub use crate::debug_trace::Trace;
pub use crate::terminal::PalBox;
pub use crate::terminal::Term;
pub use crate::terminal::TERM;

// group widget public code under single namespace
pub mod wgt {
    pub use crate::widget_def::*;
    pub use crate::widget_draw::*;
    pub use crate::widget_impl::*;
}

// ---------------------------------------------------------------------------------------------- //
