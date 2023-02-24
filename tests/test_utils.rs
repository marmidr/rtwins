//! # RTWins String extension tests

extern crate rtwins;
use rtwins::esc;
use rtwins::input::*;
use rtwins::string_ext::StringExt;
use rtwins::utils;

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

    assert_eq!(
        lines.get(0).unwrap(),
        String::new().append(esc::BOLD).append("🔶 Lorem ")
    );
    assert_eq!(lines.get(1).unwrap(), "ipsum ");
    assert_eq!(lines.get(2).unwrap(), "dolor sit");
    assert_eq!(lines.get(3).unwrap(), " amet, ");
    assert_eq!(lines.get(4).unwrap(), "consectetur ");
    assert_eq!(lines.get(5).unwrap(), "adipiscing ");
    assert_eq!(lines.get(6).unwrap(), "elit. ");
    assert_eq!(
        lines.get(7).unwrap(),
        String::new().append(esc::NORMAL).append("▄")
    );
}

#[test]
fn num_edit() {
    let mut ii = InputInfo::default();
    let mut txt = String::new();
    let mut cursor_pos = 0i16;

    // invalid key - handled as 'rejected'
    let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -100, 100, false);
    assert!(!handled);

    // too long number
    {
        txt.push_n('9', 20);
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -100, 100, false);
        assert!(!handled);
    }

    // out of range
    {
        // empty or under limit
        txt.clear();
        ii.evnt = InputEvent::Key(Key::Enter);
        ii.kmod.mask = KEY_MOD_SPECIAL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, 33, 911, false);
        assert!(!handled);
        assert_eq!("33", txt);

        // over limit
        txt.clear();
        txt.append("912");
        ii.evnt = InputEvent::Key(Key::Enter);
        ii.kmod.mask = KEY_MOD_SPECIAL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, 33, 911, false);
        assert!(!handled);
        assert_eq!("911", txt);
    }

    // edit accept - to be handled by caller
    {
        txt.clear();
        ii.evnt = InputEvent::Key(Key::Enter);
        ii.kmod.mask = KEY_MOD_SPECIAL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -100, 100, false);
        assert!(!handled);
    }

    // edit abort - to be handled by caller
    {
        ii.evnt = InputEvent::Key(Key::Esc);
        ii.kmod.mask = KEY_MOD_SPECIAL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -100, 100, false);
        assert!(!handled);
    }

    // UP arrow
    {
        txt.clear();
        ii.evnt = InputEvent::Key(Key::Up);
        ii.kmod.mask = KEY_MOD_SPECIAL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -100, 100, false);
        assert!(handled);
        assert_eq!("1", txt);

        // Ctrl+UP
        ii.evnt = InputEvent::Key(Key::Up);
        ii.kmod.mask = KEY_MOD_SPECIAL | KEY_MOD_CTRL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1000, 1000, false);
        assert!(handled);
        assert_eq!("11", txt);

        // Shift+UP
        ii.evnt = InputEvent::Key(Key::Up);
        ii.kmod.mask = KEY_MOD_SPECIAL | KEY_MOD_SHIFT;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1000, 1000, false);
        assert!(handled);
        assert_eq!("111", txt);
    }

    // DOWN arrow
    {
        ii.evnt = InputEvent::Key(Key::Down);
        ii.kmod.mask = KEY_MOD_SPECIAL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1000, 1000, false);
        assert!(handled);
        assert_eq!("110", txt);

        // Ctrl+Down
        ii.evnt = InputEvent::Key(Key::Down);
        ii.kmod.mask = KEY_MOD_SPECIAL | KEY_MOD_CTRL;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1000, 1000, false);
        assert!(handled);
        assert_eq!("100", txt);

        // Shift+Down
        ii.evnt = InputEvent::Key(Key::Down);
        ii.kmod.mask = KEY_MOD_SPECIAL | KEY_MOD_SHIFT;
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1000, 1000, false);
        assert!(handled);
        assert_eq!("0", txt);
    }
}

#[test]
fn num_edit_limited() {
    let mut ii = InputInfo::default();

    let mut txt;
    let mut cursor_pos = 0i16;

    // up, no wrap
    ii.evnt = InputEvent::Key(Key::Up);
    ii.kmod.mask = KEY_MOD_SPECIAL;

    {
        txt = "0".to_owned();

        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1, 1, false);
        assert!(handled);
        assert_eq!("1", txt);

        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1, 1, false);
        assert!(handled);
        assert_eq!("1", txt);
    }

    // up, wrap
    {
        txt = "1".to_owned();

        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1, 1, true);
        assert!(handled);
        assert_eq!("-1", txt);
    }

    // down, no wrap
    ii.evnt = InputEvent::Key(Key::Down);
    ii.kmod.mask = KEY_MOD_SPECIAL;

    {
        txt = "0".to_owned();

        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1, 1, false);
        assert!(handled);
        assert_eq!("-1", txt);

        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1, 1, false);
        assert!(handled);
        assert_eq!("-1", txt);
    }

    // down, wrap
    {
        txt = "-1".to_owned();
        let handled = utils::num_edit_input_evt(&ii, &mut txt, &mut cursor_pos, -1, 1, true);
        assert!(handled);
        assert_eq!("1", txt);
    }
}
