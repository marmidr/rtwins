//! Color definitions

/// Foreground colors
#[derive(Copy, Clone)]
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
    Theme,
}

/// Background colors
#[derive(Copy, Clone)]
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
    Theme,
}

/// Font attributes
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
