//! # RTWins TUI libarry
//! `RTWins` is a Rust library designed for easy creation of visual terminal applications.
//!
//! *Future goal: make it run on non-os platforms, like bare Cortex-M3.*
//!
//! It provides basic facilities required by interactive applications such as screen and cursor management, keyboard input, keymaps, color codes.

pub mod colors;
pub mod esc;
pub mod pal;
pub mod input;
pub mod widget;
pub mod widget_impl;
pub mod widget_draw;

pub use widget::*;
pub use widget_impl::*;

/// Library version
pub const VER: &str = env!("CARGO_PKG_VERSION");

use std::sync::{Mutex, MutexGuard, TryLockResult};

// -----------------------------------------------------------------------------------------------

pub type PalBox = Box<dyn crate::pal::Pal>;

// TODO: static Pal instead of PalBox
// pub struct Ctx<P: crate::pal::Pal>

pub struct Ctx {
    pub pal: PalBox,
    invalidated: Vec<crate::WId>
}

impl Ctx {
    // repeated from pal
    pub fn write_char(&mut self, c: char) -> &mut Self {
        self.pal.write_char(c);
        self
    }

    pub fn write_char_n(&mut self, c: char, repeat: i16) -> &mut Self {
        self.pal.write_char_n(c, repeat);
        self
    }

    pub fn write_str(&mut self, s: &str) -> &mut Self {
        self.pal.write_str(s);
        self
    }

    pub fn write_str_n(&mut self, s: &str, repeat: i16) -> &mut Self {
        self.pal.write_str_n(s, repeat);
        self
    }

    pub fn flush_buff(&mut self) {
        self.pal.flush_buff();
    }

    // own

    fn log(&mut self, fg: &str, prefix: &str, msg: &str) {
        self.pal.write_str(fg);
        self.pal.write_str(prefix);
        self.pal.write_str(msg);
        self.pal.write_str(esc::FG_DEFAULT);
        self.pal.flush_buff();
    }

    pub fn log_d(&mut self, msg: &str) {
        self.log(esc::FG_WHITE, "-D- ", msg);
    }

    pub fn log_w(&mut self, msg: &str) {
        self.log(esc::FG_YELLOW, "-W- ", msg);
    }

    pub fn log_e(&mut self, msg: &str) {
        self.log(esc::FG_RED, "-E- ", msg);
    }

    pub fn move_to(&mut self, col: u16, row: u16) -> &mut Self {
        let s = String::from(esc::CURSOR_GOTO_FMT)
            .replace("{0}", &col.to_string())
            .replace("{1}", &row.to_string());
        self.pal.write_str(s.as_str());
        self
    }

    pub fn move_to_col(&mut self, col: u16) -> &mut Self {
        let s = String::from(esc::CURSOR_COLUMN_FMT)
            .replace("{0}", &col.to_string());
        self.pal.write_str(s.as_str());
        self
    }

    pub fn move_by(&mut self, cols: i16, rows: i16) -> &mut Self {
        if cols != 0 {
            let fmt;
            let arg;

            if cols < 0 {
                fmt = esc::CURSOR_BACKWARD_FMT;
                arg = -cols;
            }
            else {
                fmt = esc::CURSOR_FORWARD_FMT;
                arg = cols;
            }

            let s = String::from(fmt)
                .replace("{0}", &arg.to_string());
            self.pal.write_str(s.as_str());
        }

        if rows != 0 {
            let fmt;
            let arg;

            if rows < 0 {
                fmt = esc::CURSOR_UP_FMT;
                arg = -rows;
            }
            else {
                fmt = esc::CURSOR_DOWN_FMT;
                arg = rows;
            }

            let s = String::from(fmt)
                .replace("{0}", &arg.to_string());
            self.pal.write_str(s.as_str());
        }

        self
    }

    pub fn move_to_home(&mut self) -> &mut Self {
        self.pal.write_str(esc::CURSOR_HOME);
        self
    }

    pub fn cursor_save_pos(&mut self) {
        self.pal.write_str(esc::CURSOR_POS_SAVE);
    }

    pub fn cursor_restore_pos(&mut self) {
        self.pal.write_str(esc::CURSOR_POS_RESTORE);
    }

    pub fn cursor_hide(&mut self) {
        self.pal.write_str(esc::CURSOR_HIDE);
    }

    pub fn cursor_show(&mut self) {
        self.pal.write_str(esc::CURSOR_SHOW);
    }

    pub fn insert_lines(&mut self, count: u16) {
        let s = String::from(esc::LINE_INSERT_FMT)
            .replace("{0}", &count.to_string());
        self.pal.write_str(s.as_str());
    }

    pub fn delete_lines(&mut self, count: u16) {
        let s = String::from(esc::LINE_DELETE_FMT)
            .replace("{0}", &count.to_string());
        self.pal.write_str(s.as_str());
    }

    pub fn screen_clr_above(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_ABOVE);
    }

    pub fn screen_clr_below(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_BELOW);
    }

    pub fn screen_clr_all(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_ALL);
    }

    pub fn screen_save(&mut self) {
        self.pal.write_str(esc::SCREEN_SAVE);
    }

    pub fn screen_restore(&mut self) {
        self.pal.write_str(esc::SCREEN_RESTORE);
    }

    // -----------------

    pub fn invalidate(&mut self, wnd: &crate::Widget, wids: &[crate::WId]) {
        if let widget::Type::Window(ref _w) = wnd.typ {
            // TODO: check for duplication in self.invalidated
            self.invalidated.extend_from_slice(wids);
        }
        else {
            self.log_w(format!("Widget id {} is not a Window", wnd.id).as_str());
        }
    }

    pub fn draw(&mut self, wnd: &crate::Widget, wids: &[crate::WId]) {
        for id in wids.iter() {
            for w in wnd.childs.iter() {
                if w.id == *id {
                    // draw
                }
            }
        }

        self.invalidated.retain(|x| !wids.contains(x));
    }

    pub fn draw_wnd(&mut self, wnd: &crate::Widget) {
        match wnd.typ {
            _ => {}
        }

        self.invalidated.clear();
    }
}

pub struct TWins {
    ctx: Mutex<Ctx>,
}

impl TWins {
    pub fn new(p: PalBox) -> TWins {
        TWins {
            ctx: Mutex::new(Ctx {
                pal: p,
                invalidated: vec![]
            }),
        }
    }

    pub fn lock(&mut self) -> MutexGuard<Ctx> {
        self.ctx.lock().unwrap()
    }

    pub fn try_lock(&mut self) -> TryLockResult<MutexGuard<Ctx>> {
        self.ctx.try_lock()
    }
}
