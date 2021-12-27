//! # RTWins String extensions

use unicode_width::UnicodeWidthStr;
use std::fmt::Write;
use std::ops::Shl;

/// Trait extending base `String` functionality
pub trait StringExt {
    /// Push ANSI escape sequence, replacing `{0}` with the `val`
    fn push_esc_fmt(&mut self, escfmt: &str, val: i16);
    /// Push `repeat` copies of `c`
    fn push_n(&mut self, c: char, n: i16);
    /// Set displayed width to `w` according to Unicode Standard
    fn set_displayed_width(&mut self, w: i16);
    /// Append and return ownself
    fn app(&mut self, s: &str) -> &mut Self;
    /// Returns stream operator wrapper
    fn stream(&mut self) -> StrStreamOp;
}

impl StringExt for String {
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

    fn set_displayed_width(&mut self, w: i16) {
        let disp_width = self.as_str().ansi_displayed_width();
        self.push_n(' ', w - disp_width as i16);
    }

    fn app(&mut self, s: &str) -> &mut Self {
        self.push_str(s);
        self
    }

    fn stream(&mut self) -> StrStreamOp {
        StrStreamOp::new(self)
    }
}


pub trait StrExt {
    /// Returns ANSI escape sequence length, if begins with `\x1B`
    fn ansi_esc_len(&self) -> usize;
    /// Calculate UTF-8 terminal text width, ignoring ESC sequences inside it
    fn ansi_displayed_width(&self) -> usize;
}

impl StrExt for str {
    fn ansi_esc_len(&self) -> usize {
        // ESC sequence always ends with:
        // - A..Z
        // - a..z
        // - @, ^, ~

        // ESC_BG_RGB(r,g,b)    \e[48;2;255;255;255m
        const ESC_MAX_SEQ_LEN: usize = 20;

        if self.starts_with('\x1B') {
            let seq_len = if self.len() < ESC_MAX_SEQ_LEN { self.len() } else { ESC_MAX_SEQ_LEN };
            let mut it = self.as_bytes().iter().skip(1);
            let mut seq_idx = 1;

            while seq_idx < seq_len {
                let c = *it.next().unwrap() as char;

                match c {
                    '@' | '^' | '~' =>
                        return seq_idx + 1,
                    'M' => {
                        if seq_len >= 6 && self.len() >= 6 {
                            return 6;
                        }
                        else {
                            return 0
                        };
                    },
                    _ => {
                        if c >= 'A' && c <= 'Z' && c != 'O' {
                            return seq_idx + 1;
                        }
                        if c >= 'a' && c <= 'z' {
                            return seq_idx + 1;
                        }
                    },
                }

                seq_idx += 1;
            }
        }

        return 0;
    }

    fn ansi_displayed_width(&self) -> usize {
        let mut disp_width = UnicodeWidthStr::width(self) as i32;
        let mut it = self.char_indices();

        loop {
            if let Some((idx, _)) = it.next() {
                let esc_len = self[idx..].ansi_esc_len() as i32;
                // UnicodeWidthStr::width() returns 0 for \e
                if esc_len > 0 {
                    disp_width -= esc_len - 1;
                }
            }
            else {
                break;
            }
        }

        if disp_width > 0 {
            disp_width as usize
        }
        else {
            0
        }
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

