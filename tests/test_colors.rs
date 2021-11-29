//! # RTWins colors tests

extern crate rtwins;
use rtwins::colors;
use rtwins::esc;
use rtwins::{bg_color, fg_color};

#[test]
fn test_encode_cl_fg() {
    // test incorrect values
    assert_eq!("", colors::encode_cl_fg(colors::ColorFG::Inherit));

    // test correct values
    assert_eq!(esc::FG_DEFAULT, colors::encode_cl_fg(colors::ColorFG::Default));
    assert_eq!(esc::FG_GREEN, colors::encode_cl_fg(colors::ColorFG::Green));
    assert_eq!(esc::FG_WHITE, colors::encode_cl_fg(colors::ColorFG::White));
}

#[test]
fn test_encode_cl_bg() {
    // test incorrect values
    assert_eq!("", colors::encode_cl_bg(colors::ColorBG::Inherit));

    // test correct values
    assert_eq!(esc::BG_DEFAULT, colors::encode_cl_bg(colors::ColorBG::Default));
    assert_eq!(esc::BG_GREEN, colors::encode_cl_bg(colors::ColorBG::Green));
    assert_eq!(esc::BG_WHITE, colors::encode_cl_bg(colors::ColorBG::White));
}

#[test]
fn test_transcode_cl_bg_2_fg() {
    // test invalid values
    assert_eq!("", colors::transcode_cl_bg_2_fg(""));
    assert_eq!("", colors::transcode_cl_bg_2_fg("\x1B["));
    assert_eq!("", colors::transcode_cl_bg_2_fg("01234567890"));
    assert_eq!("", colors::transcode_cl_bg_2_fg(" \x1B[10m"));

    // test correct values
    assert_eq!(esc::FG_GREEN, colors::transcode_cl_bg_2_fg(esc::FG_GREEN));
    assert_eq!(esc::FG_BLACK, colors::transcode_cl_bg_2_fg(esc::BG_BLACK));

    assert_eq!(esc::FG_WHITE_INTENSE, colors::transcode_cl_bg_2_fg(esc::BG_WHITE_INTENSE));
    assert_eq!(esc::FG_SKY_BLUE, colors::transcode_cl_bg_2_fg(esc::BG_SKY_BLUE));
    assert_eq!(fg_color!(123), colors::transcode_cl_bg_2_fg(bg_color!(123)));
}

#[test]
fn test_intensify_cl_fg() {
    // test unsupported values
    assert_eq!(colors::ColorFG::Inherit, colors::intensify_cl_fg(colors::ColorFG::Inherit));
    assert_eq!(colors::ColorFG::WhiteIntense, colors::intensify_cl_fg(colors::ColorFG::Default));

    // test correct values
    assert_eq!(colors::ColorFG::WhiteIntense, colors::intensify_cl_fg(colors::ColorFG::White));
    assert_eq!(colors::ColorFG::MagentaIntense, colors::intensify_cl_fg(colors::ColorFG::Magenta));

    // try to intensify intense color
    assert_eq!(colors::ColorFG::MagentaIntense, colors::intensify_cl_fg(colors::ColorFG::MagentaIntense));
}

#[test]
fn test_intensify_cl_bg() {
    // test unsupported values
    assert_eq!(colors::ColorBG::Inherit, colors::intensify_cl_bg(colors::ColorBG::Inherit));
    assert_eq!(colors::ColorBG::WhiteIntense, colors::intensify_cl_bg(colors::ColorBG::Default));

    // test correct values
    assert_eq!(colors::ColorBG::WhiteIntense, colors::intensify_cl_bg(colors::ColorBG::White));
    assert_eq!(colors::ColorBG::MagentaIntense, colors::intensify_cl_bg(colors::ColorBG::Magenta));

    // try to intensify intense color
    assert_eq!(colors::ColorBG::MagentaIntense, colors::intensify_cl_bg(colors::ColorBG::MagentaIntense));
}
