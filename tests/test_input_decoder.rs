//! # RTWins ESC inp decoder tests

extern crate rtwins;
use rtwins::input_decoder::{Decoder, InputQue};
use rtwins::{input::*, utils};

trait EasyInput {
    fn push_back_str(&mut self, s: &str);
}

impl EasyInput for InputQue {
    fn push_back_str(&mut self, s: &str) {
        self.extend(s.as_bytes().iter());
    }
}

// -----------------------------------------------------------------------------

#[test]
fn inp_empty() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    assert_eq!(0, inp.len());
    dec.decode_input_seq(&mut inp, &mut kc);

    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!(Key::None, kc.key);
    assert_eq!("", kc.name);
}

#[test]
fn inp_unknown_esc() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\x1B[1234");
    dec.decode_input_seq(&mut inp, &mut kc);

    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!(Key::None, kc.key);
    assert_eq!("", kc.name);
}

#[test]
fn utf8_character_ok() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    let test_str = "Łó+dź😎";
    inp.push_back_str(test_str);
    let mut test_str_it = test_str.chars();
    let mut decoded = 0;

    while !inp.is_empty() {
        dec.decode_input_seq(&mut inp, &mut kc);
        let kc_utf8 = kc.utf8str();

        if let Some(c) = test_str_it.next() {
            let mut cbuf = [0; 4];
            let expected = c.encode_utf8(&mut cbuf);
            assert_eq!(expected, kc_utf8);
            // dbg!(expected, s);
            decoded += 1;
        }

        assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
        assert_eq!(Key::None, kc.key);
        assert_eq!("<.>", kc.name);
    }

    assert_eq!(6, decoded);
}

#[test]
fn utf8_character_incomplete_ok() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    let smile = [240u8, 159, 152, 142]; // 😎
    assert_eq!(4, utils::utf8_char_width(smile[0]));
    inp.push_back(smile[0]);
    inp.push_back(smile[1]);
    inp.push_back(smile[2]);
    assert_eq!(3, inp.len());

    // try to decode incomplete input
    dec.decode_input_seq(&mut inp, &mut kc);
    dec.decode_input_seq(&mut inp, &mut kc);
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(3, inp.len());
    assert_eq!(Key::None, kc.key);
    assert_eq!(0, kc.utf8sl);

    inp.push_back(smile[3]);
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(0, inp.len());
    assert_eq!(4, kc.utf8sl);
}

#[test]
fn utf8_character_incomplete_err() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    let smile = [240u8, 159, 152, 142]; // 😎
    assert_eq!(4, utils::utf8_char_width(smile[0]));
    // three bytes out of 4 in sequence
    inp.push_back(smile[0]);
    inp.push_back(smile[1]);
    inp.push_back(smile[2]);
    // invalid byte
    inp.push_back(b' ');
    // character following the sequence
    inp.push_back(b'+');
    assert_eq!(5, inp.len());

    // try to decode invalid input
    dec.decode_input_seq(&mut inp, &mut kc);
    // input drained from sequence
    assert_eq!(1, inp.len());
    assert_eq!(Key::None, kc.key);
    // sequence is invalid, thus, validation returns empty string
    assert_eq!("", kc.utf8str());

    // decode remaining
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(0, inp.len());
    assert_eq!("+", kc.utf8str());
}

#[test]
fn esc_followed_by_esc() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    // write and decode single ESC - first attempt shall be ignored, waiting for sequence data
    inp.push_back(AnsiCodes::ESC as u8);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::None, kc.key);

    // write second ESC
    inp.push_back(AnsiCodes::ESC as u8);
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Esc, kc.key);
    assert_eq!(KEY_MOD_SPECIAL, kc.kmod.mask);
}

#[test]
fn esc_followed_by_nothing() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    // write and decode single ESC - first attempt shall be ignored, waiting for sequence data
    inp.push_back(AnsiCodes::ESC as u8);
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::None, kc.key);

    // second attempt to decode the same buffer - shall output ESC code
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Esc, kc.key);
}

#[test]
fn ctrl_s() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back(0x13);
    dec.decode_input_seq(&mut inp, &mut kc);

    assert_eq!("S", kc.utf8str());
    assert_eq!(KEY_MOD_CTRL, kc.kmod.mask);
}

#[test]
fn ctrl_home() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\x1B[1;5H");
    dec.decode_input_seq(&mut inp, &mut kc);

    assert_eq!(Key::Home, kc.key);
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, kc.kmod.mask);
    assert_eq!("C-Home", kc.name);
}

#[test]
fn ctrl_f3() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\x1BOR");
    dec.decode_input_seq(&mut inp, &mut kc);

    assert_eq!(Key::F3, kc.key);
    assert_eq!(KEY_MOD_SPECIAL, kc.kmod.mask);
    assert_eq!("F3", kc.name);
}

#[test]
fn unknown_seq_ctrl_home() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\x1B*42~");
    // valid ESC followed by '+'
    inp.push_back_str("\x1B[1;5H+");

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Home, kc.key);
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, kc.kmod.mask);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!("+", kc.utf8str());
}

#[test]
fn loong_unknown_seq_ctrl_home() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    // next ESC is more that 7 bytes further,
    // so entire buffer will be cleared
    inp.push_back_str("\x1B*123456789~");
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::None, kc.key);

    inp.push_back_str("\x1B[1;5H");
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::None, kc.key);

    inp.push_back_str("+");
    dec.decode_input_seq(&mut inp, &mut kc); // 3rd try - abandon
    assert_eq!(Key::None, kc.key);
    assert_eq!(0, inp.len());
}

#[test]
fn nul_in_input() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back(b'\0');
    inp.push_back(b'\t');
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::None, kc.key);
}

#[test]
fn ctrl_f1_incomplete() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\x1B[");
    assert_eq!(2, inp.len());
    dec.decode_input_seq(&mut inp, &mut kc);

    assert_eq!(2, inp.len());
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!(Key::None, kc.key);

    // write rest of previous sequence and additional sole ESC key
    inp.push_back_str("1;5H\x1B");
    assert_eq!(7, inp.len());
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(1, inp.len());
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, kc.kmod.mask);
    assert_eq!(Key::Home, kc.key);

    // decode rest of the inp - freestanding ESC
    // after first attempt nothing happens -> waiting for some more ESC sequence data
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(1, inp.len());
    assert_eq!(Key::None, kc.key);
    // second attmpt - the sole ESC will be taken into consideration
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Esc, kc.key);
    assert_eq!(KEY_MOD_SPECIAL, kc.kmod.mask);
}

#[test]
fn mix_up() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    // write rest of previous sequence and additional one key
    inp.push_back_str("Ł\x1B[1;6AÓ*");

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!("Ł", kc.utf8str());
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Up, kc.key);
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL, kc.kmod.mask);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!("Ó", kc.utf8str());
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);

    // remains '*'
    assert_eq!(1, inp.len());
}

#[test]
fn cr() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\r\r\t");

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Tab, kc.key);
}

#[test]
fn lf() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\n\n\t");

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Tab, kc.key);
}

#[test]
fn cr_lf_cr() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\n\r\n\t\n\r\t");

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Tab, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Enter, kc.key);

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::Tab, kc.key);
}

#[test]
fn mouse_click_at_11() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\x1B[M !!");
    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::None, kc.key);
    assert_eq!(MouseBtn::ButtonLeft, kc.mouse.btn);
    assert_eq!(1, kc.mouse.col);
    assert_eq!(1, kc.mouse.row);
    assert_eq!(0, kc.kmod.mask);
}

#[test]
fn mouse_wheel_down() {
    let mut dec = Decoder::new();
    let mut inp = InputQue::new();
    let mut kc = KeyCode::new();

    inp.push_back_str("\x1B[Ma$\"");

    dec.decode_input_seq(&mut inp, &mut kc);
    assert_eq!(Key::None, kc.key);
    assert_eq!(MouseBtn::WheelDown, kc.mouse.btn);
    assert_eq!(4, kc.mouse.col);
    assert_eq!(2, kc.mouse.row);
    assert_eq!(0, kc.kmod.mask);
}
