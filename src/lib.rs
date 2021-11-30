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
pub use colors::*;

/// Library version
pub const VER: &str = env!("CARGO_PKG_VERSION");

use std::sync::{Mutex, MutexGuard, TryLockResult};

// -----------------------------------------------------------------------------------------------

// rename Tui
pub struct TWins {
    ctx: Mutex<Ctx>,
}

impl TWins {
    pub fn new(p: PalBox) -> TWins {
        TWins {
            ctx: Mutex::new(Ctx::new(p)),
        }
    }

    pub fn lock(&mut self) -> MutexGuard<Ctx> {
        self.ctx.lock().unwrap()
    }

    pub fn try_lock(&mut self) -> TryLockResult<MutexGuard<Ctx>> {
        self.ctx.try_lock()
    }
}

// -----------------------------------------------------------------------------------------------

pub type PalBox = Box<dyn crate::pal::Pal>;

// TODO: static Pal instead of PalBox
// pub struct Ctx<P: crate::pal::Pal>

pub struct Ctx {
    pub pal: PalBox,

    invalidated: Vec<WId>,
    current_cl_fg: ColorFG,
    current_cl_bg: ColorBG,
    attr_faint: i8,
    _log_raw_font_memento: FontMementoManual,
    pub(crate) stack_cl_fg: Vec<ColorFG>,
    pub(crate) stack_cl_bg: Vec<ColorBG>,
    pub(crate) stack_attr: Vec<FontAttrib>,
}

///
impl Ctx {
    ///
    pub fn new(p: PalBox) -> Self {
        Ctx{
            pal: p,
            // stat: Box::new(WindowStateStub::new()),
            invalidated: vec![],
            current_cl_fg: ColorFG::Default,
            current_cl_bg: ColorBG::Default,
            attr_faint: 0,
            _log_raw_font_memento: FontMementoManual::new(),
            stack_cl_fg: vec![],
            stack_cl_bg: vec![],
            stack_attr: vec![],
        }
    }

    ///
    pub fn write_char(&mut self, c: char) -> &mut Self {
        self.pal.write_char(c);
        self
    }

    ///
    pub fn write_char_n(&mut self, c: char, repeat: i16) -> &mut Self {
        self.pal.write_char_n(c, repeat);
        self
    }

    ///
    pub fn write_str(&mut self, s: &str) -> &mut Self {
        self.pal.write_str(s);
        self
    }

    ///
    pub fn write_str_n(&mut self, s: &str, repeat: i16) -> &mut Self {
        self.pal.write_str_n(s, repeat);
        self
    }

    ///
    pub fn flush_buff(&mut self) {
        self.pal.flush_buff();
    }

    // Logs

    ///
    fn log(&mut self, fg: &str, prefix: &str, msg: &str) {
        self.pal.write_str(fg);
        self.pal.write_str(prefix);
        self.pal.write_str(msg);
        self.pal.write_str(esc::FG_DEFAULT);
        self.pal.flush_buff();
    }

    ///
    pub fn log_d(&mut self, msg: &str) {
        self.log(esc::FG_BLACK_INTENSE, "-D- ", msg);
    }

    ///
    pub fn log_i(&mut self, msg: &str) {
        self.log(esc::FG_WHITE, "-I- ", msg);
    }

    ///
    pub fn log_w(&mut self, msg: &str) {
        self.log(esc::FG_YELLOW, "-W- ", msg);
    }

    ///
    pub fn log_e(&mut self, msg: &str) {
        self.log(esc::FG_RED, "-E- ", msg);
    }

    // Cursor manipulation

    ///
    pub fn move_to(&mut self, col: u16, row: u16) -> &mut Self {
        let s = String::from(esc::CURSOR_GOTO_FMT)
            .replace("{0}", &col.to_string())
            .replace("{1}", &row.to_string());
        self.pal.write_str(s.as_str());
        self
    }

    ///
    pub fn move_to_col(&mut self, col: u16) -> &mut Self {
        let s = String::from(esc::CURSOR_COLUMN_FMT)
            .replace("{0}", &col.to_string());
        self.pal.write_str(s.as_str());
        self
    }

    ///
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

    ///
    pub fn move_to_home(&mut self) -> &mut Self {
        self.pal.write_str(esc::CURSOR_HOME);
        self
    }

    ///
    pub fn cursor_save_pos(&mut self) {
        self.pal.write_str(esc::CURSOR_POS_SAVE);
    }

    ///
    pub fn cursor_restore_pos(&mut self) {
        self.pal.write_str(esc::CURSOR_POS_RESTORE);
    }

    ///
    pub fn cursor_hide(&mut self) {
        self.pal.write_str(esc::CURSOR_HIDE);
    }

    ///
    pub fn cursor_show(&mut self) {
        self.pal.write_str(esc::CURSOR_SHOW);
    }

    // Lines manipulation

    ///
    pub fn insert_lines(&mut self, count: u16) {
        let s = String::from(esc::LINE_INSERT_FMT)
            .replace("{0}", &count.to_string());
        self.pal.write_str(s.as_str());
    }

    ///
    pub fn delete_lines(&mut self, count: u16) {
        let s = String::from(esc::LINE_DELETE_FMT)
            .replace("{0}", &count.to_string());
        self.pal.write_str(s.as_str());
    }

    // Screen manipulation

    ///
    pub fn screen_clr_above(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_ABOVE);
    }

    ///
    pub fn screen_clr_below(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_BELOW);
    }

    ///
    pub fn screen_clr_all(&mut self) {
        self.pal.write_str(esc::SCREEN_ERASE_ALL);
    }

    ///
    pub fn screen_save(&mut self) {
        self.pal.write_str(esc::SCREEN_SAVE);
    }

    ///
    pub fn screen_restore(&mut self) {
        self.pal.write_str(esc::SCREEN_RESTORE);
    }

    // Foreground color stack

    ///
    pub fn push_cl_fg(&mut self, cl: ColorFG) {
        self.stack_cl_fg.push(self.current_cl_fg);
        self.current_cl_fg = cl;
        self.write_str(encode_cl_fg(self.current_cl_fg));
    }

    ///
    pub fn pop_cl_fg_n(&mut self, mut n: i8) {
        while !self.stack_cl_fg.is_empty() && n > 0 {
            self.current_cl_fg = self.stack_cl_fg.pop().unwrap();
            n -= 1;
        }

        self.write_str(encode_cl_fg(self.current_cl_fg));
    }

    ///
    pub fn pop_cl_fg(&mut self) {
        self.pop_cl_fg_n(1);
    }

    ///
    pub fn reset_cl_fg(&mut self) {
        self.stack_cl_fg.clear();
        self.write_str(esc::FG_DEFAULT);
    }

    // Background color stack

    ///
    pub fn push_cl_bg(&mut self, cl: ColorBG) {
        self.stack_cl_bg.push(self.current_cl_bg);
        self.current_cl_bg = cl;
        self.write_str(encode_cl_bg(self.current_cl_bg));
    }

    ///
    pub fn pop_cl_bg_n(&mut self, mut n: i8) {
        while !self.stack_cl_bg.is_empty() && n > 0 {
            self.current_cl_bg = self.stack_cl_bg.pop().unwrap();
            n -= 1;
        }

        self.write_str(encode_cl_bg(self.current_cl_bg));
    }

    ///
    pub fn pop_cl_bg(&mut self) {
        self.pop_cl_bg_n(1);
    }

    ///
    pub fn reset_cl_bg(&mut self) {
        self.stack_cl_bg.clear();
        self.write_str(esc::BG_DEFAULT);
    }

    // Font attributes stack

    pub fn push_attr(&mut self, attr: FontAttrib) {
        self.stack_attr.push(attr);

        match attr {
            FontAttrib::Bold =>         { if self.attr_faint == 0 { self.write_str(esc::BOLD); }},
            FontAttrib::Faint =>        { self.attr_faint += 1; self.write_str(esc::FAINT); },
            FontAttrib::Italics =>      { self.write_str(esc::ITALICS_ON); },
            FontAttrib::Underline =>    { self.write_str(esc::UNDERLINE_ON); },
            FontAttrib::Blink =>        { self.write_str(esc::BLINK); },
            FontAttrib::Inverse =>      { self.write_str(esc::INVERSE_ON); },
            FontAttrib::Invisible =>    { self.write_str(esc::INVISIBLE_ON); },
            FontAttrib::StrikeThrough => { self.write_str(esc::STRIKETHROUGH_ON); },
            _  => {}
        }
    }

    ///
    pub fn pop_attr_n(&mut self, mut n: i8) {
        while !self.stack_attr.is_empty() && n > 0 {
            let attr = self.stack_attr.pop().unwrap();

            match attr {
                FontAttrib::Bold =>         { if self.attr_faint == 0 { self.write_str(esc::NORMAL); }},
                FontAttrib::Faint =>        { if self.attr_faint > 0 { self.attr_faint -= 1; self.write_str(esc::NORMAL); }},
                FontAttrib::Italics =>      { self.write_str(esc::ITALICS_OFF);  },
                FontAttrib::Underline =>    { self.write_str(esc::UNDERLINE_OFF);  },
                FontAttrib::Blink =>        { self.write_str(esc::BLINK_OFF);  },
                FontAttrib::Inverse =>      { self.write_str(esc::INVERSE_OFF);  },
                FontAttrib::Invisible =>    { self.write_str(esc::INVISIBLE_OFF);  },
                FontAttrib::StrikeThrough => { self.write_str(esc::STRIKETHROUGH_OFF);  },
                _  => {}
            }
            n -= 1;
        }
    }

    ///
    pub fn pop_attr(&mut self) {
        self.pop_attr_n(1);
    }

    ///
    pub fn reset_attr(&mut self) {
        self.attr_faint = 0;
        self.stack_attr.clear();
        self.write_str(esc::ATTRIBUTES_DEFAULT);
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

    pub fn draw_wnd(&mut self, ws: &mut dyn crate::WindowState) {
        widget_draw::draw_widgets(self, ws, &[WIDGET_ID_ALL]);
        self.invalidated.clear();
    }
}


// -----------------------------------------------------------------------------------------------

/// Font attributes
#[derive(Clone, Copy)]
pub enum FontAttrib {
    None,
    Bold,
    Faint,
    Italics,
    Underline,
    Blink,
    Inverse,
    Invisible,
    StrikeThrough,
}

#[allow(unused_variables)]
struct FontMementoManual {
    sz_fg : i8,
    sz_bg : i8,
    sz_attr : i8,
}

#[allow(dead_code)]
impl FontMementoManual {
    fn new() -> Self {
        FontMementoManual {
            sz_fg: 0, sz_bg: 0, sz_attr: 0
        }
    }

    fn store(&mut self, ctx: &Ctx) {
        self.sz_fg = ctx.stack_cl_fg.len() as i8;
        self.sz_bg = ctx.stack_cl_bg.len() as i8;
        self.sz_attr = ctx.stack_attr.len() as i8;
    }

    fn restore(&mut self, ctx: &mut Ctx) {
        ctx.pop_cl_fg_n(ctx.stack_cl_fg.len() as i8 - self.sz_fg);
        ctx.pop_cl_bg_n(ctx.stack_cl_bg.len() as i8 - self.sz_bg);
        ctx.pop_attr_n(ctx.stack_cl_bg.len() as i8 - self.sz_attr);
    }
}

/// Helper for automatic restoring terminal font attributes
// https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_coercion.html
// lifetime of `a` is >= lifetime of `b`
struct FontMemento<'b, 'a: 'b> {
    fg_stack_len : i8,
    bg_stack_len : i8,
    at_stack_len : i8,
    ctx: &'b std::cell::RefCell<&'a mut Ctx>,
}

#[allow(dead_code)]
impl <'b, 'a> FontMemento<'b, 'a> {
    fn new(ctx: &'b std::cell::RefCell<&'a mut Ctx>) -> Self {
        let fg;
        let bg;
        let at;

        {
            let ref_ctx = ctx.borrow();
            fg = ref_ctx.stack_cl_fg.len() as i8;
            bg = ref_ctx.stack_cl_bg.len() as i8;
            at = ref_ctx.stack_attr.len() as i8;
        }

        FontMemento{
            fg_stack_len: fg,
            bg_stack_len: bg,
            at_stack_len: at,
            ctx
        }
    }
}

impl <'b, 'a> Drop for FontMemento<'b, 'a> {
    fn drop(&mut self) {
        let mut ctx = self.ctx.borrow_mut();
        let new_fg = ctx.stack_cl_fg.len() as i8 - self.fg_stack_len;
        let new_bg = ctx.stack_cl_bg.len() as i8 - self.bg_stack_len;
        let new_at = ctx.stack_cl_bg.len() as i8 - self.at_stack_len;
        ctx.pop_cl_fg_n(new_fg);
        ctx.pop_cl_bg_n(new_bg);
        ctx.pop_attr_n(new_at);
    }
}
