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
fn test_ansi_displayed_width() {
    assert_eq!(0, "".ansi_displayed_width());
    assert_eq!(5, "Title".ansi_displayed_width());
    assert_eq!(5, format!("{}Title{}", esc::BOLD, esc::NORMAL).as_str().ansi_displayed_width());
    assert_eq!(7, "-😉-🍺-".ansi_displayed_width());
}

#[test]
fn test_str_stream() {
    let mut s = String::from("Hello");

    let _ = s.stream()
        << " darkness"
        << ',' << ' '
        << "my old friend. "
        << ('*', 3);

    assert_eq!("Hello darkness, my old friend. ***", s.as_str());
}

#[test]
fn test_str_append() {
    let mut s = String::from("Hello");

    s.append(" darkness")
        .append(",")
        .append(" ")
        .append("my old friend. ")
        .append("***");

    assert_eq!("Hello darkness, my old friend. ***", s.as_str());
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

#[test]
fn test_set_displayed_width() {
    // no change
    {
        let mut s = String::from("");
        s.set_displayed_width(0);
        assert_eq!("", s);
    }

    // expand empty
    {
        let mut s = String::from("");
        s.set_displayed_width(3);
        assert_eq!("   ", s);
    }

    // expand non-empty
    {
        let mut s = String::from("***");
        s.set_displayed_width(3);
        assert_eq!("***", s);
    }

    // non-empty with double-width char - no-change
    {
        let mut s = String::from("**😁");
        s.set_displayed_width(4);
        assert_eq!("**😁", s);
    }

    // expand non-empty with double-width char
    {
        let mut s = String::from("**😁");
        s.set_displayed_width(6);
        assert_eq!("**😁  ", s);
    }

    // expand non-empty
    {
        let mut s = String::from("***");
        s.set_displayed_width(5);
        assert_eq!("***  ", s);
    }

    // shrink to empty
    {
        let mut s = String::from("***");
        s.set_displayed_width(0);
        assert_eq!("", s);
    }

    // shrink
    {
        let mut s = String::from("***");
        s.set_displayed_width(2);
        assert_eq!("*…", s);
    }

    // shrink non-empty with double-width char
    {
        let mut s = String::from("**😁");
        s.set_displayed_width(3);
        assert_eq!("**…", s);
    }
}