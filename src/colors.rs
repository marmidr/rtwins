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
    // Theme,
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
    // Theme,
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

/// Convert color identifier to ANSI escape code
pub fn encode_cl_fg(cl: ColorFG) -> &'static str {
    if (cl as usize) < MAP_CL_FG.len() {
        return MAP_CL_FG[cl as usize];
    }

    // if (INRANGE(cl, ColorFG::ThemeBegin, ColorFG::ThemeEnd))
    //     return encodeClTheme(cl);

    ""
}

/// Convert color identifier to ANSI escape code
pub fn encode_cl_bg(cl: ColorBG) -> &'static str {
    if (cl as usize) < MAP_CL_BG.len() {
        return MAP_CL_BG[cl as usize];
    }

    // if (INRANGE(cl, ColorBG::ThemeBegin, ColorBG::ThemeEnd))
    //     return encodeClTheme(cl);

    ""
}

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

/// Convert Normal into Intense color
pub fn intensify_cl_fg(cl: ColorFG) -> ColorFG {
    if cl > ColorFG::Default && cl < ColorFG::WhiteIntense {
        let cl_next_val = (cl as u8) + 1;
        // intensified has odd value
        if cl_next_val & 0x01 != 0 {
            let cl_new = ColorFG::try_from(cl_next_val).unwrap_or(cl);
            return cl_new;
        }
    }
    else if cl == ColorFG::Default {
        // force something bright
        return ColorFG::WhiteIntense;
    }

    /*
        if (INRANGE(cl, ColorFG::ThemeBegin, ColorFG::ThemeEnd))
            return intensifyClTheme(cl);
    */
    cl
}

/// Convert Normal into Intense color
pub fn intensify_cl_bg(cl: ColorBG) -> ColorBG {
    if cl > ColorBG::Default && cl < ColorBG::WhiteIntense {
        let cl_next_val = (cl as u8) + 1;
        // intensified has odd value
        if cl_next_val & 0x01 != 0 {
            let cl_new = ColorBG::try_from(cl_next_val).unwrap_or(cl);
            return cl_new;
        }
    }
    else if cl == ColorBG::Default {
        // force something bright
        return ColorBG::WhiteIntense;
    }

    /*
        if (INRANGE(cl, ColorBG::ThemeBegin, ColorBG::ThemeEnd))
            return intensifyClTheme(cl);
    */
    cl
}
