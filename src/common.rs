//! # RTWins common definitions

use crate::Term;

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

    /// Creates a new coordinates with column and row set to given `c` and `r`
    pub const fn new(c: u8, r: u8) -> Self {
        Coord { col: c, row: r }
    }
}

impl core::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Coord) -> Coord {
        Coord {
            col: self.col.saturating_add(rhs.col),
            row: self.row.saturating_add(rhs.row),
        }
    }
}

impl core::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.col = self.col.saturating_add(rhs.col);
        self.row = self.row.saturating_add(rhs.row);
    }
}

impl core::ops::Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Coord) -> Coord {
        Coord {
            col: self.col.saturating_sub(rhs.col),
            row: self.row.saturating_sub(rhs.row),
        }
    }
}

impl core::ops::SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.col = self.col.saturating_sub(rhs.col);
        self.row = self.row.saturating_sub(rhs.row);
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
        Size {
            width: 0,
            height: 0,
        }
    }

    /// Creates a new size object with width and height set to `w` and `h`
    pub const fn new(w: u8, h: u8) -> Self {
        Size {
            width: w,
            height: h,
        }
    }
}

impl core::ops::Sub for Size {
    type Output = Self;
    fn sub(self, other: Size) -> Size {
        Size {
            width: self.width.saturating_sub(other.width),
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

    /// Creates a new rect object with coordinates and size set
    // according to `col, row` and `w, h`
    pub fn new(col: u8, row: u8, w: u8, h: u8) -> Rect {
        Rect {
            coord: Coord::new(col, row),
            size: Size::new(w, h),
        }
    }

    pub fn set_max(&mut self) {
        self.coord.col = 0;
        self.coord.row = 0;
        self.size.width = u8::MAX;
        self.size.height = u8::MAX;
    }

    /// Checks if given point at `col:row` is within this rectangle
    pub fn is_point_within(&self, col: u8, row: u8) -> bool {
        col >= self.coord.col
            && col < self.coord.col + self.size.width
            && row >= self.coord.row
            && row < self.coord.row + self.size.height
    }

    /// Check if `r` fits within this rectangle
    pub fn is_rect_within(&self, r: &Rect) -> bool {
        r.coord.col >= self.coord.col
            && r.coord.col + r.size.width <= self.coord.col + self.size.width
            && r.coord.row >= self.coord.row
            && r.coord.row + r.size.height <= self.coord.row + self.size.height
    }
}

/// Font attributes.
/// Some of them may be combined
#[derive(Clone, Copy)]
pub enum FontAttrib {
    /// Styles - only one at a time
    None,
    Bold,
    Faint,
    Italics,
    /// Decorators - may be combined
    Underline,
    Blink,
    Inverse,
    Invisible,
    StrikeThrough,
}

// ---------------------------------------------------------------------------------------------- //

/// Remembers and restores font attributes on request
pub(crate) struct FontMementoManual {
    fg_stack_len: i8,
    bg_stack_len: i8,
    at_stack_len: i8,
}

impl FontMementoManual {
    pub fn new() -> Self {
        FontMementoManual {
            fg_stack_len: 0,
            bg_stack_len: 0,
            at_stack_len: 0,
        }
    }

    pub fn from_term(term: &Term) -> Self {
        let mut fm = FontMementoManual {
            fg_stack_len: 0,
            bg_stack_len: 0,
            at_stack_len: 0,
        };
        fm.store(term);
        fm
    }

    pub fn store(&mut self, term: &Term) {
        self.fg_stack_len = term.stack_cl_fg.len() as i8;
        self.bg_stack_len = term.stack_cl_bg.len() as i8;
        self.at_stack_len = term.stack_attr.len() as i8;
    }

    pub fn restore(&mut self, term: &mut Term) {
        term.pop_cl_fg_n(term.stack_cl_fg.len() as i8 - self.fg_stack_len);
        term.pop_cl_bg_n(term.stack_cl_bg.len() as i8 - self.bg_stack_len);
        term.pop_attr_n(term.stack_attr.len() as i8 - self.at_stack_len);
    }
}

/// Helper for automatic restoring terminal font attributes
// https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_coercion.html
// lifetime of `term` is >= lifetime of `cell`
pub(crate) struct FontMemento<'cell, 'term: 'cell> {
    fg_stack_len: i8,
    bg_stack_len: i8,
    at_stack_len: i8,
    term: &'cell core::cell::RefCell<&'term mut Term>,
}

impl<'cell, 'term> FontMemento<'cell, 'term> {
    pub fn new(term: &'cell core::cell::RefCell<&'term mut Term>) -> Self {
        let fg;
        let bg;
        let at;

        {
            let ref_term = term.borrow();
            fg = ref_term.stack_cl_fg.len() as i8;
            bg = ref_term.stack_cl_bg.len() as i8;
            at = ref_term.stack_attr.len() as i8;
        }

        FontMemento {
            fg_stack_len: fg,
            bg_stack_len: bg,
            at_stack_len: at,
            term,
        }
    }
}

impl<'cell, 'term> Drop for FontMemento<'cell, 'term> {
    fn drop(&mut self) {
        let mut ref_term = self.term.borrow_mut();
        let new_fg = ref_term.stack_cl_fg.len() as i8 - self.fg_stack_len;
        let new_bg = ref_term.stack_cl_bg.len() as i8 - self.bg_stack_len;
        let new_at = ref_term.stack_attr.len() as i8 - self.at_stack_len;
        ref_term.pop_cl_fg_n(new_fg);
        ref_term.pop_cl_bg_n(new_bg);
        ref_term.pop_attr_n(new_at);
    }
}

// ---------------------------------------------------------------------------------------------- //

/// Mouse reporting modes
pub enum MouseMode {
    /// reporting disabled
    Off,
    /// only buttons
    M1,
    /// buttons and wheel
    M2,
}
