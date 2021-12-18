//! # RTWins String extensions

use unicode_width::UnicodeWidthStr;
use std::fmt::Write;
use std::ops::Shl;

/// Trait extending base `String` functionality
pub trait StrExt {
    /// Push ANSI escape sequence, replacing `{0}` with the `val`
    fn push_esc_fmt(&mut self, escfmt: &str, val: i16);
    /// Push `repeat` copies of `c`
    fn push_n(&mut self, c: char, n: i16);
    /// Set displayed width to `w` according to Unicode Standard
    fn set_width(&mut self, w: i16);
    /// Append and return ownself
    fn app(&mut self, s: &str) -> &mut Self;
}

impl StrExt for String {
    fn push_esc_fmt(&mut self, escfmt: &str, val: i16) {
        if let Some((a, b)) = escfmt.split_once("{0}") {
            self.write_fmt(format_args!("{}{}{}", a, val, b)).unwrap_or_default();
        }
    }

    fn push_n(&mut self, c: char, repeat: i16) {
        for _ in 0..repeat {
            self.push(c);
        }
    }

    fn set_width(&mut self, w: i16) {
        let n = UnicodeWidthStr::width(self.as_str());
        self.push_n(' ', w - n as i16);
    }

    fn app(&mut self, s: &str) -> &mut Self {
        self.push_str(s);
        self
    }
}


/// Additional operators for `String` type.
/// Works on a reference, as a decorator, so there is no data moved in nor out
///
/// # Examples
///
/// ```
/// use rtwins::string_ext::*;
/// let mut s = String::from("Magic bookstore");
/// let _ = StrOps::new(&mut s)
///     << ": "
///     << "Vol.2 "
///     << "Iroh's Bookstore"
///     ;
/// assert_eq!(s, "Magic bookstore: Vol.2 Iroh's Bookstore");
/// ```
pub struct StrOps<'a> {
    intern: &'a mut String,
}

impl<'a> StrOps<'a> {
    /// Constructs a new decorator
    pub fn new(s: &'a mut String) -> Self {
        StrOps { intern: s }
    }

    /// Destroys decorator returning internal string reference
    #[inline]
    pub fn take(self) -> &'a mut String {
        self.intern
    }
}

/// Implementation of shift left operator `<<`
///
impl<'a> Shl<&str> for StrOps<'a> {
    type Output = StrOps<'a>;

    #[inline]
    fn shl(self, rhs: &str) -> Self::Output {
        let ret = StrOps::new(self.take());
        ret.intern.push_str(rhs);
        ret
    }
}

