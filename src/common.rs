//! # RTWins common definitions

use std::ops::{Add, Sub};

use crate::Ctx;

// ---------------------------------------------------------------------------------------------- //

/// Widget coordinates on screen or on parent widget
#[derive(Clone, Copy, Default)]
pub struct Coord {
    pub col: u8,
    pub row: u8,
}

impl Coord {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Coord { col: 0, row: 0 }
    }

    pub const fn new(c: u8, r: u8) -> Self {
        Coord { col: c, row: r }
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Coord) -> Coord {
        Coord {
            col: self.col.saturating_add(other.col),
            row: self.row.saturating_add(other.row),
        }
    }
}

/// Widget size
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

impl Size {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Size {
        Size { width: 0, height: 0 }
    }

    pub const fn new(w: u8, h: u8) -> Self {
        Size { width: w, height: h }
    }
}

impl Sub for Size {
    type Output = Self;
    fn sub(self, other: Size) -> Size {
        Size {
            width:  self.width.saturating_sub(other.width),
            height: self.height.saturating_sub(other.height),
        }
    }
}

/// Rectangle area
#[derive(Clone, Copy)]
pub struct Rect {
    pub coord: Coord,
    pub size: Size,
}

impl Rect {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Rect {
        Rect {
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
        }
    }

    pub fn new(c: u8, r: u8, w: u8, h: u8) -> Rect {
        Rect{coord: Coord::new(c, r), size: Size::new(w, h)}
    }

    pub fn set_max(&mut self) {
        self.coord.col = 0;
        self.coord.row = 0;
        self.size.width = u8::MAX;
        self.size.height = u8::MAX;
    }

    /// Checks if given point at `col:row` is within this rectangle
    pub fn is_point_within(&self, col: u8, row: u8) -> bool {
        col >= self.coord.col &&
        col <  self.coord.col + self.size.width &&
        row >= self.coord.row &&
        row <  self.coord.row + self.size.height
    }

    /// Check if `r` fits within this rectangle
    pub fn is_rect_within(&self, r: &Rect) -> bool {
        r.coord.col                 >= self.coord.col &&
        r.coord.col + r.size.width  <= self.coord.col + self.size.width &&
        r.coord.row                 >= self.coord.row &&
        r.coord.row + r.size.height <= self.coord.row + self.size.height
    }
}

/// Font attributes.
/// Some of them may be combined
#[derive(Clone, Copy)]
pub enum FontAttrib {
    /// Style
    None,
    /// Style
    Bold,
    /// Style
    Faint,
    /// Style
    Italics,
    // Decorator
    Underline,
    // Decorator
    Blink,
    // Decorator
    Inverse,
    // Decorator
    Invisible,
    // Decorator
    StrikeThrough,
}

// ---------------------------------------------------------------------------------------------- //

/// Remembers and restores font attributes on request
pub(crate) struct FontMementoManual {
    fg_stack_len : i8,
    bg_stack_len : i8,
    at_stack_len : i8,
}

impl FontMementoManual {
    pub fn new() -> Self {
        FontMementoManual {
            fg_stack_len: 0, bg_stack_len: 0, at_stack_len: 0
        }
    }

    pub fn from_ctx(ctx: &Ctx) -> Self {
        let mut fm = FontMementoManual {
            fg_stack_len: 0, bg_stack_len: 0, at_stack_len: 0
        };
        fm.store(ctx);
        fm
    }

    pub fn store(&mut self, ctx: &Ctx) {
        self.fg_stack_len = ctx.stack_cl_fg.len() as i8;
        self.bg_stack_len = ctx.stack_cl_bg.len() as i8;
        self.at_stack_len = ctx.stack_attr.len()  as i8;
    }

    pub fn restore(&mut self, ctx: &mut Ctx) {
        ctx.pop_cl_fg_n(ctx.stack_cl_fg.len() as i8 - self.fg_stack_len);
        ctx.pop_cl_bg_n(ctx.stack_cl_bg.len() as i8 - self.bg_stack_len);
        ctx.pop_attr_n(ctx.stack_attr.len()   as i8 - self.at_stack_len);
    }
}

/// Helper for automatic restoring terminal font attributes
// https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_coercion.html
// lifetime of `a` is >= lifetime of `b`
pub(crate) struct FontMemento<'b, 'a: 'b> {
    fg_stack_len : i8,
    bg_stack_len : i8,
    at_stack_len : i8,
    ctx: &'b std::cell::RefCell<&'a mut Ctx>,
}

impl <'b, 'a> FontMemento<'b, 'a> {
    pub fn new(ctx: &'b std::cell::RefCell<&'a mut Ctx>) -> Self {
        let fg;
        let bg;
        let at;

        {
            let ref_ctx = ctx.borrow();
            fg = ref_ctx.stack_cl_fg.len() as i8;
            bg = ref_ctx.stack_cl_bg.len() as i8;
            at = ref_ctx.stack_attr.len()  as i8;
        }

        FontMemento {
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
        let new_at = ctx.stack_attr.len()  as i8 - self.at_stack_len;
        ctx.pop_cl_fg_n(new_fg);
        ctx.pop_cl_bg_n(new_bg);
        ctx.pop_attr_n(new_at);
    }
}

// ---------------------------------------------------------------------------------------------- //

/// Mouse reporting modes
pub enum MouseMode {
    Off,
    M1,
    M2
}
