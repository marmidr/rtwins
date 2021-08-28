//! # RTWins TUI libarry
//! `RTWins` is a Rust library designed for easy creation of visual terminal applications.
//!
//! *Future goal: make it run on non-os platforms, like bare Cortex-M3.*
//!
//! It provides basic facilities required by interactive applications such as screen and cursor management, keyboard input, keymaps, color codes.

pub mod colors;
pub mod esc;
pub mod pal;
pub mod widget;
pub mod widget_impl;

pub use widget::*;
pub use widget_impl::*;

pub const VER: &str = "0.1.0";

use std::sync::{Mutex, MutexGuard};

// -----------------------------------------------------------------------------------------------

pub type PalBox = Box<dyn crate::pal::Pal>;

pub struct Ctx {
    pub pal: PalBox,
    // invalidated: Vec<crate::WId>
}

impl Ctx {
    pub fn log_d(&mut self, s: &str) {
        self.pal.write_str("-D- ");
        self.pal.write_str(s);
        self.pal.flush_buff();
    }

    pub fn log_w(&mut self, s: &str) {
        self.pal.write_str(esc::FG_YELLOW);
        self.pal.write_str("-W- ");
        self.pal.write_str(s);
        self.pal.write_str(esc::FG_DEFAULT);
        self.pal.flush_buff();
    }

    pub fn log_e(&mut self, s: &str) {
        self.pal.write_str(esc::FG_RED);
        self.pal.write_str("-E- ");
        self.pal.write_str(s);
        self.pal.write_str(esc::FG_DEFAULT);
        self.pal.flush_buff();
    }

    pub fn flush_buff(&mut self) {
        self.pal.flush_buff();
    }

    pub fn move_to(&mut self, col: u16, row: u16) {
        let s = String::from(esc::CURSOR_GOTO_FMT)
            .replace("{1}", &col.to_string())
            .replace("{2}", &row.to_string());
        self.pal.write_str(s.as_str());
    }

    pub fn move_to_col(&mut self, col: u16) {
        let s = String::from(esc::CURSOR_COLUMN_FMT)
            .replace("{1}", &col.to_string());
        self.pal.write_str(s.as_str());
    }

    pub fn move_by(&mut self, cols: i16, rows: i16) {
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
                .replace("{1}", &arg.to_string());
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
                .replace("{1}", &arg.to_string());
            self.pal.write_str(s.as_str());
        }
    }

    pub fn move_to_home(&mut self) {
        self.pal.write_str(esc::CURSOR_HOME);
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
            .replace("{1}", &count.to_string());
        self.pal.write_str(s.as_str());
    }

    pub fn delete_lines(&mut self, count: u16) {
        let s = String::from(esc::LINE_DELETE_FMT)
            .replace("{1}", &count.to_string());
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

    pub fn invalidate(&mut self, wnd: &crate::Widget, _wids: &[crate::WId]) {
        if let widget::Type::Window(_w) = wnd.typ {
            // self.invalidated.extend_from_slice(wid);
        } else {
            self.log_w(format!("Widget id {} is not a Window", wnd.id).as_str());
        }
    }

    pub fn draw(&mut self, wnd: &crate::Widget, wids: &[crate::WId]) {
        for id in wids.iter() {
            for w in wnd.link.iter() {
                if w.id == *id {
                    // draw
                }
            }
        }

        // self.invalidated.retain(|x| !wids.contains(x));
    }

    pub fn draw_wnd(&mut self, wnd: &crate::Widget) {
        match wnd.typ {
            _ => {}
        }

        // self.invalidated.clear();
    }
}

pub struct TWins {
    ctx: Mutex<Ctx>,
}

impl TWins {
    pub fn new(p: PalBox) -> TWins {
        TWins {
            ctx: Mutex::new(Ctx { pal: p }),
        }
    }

    pub fn lock(&mut self) -> MutexGuard<Ctx> {
        self.ctx.lock().unwrap()
    }
}
