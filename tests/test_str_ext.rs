//! # RTWins String extension tests

extern crate rtwins;
use rtwins::esc;
use rtwins::string_ext::*;

use pretty_assertions::assert_eq;

#[test]
fn esc_len() {
    assert_eq!(0, "".esc_seq_len());
    // Up prefixed with space
    assert_eq!(0, " \x1B[A".esc_seq_len());
    // Up
    assert_eq!(3, "\x1B[A".esc_seq_len());
    // Home
    assert_eq!(4, "\x1B[1~".esc_seq_len());
    // F1
    assert_eq!(5, "\x1B[23^".esc_seq_len());
    // F1
    assert_eq!(3, "\x1BOP".esc_seq_len());
    // C-S-F1
    assert_eq!(5, "\x1B[23@".esc_seq_len());
    // Mouse l-click
    assert_eq!(6, "\x1B[M !!".esc_seq_len());
    // Mouse wheel down
    assert_eq!(6, "\x1B[Ma$\"".esc_seq_len());
    // Home - incomplete
    assert_eq!(0, "\x1B[1".esc_seq_len());
    //  Mouse wheel down - incomplete
    assert_eq!(0, "\x1B[Ma".esc_seq_len());
}

#[test]
fn displayed_width() {
    assert_eq!(0, "".displayed_width());
    assert_eq!(5, "Title".displayed_width());
    assert_eq!(
        5,
        format!("{}Title{}", esc::BOLD, esc::NORMAL)
            .as_str()
            .displayed_width()
    );
    assert_eq!(7, "-😉-🍺-".displayed_width());
}

#[test]
fn str_stream() {
    let mut s = String::from("Hello");

    let _ = s.stream() << " darkness" << ',' << ' ' << "my old friend. " << ('*', 3);

    assert_eq!("Hello darkness, my old friend. ***", s.as_str());
}

#[test]
fn str_append() {
    let mut s = String::from("Hello");

    s.append(" darkness")
        .append(",")
        .append(" ")
        .append("my old friend. ")
        .append("***");

    assert_eq!("Hello darkness, my old friend. ***", s.as_str());
}

#[test]
fn push_esc_fmt() {
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
fn set_displayed_width() {
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

#[test]
fn erase_range() {
    // empty str
    {
        let mut s = String::from("");
        s.erase_range(0, 0);
        assert_eq!("", s);

        s.erase_range(0, 1);
        assert_eq!("", s);

        s.erase_range(1, 1);
        assert_eq!("", s);
    }

    // idx out of range
    {
        let mut s = String::from("😎");
        s.erase_range(10, 1);
        assert_eq!("😎", s);

        s.erase_range(10, 1);
        assert_eq!("😎", s);
    }

    // idx in range
    {
        let mut s = String::from("*Good-morning 🌄 star!");
        s.erase_range(0, 0);
        assert_eq!("*Good-morning 🌄 star!", s);

        s.erase_range(0, 1);
        assert_eq!("Good-morning 🌄 star!", s);

        s.erase_range(4, 1);
        assert_eq!("Goodmorning 🌄 star!", s);

        s.erase_range(4, 7);
        assert_eq!("Good 🌄 star!", s);

        s.erase_range(7, 4);
        assert_eq!("Good 🌄 !", s);

        // len out of range
        s.erase_range(1, 10);
        assert_eq!("G", s);
    }
}

#[test]
fn trim_at() {
    // empty str
    {
        let mut s = String::from("");
        s.trim_at(0);
        assert_eq!("", s);

        s.trim_at(3);
        assert_eq!("", s);
    }

    // idx out of range
    {
        let mut s = String::from("🌄!");
        s.trim_at(2);
        assert_eq!("🌄!", s);

        s.trim_at(20);
        assert_eq!("🌄!", s);
    }

    // idx in range
    {
        let mut s = String::from("Hello 🌄!!!");
        s.trim_at(8);
        assert_eq!("Hello 🌄!", s);

        s.trim_at(6);
        assert_eq!("Hello ", s);

        s.trim_at(0);
        assert_eq!("", s);
    }
}
