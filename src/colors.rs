//! Color definitions

/// Foreground colors
#[derive(Copy, Clone, PartialEq)]
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
#[derive(Copy, Clone, PartialEq)]
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

/// Font attributes
#[derive(Clone, Copy)]
pub enum FontAttrib {
    None,
    Bold,
    Faint,
    Italics,
    Underline,
    Blink,
    Inverse,
    Invisible,
    StrikeThrough,
}

use crate::esc::*;

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

    // #ifdef TWINS_THEMES
    // if (INRANGE(cl, ColorFG::ThemeBegin, ColorFG::ThemeEnd))
    //     return encodeClTheme(cl);
    // #endif

    ""
}

/// Convert color identifier to ASCII ESC code
pub fn encode_cl_bg(cl: ColorBG) -> &'static str {
    if (cl as usize) < MAP_CL_BG.len() {
        return MAP_CL_BG[cl as usize];
    }

    // #ifdef TWINS_THEMES
    // if (INRANGE(cl, ColorBG::ThemeBegin, ColorBG::ThemeEnd))
    //     return encodeClTheme(cl);
    // #endif

    ""
}

// implemented in user code:
// const char* encodeClTheme(ColorFG cl);
// const char* encodeClTheme(ColorBG cl);

///
pub fn transcode_cl_bg_2_fg(bg_color_code: &str) -> String {
    let mut cl_buff = bg_color_code.to_string();

    if !cl_buff.starts_with("\x1B[") {
        return cl_buff;
    }

/*
    strncpy(clCodeBuffer, bgColorCode, sizeof(clCodeBuffer));
    clCodeBuffer[sizeof(clCodeBuffer)-1] = '\0';

    // \e[4?m               --> \e[3?m
    // \e[48;2;000;111;222m --> \e[38;2;000;111;222m
    // \e[48;5;253m         --> \e[38;5;253m
    // \e[10?m              --> \e[9?m
    char c2 = clCodeBuffer[2];
    char c3 = clCodeBuffer[3];

    // lazy, lazy check...
    if (c2 == '4')
    {
        clCodeBuffer[2] = '3';
    }
    else if (c2 == '1' && c3 == '0')
    {
        memmove(clCodeBuffer+3, clCodeBuffer+4, sizeof(clCodeBuffer)-4);
        clCodeBuffer[2] = '9';
    }

    return clCodeBuffer;
*/
    "".to_string()
}

///
pub fn intensify_cl_fg(cl: ColorFG) -> ColorFG {
/*
    // normal -> intense
    if (cl > ColorFG::Default && cl < ColorFG::WhiteIntense)
        return ColorFG((int)cl + 1);

    if (cl == ColorFG::Default) // may not be correct
        return ColorFG::WhiteIntense;

    #ifdef TWINS_THEMES
    if (INRANGE(cl, ColorFG::ThemeBegin, ColorFG::ThemeEnd))
        return intensifyClTheme(cl);
    #endif

    return cl;
*/
    cl
}

///
pub fn intensify_cl_bg(cl: ColorBG) -> ColorBG {
    // unimplemented!();
/*
    // normal -> intense
    if (cl > ColorBG::Default && cl < ColorBG::WhiteIntense)
        return ColorBG((int)cl + 1);

    if (cl == ColorBG::Default) // may not be correct
        return ColorBG::BlackIntense;

    #ifdef TWINS_THEMES
    if (INRANGE(cl, ColorBG::ThemeBegin, ColorBG::ThemeEnd))
        return intensifyClTheme(cl);
    #endif

    return cl;
*/
    cl
}

