//! *ANSI ESC codes*
//!
//! See more: https://en.m.wikipedia.org/wiki/ANSI_escape_code
//!

pub mod esc {

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
macro_rules! st {
    () => {
        ""
    };

    ($elem:expr) => {
        concat!("\x27\\", $elem)
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

// pub const ESC_FONT(id) : & str = csi!("1" #id "m"); // id: 1..9
// pub const ESC_FONT_DEFAULT : & str = csi!("10m");
// // '\u001B]8;;https://github.com\u0007Click\u001B]8;;\u0007'
// pub const ESC_LINK(url, caption) : & str = osc!("8;;") url "\u0007" caption ANSI_OSC("8;;\u{0007}")
// pub const ESC_LINK_FMT : & str =                     ESC_LINK("%s", "%s");

}