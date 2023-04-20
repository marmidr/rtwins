//! # RTWins String extension tests

extern crate rtwins;
use rtwins::esc;
use rtwins::string_ext::*;

use pretty_assertions::assert_eq;

#[test]
fn esc_len() {
    assert_eq!(0, "".esc_seq_len());
    // Bold
    assert_eq!(3, "\x1B1m".esc_seq_len());
    // Normal
    assert_eq!(4, "\x1B22m".esc_seq_len());
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
    assert_eq!(9, "Multi\nLine".displayed_width());
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
fn set_displayed_width_with_esc() {
    // expand with ESC sentences
    {
        let mut s = String::new();
        s.push_str(esc::BOLD);
        s.push_str("Item");
        s.push_str(esc::NORMAL);
        s.push_str("#");

        let mut s_exppected = s.clone();
        s_exppected.push_str("  ");
        s.set_displayed_width(7);
        assert_eq!(s_exppected, s);
    }

    // shrink with ESC sentences
    {
        let mut s = String::new();
        s.push_str(esc::BOLD);
        s.push_str("Item");
        s.push_str(esc::NORMAL);

        let mut s_exppected = s.clone();
        s.push_str(" 0034567890123456789*");
        s.set_displayed_width(16);

        s_exppected.push_str(" 0034567890…");
        assert_eq!(s_exppected, s);
    }

    // shrink with ESC sentences
    {
        let mut s = String::new();
        s.push_str(esc::BOLD);
        let mut s_exppected = s.clone();

        s.push_str("Item");
        s.push_str(esc::NORMAL);
        s.set_displayed_width(1);

        s_exppected.push_str("…");
        assert_eq!(s_exppected, s);
    }
}

#[test]
fn erase_char_range() {
    // empty str
    {
        let mut s = String::from("");
        s.erase_char_range(0, 0);
        assert_eq!("", s);

        s.erase_char_range(0, 1);
        assert_eq!("", s);

        s.erase_char_range(1, 1);
        assert_eq!("", s);
    }

    // idx out of range
    {
        let mut s = String::from("😎");
        s.erase_char_range(10, 1);
        assert_eq!("😎", s);

        s.erase_char_range(10, 1);
        assert_eq!("😎", s);
    }

    // idx in range
    {
        let mut s = String::from("*Good-morning 🌄 star!");
        s.erase_char_range(0, 0);
        assert_eq!("*Good-morning 🌄 star!", s);

        s.erase_char_range(0, 1);
        assert_eq!("Good-morning 🌄 star!", s);

        s.erase_char_range(4, 1);
        assert_eq!("Goodmorning 🌄 star!", s);

        s.erase_char_range(4, 7);
        assert_eq!("Good 🌄 star!", s);

        s.erase_char_range(7, 4);
        assert_eq!("Good 🌄 !", s);

        // len out of range
        s.erase_char_range(1, 10);
        assert_eq!("G", s);
    }
}

#[test]
fn trim_at_char_idx() {
    // empty str
    {
        let mut s = String::from("");
        s.trim_at_char_idx(0);
        assert_eq!("", s);

        s.trim_at_char_idx(3);
        assert_eq!("", s);
    }

    // idx out of range
    {
        let mut s = String::from("🌄!");
        s.trim_at_char_idx(2);
        assert_eq!("🌄!", s);

        s.trim_at_char_idx(20);
        assert_eq!("🌄!", s);
    }

    // idx in range
    {
        let mut s = String::from("Hello 🌄!!!");
        s.trim_at_char_idx(8);
        assert_eq!("Hello 🌄!", s);

        s.trim_at_char_idx(6);
        assert_eq!("Hello ", s);

        s.trim_at_char_idx(0);
        assert_eq!("", s);
    }
}

#[test]
fn insert_str_at_char_idx() {
    let mut s = String::from("🌄");
    s.insert_str_at_char_idx(0, "+");
    assert_eq!("+🌄", s);

    s.insert_str_at_char_idx(2, "-");
    assert_eq!("+🌄-", s);
}

#[test]
fn split_at_char() {
    let s = String::from("Łódź 🌄 wita!");
    assert_eq!("", s.split_at_char_idx(100));
    assert_eq!("Łódź 🌄 wita!", s.split_at_char_idx(0));
    assert_eq!("🌄 wita!", s.split_at_char_idx(5));
}
