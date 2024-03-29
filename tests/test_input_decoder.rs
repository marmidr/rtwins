//! # RTWins ESC inp decoder tests

extern crate rtwins;
use rtwins::input::*;
use rtwins::input_decoder::{Decoder, InputQue};
use rtwins::utils;

trait EasyInput {
    fn push_str(&mut self, s: &str);
}

impl EasyInput for InputQue {
    fn push_str(&mut self, s: &str) {
        self.extend(s.as_bytes().iter());
    }
}

// -----------------------------------------------------------------------------

#[test]
fn inp_empty() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    assert_eq!(0, inp.len());
    dec.decode_input_seq(&mut inp, &mut ii);

    if let InputEvent::None = ii.evnt {
        assert_eq!(KEY_MOD_NONE, ii.kmod.mask);
        assert_eq!("", ii.name);
    }
    else {
        // assert_matches!(other, InputType::None);
        assert!(false);
    }
}

#[test]
fn inp_unknown_esc() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\x1B[1234");
    dec.decode_input_seq(&mut inp, &mut ii);

    assert!(matches!(ii.evnt, InputEvent::None));

    if let InputEvent::None = ii.evnt {
        assert_eq!(KEY_MOD_NONE, ii.kmod.mask);
        assert_eq!("", ii.name);
    }
    else {
        // assert_matches!(other, InputType::None);
        assert!(false);
    }
}

#[test]
fn utf8_character_ok() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    let test_str = "Łó+dź😎";
    inp.push_str(test_str);
    let mut test_str_it = test_str.chars();
    let mut decoded = 0;

    while !inp.is_empty() {
        dec.decode_input_seq(&mut inp, &mut ii);
        assert!(matches!(ii.evnt, InputEvent::Char(_)));

        if let InputEvent::Char(ref cb) = ii.evnt {
            let kc_str = cb.as_str();

            if let Some(c) = test_str_it.next() {
                let mut cbuf = [0; 4];
                let expected = c.encode_utf8(&mut cbuf);
                assert_eq!(expected, kc_str);
                // dbg!(expected, s);
                decoded += 1;
            }

            assert_eq!(KEY_MOD_NONE, ii.kmod.mask);
            assert_eq!("<Char>", ii.name);
        }
    }

    assert_eq!(6, decoded);
}

#[test]
fn utf8_character_incomplete_ok() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    let smile = [240u8, 159, 152, 142]; // 😎
    assert_eq!(4, utils::utf8_char_width(smile[0]));
    assert_eq!(0, utils::utf8_char_width(smile[1]));
    assert_eq!(0, utils::utf8_char_width(smile[2]));
    assert_eq!(0, utils::utf8_char_width(smile[3]));
    inp.push(smile[0]);
    inp.push(smile[1]);
    inp.push(smile[2]);
    assert_eq!(3, inp.len());

    // try to decode incomplete input
    dec.decode_input_seq(&mut inp, &mut ii);
    dec.decode_input_seq(&mut inp, &mut ii);
    dec.decode_input_seq(&mut inp, &mut ii);
    assert_eq!(3, inp.len());
    assert!(matches!(ii.evnt, InputEvent::None));

    inp.push(smile[3]);
    dec.decode_input_seq(&mut inp, &mut ii);
    assert_eq!(0, inp.len());
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!(4, cb.utf8sl);
        assert_eq!('😎', cb.as_char());
    }
}

#[test]
fn not_utf8_boundary() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    let garbage = [159, 152, 142, b'+']; // ...+
    assert_eq!(0, utils::utf8_char_width(garbage[0]));
    assert_eq!(0, utils::utf8_char_width(garbage[1]));
    assert_eq!(0, utils::utf8_char_width(garbage[2]));
    assert_eq!(1, utils::utf8_char_width(garbage[3]));
    inp.push(garbage[0]);
    inp.push(garbage[1]);
    inp.push(garbage[2]);
    inp.push(garbage[3]);

    // try to decode invalid input
    dec.decode_input_seq(&mut inp, &mut ii);
    // 3 invalid bytes skipped, recognized '+'
    assert_eq!(0, inp.len());
    assert!(matches!(ii.evnt, InputEvent::Char(_)));

    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!(1, cb.utf8sl);
        assert_eq!('+', cb.as_char());
    }
}

#[test]
fn utf8_character_incomplete_err() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    let smile = [240u8, 159, 152, 142]; // 😎
    assert_eq!(4, utils::utf8_char_width(smile[0]));
    // three bytes out of 4 in sequence
    inp.push(smile[0]);
    inp.push(smile[1]);
    inp.push(smile[2]);
    // invalid byte
    inp.push(b'@');
    // character following the sequence
    inp.push(b'+');
    assert_eq!(5, inp.len());

    // try to decode invalid input - entire incomplete sequence will be skipped
    dec.decode_input_seq(&mut inp, &mut ii);
    // 3 bytes dropped, expect '@'
    assert_eq!(1, inp.len());
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!("@", cb.as_str());
    }

    // decode remaining
    dec.decode_input_seq(&mut inp, &mut ii);
    assert_eq!(0, inp.len());
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!("+", cb.as_str());
    }
}

#[test]
fn esc_followed_by_esc() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    // write and decode single ESC - first attempt shall be ignored, waiting for sequence data
    inp.push(AnsiCodes::ESC as u8);

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::None));

    // write second ESC
    inp.push(AnsiCodes::ESC as u8);
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Esc)));
    assert_eq!(KEY_MOD_SPECIAL, ii.kmod.mask);
}

#[test]
fn esc_followed_by_nothing() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    // write and decode single ESC - first attempt shall be ignored, waiting for sequence data
    inp.push(AnsiCodes::ESC as u8);
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::None));

    // second attempt to decode the same buffer - shall output ESC code
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Esc)));
}

#[test]
fn ctrl_s() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push(0x13);
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!("S", cb.as_str());
        assert_eq!(KEY_MOD_CTRL, ii.kmod.mask);
    }
}

#[test]
fn ctrl_home() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\x1B[1;5H");
    dec.decode_input_seq(&mut inp, &mut ii);

    assert!(matches!(ii.evnt, InputEvent::Key(Key::Home)));
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, ii.kmod.mask);
    assert_eq!("C-Home", ii.name);
}

#[test]
fn ctrl_f3() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\x1BOR");
    dec.decode_input_seq(&mut inp, &mut ii);

    assert!(matches!(ii.evnt, InputEvent::Key(Key::F3)));
    assert_eq!(KEY_MOD_SPECIAL, ii.kmod.mask);
    assert_eq!("F3", ii.name);
}

#[test]
fn unknown_seq_ctrl_home() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\x1B*42~");
    // valid ESC followed by '+'
    inp.push_str("\x1B[1;5H+");

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Home)));
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, ii.kmod.mask);

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!("+", cb.as_str());
        assert_eq!(KEY_MOD_NONE, ii.kmod.mask);
    }
}

#[test]
fn loong_unknown_seq_ctrl_home() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    // next ESC is more that 7 bytes further,
    // so entire buffer will be cleared
    inp.push_str("\x1B*123456789~");
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::None));

    inp.push_str("\x1B[1;5H");
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::None));

    inp.push_str("+");
    dec.decode_input_seq(&mut inp, &mut ii); // 3rd try - abandon
    assert!(matches!(ii.evnt, InputEvent::None));
    assert_eq!(0, inp.len());
}

#[test]
fn nul_in_input() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push(b'\0');
    inp.push(b'\t');
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!(1, cb.utf8sl);
    }
}

#[test]
fn ctrl_f1_incomplete() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\x1B[");
    assert_eq!(2, inp.len());
    dec.decode_input_seq(&mut inp, &mut ii);

    assert_eq!(2, inp.len());
    assert_eq!(KEY_MOD_NONE, ii.kmod.mask);
    assert!(matches!(ii.evnt, InputEvent::None));

    // write rest of previous sequence and additional sole ESC key
    inp.push_str("1;5H\x1B");
    assert_eq!(7, inp.len());
    dec.decode_input_seq(&mut inp, &mut ii);
    assert_eq!(1, inp.len());
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, ii.kmod.mask);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Home)));

    // decode rest of the inp - freestanding ESC
    // after first attempt nothing happens -> waiting for some more ESC sequence data
    dec.decode_input_seq(&mut inp, &mut ii);
    assert_eq!(1, inp.len());
    assert!(matches!(ii.evnt, InputEvent::None));
    // second attmpt - the sole ESC will be taken into consideration
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Esc)));
    assert_eq!(KEY_MOD_SPECIAL, ii.kmod.mask);
}

#[test]
fn mix_up() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    // write rest of previous sequence and additional one key
    inp.push_str("🔰\x1B[1;6AÓ*");

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!("🔰", cb.as_str());
        assert_eq!(KEY_MOD_NONE, ii.kmod.mask);
    }

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Up)));
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL, ii.kmod.mask);

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Char(_)));
    if let InputEvent::Char(ref cb) = ii.evnt {
        assert_eq!("Ó", cb.as_str());
        assert_eq!(KEY_MOD_NONE, ii.kmod.mask);
    }

    // remains '*'
    assert_eq!(1, inp.len());
}

#[test]
fn cr() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\r\r\t");

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Tab)));
}

#[test]
fn lf() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\n\n\t");

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Tab)));
}

#[test]
fn cr_lf_cr() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\n\r\n\t\n\r\t");

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Tab)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Enter)));

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Key(Key::Tab)));
}

#[test]
fn mouse_click_at_11() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\x1B[M !!");
    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Mouse(_)));
    if let InputEvent::Mouse(ref m) = ii.evnt {
        assert_eq!(MouseEvent::ButtonLeft, m.evt);
        assert_eq!(1, m.col);
        assert_eq!(1, m.row);
    }
    assert_eq!(0, ii.kmod.mask);
}

#[test]
fn mouse_wheel_down() {
    let mut dec = Decoder::default();
    let mut inp = InputQue::new();
    let mut ii = InputInfo::default();

    inp.push_str("\x1B[Ma$\"");

    dec.decode_input_seq(&mut inp, &mut ii);
    assert!(matches!(ii.evnt, InputEvent::Mouse(_)));
    if let InputEvent::Mouse(ref m) = ii.evnt {
        assert_eq!(MouseEvent::WheelDown, m.evt);
        assert_eq!(4, m.col);
        assert_eq!(2, m.row);
        assert_eq!(0, ii.kmod.mask);
    }
}
