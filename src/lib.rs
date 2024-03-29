//! # RTWins TUI library
//! `RTWins` is a Rust library designed for easy creation of terminal applications,
//! targetting non-os, resource-constrained devices, like bare metal Cortex-M3 devices (128KiB Flash or more).
//!
//! It provides basic facilities required by interactive applications
//! such as screen and cursor management, keyboard input, keymaps, color codes.

#![no_std]

use core::env;

/// Library version from Cargo.toml
pub const VER: &str = env!("CARGO_PKG_VERSION");

// public modules
pub mod colors;
pub mod common;
pub mod esc;
pub mod input;
pub mod input_decoder;
pub mod pal;
pub mod string_ext;
pub mod utils;
pub mod wnd_manager;

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
