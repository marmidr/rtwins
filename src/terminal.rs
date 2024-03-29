//! # RTWins Context definition

use crate::colors::*;
use crate::common::*;
use crate::esc;
use crate::wgt;
use crate::widget_def::*;

use atomic_once_cell::AtomicLazy;
use try_lock::TryLock;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------------------------- //

// https://github.com/rust-lang-nursery/lazy-static.rs/issues/169
pub type PalBox = Box<dyn crate::pal::Pal + Send + Sync>;

/// Global instance of Terminal
pub static TERM: AtomicLazy<TryLock<Term>> = AtomicLazy::new(|| TryLock::new(Term::default()));

/// Terminal low level API and context
///
pub struct Term {
    pub pal: PalBox,
    pub trace_row: u16,
    current_cl_fg: ColorFg,
    current_cl_bg: ColorBg,
    attr_faint: i8,
    _log_raw_font_memento: FontMementoManual,
    invalidated: Vec<WId>,
    pub(crate) stack_cl_fg: Vec<ColorFg>,
    pub(crate) stack_cl_bg: Vec<ColorBg>,
    pub(crate) stack_attr: Vec<FontAttrib>,
}

// pub type TermGuard = RwLockWriteGuard<'static, Term>;

impl Default for Term {
    fn default() -> Self {
        Term {
            pal: Box::<crate::pal::PalStub>::default(),
            trace_row: 0,
            current_cl_fg: ColorFg::Default,
            current_cl_bg: ColorBg::Default,
            attr_faint: 0,
            _log_raw_font_memento: FontMementoManual::new(),
            invalidated: Vec::with_capacity(4),
            stack_cl_fg: vec![],
            stack_cl_bg: vec![],
            stack_attr: vec![],
        }
    }
}

impl Term {
    /// Write single character
    pub fn write_char(&mut self, c: char) -> &mut Self {
        self.pal.write_char(c);
        self
    }

    /// Write character multiple times
    pub fn write_char_n(&mut self, c: char, repeat: i16) -> &mut Self {
        self.pal.write_char_n(c, repeat);
        self
    }

    /// Write single string
    pub fn write_str(&mut self, s: &str) -> &mut Self {
        if self.is_faint() {
            // search for BOLD and NORNAL attributes and skip them,
            // to preserve FAINT mode, used for disabled widgets
            let mut it = s.char_indices();

            while let Some((idx, c)) = it.next() {
                if c == esc::ESC {
                    let mut advance_by = 0;

                    if s[idx..].starts_with(esc::BOLD) {
                        // skip BOLD sequence
                        advance_by = esc::BOLD.len() - 1;
                    }
                    else if s[idx..].starts_with(esc::NORMAL) {
                        // skip NORMAL sequence
                        advance_by = esc::NORMAL.len() - 1;
                    }

                    if advance_by > 0 {
                        while advance_by > 0 && it.next().is_some() {
                            advance_by -= 1;
                        }
                        continue;
                    }
                }

                self.write_char(c);
            }
        }
        else {
            self.pal.write_str(s);
        }

        self
    }

    /// Write string multiple times
    pub fn write_str_n(&mut self, s: &str, repeat: i16) -> &mut Self {
        self.pal.write_str_n(s, repeat);
        self
    }

    /// Flush buffer to the terminal (depends on PAL)
    pub fn flush_buff(&mut self) {
        self.pal.flush_buff();
    }

    // Used by tr_ macros
    pub fn trace_message(&mut self, fg_color: &str, time_str: &str, prefix: &str, msg: &str) {
        self.pal.flush_buff();
        self.cursor_save_pos();
        self.move_to(0, self.trace_row);
        self.push_cl_bg(ColorBg::Default);
        self.pal.write_str(esc::FG_DEFAULT);
        self.insert_lines(1);

        self.pal.mark_logging(true);
        self.pal.write_str(time_str);
        self.pal.write_str(fg_color);
        self.pal.write_str(prefix);
        self.pal.write_str(msg);
        self.pal.write_str(esc::FG_DEFAULT);
        self.pal.mark_logging(false);

        self.pop_cl_bg();
        self.pal.write_char('\n');
        self.cursor_restore_pos();
        self.pal.flush_buff();
    }

    /// Clear logs
    pub fn trace_area_clear(&mut self) {
        self.cursor_save_pos();
        self.move_to(0, self.trace_row);
        self.screen_clr_below();
        self.cursor_restore_pos();
    }

    // Cursor manipulation

    /// Move cursor to given `col`:`row`
    pub fn move_to(&mut self, col: u16, row: u16) -> &mut Self {
        let s = String::from(esc::CURSOR_GOTO_FMT)
            .replace("{0}", &row.to_string())
            .replace("{1}", &col.to_string());
        self.pal.write_str(s.as_str());
        self
    }

    /// Set cursor at column `col`
    pub fn move_to_col(&mut self, col: u16) -> &mut Self {
        let s = String::from(esc::CURSOR_COLUMN_FMT).replace("{0}", &col.to_string());
        self.pal.write_str(s.as_str());
        self
    }

    /// Move cursor by given offsets
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

            let s = String::from(fmt).replace("{0}", &arg.to_string());
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

            let s = String::from(fmt).replace("{0}", &arg.to_string());
            self.pal.write_str(s.as_str());
        }

        self
    }

    /// Move cursor to Home position (1:1)
    pub fn move_to_home(&mut self) -> &mut Self {
        self.pal.write_str(esc::CURSOR_HOME);
        self
    }

    /// Tell the terminal to remember cursor position
    pub fn cursor_save_pos(&mut self) {
        self.pal.write_str(esc::CURSOR_POS_SAVE);
    }

    /// Tell the terminal to restore cursor position
    pub fn cursor_restore_pos(&mut self) {
        self.pal.write_str(esc::CURSOR_POS_RESTORE);
    }

    /// Hide cursor
    pub fn cursor_hide(&mut self) {
        self.pal.write_str(esc::CURSOR_HIDE);
    }

    /// Show cursor
    pub fn cursor_show(&mut self) {
        self.pal.write_str(esc::CURSOR_SHOW);
    }

    // Lines manipulation

    /// Insert empty lines at current cursor row
    pub fn insert_lines(&mut self, count: u16) {
        let s = String::from(esc::LINE_INSERT_FMT).replace("{0}", &count.to_string());
        self.pal.write_str(s.as_str());
    }

    /// Delete lines starting at current cursor row
    pub fn delete_lines(&mut self, count: u16) {
        let s = String::from(esc::LINE_DELETE_FMT).replace("{0}", &count.to_string());
        self.pal.write_str(s.as_str());
    }

    // Screen manipulation

    /// Clear screan above the current cursor row
    pub fn screen_clr_above(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_ABOVE);
    }

    /// Clear screan below the current cursor row
    pub fn screen_clr_below(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_BELOW);
    }

    /// Clear the whole screan
    pub fn screen_clr_all(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_ALL);
    }

    /// Tell the terminal to remember screen content
    pub fn screen_save(&mut self) {
        self.pal.write_str(esc::SCREEN_SAVE);
    }

    /// Tell the terminal to restore screen content
    pub fn screen_restore(&mut self) {
        self.pal.write_str(esc::SCREEN_RESTORE);
    }

    // Foreground color stack

    /// Set new foreground color, put current color on stack
    pub fn push_cl_fg(&mut self, cl: ColorFg) {
        self.stack_cl_fg.push(self.current_cl_fg);
        self.current_cl_fg = cl;
        self.write_str(self.current_cl_fg.encode());
    }

    /// Restore current-n foreground color from the stack
    pub fn pop_cl_fg_n(&mut self, mut n: i8) {
        while !self.stack_cl_fg.is_empty() && n > 0 {
            self.current_cl_fg = self.stack_cl_fg.pop().unwrap();
            n -= 1;
        }

        self.write_str(self.current_cl_fg.encode());
    }

    /// Restore previous foreground color
    #[inline]
    pub fn pop_cl_fg(&mut self) {
        self.pop_cl_fg_n(1);
    }

    /// Reset foreground color stack, set to the DEFAULT
    pub fn reset_cl_fg(&mut self) {
        self.stack_cl_fg.clear();
        self.write_str(esc::FG_DEFAULT);
    }

    // Background color stack

    /// Set new background color, put current color on stack
    pub fn push_cl_bg(&mut self, cl: ColorBg) {
        self.stack_cl_bg.push(self.current_cl_bg);
        self.current_cl_bg = cl;
        self.write_str(self.current_cl_bg.encode());
    }

    /// Restore current-n background color from the stack
    pub fn pop_cl_bg_n(&mut self, mut n: i8) {
        while !self.stack_cl_bg.is_empty() && n > 0 {
            self.current_cl_bg = self.stack_cl_bg.pop().unwrap();
            n -= 1;
        }

        self.write_str(self.current_cl_bg.encode());
    }

    /// Restore previous background color
    #[inline]
    pub fn pop_cl_bg(&mut self) {
        self.pop_cl_bg_n(1);
    }

    /// Reset background color stack, set to the DEFAULT
    pub fn reset_cl_bg(&mut self) {
        self.stack_cl_bg.clear();
        self.write_str(esc::BG_DEFAULT);
    }

    // Font attributes stack

    /// Set new font attribute, remember it on stack
    pub fn push_attr(&mut self, attr: FontAttrib) {
        self.stack_attr.push(attr);

        match attr {
            FontAttrib::Bold => {
                if self.attr_faint == 0 {
                    self.write_str(esc::BOLD);
                }
            }
            FontAttrib::Faint => {
                self.attr_faint += 1;
                self.write_str(esc::FAINT);
            }
            FontAttrib::Italics => {
                self.write_str(esc::ITALICS_ON);
            }
            FontAttrib::Underline => {
                self.write_str(esc::UNDERLINE_ON);
            }
            FontAttrib::Blink => {
                self.write_str(esc::BLINK);
            }
            FontAttrib::Inverse => {
                self.write_str(esc::INVERSE_ON);
            }
            FontAttrib::Invisible => {
                self.write_str(esc::INVISIBLE_ON);
            }
            FontAttrib::StrikeThrough => {
                self.write_str(esc::STRIKETHROUGH_ON);
            }
            _ => {}
        }
    }

    /// Restore current-n font attribute
    pub fn pop_attr_n(&mut self, mut n: i8) {
        while !self.stack_attr.is_empty() && n > 0 {
            let attr = self.stack_attr.pop().unwrap();

            match attr {
                FontAttrib::Bold => {
                    if self.attr_faint == 0 {
                        self.write_str(esc::NORMAL);
                    }
                }
                FontAttrib::Faint => {
                    if self.attr_faint > 0 {
                        self.attr_faint -= 1;
                        if self.attr_faint == 0 {
                            self.write_str(esc::NORMAL);
                        }
                    }
                }
                FontAttrib::Italics => {
                    self.write_str(esc::ITALICS_OFF);
                }
                FontAttrib::Underline => {
                    self.write_str(esc::UNDERLINE_OFF);
                }
                FontAttrib::Blink => {
                    self.write_str(esc::BLINK_OFF);
                }
                FontAttrib::Inverse => {
                    self.write_str(esc::INVERSE_OFF);
                }
                FontAttrib::Invisible => {
                    self.write_str(esc::INVISIBLE_OFF);
                }
                FontAttrib::StrikeThrough => {
                    self.write_str(esc::STRIKETHROUGH_OFF);
                }
                _ => {}
            }
            n -= 1;
        }
    }

    /// Restore previous font attribute
    #[inline]
    pub fn pop_attr(&mut self) {
        self.pop_attr_n(1);
    }

    /// Reset font attribute stack, resest terminal font attributes
    pub fn reset_attr(&mut self) {
        self.attr_faint = 0;
        self.stack_attr.clear();
        self.write_str(esc::ATTRIBUTES_DEFAULT);
    }

    /// Returns attributes stack size
    #[inline]
    pub fn attr_stack_len(&self) -> usize {
        self.stack_attr.len()
    }

    /// Returns true if faint style is active
    #[inline]
    pub fn is_faint(&self) -> bool {
        self.attr_faint > 0
    }

    // -----------------

    /// Mouse reporting
    pub fn mouse_mode(&mut self, mode: MouseMode) {
        match mode {
            MouseMode::Off => {
                self.write_str(esc::MOUSE_REPORTING_M1_OFF);
                self.write_str(esc::MOUSE_REPORTING_M2_OFF);
            }
            MouseMode::M1 => {
                self.write_str(esc::MOUSE_REPORTING_M1_ON);
            }
            MouseMode::M2 => {
                self.write_str(esc::MOUSE_REPORTING_M2_ON);
            }
        }
    }

    // -----------------

    /// Draw given widgets; flushes the buffer
    pub fn draw(&mut self, ws: &mut dyn WindowState, wids: &[WId]) {
        wgt::draw_widgets(self, ws, wids);
    }

    /// Draw widgets marked as invalidated; flushes the buffer.
    /// Clears the invalidated widgets list
    pub fn draw_invalidated(&mut self, ws: &mut dyn WindowState) {
        self.invalidated.clear();
        // this functions performs internal swap of the passed vector with its own vector
        ws.get_invalidated(&mut self.invalidated);

        if !self.invalidated.is_empty() {
            // move out the vector, to avoid passing of &mut self + &self to the draw_widgets()
            let invalidated = core::mem::take(&mut self.invalidated);
            wgt::draw_widgets(self, ws, &invalidated[..]);
            // restore the vector
            self.invalidated = invalidated;
        }
    }

    /// Draw entire window; flushes the buffer
    pub fn draw_wnd(&mut self, ws: &mut dyn WindowState) {
        wgt::draw_widgets(self, ws, &[WIDGET_ID_ALL]);
        ws.clear_invalidated();
    }
}
