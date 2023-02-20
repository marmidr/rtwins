//! # RTWins String extensions

use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};
use std::fmt::Write;
use std::ops::Shl;

/// Trait extending base `String` functionality
pub trait StringExt {
    /// Push ANSI escape sequence, replacing `{0}` with the `val`
    fn push_esc_fmt(&mut self, escfmt: &str, val: i16);
    /// Push `repeat` copies of `c`
    fn push_n(&mut self, c: char, n: i16);
    /// Set displayed width to `expected_disp_w` according to Unicode Standard
    fn set_displayed_width(&mut self, expected_disp_w: i16);
    /// Append and return mutable reference to itself
    fn append(&mut self, s: &str) -> &mut Self;
    /// Returns stream operator wrapper
    fn stream(&mut self) -> StrStreamOp;
    /// Erase characters (not byte, like `remove()`) range; Never panics
    fn erase_range(&mut self, ch_from: usize, len: usize);
    /// Remove trailing text, starting at given char (not byte) index
    fn trim_at(&mut self, ch_idx: usize);
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

    fn set_displayed_width(&mut self, expected_disp_w: i16) {
        let disp_width = self.as_str().displayed_width();
        let n = expected_disp_w - disp_width as i16;

        if n > 0 {
            // too narrow -> append spaces
            self.push_n(' ', n);
        }
        else if n < 0 {
            // too wide -> truncate + ellipsis
            let mut sum_w = 0usize;
            for (i, c) in self.char_indices() {
                if let Some(cw) = c.width() {
                    sum_w += cw;
                }

                if sum_w >= expected_disp_w as usize {
                    self.truncate(i);
                    if expected_disp_w > 0 {
                        self.push('…');
                    }
                    break;
                }
            }
        }
    }

    fn append(&mut self, s: &str) -> &mut Self {
        self.push_str(s);
        self
    }

    fn stream(&mut self) -> StrStreamOp {
        StrStreamOp::new(self)
    }

    fn erase_range(&mut self, ch_from: usize, len: usize) {
        let mut ch_idx = 0;

        for (byte_idx, _) in self.char_indices() {
            if ch_idx == ch_from {
                (0..len).for_each(|_| {
                    if byte_idx < self.len() {
                        self.remove(byte_idx);
                    }
                });
                break;
            }
            ch_idx += 1;
        }
    }

    fn trim_at(&mut self, ch_from: usize) {
        let mut ch_idx = 0;

        for (byte_idx, _) in self.char_indices() {
            if ch_idx == ch_from {
                self.truncate(byte_idx);
                break;
            }
            ch_idx += 1;
        }
    }
}


pub trait StrExt {
    /// Returns ANSI escape sequence length, if begins with `\x1B`
    fn esc_seq_len(&self) -> usize;
    /// Calculate UTF-8 terminal text width, ignoring ESC sequences inside it
    fn displayed_width(&self) -> usize;
}

impl StrExt for str {
    fn esc_seq_len(&self) -> usize {
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
                        if ('A'..='Z').contains(&c) && c != 'O' {
                            return seq_idx + 1;
                        }
                        if ('a'..='z').contains(&c) {
                            return seq_idx + 1;
                        }
                    },
                }

                seq_idx += 1;
            }
        }

        0
    }

    fn displayed_width(&self) -> usize {
        let mut disp_width = UnicodeWidthStr::width(self) as i32;
        let mut it = self.char_indices();

        #[allow(clippy::while_let_on_iterator)]
        while let Some((idx, _)) = it.next() {
            let esc_len = self[idx..].esc_seq_len() as i32;
            if esc_len > 0 {
                // UnicodeWidthStr::width() returns 0 for \e, thus, esc_len decreased by 1
                disp_width -= esc_len - 1;
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

    // return nonmutable internal string
    pub fn as_string(&'a self) -> &'a String {
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

impl<'a> Shl<char> for StrStreamOp<'a> {
    type Output = StrStreamOp<'a>;

    #[inline]
    fn shl(self, rhs: char) -> Self::Output {
        self.intern.push(rhs);
        self
    }
}

impl<'a> Shl<(char, i16)> for StrStreamOp<'a> {
    type Output = StrStreamOp<'a>;

    #[inline]
    fn shl(self, rhs: (char, i16)) -> Self::Output {
        self.intern.push_n(rhs.0, rhs.1);
        self
    }
}
