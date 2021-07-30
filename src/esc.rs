//! *ANSI ESC codes*
//!
//! See more: https://en.m.wikipedia.org/wiki/ANSI_escape_code
//!

// https://doc.rust-lang.org/reference/macros-by-example.html

/// Control Sequence Introducer
#[macro_export]
macro_rules! csi {
    () => {
        ""
    };

    ($elem:expr) => {
        concat!("\x1B[", $elem)
    };
}

/// Operating System Command
#[macro_export]
#[allow(unused_macros)]
macro_rules! osc {
    () => {
        ""
    };

    ($elem:expr) => {
        concat!("\x1B]", $elem)
    };
}

/// String Terminator
#[macro_export]
#[allow(unused_macros)]
macro_rules! st {
    () => {
        ""
    };

    ($elem:expr) => {
        concat!("\x1B\\", $elem)
    };
}

//*******************************************************************************
/// Text Display Modifier Escape Sequences

pub const BOLD : & str =                csi!("1m");
pub const FAINT : & str =               csi!("2m");
pub const NORMAL : & str =              csi!("22m");

// if not italics, may be same as inverse
pub const ITALICS_ON : & str =          csi!("3m");
pub const ITALICS_OFF : & str =         csi!("23m");

pub const UNDERLINE_ON : & str =        csi!("4m");
pub const UNDERLINE_OFF : & str =       csi!("24m");

pub const OVERLINE_ON : & str =         csi!("53m");
pub const OVERLINE_OFF : & str =        csi!("55m");

// if not blinks, the bg color may be lighter
pub const BLINK : & str =               csi!("5m");
pub const BLINK_OFF : & str =           csi!("25m");

pub const INVERSE_ON : & str =          csi!("7m");
pub const INVERSE_OFF : & str =         csi!("27m");

pub const INVISIBLE_ON : & str =        csi!("8m");
pub const INVISIBLE_OFF : & str =       csi!("28m");

pub const STRIKETHROUGH_ON : & str =    csi!("9m");
pub const STRIKETHROUGH_OFF : & str =   csi!("29m");

pub const ATTRIBUTES_DEFAULT : & str =  csi!("10;22;23;24;25;27;28;29m");

pub const FONT_DEFAULT : & str =        csi!("10m");
pub const FONT_1 : & str =              csi!("11m");
pub const FONT_2 : & str =              csi!("12m");
pub const FONT_3 : & str =              csi!("13m");
pub const FONT_4 : & str =              csi!("14m");
pub const FONT_5 : & str =              csi!("15m");
pub const FONT_6 : & str =              csi!("16m");
pub const FONT_7 : & str =              csi!("17m");
pub const FONT_8 : & str =              csi!("18m");
pub const FONT_9 : & str =              csi!("19m");

// '\u001B]8;;https://github.com\u0007Click\u001B]8;;\u0007'
pub fn link(url: &str, capt: &str) -> String {
    let x = osc!("8;;").to_string() + url + "\u{0007}" + capt + osc!("8;;\u{0007}");
    let mut out = String::from("\x1B]");
    out.push_str(&x);
    out
}
