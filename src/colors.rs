//! Color definitions

use crate::esc::*;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

// ---------------------------------------------------------------------------------------------- //

/// Foreground colors
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum ColorFG {
    Inherit,
    Default, // Reset to terminal default
    Black,
    BlackIntense,
    Red,
    RedIntense,
    Green,
    GreenIntense,
    Yellow,
    YellowIntense,
    Blue,
    BlueIntense,
    Magenta,
    MagentaIntense,
    Cyan,
    CyanIntense,
    White,
    WhiteIntense,
    //
    Theme0,
    Theme1,
    Theme2,
    Theme3,
    Theme4,
    Theme5,
    Theme6,
    Theme7,
    Theme8,
    Theme9,
    Theme10,
    Theme11,
    Theme12,
    Theme13,
    Theme14,
    Theme15,
    Theme16,
    Theme17,
    Theme18,
    Theme19,
    Theme20,
    Theme21,
    Theme22,
    Theme23,
    Theme24,
    Theme25,
    Theme26,
    Theme27,
    Theme28,
    Theme29,
    ThemeEnd,
}

/// Background colors
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum ColorBG {
    Inherit,
    Default, // Reset to terminal default
    Black,
    BlackIntense,
    Red,
    RedIntense,
    Green,
    GreenIntense,
    Yellow,
    YellowIntense,
    Blue,
    BlueIntense,
    Magenta,
    MagentaIntense,
    Cyan,
    CyanIntense,
    White,
    WhiteIntense,
    //
    Theme0,
    Theme1,
    Theme2,
    Theme3,
    Theme4,
    Theme5,
    Theme6,
    Theme7,
    Theme8,
    Theme9,
    Theme10,
    Theme11,
    Theme12,
    Theme13,
    Theme14,
    Theme15,
    Theme16,
    Theme17,
    Theme18,
    Theme19,
    Theme20,
    Theme21,
    Theme22,
    Theme23,
    Theme24,
    Theme25,
    Theme26,
    Theme27,
    Theme28,
    Theme29,
    ThemeEnd,
}

const MAP_CL_FG: [&str; 18] = [
    "",
    FG_DEFAULT,
    FG_BLACK,
    FG_BLACK_INTENSE,
    FG_RED,
    FG_RED_INTENSE,
    FG_GREEN,
    FG_GREEN_INTENSE,
    FG_YELLOW,
    FG_YELLOW_INTENSE,
    FG_BLUE,
    FG_BLUE_INTENSE,
    FG_MAGENTA,
    FG_MAGENTA_INTENSE,
    FG_CYAN,
    FG_CYAN_INTENSE,
    FG_WHITE,
    FG_WHITE_INTENSE,
];

const MAP_CL_BG: [&str; 18] = [
    "",
    BG_DEFAULT,
    BG_BLACK,
    BG_BLACK_INTENSE,
    BG_RED,
    BG_RED_INTENSE,
    BG_GREEN,
    BG_GREEN_INTENSE,
    BG_YELLOW,
    BG_YELLOW_INTENSE,
    BG_BLUE,
    BG_BLUE_INTENSE,
    BG_MAGENTA,
    BG_MAGENTA_INTENSE,
    BG_CYAN,
    BG_CYAN_INTENSE,
    BG_WHITE,
    BG_WHITE_INTENSE,
];

// -----------------------------------------------------------------------------

// implemented in user code:
// const char* encodeClTheme(ColorFG cl);
// const char* encodeClTheme(ColorBG cl);

/// For given background color, returns matching foreground color
pub fn transcode_cl_bg_2_fg(bg_color_code: &str) -> String {
    // \e[4?m               --> \e[3?m
    // \e[48;2;000;111;222m --> \e[38;2;000;111;222m
    // \e[48;5;253m         --> \e[38;5;253m
    // \e[10?m              --> \e[9?m
    if !bg_color_code.as_bytes().starts_with(b"\x1B[") || bg_color_code.len() < 5 {
        return String::new();
    }

    let mut out = String::from(bg_color_code);

    unsafe {
        // `out` length is known, so we can safely use unchecked versions of `get`
        let c3 = *out.as_bytes().get_unchecked(3);
        let c2 = out.as_bytes_mut().get_unchecked_mut(2);

        if *c2 == b'4' {
            *c2 = b'3';
        }
        else if *c2 == b'1' && c3 == b'0' {
            *c2 = b'9';
            out.remove(3);
        }
    }

    out
}

// -----------------------------------------------------------------------------

// stub function
fn encode_fg(_: ColorFG) -> &'static str {
    ""
}

// stub function
fn encode_bg(_: ColorBG) -> &'static str {
    ""
}

// pointers to user provided function encoding theme colors
thread_local!(
    static CL_FG_THEME: std::cell::Cell<fn(ColorFG) -> &'static str> =
        std::cell::Cell::new(encode_fg);
    static CL_BG_THEME: std::cell::Cell<fn(ColorBG) -> &'static str> =
        std::cell::Cell::new(encode_bg);
);

// ---------------------------------------------------------------------------------------------- //

impl ColorFG {
    /// Converts Normal into Intense color
    pub fn intensify(self) -> Self {
        if self > ColorFG::Default && self < ColorFG::WhiteIntense {
            let cl_next_val = (self as u8) + 1;
            // intensified has odd value
            if cl_next_val & 0x01 != 0 {
                let cl_new = ColorFG::try_from(cl_next_val).unwrap_or(self);
                return cl_new;
            }
        }
        else if self == ColorFG::Default {
            // force something bright
            return ColorFG::WhiteIntense;
        }
        else if self >= Self::Theme0 && self < Self::ThemeEnd {
            // TODO: return intensifyClTheme(self);
        }

        self
    }

    /// Conditionally converts Normal into Intense color
    #[inline]
    pub fn intensify_if(self, cond: bool) -> Self {
        if cond {
            self.intensify()
        }
        else {
            self
        }
    }

    /// Converts color identifier to ANSI escape code
    pub fn encode(self) -> &'static str {
        if (self as usize) < MAP_CL_FG.len() {
            return MAP_CL_FG[self as usize];
        }
        else if self >= Self::Theme0 && self < Self::ThemeEnd {
            let seq = CL_FG_THEME.with(|cell| cell.get()(self));

            return seq;
        }

        ""
    }

    /// Sets encoder function for theme colors
    pub fn set_theme_encoder(encoder: fn(ColorFG) -> &'static str) {
        CL_FG_THEME.with(|cell| {
            cell.set(encoder);
        });
    }
}

impl ColorBG {
    /// Converts Normal into Intense color
    pub fn intensify(self) -> Self {
        if self > ColorBG::Default && self < ColorBG::WhiteIntense {
            let cl_next_val = (self as u8) + 1;
            // intensified has odd value
            if cl_next_val & 0x01 != 0 {
                let cl_new = ColorBG::try_from(cl_next_val).unwrap_or(self);
                return cl_new;
            }
        }
        else if self == ColorBG::Default {
            // force something bright
            return ColorBG::WhiteIntense;
        }
        else if self >= Self::Theme0 && self < Self::ThemeEnd {
            // TODO: return intensifyClTheme(self);
        }

        self
    }

    /// Conditionally converts Normal into Intense color
    #[inline]
    pub fn intensify_if(self, cond: bool) -> Self {
        if cond {
            self.intensify()
        }
        else {
            self
        }
    }

    /// Converts color identifier to ANSI escape code
    pub fn encode(self) -> &'static str {
        if (self as usize) < MAP_CL_BG.len() {
            return MAP_CL_BG[self as usize];
        }
        else if self >= Self::Theme0 && self < Self::ThemeEnd {
            let seq = CL_BG_THEME.with(|cell| cell.get()(self));

            return seq;
        }

        ""
    }

    /// Returns matching foreground color sequence string
    #[inline]
    pub fn transcode_2_fg(self) -> String {
        let bg_color_code = self.encode();
        transcode_cl_bg_2_fg(bg_color_code)
    }

    /// Sets encoder function for theme colors
    pub fn set_theme_encoder(encoder: fn(ColorBG) -> &'static str) {
        CL_BG_THEME.with(|cell| {
            cell.set(encoder);
        });
    }
}
