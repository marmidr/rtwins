//! Color definitions

use enum_iterator::IntoEnumIterator;
use crate::esc::*;

/// Foreground colors
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, IntoEnumIterator)]
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
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, IntoEnumIterator)]
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


const MAP_CL_FG: [&'static str; 18] =
[
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
    FG_WHITE_INTENSE
];

const MAP_CL_BG: [&'static str; 18] =
[
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
    BG_WHITE_INTENSE
];

// -----------------------------------------------------------------------------

/// Convert color identifier to ASCII ESC code
pub fn encode_cl_fg(cl: ColorFG) -> &'static str {
    if (cl as usize) < MAP_CL_FG.len() {
        return MAP_CL_FG[cl as usize];
    }

    // if (INRANGE(cl, ColorFG::ThemeBegin, ColorFG::ThemeEnd))
    //     return encodeClTheme(cl);

    ""
}

/// Convert color identifier to ASCII ESC code
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
///
/// \e[4?m               --> \e[3?m
/// \e[48;2;000;111;222m --> \e[38;2;000;111;222m
/// \e[48;5;253m         --> \e[38;5;253m
/// \e[10?m              --> \e[9?m
///
pub fn transcode_cl_bg_2_fg(bg_color_code: &str) -> String {

    if !bg_color_code.as_bytes().starts_with(b"\x1B[") || bg_color_code.len() < 5 {
        return String::new();
    }

    let mut out = String::from(bg_color_code);

    unsafe
    {
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
    // TODO: try this instead https://crates.io/crates/num_enum
    if cl > ColorFG::Default && cl < ColorFG::WhiteIntense {
        let mut it = ColorFG::into_enum_iter();
        while let Some(c) = it.next() {
            if c == cl {
                return it.next().unwrap();
            }
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
        let mut it = ColorBG::into_enum_iter();
        while let Some(c) = it.next() {
            if c == cl {
                return it.next().unwrap();
            }
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

