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
pub mod esc;
pub mod pal;
pub mod input;
#[cfg(target_os = "linux")]
pub mod input_tty;
pub mod input_decoder;
pub mod string_ext;
pub mod utils;
pub mod common;

// private modules
mod widget_def;
mod widget_impl;
mod widget_draw;
mod terminal;

// import common definition into library's namespace
pub use crate::common::*;
pub use crate::terminal::*;
pub use crate::widget_def::*;

// group widget public code under single namespace
pub mod wgt {
    pub use crate::widget_impl::*;
    pub use crate::widget_draw::*;
}

// ---------------------------------------------------------------------------------------------- //

use std::sync::{Mutex, MutexGuard, TryLockResult};

// rename Tui
pub struct TWins {
    term: Mutex<Term>,
}

impl TWins {
    /// Create new instance
    pub fn new(p: PalBox) -> TWins {
        TWins {
            term: Mutex::new(Term::new(p)),
        }
    }

    /// Get access to mutex-protexted internal instance
    pub fn lock(&mut self) -> MutexGuard<Term> {
        self.term.lock().unwrap()
    }

    /// Try to get access to mutex-protexted internal instance
    pub fn try_lock(&mut self) -> TryLockResult<MutexGuard<Term>> {
        self.term.try_lock()
    }
}

// ---------------------------------------------------------------------------------------------- //

pub struct Ui {

}
