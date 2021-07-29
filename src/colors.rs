//! Color definitions

/// Foreground colors
pub enum ColorFG
{
    Inherit,
    Default,    // Reset to terminal default
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
    // Theme(u8),
}

/// Background colors
pub enum ColorBG
{
    Inherit,
    Default,    // Reset to terminal default
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
    // Theme(u8),
}

/// Font attributes
pub enum FontAttrib
{
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
