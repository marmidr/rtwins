//! # RTWins String extension tests

extern crate rtwins;
use rtwins::esc;
use rtwins::string_ext::*;

#[test]
fn test_ansi_esc_len() {
    assert_eq!(0, "".ansi_esc_len());
    // Up prefixed with space
    assert_eq!(0, " \x1B[A".ansi_esc_len());
    // Up
    assert_eq!(3, "\x1B[A".ansi_esc_len());
    // Home
    assert_eq!(4, "\x1B[1~".ansi_esc_len());
    // F1
    assert_eq!(5, "\x1B[23^".ansi_esc_len());
    // F1
    assert_eq!(3, "\x1BOP".ansi_esc_len());
    // C-S-F1
    assert_eq!(5, "\x1B[23@".ansi_esc_len());
    // Mouse l-click
    assert_eq!(6, "\x1B[M !!".ansi_esc_len());
    // Mouse wheel down
    assert_eq!(6, "\x1B[Ma$\"".ansi_esc_len());
    // Home - incomplete
    assert_eq!(0, "\x1B[1".ansi_esc_len());
    //  Mouse wheel down - incomplete
    assert_eq!(0, "\x1B[Ma".ansi_esc_len());
}

#[test]
fn test_str_ops() {
    let mut s = String::from("Hello");

    let _ = s.stream()
        << " darkness"
        << ", "
        << "my old friend."
        ;

    assert_eq!("Hello darkness, my old friend.", s.as_str());
}

#[test]
fn test_push_esc_fmt() {
    {
        let mut s = String::new();
        s.push_esc_fmt("ABCD", 13);
        assert_eq!("ABCD", s.as_str());
    }

    {
        let mut s = String::new();
        s.push_esc_fmt(esc::CURSOR_UP_FMT, 13);
        assert_eq!("\x1B[13A", s.as_str());
    }
}
