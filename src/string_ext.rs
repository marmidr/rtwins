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
    /// Returns stream operator wrapper
    fn stream(&mut self) -> StrStreamOp;
}

impl StrExt for String {
    fn push_esc_fmt(&mut self, escfmt: &str, val: i16) {
        if let Some((a, b)) = escfmt.split_once("{0}") {
            self.write_fmt(format_args!("{}{}{}", a, val, b)).unwrap_or_default();
        }
        else {
            self.write_str(escfmt).unwrap_or_default();
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

    fn stream(&mut self) -> StrStreamOp {
        StrStreamOp::new(self)
    }
}


/// Returns ANSI escape sequence length, if begins with `\x1B`
pub trait AnsiEsc {
    ///
    fn ansi_esc_len(&self) -> usize;
}

impl AnsiEsc for str {
    fn ansi_esc_len(&self) -> usize {
        // ESC sequence always ends with:
        // - A..Z
        // - a..z
        // - @, ^, ~

        // ESC_BG_RGB(r,g,b)    \e[48;2;255;255;255m
        const ESC_MAX_SEQ_LEN: usize = 20;

        if self.starts_with('\x1B') {
            let mut sl = 1;
            let max_sl = if self.len() < ESC_MAX_SEQ_LEN { self.len() } else { ESC_MAX_SEQ_LEN };
            let mut it = self.as_bytes().iter().skip(1);

            while sl < max_sl {
                let c = *it.next().unwrap() as char;

                match c {
                    '@' | '^' | '~' =>
                        return sl + 1,
                    'M' => {
                        if max_sl >= 6 && self.len() >= 6 {
                            return 6;
                        }
                        else {
                            return 0
                        };
                    },
                    _ => {
                        if c >= 'A' && c <= 'Z' && c != 'O' {
                            return sl + 1;
                        }
                        if c >= 'a' && c <= 'z' {
                            return sl + 1;
                        }
                    },
                }

                sl += 1;
            }
        }

        return 0;
    }
}

/// C++ like stream operator for `String` type.
/// Works on a reference, as a decorator, so there is no data moved in nor out
///
/// # Examples
///
/// ```
/// use rtwins::string_ext::*;
/// let mut s = String::from("Magic bookstore");
/// let _ = s.stream()
///     << ": "
///     << "Vol.2 "
///     << "Iroh's Bookstore"
///     ;
/// assert_eq!(s, "Magic bookstore: Vol.2 Iroh's Bookstore");
/// ```
pub struct StrStreamOp<'a> {
    intern: &'a mut String,
}

impl<'a> StrStreamOp<'a> {
    /// Constructs a new decorator
    #[inline]
    pub fn new(s: &'a mut String) -> Self {
        StrStreamOp { intern: s }
    }

    /// Destroys decorator returning internal string reference
    #[inline]
    pub fn take(self) -> &'a mut String {
        self.intern
    }
}

/// Implementation of shift left operator `<<`
///
impl<'a> Shl<&str> for StrStreamOp<'a> {
    type Output = StrStreamOp<'a>;

    #[inline]
    fn shl(self, rhs: &str) -> Self::Output {
        self.intern.push_str(rhs);
        self
    }
}

