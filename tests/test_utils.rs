//! # RTWins String extension tests

extern crate rtwins;
use rtwins::utils;
use rtwins::esc;
use rtwins::string_ext::StringExt;

#[test]
fn word_wrap() {
    let mut txt = String::new();
    let _ = txt.stream()
        << esc::BOLD
        << "🔶 Lorem ipsum \ndolor sit\n amet, consectetur adipiscing elit. \n"
        << esc::NORMAL
        << "▄";

    let linesrc = utils::word_wrap(10, &txt);
    let lines = linesrc.borrow();

    assert_eq!(lines.get(0).unwrap(), String::new().append(esc::BOLD).append("🔶 Lorem "));
    assert_eq!(lines.get(1).unwrap(), "ipsum ");
    assert_eq!(lines.get(2).unwrap(), "dolor sit");
    assert_eq!(lines.get(3).unwrap(), " amet, ");
    assert_eq!(lines.get(4).unwrap(), "consectetur ");
    assert_eq!(lines.get(5).unwrap(), "adipiscing ");
    assert_eq!(lines.get(6).unwrap(), "elit. ");
    assert_eq!(lines.get(7).unwrap(), String::new().append(esc::NORMAL).append("▄"));
}
