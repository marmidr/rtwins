//! # RTWins Utils

use crate::input::*;
use crate::string_ext::*;

use std::fmt::Write;

// ---------------------------------------------------------------------------------------------- //

/// Mimics C++ operator: cond ? a : b;
#[macro_export]
macro_rules! tetrary {
    ($cond: expr, $then: expr, $else: expr) => {
        if $cond {
            $then
        }
        else {
            $else
        }
    };
}

// https://internals.rust-lang.org/t/nicer-static-assertions/15986
// see also: https://crates.io/crates/static_assertions
/// Assertion at compile time
#[macro_export]
macro_rules! static_assert {
    ($cond: expr) => {
        #[allow(dead_code)]
        const _: () = assert!($cond);
    };
}

// ---------------------------------------------------------------------------------------------- //

pub type StringListRc = std::rc::Rc<std::cell::RefCell<Vec<String>>>;

/// Splits given string into lines so that each line is not wider than `max_disp_w`.
///
/// Display width is calculated using Unicode data to determine if character is single or double width.
/// Returns Vector of String's (not slices)
pub fn word_wrap(max_disp_w: usize, src: &str) -> StringListRc {
    let it = src
        .split_inclusive(char::is_whitespace)
        .scan((0usize, 0usize, 0usize), |state, word| {
            let word_w = word.displayed_width();
            // println!("w:'{}', wl:{}", word, word.len());

            if state.0 + word_w > max_disp_w {
                // ready to output the line as the current word would made it too wide
                let outslice = &src[state.1..state.2];
                // println!(">'{}'", outslice);
                state.0 = word_w; // line_w = word_w
                state.1 = state.2; // begin = end
                state.2 += word.len(); // end += word.len
                Some(outslice.to_string())
            }
            else if word.ends_with('\n') {
                // shorter than possible, but ends with new line
                state.2 += word.len(); // end += word.len
                state.2 -= 1; // do not put '\n' to the output
                let outslice = &src[state.1..state.2];
                // println!("<'{}'", outslice);
                state.0 = 0;
                state.2 += 1; // do not put '\n' to the output
                state.1 = state.2; // begin = end
                Some(outslice.to_string())
            }
            else {
                // single word not ending with \n
                state.0 += word_w; // line_w += word_w
                state.2 += word.len(); // end += word.len

                if state.2 == src.len() {
                    // `word` is the last item in a string -> output it
                    // println!("<'{}'", src[state.1..state.2].to_string());
                    Some(word.to_string())
                }
                else {
                    // keep iterating -> return Some
                    Some("".to_string()) // empty str -> will be filtered out
                }
            }
        })
        .filter(|line| !line.is_empty());

    let out = StringListRc::default();
    out.borrow_mut().extend(it);
    out
}

/// Custom handler for text edit - edit integer value
///
/// Returns true if input event was handled and needs no more processing, false otherwise.
pub fn num_edit_input_evt(
    ii: &InputInfo,
    txt: &mut String,
    cursor_pos: &mut i16,
    limit_min: i64,
    limit_max: i64,
    wrap: bool,
) -> bool {
    if ii.kmod.is_empty() {
        // reject non-digits and avoid too long numbers
        // 0x7fffffffffffffff = 9223372036854775807
        if let InputEvent::Char(ref ch) = ii.evnt {
            if ch.utf8seq[0] < b'0' || ch.utf8seq[0] > b'9' || txt.len() >= 19 {
                if let Ok(mut term_guard) = crate::TERM.try_write() {
                    term_guard.write_str(crate::esc::BELL);
                    term_guard.flush_buff();
                }
                else {
                    crate::tr_warn!("Cannot lock TERM");
                }
                return true;
            }
        }
    }

    if let InputEvent::Key(ref key) = ii.evnt {
        match *key {
            Key::Tab => {
                return true;
            }
            Key::Enter => {
                let mut n: i64 = txt.parse().unwrap_or_default();
                n = n.clamp(limit_min, limit_max);
                txt.clear();
                write!(txt, "{n}").unwrap();
                return false;
            }
            Key::Esc => {
                return false;
            }
            Key::Up | Key::Down => {
                let mut n: i64 = txt.parse().unwrap_or_default();
                let mut delta: i64 = if ii.kmod.has_shift() {
                    100
                }
                else if ii.kmod.has_ctrl() {
                    10
                }
                else {
                    1
                };

                if *key == Key::Down {
                    delta = -delta;
                }

                n += delta;

                if n < limit_min {
                    n = if wrap { limit_max } else { limit_min };
                }

                if n > limit_max {
                    n = if wrap { limit_min } else { limit_max };
                }

                txt.clear();
                write!(txt, "{n}").unwrap();
                *cursor_pos = txt.chars().count() as i16;
                return true;
            }
            _ => {}
        }
    }

    false
}

// copied from core::str::validations.rs
// https://tools.ietf.org/html/rfc3629
const UTF8_CHAR_WIDTH: &[u8; 256] = &[
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];

/// Given a first byte, determines how many bytes are in this UTF-8 character.
#[inline]
pub const fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
