//! # RTWins colors tests

extern crate rtwins;
use rtwins::colors;
use rtwins::esc;
use rtwins::{bg_color, fg_color};

#[test]
fn encode_cl_fg() {
    // test incorrect values
    assert_eq!("", colors::ColorFG::Inherit.encode());

    // test correct values
    assert_eq!(esc::FG_DEFAULT, colors::ColorFG::Default.encode());
    assert_eq!(esc::FG_GREEN, colors::ColorFG::Green.encode());
    assert_eq!(esc::FG_WHITE, colors::ColorFG::White.encode());
}

#[test]
fn encode_cl_bg() {
    // test incorrect values
    assert_eq!("", colors::ColorBG::Inherit.encode());

    // test correct values
    assert_eq!(esc::BG_DEFAULT, colors::ColorBG::Default.encode());
    assert_eq!(esc::BG_GREEN, colors::ColorBG::Green.encode());
    assert_eq!(esc::BG_WHITE, colors::ColorBG::White.encode());
}

#[test]
fn transcode_cl_bg_2_fg() {
    // test invalid values
    assert_eq!("", colors::transcode_cl_bg_2_fg(""));
    assert_eq!("", colors::transcode_cl_bg_2_fg("\x1B["));
    assert_eq!("", colors::transcode_cl_bg_2_fg("01234567890"));
    assert_eq!("", colors::transcode_cl_bg_2_fg(" \x1B[10m"));

    // test correct values
    assert_eq!(esc::FG_GREEN, colors::transcode_cl_bg_2_fg(esc::FG_GREEN));
    assert_eq!(esc::FG_BLACK, colors::transcode_cl_bg_2_fg(esc::BG_BLACK));

    assert_eq!(
        colors::ColorFG::WhiteIntense.encode(),
        colors::ColorBG::WhiteIntense.transcode_2_fg()
    );

    assert_eq!(
        esc::FG_SKY_BLUE,
        colors::transcode_cl_bg_2_fg(esc::BG_SKY_BLUE)
    );

    assert_eq!(fg_color!(123), colors::transcode_cl_bg_2_fg(bg_color!(123)));
}

#[test]
fn intensify_cl_fg() {
    // test unsupported values
    assert_eq!(
        colors::ColorFG::Inherit,
        colors::ColorFG::Inherit.intensify()
    );
    assert_eq!(
        colors::ColorFG::WhiteIntense,
        colors::ColorFG::Default.intensify()
    );

    // test correct values
    assert_eq!(
        colors::ColorFG::WhiteIntense,
        colors::ColorFG::White.intensify()
    );
    assert_eq!(
        colors::ColorFG::MagentaIntense,
        colors::ColorFG::Magenta.intensify()
    );

    // try to intensify intense color
    assert_eq!(
        colors::ColorFG::MagentaIntense,
        colors::ColorFG::MagentaIntense.intensify()
    );
}

#[test]
fn intensify_cl_bg() {
    // test unsupported values
    assert_eq!(
        colors::ColorBG::Inherit,
        colors::ColorBG::Inherit.intensify()
    );
    assert_eq!(
        colors::ColorBG::WhiteIntense,
        colors::ColorBG::Default.intensify()
    );

    // test correct values
    assert_eq!(
        colors::ColorBG::WhiteIntense,
        colors::ColorBG::White.intensify()
    );
    assert_eq!(
        colors::ColorBG::MagentaIntense,
        colors::ColorBG::Magenta.intensify()
    );

    // try to intensify intense color
    assert_eq!(
        colors::ColorBG::MagentaIntense,
        colors::ColorBG::MagentaIntense.intensify()
    );
}
