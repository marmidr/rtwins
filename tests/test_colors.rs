//! # RTWins colors tests

extern crate rtwins;
use rtwins::colors;
use rtwins::esc;
use rtwins::{bg_color, fg_color};

#[test]
fn encode_cl_fg() {
    // test incorrect values
    assert_eq!("", colors::ColorFg::Inherit.encode());

    // test correct values
    assert_eq!(esc::FG_DEFAULT, colors::ColorFg::Default.encode());
    assert_eq!(esc::FG_GREEN, colors::ColorFg::Green.encode());
    assert_eq!(esc::FG_WHITE, colors::ColorFg::White.encode());
}

#[test]
fn encode_cl_bg() {
    // test incorrect values
    assert_eq!("", colors::ColorBg::Inherit.encode());

    // test correct values
    assert_eq!(esc::BG_DEFAULT, colors::ColorBg::Default.encode());
    assert_eq!(esc::BG_GREEN, colors::ColorBg::Green.encode());
    assert_eq!(esc::BG_WHITE, colors::ColorBg::White.encode());
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
        colors::ColorFg::WhiteIntense.encode(),
        colors::ColorBg::WhiteIntense.transcode_2_fg()
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
        colors::ColorFg::Inherit,
        colors::ColorFg::Inherit.intensify()
    );
    assert_eq!(
        colors::ColorFg::WhiteIntense,
        colors::ColorFg::Default.intensify()
    );

    // test correct values
    assert_eq!(
        colors::ColorFg::WhiteIntense,
        colors::ColorFg::White.intensify()
    );
    assert_eq!(
        colors::ColorFg::MagentaIntense,
        colors::ColorFg::Magenta.intensify()
    );

    // try to intensify intense color
    assert_eq!(
        colors::ColorFg::MagentaIntense,
        colors::ColorFg::MagentaIntense.intensify()
    );
}

#[test]
fn intensify_cl_bg() {
    // test unsupported values
    assert_eq!(
        colors::ColorBg::Inherit,
        colors::ColorBg::Inherit.intensify()
    );
    assert_eq!(
        colors::ColorBg::WhiteIntense,
        colors::ColorBg::Default.intensify()
    );

    // test correct values
    assert_eq!(
        colors::ColorBg::WhiteIntense,
        colors::ColorBg::White.intensify()
    );
    assert_eq!(
        colors::ColorBg::MagentaIntense,
        colors::ColorBg::Magenta.intensify()
    );

    // try to intensify intense color
    assert_eq!(
        colors::ColorBg::MagentaIntense,
        colors::ColorBg::MagentaIntense.intensify()
    );
}
