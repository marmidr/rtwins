//! # RTWins Utils

use crate::string_ext::*;

pub type StringListRc = std::rc::Rc<std::cell::RefCell<Vec<String>>>;

/// Splits given string into lines so that each line is not wider than `max_disp_w`.
///
/// Display width is calculated using Unicode data to determine if character is single or double width.
/// Returns Vector of String's (not slices)
pub fn word_wrap(max_disp_w: usize, src: &String) -> StringListRc {
    let it = src.split_inclusive(char::is_whitespace)
        .scan((0usize, 0usize, 0usize), |state, word| {
            let word_w = word.displayed_width();
            // println!("w:'{}', wl:{}", word, word.len());

            if state.0 + word_w > max_disp_w {
                // ready to output the line as the current word would made it too wide
                let outslice = &src[state.1..state.2];
                // println!(">'{}'", outslice);
                state.0 = word_w;       // line_w = word_w
                state.1 = state.2;      // begin = end
                state.2 += word.len();  // end += word.len
                return Some(outslice.to_string());
            }
            else {
                if word.ends_with('\n') {
                    // shorter than possible, but ends with new line
                    state.2 += word.len();  // end += word.len
                    state.2 -= 1;           // do not put '\n' to the output
                    let outslice = &src[state.1..state.2];
                    // println!("<'{}'", outslice);
                    state.0 = 0;
                    state.2 += 1;           // do not put '\n' to the output
                    state.1 = state.2;      // begin = end
                    return Some(outslice.to_string());
                }
                else {
                    // single word not ending with \n
                    state.0 += word_w;      // line_w += word_w
                    state.2 += word.len();  // end += word.len

                    if state.2 == src.len() {
                        // `word` is the last item in a string -> output it
                        // println!("<'{}'", src[state.1..state.2].to_string());
                        Some(word.to_string())
                    }
                    else {
                        // keep iterating -> return Some
                        Some("".to_string())    // empty str -> will be filtered out
                    }
                }
            }
        })
        .filter(|line| !line.is_empty());

    let out = StringListRc::default();
    out.borrow_mut().extend(it);
    out
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
