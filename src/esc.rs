//! # ANSI ESC codes
//!
//! [More on Wikipedia](https://en.m.wikipedia.org/wiki/ANSI_escape_code)
//!

// -----------------------------------------------------------------------------------------------
/// # Escape sequence generation

/// ESC prefix
#[macro_export]
macro_rules! esc {
    () => {
        "\x1B"
    };

    ($elem:expr) => {
        concat!("\x1B", $elem)
    };
}

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

/// Foreground RGB color
#[macro_export]
macro_rules! fg_rgb {
    // r,g,b: 0..255
    ($r:expr, $g:expr, $b:expr) => {
        $crate::csi!(concat!("38;2;", stringify!($r), ";", stringify!($g), ";", stringify!($b), "m"));
    };
}

/// Foreground 8-bit color
#[macro_export]
macro_rules! fg_color {
    // clno: 1..255, 232..255=black->white
    ($clno:expr) => {
        $crate::csi!(concat!("38;5;", stringify!($clno), "m"));
    };
}

/// Background RGB color
#[macro_export]
macro_rules! bg_rgb {
    // r,g,b: 0..255
    ($r:expr, $g:expr, $b:expr) => {
        $crate::csi!(concat!("48;2;", stringify!($r), ";", stringify!($g), ";", stringify!($b), "m"));
    };
}

/// Background 8-bit color
#[macro_export]
macro_rules! bg_color {
    // clno: 1..255, 232..255=black->white
    ($clno:expr) => {
        $crate::csi!(concat!("48;5;", stringify!($clno), "m"));
    };
}


// -----------------------------------------------------------------------------------------------
/// # Text Display Modifier Escape Sequences

pub const BOLD                : &str = csi!("1m");
pub const FAINT               : &str = csi!("2m");
pub const NORMAL              : &str = csi!("22m");

/// # Text attributes
// if not italics, may be same as inverse
pub const ITALICS_ON          : &str = csi!("3m");
pub const ITALICS_OFF         : &str = csi!("23m");

pub const UNDERLINE_ON        : &str = csi!("4m");
pub const UNDERLINE_OFF       : &str = csi!("24m");

pub const OVERLINE_ON         : &str = csi!("53m");
pub const OVERLINE_OFF        : &str = csi!("55m");

// if not blinks, the bg color may be lighter
pub const BLINK               : &str = csi!("5m");
pub const BLINK_OFF           : &str = csi!("25m");

pub const INVERSE_ON          : &str = csi!("7m");
pub const INVERSE_OFF         : &str = csi!("27m");

pub const INVISIBLE_ON        : &str = csi!("8m");
pub const INVISIBLE_OFF       : &str = csi!("28m");

pub const STRIKETHROUGH_ON    : &str = csi!("9m");
pub const STRIKETHROUGH_OFF   : &str = csi!("29m");

pub const ATTRIBUTES_DEFAULT  : &str = csi!("10;22;23;24;25;27;28;29m");

/// # Font selection
pub const FONT_DEFAULT    : &str = csi!("10m");
pub const FONT_1          : &str = csi!("11m");
pub const FONT_2          : &str = csi!("12m");
pub const FONT_3          : &str = csi!("13m");
pub const FONT_4          : &str = csi!("14m");
pub const FONT_5          : &str = csi!("15m");
pub const FONT_6          : &str = csi!("16m");
pub const FONT_7          : &str = csi!("17m");
pub const FONT_8          : &str = csi!("18m");
pub const FONT_9          : &str = csi!("19m");

// -----------------------------------------------------------------------------------------------
/// # Text Color Control Sequences
/// 4/8/24-bit ANSI colors
/// https://en.wikipedia.org/wiki/ANSI_escape_code

pub const FG_BLACK            : &str = csi!("30m");
pub const FG_BLACK_INTENSE    : &str = csi!("90m");
pub const FG_RED              : &str = csi!("31m");
pub const FG_RED_INTENSE      : &str = csi!("91m");
pub const FG_GREEN            : &str = csi!("32m");
pub const FG_GREEN_INTENSE    : &str = csi!("92m");
pub const FG_YELLOW           : &str = csi!("33m");
pub const FG_YELLOW_INTENSE   : &str = csi!("93m");
pub const FG_BLUE             : &str = csi!("34m");
pub const FG_BLUE_INTENSE     : &str = csi!("94m");
pub const FG_MAGENTA          : &str = csi!("35m");
pub const FG_MAGENTA_INTENSE  : &str = csi!("95m");
pub const FG_CYAN             : &str = csi!("36m");
pub const FG_CYAN_INTENSE     : &str = csi!("96m");
pub const FG_WHITE            : &str = csi!("37m");
pub const FG_WHITE_INTENSE    : &str = csi!("97m");
pub const FG_DEFAULT          : &str = csi!("39m");


pub const BG_BLACK            : &str = csi!("40m");
pub const BG_BLACK_INTENSE    : &str = csi!("100m");
pub const BG_RED              : &str = csi!("41m");
pub const BG_RED_INTENSE      : &str = csi!("101m");
pub const BG_GREEN            : &str = csi!("42m");
pub const BG_GREEN_INTENSE    : &str = csi!("102m");
pub const BG_YELLOW           : &str = csi!("43m");
pub const BG_YELLOW_INTENSE   : &str = csi!("103m");
pub const BG_BLUE             : &str = csi!("44m");
pub const BG_BLUE_INTENSE     : &str = csi!("104m");
pub const BG_MAGENTA          : &str = csi!("45m");
pub const BG_MAGENTA_INTENSE  : &str = csi!("105m");
pub const BG_CYAN             : &str = csi!("46m");
pub const BG_CYAN_INTENSE     : &str = csi!("106m");
pub const BG_WHITE            : &str = csi!("47m");
pub const BG_WHITE_INTENSE    : &str = csi!("107m");
pub const BG_DEFAULT          : &str = csi!("49m");

/// Put Foreground and Background colors to their defaults
pub const COLORS_DEFAULT : &str =               csi!("0m");

// -----------------------------------------------------------------------------------------------
/// # WEB colors
/// https://en.wikipedia.org/wiki/Web_colors

/// Pink colors
pub const FG_PINK               : &str = fg_rgb!(255, 192, 203);
pub const BG_PINK               : &str = bg_rgb!(255, 192, 203);
pub const FG_LIGHT_PINK         : &str = fg_rgb!(255, 182, 193);
pub const BG_LIGHT_PINK         : &str = bg_rgb!(255, 182, 193);
pub const FG_HOT_PINK           : &str = fg_rgb!(255, 105, 180);
pub const BG_HOT_PINK           : &str = bg_rgb!(255, 105, 180);
pub const FG_DEEP_PINK          : &str = fg_rgb!(255, 20, 147);
pub const BG_DEEP_PINK          : &str = bg_rgb!(255, 20, 147);
pub const FG_PALE_VIOLET_RED    : &str = fg_rgb!(219, 112, 147);
pub const BG_PALE_VIOLET_RED    : &str = bg_rgb!(219, 112, 147);
pub const FG_MEDIUM_VIOLET_RED  : &str = fg_rgb!(199, 21, 133);
pub const BG_MEDIUM_VIOLET_RED  : &str = bg_rgb!(199, 21, 133);

/// Red colors
pub const FG_LIGHT_SALMON   : &str = fg_rgb!(255, 160, 122);
pub const BG_LIGHT_SALMON   : &str = bg_rgb!(255, 160, 122);
pub const FG_SALMON         : &str = fg_rgb!(250, 128, 114);
pub const BG_SALMON         : &str = bg_rgb!(250, 128, 114);
pub const FG_DARK_SALMON    : &str = fg_rgb!(233, 150, 122);
pub const BG_DARK_SALMON    : &str = bg_rgb!(233, 150, 122);
pub const FG_LIGHT_CORAL    : &str = fg_rgb!(240, 128, 128);
pub const BG_LIGHT_CORAL    : &str = bg_rgb!(240, 128, 128);
pub const FG_INDIAN_RED     : &str = fg_rgb!(205, 92, 92);
pub const BG_INDIAN_RED     : &str = bg_rgb!(205, 92, 92);
pub const FG_CRIMSON        : &str = fg_rgb!(220, 20, 60);
pub const BG_CRIMSON        : &str = bg_rgb!(220, 20, 60);
pub const FG_FIREBRICK      : &str = fg_rgb!(178, 34, 34);
pub const BG_FIREBRICK      : &str = bg_rgb!(178, 34, 34);
pub const FG_DARK_RED       : &str = fg_rgb!(139, 0, 0);
pub const BG_DARK_RED       : &str = bg_rgb!(139, 0, 0);
pub const FG_RED_RGB        : &str = fg_rgb!(255, 0, 0);
pub const BG_RED_RGB        : &str = bg_rgb!(255, 0, 0);

/// Orange colors
pub const FG_ORANGE_RED     : &str = fg_rgb!(255, 69, 0);
pub const BG_ORANGE_RED     : &str = bg_rgb!(255, 69, 0);
pub const FG_TOMATO         : &str = fg_rgb!(255, 99, 71);
pub const BG_TOMATO         : &str = bg_rgb!(255, 99, 71);
pub const FG_CORAL          : &str = fg_rgb!(255, 127, 80);
pub const BG_CORAL          : &str = bg_rgb!(255, 127, 80);
pub const FG_DARK_ORANGE    : &str = fg_rgb!(255, 140, 0);
pub const BG_DARK_ORANGE    : &str = bg_rgb!(255, 140, 0);
pub const FG_ORANGE         : &str = fg_rgb!(255, 165, 0);
pub const BG_ORANGE         : &str = bg_rgb!(255, 165, 0);

/// Yellow colors
pub const FG_YELLOW_RGB               : &str = fg_rgb!(255, 255, 0);
pub const BG_YELLOW_RGB               : &str = bg_rgb!(255, 255, 0);
pub const FG_LIGHT_YELLOW             : &str = fg_rgb!(255, 255, 224);
pub const BG_LIGHT_YELLOW             : &str = bg_rgb!(255, 255, 224);
pub const FG_LEMON_CHIFFON            : &str = fg_rgb!(255, 250, 205);
pub const BG_LEMON_CHIFFON            : &str = bg_rgb!(255, 250, 205);
pub const FG_LIGHT_GOLDENROD_YELLOW   : &str = fg_rgb!(250, 250, 210);
pub const BG_LIGHT_GOLDENROD_YELLOW   : &str = bg_rgb!(250, 250, 210);
pub const FG_PAPAYA_WHIP              : &str = fg_rgb!(255, 239, 213);
pub const BG_PAPAYA_WHIP              : &str = bg_rgb!(255, 239, 213);
pub const FG_MOCCASIN                 : &str = fg_rgb!(255, 228, 181);
pub const BG_MOCCASIN                 : &str = bg_rgb!(255, 228, 181);
pub const FG_PEACH_PUFF               : &str = fg_rgb!(255, 218, 185);
pub const BG_PEACH_PUFF               : &str = bg_rgb!(255, 218, 185);
pub const FG_PALE_GOLDENROD           : &str = fg_rgb!(238, 232, 170);
pub const BG_PALE_GOLDENROD           : &str = bg_rgb!(238, 232, 170);
pub const FG_KHAKI                    : &str = fg_rgb!(240, 230, 140);
pub const BG_KHAKI                    : &str = bg_rgb!(240, 230, 140);
pub const FG_DARK_KHAKI               : &str = fg_rgb!(189, 183, 107);
pub const BG_DARK_KHAKI               : &str = bg_rgb!(189, 183, 107);
pub const FG_GOLD                     : &str = fg_rgb!(255, 215, 0);
pub const BG_GOLD                     : &str = bg_rgb!(255, 215, 0);

/// Brown colors
pub const FG_CORNSILK                 : &str = fg_rgb!(255, 248, 220);
pub const BG_CORNSILK                 : &str = bg_rgb!(255, 248, 220);
pub const FG_BLANCHED_ALMOND          : &str = fg_rgb!(255, 235, 205);
pub const BG_BLANCHED_ALMOND          : &str = bg_rgb!(255, 235, 205);
pub const FG_BISQUE                   : &str = fg_rgb!(255, 228, 196);
pub const BG_BISQUE                   : &str = bg_rgb!(255, 228, 196);
pub const FG_NAVAJO_WHITE             : &str = fg_rgb!(255, 222, 173);
pub const BG_NAVAJO_WHITE             : &str = bg_rgb!(255, 222, 173);
pub const FG_WHEAT                    : &str = fg_rgb!(245, 222, 179);
pub const BG_WHEAT                    : &str = bg_rgb!(245, 222, 179);
pub const FG_BURLYWOOD                : &str = fg_rgb!(222, 184, 135);
pub const BG_BURLYWOOD                : &str = bg_rgb!(222, 184, 135);
pub const FG_TAN                      : &str = fg_rgb!(210, 180, 140);
pub const BG_TAN                      : &str = bg_rgb!(210, 180, 140);
pub const FG_ROSY_BROWN               : &str = fg_rgb!(188, 143, 143);
pub const BG_ROSY_BROWN               : &str = bg_rgb!(188, 143, 143);
pub const FG_SANDY_BROWN              : &str = fg_rgb!(244, 164, 96);
pub const BG_SANDY_BROWN              : &str = bg_rgb!(244, 164, 96);
pub const FG_GOLDENROD                : &str = fg_rgb!(218, 165, 32);
pub const BG_GOLDENROD                : &str = bg_rgb!(218, 165, 32);
pub const FG_DARK_GOLDENROD           : &str = fg_rgb!(184, 134, 11);
pub const BG_DARK_GOLDENROD           : &str = bg_rgb!(184, 134, 11);
pub const FG_PERU                     : &str = fg_rgb!(205, 133, 63);
pub const BG_PERU                     : &str = bg_rgb!(205, 133, 63);
pub const FG_CHOCOLATE                : &str = fg_rgb!(210, 105, 30);
pub const BG_CHOCOLATE                : &str = bg_rgb!(210, 105, 30);
pub const FG_SADDLE_BROWN             : &str = fg_rgb!(139, 69, 19);
pub const BG_SADDLE_BROWN             : &str = bg_rgb!(139, 69, 19);
pub const FG_SIENNA                   : &str = fg_rgb!(160, 82, 45);
pub const BG_SIENNA                   : &str = bg_rgb!(160, 82, 45);
pub const FG_BROWN                    : &str = fg_rgb!(165, 42, 42);
pub const BG_BROWN                    : &str = bg_rgb!(165, 42, 42);
pub const FG_MAROON                   : &str = fg_rgb!(128, 0, 0);
pub const BG_MAROON                   : &str = bg_rgb!(128, 0, 0);

/// Green colors
pub const FG_DARK_OLIVE_GREEN       : &str = fg_rgb!(85, 107, 47);
pub const BG_DARK_OLIVE_GREEN       : &str = bg_rgb!(85, 107, 47);
pub const FG_OLIVE                  : &str = fg_rgb!(128, 128, 0);
pub const BG_OLIVE                  : &str = bg_rgb!(128, 128, 0);
pub const FG_OLIVE_DRAB             : &str = fg_rgb!(107, 142, 35);
pub const BG_OLIVE_DRAB             : &str = bg_rgb!(107, 142, 35);
pub const FG_YELLOW_GREEN           : &str = fg_rgb!(154, 205, 50);
pub const BG_YELLOW_GREEN           : &str = bg_rgb!(154, 205, 50);
pub const FG_LIME_GREEN             : &str = fg_rgb!(50, 205, 50);
pub const BG_LIME_GREEN             : &str = bg_rgb!(50, 205, 50);
pub const FG_LIME                   : &str = fg_rgb!(0, 255, 0);
pub const BG_LIME                   : &str = bg_rgb!(0, 255, 0);
pub const FG_LAWN_GREEN             : &str = fg_rgb!(124, 252, 0);
pub const BG_LAWN_GREEN             : &str = bg_rgb!(124, 252, 0);
pub const FG_CHARTREUSE             : &str = fg_rgb!(127, 255, 0);
pub const BG_CHARTREUSE             : &str = bg_rgb!(127, 255, 0);
pub const FG_GREEN_YELLOW           : &str = fg_rgb!(173, 255, 47);
pub const BG_GREEN_YELLOW           : &str = bg_rgb!(173, 255, 47);
pub const FG_SPRING_GREEN           : &str = fg_rgb!(0, 255, 127);
pub const BG_SPRING_GREEN           : &str = bg_rgb!(0, 255, 127);
pub const FG_MEDIUM_SPRING_GREEN    : &str = fg_rgb!(0, 250, 154);
pub const BG_MEDIUM_SPRING_GREEN    : &str = bg_rgb!(0, 250, 154);
pub const FG_LIGHT_GREEN            : &str = fg_rgb!(144, 238, 144);
pub const BG_LIGHT_GREEN            : &str = bg_rgb!(144, 238, 144);
pub const FG_PALE_GREEN             : &str = fg_rgb!(152, 251, 152);
pub const BG_PALE_GREEN             : &str = bg_rgb!(152, 251, 152);
pub const FG_DARK_SEA_GREEN         : &str = fg_rgb!(143, 188, 143);
pub const BG_DARK_SEA_GREEN         : &str = bg_rgb!(143, 188, 143);
pub const FG_MEDIUM_AQUAMARINE      : &str = fg_rgb!(102, 205, 170);
pub const BG_MEDIUM_AQUAMARINE      : &str = bg_rgb!(102, 205, 170);
pub const FG_MEDIUM_SEA_GREEN       : &str = fg_rgb!(60, 179, 113);
pub const BG_MEDIUM_SEA_GREEN       : &str = bg_rgb!(60, 179, 113);
pub const FG_SEA_GREEN              : &str = fg_rgb!(46, 139, 87);
pub const BG_SEA_GREEN              : &str = bg_rgb!(46, 139, 87);
pub const FG_FOREST_GREEN           : &str = fg_rgb!(34, 139, 34);
pub const BG_FOREST_GREEN           : &str = bg_rgb!(34, 139, 34);
pub const FG_GREEN_RGB              : &str = fg_rgb!(0, 128, 0);
pub const BG_GREEN_RGB              : &str = bg_rgb!(0, 128, 0);
pub const FG_DARK_GREEN             : &str = fg_rgb!(0, 100, 0);
pub const BG_DARK_GREEN             : &str = bg_rgb!(0, 100, 0);

/// Cyan colors
pub const FG_AQUA               : &str = fg_rgb!(0, 255, 255);
pub const BG_AQUA               : &str = bg_rgb!(0, 255, 255);
pub const FG_CYAN_RGB           : &str = fg_rgb!(0, 255, 255);
pub const BG_CYAN_RGB           : &str = bg_rgb!(0, 255, 255);
pub const FG_LIGHT_CYAN         : &str = fg_rgb!(224, 255, 255);
pub const BG_LIGHT_CYAN         : &str = bg_rgb!(224, 255, 255);
pub const FG_PALE_TURQUOISE     : &str = fg_rgb!(175, 238, 238);
pub const BG_PALE_TURQUOISE     : &str = bg_rgb!(175, 238, 238);
pub const FG_AQUAMARINE         : &str = fg_rgb!(127, 255, 212);
pub const BG_AQUAMARINE         : &str = bg_rgb!(127, 255, 212);
pub const FG_TURQUOISE          : &str = fg_rgb!(64, 224, 208);
pub const BG_TURQUOISE          : &str = bg_rgb!(64, 224, 208);
pub const FG_MEDIUM_TURQUOISE   : &str = fg_rgb!(72, 209, 204);
pub const BG_MEDIUM_TURQUOISE   : &str = bg_rgb!(72, 209, 204);
pub const FG_DARK_TURQUOISE     : &str = fg_rgb!(0, 206, 209);
pub const BG_DARK_TURQUOISE     : &str = bg_rgb!(0, 206, 209);
pub const FG_LIGHT_SEA_GREEN    : &str = fg_rgb!(32, 178, 170);
pub const BG_LIGHT_SEA_GREEN    : &str = bg_rgb!(32, 178, 170);
pub const FG_CADET_BLUE         : &str = fg_rgb!(95, 158, 160);
pub const BG_CADET_BLUE         : &str = bg_rgb!(95, 158, 160);
pub const FG_DARK_CYAN          : &str = fg_rgb!(0, 139, 139);
pub const BG_DARK_CYAN          : &str = bg_rgb!(0, 139, 139);
pub const FG_TEAL               : &str = fg_rgb!(0, 128, 128);
pub const BG_TEAL               : &str = bg_rgb!(0, 128, 128);

/// Blue colors
pub const FG_LIGHT_STEEL_BLUE   : &str = fg_rgb!(176, 196, 222);
pub const BG_LIGHT_STEEL_BLUE   : &str = bg_rgb!(176, 196, 222);
pub const FG_POWDER_BLUE        : &str = fg_rgb!(176, 224, 230);
pub const BG_POWDER_BLUE        : &str = bg_rgb!(176, 224, 230);
pub const FG_LIGHT_BLUE         : &str = fg_rgb!(173, 216, 230);
pub const BG_LIGHT_BLUE         : &str = bg_rgb!(173, 216, 230);
pub const FG_SKY_BLUE           : &str = fg_rgb!(135, 206, 235);
pub const BG_SKY_BLUE           : &str = bg_rgb!(135, 206, 235);
pub const FG_LIGHT_SKY_BLUE     : &str = fg_rgb!(135, 206, 250);
pub const BG_LIGHT_SKY_BLUE     : &str = bg_rgb!(135, 206, 250);
pub const FG_DEEP_SKY_BLUE      : &str = fg_rgb!(0, 191, 255);
pub const BG_DEEP_SKY_BLUE      : &str = bg_rgb!(0, 191, 255);
pub const FG_DODGER_BLUE        : &str = fg_rgb!(30, 144, 255);
pub const BG_DODGER_BLUE        : &str = bg_rgb!(30, 144, 255);
pub const FG_CORNFLOWER_BLUE    : &str = fg_rgb!(100, 149, 237);
pub const BG_CORNFLOWER_BLUE    : &str = bg_rgb!(100, 149, 237);
pub const FG_STEEL_BLUE         : &str = fg_rgb!(70, 130, 180);
pub const BG_STEEL_BLUE         : &str = bg_rgb!(70, 130, 180);
pub const FG_ROYAL_BLUE         : &str = fg_rgb!(65, 105, 225);
pub const BG_ROYAL_BLUE         : &str = bg_rgb!(65, 105, 225);
pub const FG_BLUE_RGB           : &str = fg_rgb!(0, 0, 255);
pub const BG_BLUE_RGB           : &str = bg_rgb!(0, 0, 255);
pub const FG_MEDIUM_BLUE        : &str = fg_rgb!(0, 0, 205);
pub const BG_MEDIUM_BLUE        : &str = bg_rgb!(0, 0, 205);
pub const FG_DARK_BLUE          : &str = fg_rgb!(0, 0, 139);
pub const BG_DARK_BLUE          : &str = bg_rgb!(0, 0, 139);
pub const FG_NAVY               : &str = fg_rgb!(0, 0, 128);
pub const BG_NAVY               : &str = bg_rgb!(0, 0, 128);
pub const FG_MIDNIGHT_BLUE      : &str = fg_rgb!(25, 25, 112);
pub const BG_MIDNIGHT_BLUE      : &str = bg_rgb!(25, 25, 112);

/// Purple, violet, and magenta colors
pub const FG_LAVENDER           : &str = fg_rgb!(230, 230, 250);
pub const BG_LAVENDER           : &str = bg_rgb!(230, 230, 250);
pub const FG_THISTLE            : &str = fg_rgb!(216, 191, 216);
pub const BG_THISTLE            : &str = bg_rgb!(216, 191, 216);
pub const FG_PLUM               : &str = fg_rgb!(221, 160, 221);
pub const BG_PLUM               : &str = bg_rgb!(221, 160, 221);
pub const FG_VIOLET             : &str = fg_rgb!(238, 130, 238);
pub const BG_VIOLET             : &str = bg_rgb!(238, 130, 238);
pub const FG_ORCHID             : &str = fg_rgb!(218, 112, 214);
pub const BG_ORCHID             : &str = bg_rgb!(218, 112, 214);
pub const FG_FUCHSIA            : &str = fg_rgb!(255, 0, 255);
pub const BG_FUCHSIA            : &str = bg_rgb!(255, 0, 255);
pub const FG_MAGENTA_RGB        : &str = fg_rgb!(255, 0, 255);
pub const BG_MAGENTA_RGB        : &str = bg_rgb!(255, 0, 255);
pub const FG_MEDIUM_ORCHID      : &str = fg_rgb!(186, 85, 211);
pub const BG_MEDIUM_ORCHID      : &str = bg_rgb!(186, 85, 211);
pub const FG_MEDIUM_PURPLE      : &str = fg_rgb!(147, 112, 219);
pub const BG_MEDIUM_PURPLE      : &str = bg_rgb!(147, 112, 219);
pub const FG_BLUE_VIOLET        : &str = fg_rgb!(138, 43, 226);
pub const BG_BLUE_VIOLET        : &str = bg_rgb!(138, 43, 226);
pub const FG_DARK_VIOLET        : &str = fg_rgb!(148, 0, 211);
pub const BG_DARK_VIOLET        : &str = bg_rgb!(148, 0, 211);
pub const FG_DARK_ORCHID        : &str = fg_rgb!(153, 50, 204);
pub const BG_DARK_ORCHID        : &str = bg_rgb!(153, 50, 204);
pub const FG_DARK_MAGENTA       : &str = fg_rgb!(139, 0, 139);
pub const BG_DARK_MAGENTA       : &str = bg_rgb!(139, 0, 139);
pub const FG_PURPLE             : &str = fg_rgb!(128, 0, 128);
pub const BG_PURPLE             : &str = bg_rgb!(128, 0, 128);
pub const FG_INDIGO             : &str = fg_rgb!(75, 0, 130);
pub const BG_INDIGO             : &str = bg_rgb!(75, 0, 130);
pub const FG_DARK_SLATE_BLUE    : &str = fg_rgb!(72, 61, 139);
pub const BG_DARK_SLATE_BLUE    : &str = bg_rgb!(72, 61, 139);
pub const FG_SLATE_BLUE         : &str = fg_rgb!(106, 90, 205);
pub const BG_SLATE_BLUE         : &str = bg_rgb!(106, 90, 205);
pub const FG_MEDIUM_SLATE_BLUE  : &str = fg_rgb!(123, 104, 238);
pub const BG_MEDIUM_SLATE_BLUE  : &str = bg_rgb!(123, 104, 238);

/// Gray and black colors
pub const FG_GAINSBORO          : &str = fg_rgb!(220, 220, 220);
pub const BG_GAINSBORO          : &str = bg_rgb!(220, 220, 220);
pub const FG_LIGHT_GRAY         : &str = fg_rgb!(211, 211, 211);
pub const BG_LIGHT_GRAY         : &str = bg_rgb!(211, 211, 211);
pub const FG_SILVER             : &str = fg_rgb!(192, 192, 192);
pub const BG_SILVER             : &str = bg_rgb!(192, 192, 192);
pub const FG_DARK_GRAY          : &str = fg_rgb!(169, 169, 169);
pub const BG_DARK_GRAY          : &str = bg_rgb!(169, 169, 169);
pub const FG_GRAY               : &str = fg_rgb!(128, 128, 128);
pub const BG_GRAY               : &str = bg_rgb!(128, 128, 128);
pub const FG_DIM_GRAY           : &str = fg_rgb!(105, 105, 105);
pub const BG_DIM_GRAY           : &str = bg_rgb!(105, 105, 105);
pub const FG_LIGHT_SLATE_GRAY   : &str = fg_rgb!(119, 136, 153);
pub const BG_LIGHT_SLATE_GRAY   : &str = bg_rgb!(119, 136, 153);
pub const FG_SLATE_GRAY         : &str = fg_rgb!(112, 128, 144);
pub const BG_SLATE_GRAY         : &str = bg_rgb!(112, 128, 144);
pub const FG_DARK_SLATE_GRAY    : &str = fg_rgb!(47, 79, 79);
pub const BG_DARK_SLATE_GRAY    : &str = bg_rgb!(47, 79, 79);
pub const FG_BLACK_RGB          : &str = fg_rgb!(0, 0, 0);
pub const BG_BLACK_RGB          : &str = bg_rgb!(0, 0, 0);

// -----------------------------------------------------------------------------------------------
/// # Cursor Positioning Control Sequences

pub const CURSOR_POS_SAVE       : &str = csi!("s");
pub const CURSOR_POS_RESTORE    : &str = csi!("u");

pub const CURSOR_HIDE           : &str = csi!("?25l");
pub const CURSOR_SHOW           : &str = csi!("?25h");

pub const CURSOR_HOME           : &str = csi!("H");

#[macro_export]
macro_rules! cursor_column { ($col:expr)            => { $crate::csi!(concat!(stringify!($col), "G")); }; }

#[macro_export]
macro_rules! cursor_goto { ($row:expr, $col:expr)   => { $crate::csi!(concat!(stringify!($row), ";", stringify!($col), "H")); }; }

#[macro_export]
macro_rules! cursor_up { ($n:expr)                  => { $crate::csi!(concat!(stringify!($n), "A")); }; }

#[macro_export]
macro_rules! cursor_down { ($n:expr)                => { $crate::csi!(concat!(stringify!($n), "B")); }; }

#[macro_export]
macro_rules! cursor_forward { ($n:expr)             => { $crate::csi!(concat!(stringify!($n), "C")); }; }

#[macro_export]
macro_rules! cursor_backward { ($n:expr)            => { $crate::csi!(concat!(stringify!($n), "D")); }; }

pub const CURSOR_COLUMN_FMT   : &str = cursor_column!("{}");
pub const CURSOR_GOTO_FMT     : &str = cursor_goto!("{}", "{}");
pub const CURSOR_UP_FMT       : &str = cursor_up!("{}");
pub const CURSOR_DOWN_FMT     : &str = cursor_down!("{}");
pub const CURSOR_FORWARD_FMT  : &str = cursor_forward!("{}");
pub const CURSOR_BACKWARD_FMT : &str = cursor_backward!("{}");

// -----------------------------------------------------------------------------------------------
/// # Line control

/// Erases the current line, returning the cursor to the far left
pub const LINE_ERASE_ALL    : &str = csi!("2K");
pub const LINE_ERASE_RIGHT  : &str = csi!("0K");
pub const LINE_ERASE_LEFT   : &str = csi!("1K");

/// Insert `n` lines
#[macro_export]
macro_rules! line_insert { ($n:expr) => { $crate::csi!(concat!(stringify!($n), "L")); }; }

/// Delete `n` lines
#[macro_export]
macro_rules! line_delete { ($n:expr) => { $crate::csi!(concat!(stringify!($n), "M")); }; }

pub const LINE_INSERT_FMT: &str =  line_insert!("{}");
pub const LINE_DELETE_FMT: &str =  line_delete!("{}");

// -----------------------------------------------------------------------------------------------
/// # Character control

/// Repeat last character `n` times - not supported on every platform
#[macro_export]
macro_rules! char_repeat_last { ($n:expr)       => { $crate::csi!(concat!(stringify!($n), "b")); }; }

/// Erase `n` characters (replace with space)
#[macro_export]
macro_rules! char_erase { ($n:expr)             => { $crate::csi!(concat!(stringify!($n), "X")); }; }

/// Delete `n` characters
#[macro_export]
macro_rules! char_delete { ($n:expr)            => { $crate::csi!(concat!(stringify!($n), "P")); }; }

/// Insert `n` characters
#[macro_export]
macro_rules! char_insert { ($n:expr)            => { $crate::csi!(concat!(stringify!($n), "@")); }; }

pub const CHAR_REPEAT_LAST_FMT : &str = char_repeat_last!("{}");
pub const CHAR_ERASE_FMT       : &str = char_erase!("{}");
pub const CHAR_DELETE_FMT      : &str = char_delete!("{}");
pub const CHAR_INSERT_FMT      : &str = char_insert!("{}");

// -----------------------------------------------------------------------------------------------
/// # Screen Control Sequences

/// Erases the entire display, returning the cursor to the top left.
pub const SCREEN_ERASE_ALL    : &str = csi!("2J");
pub const SCREEN_ERASE_BELOW  : &str = csi!("0J");
pub const SCREEN_ERASE_ABOVE  : &str = csi!("1J");

/// Save / restore screen content
pub const SCREEN_SAVE         : &str = csi!("?47h");
pub const SCREEN_RESTORE      : &str = csi!("?47l");

/// Reverse/normal video mode (BG <--> FG)
pub const SCREEN_REVERSE_ON   : &str = csi!("?5h");
pub const SCREEN_REVERSE_OFF  : &str = csi!("?5l");

/// Scrool screen up `n' lines
#[macro_export]
macro_rules! screen_scroll_up   { ($n:expr) => { $crate::csi!(concat!(stringify!($n), "S")); }; }

/// Scrool screen down `n' lines
#[macro_export]
macro_rules! screen_scroll_down { ($n:expr) => { $crate::csi!(concat!(stringify!($n), "T")); }; }

pub const SCREEN_SCROLL_UP_FMT    : &str = screen_scroll_up!("{}");
pub const SCREEN_SCROLL_DOWN_FMT  : &str = screen_scroll_down!("{}");

// -----------------------------------------------------------------------------------------------
/// # Mouse Control Sequences
/// https://www.systutorials.com/docs/linux/man/4-console_codes/#lbAF

/// Mode1: only click
pub const MOUSE_REPORTING_M1_ON  : &str = csi!("?9h");
pub const MOUSE_REPORTING_M1_OFF : &str = csi!("?9l");

/// Mode2: click + release + wheel + Ctrl/Alt/Shift
pub const MOUSE_REPORTING_M2_ON  : &str = csi!("?1000h");
pub const MOUSE_REPORTING_M2_OFF : &str = csi!("?1000l");

// -----------------------------------------------------------------------------------------------
/// # Miscellaneous Control Sequences

/// Terminal reset - clear the screen and scroll buffer
pub const TERM_RESET            : &str = esc!("c");

/// Bell signal
pub const BELL                  : &str = "\x07";

/// Character encoding
pub const ENCODING_ISO8858_1    : &str = esc!("%@");
pub const ENCODING_UTF8         : &str = esc!("%G");

/// Terminal properties reporting
pub const REPORT_WINDOW_CHARS   : &str = csi!("18t");
pub const REPORT_SCREEN_CHARS   : &str = csi!("19t");
pub const REPORT_CAPABILITIES   : &str = csi!("c");

/// Maximum ESC sequence length (including null)
pub const SEQ_MAX_LENGTH        : u8 = 8;

// bash: blink screen until key pressed
// { while true; do printf \\e[?5h; sleep 0.3; printf \\e[?5l; read -s -n1 -t1 && break; done; }

// -----------------------------------------------------------------------------------------------

/// Clickable URL with title
/// `\u001B]8;;https://github.com\u0007Click\u001B]8;;\u0007`

// pub fn link(url: &str, capt: &str) -> String {
//     let lnk = osc!("8;;").to_string() + url + "\u{0007}" + capt + osc!("8;;\u{0007}");
//     let mut out = String::from("\x1B]");
//     out.push_str(&lnk);
//     out
// }

#[macro_export]
macro_rules! link {
    ($url:expr, $capt:expr) => {
        concat!(
            $crate::osc!("8;;"),
            $url, "\u{0007}", $capt,
            $crate::osc!("8;;\u{0007}")
        );
    };
}
