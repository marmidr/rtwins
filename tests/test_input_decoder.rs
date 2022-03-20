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

    inp.push_back_str("\033[1234");
    dec.decode_input_seq(&mut inp, &mut kc);

    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!(Key::None, kc.key);
    assert_eq!("<.>", kc.name);
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

/*
fn Ctrl_S)
{
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write((char)0x13);
    decodeInputSeq(inp, kc);

    assert_eq!(KEY_MOD_CTRL, kc.kmod.mask);
    assert_eq!("S", kc.utf8);
    EXPECT_STRNE("", kc.name);
    EXPECT_STRNE("<?>", kc.name);
}

fn Ctrl_F1)
{
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write("\033[1;5H");
    decodeInputSeq(inp, kc);

    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, kc.kmod.mask);
    assert_eq!(Key::Home, kc.key);
    EXPECT_STRNE("", kc.name);
    EXPECT_STRNE("<?>", kc.name);
}

fn UnknownSeq__Ctrl_Home)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write("\033*42~");
    inp.write("\033[1;5H@");

    decodeInputSeq(inp, kc);
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, kc.kmod.mask);
    assert_eq!(Key::Home, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!("@", kc.utf8);
}

fn LoongUnknownSeq__Ctrl_Home)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    // next ESC is more that 7 bytes further,
    // so entire buffer will be cleared
    inp.write("\033*123456789~");
    decodeInputSeq(inp, kc);
    assert_eq!(Key::None, kc.key);

    inp.write("\033[1;5H");
    decodeInputSeq(inp, kc);
    assert_eq!(Key::None, kc.key);

    inp.write("+");
    decodeInputSeq(inp, kc); // 3rd try - abandon
    assert_eq!(Key::None, kc.key);
    assert_eq!(0, inp.size());
}

fn NUL_InInput)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write('\0');
    inp.write("\t");
    decodeInputSeq(inp, kc);
    assert_eq!(Key::None, kc.key);
}

fn DISABLED_Ctrl_F1__incomplete)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    EXPECT_TRUE(inp.write("\033["));
    assert_eq!(2, inp.size());
    decodeInputSeq(inp, kc);

    assert_eq!(2, inp.size());
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!(Key::None, kc.key);

    // write rest of previous sequence and additional one key
    EXPECT_TRUE(inp.write("1;5H\033"));
    assert_eq!(7, inp.size());
    decodeInputSeq(inp, kc);
    assert_eq!(1, inp.size());
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_CTRL, kc.kmod.mask);
    assert_eq!(Key::Home, kc.key);

    // decode rest of the inp
    decodeInputSeq(inp, kc);
    assert_eq!(0, inp.size());
    assert_eq!(KEY_MOD_SPECIAL, kc.kmod.mask);
    assert_eq!(Key::Esc, kc.key);
}

fn L__S_C_UP__O)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    // write rest of previous sequence and additional one key
    EXPECT_TRUE(inp.write("Ł\033[1;6AÓ*"));

    decodeInputSeq(inp, kc);
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!("Ł", kc.utf8);

    decodeInputSeq(inp, kc);
    assert_eq!(KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL, kc.kmod.mask);
    assert_eq!(Key::Up, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(KEY_MOD_NONE, kc.kmod.mask);
    assert_eq!("Ó", kc.utf8);

    // remains '*'
    assert_eq!(1, inp.size());
}

fn CR)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write("\r\r\t");

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Tab, kc.key);
}

fn LF)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write("\n\n\t");

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Tab, kc.key);
}

fn CR_LF_CR)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write("\n\r\n\t\n\r\t");

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Tab, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Enter, kc.key);

    decodeInputSeq(inp, kc);
    assert_eq!(Key::Tab, kc.key);
}

fn Mouse_click_at_11)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write("\e[M !!");

    decodeInputSeq(inp, kc);
    assert_eq!(Key::MouseEvent, kc.key);
    assert_eq!(twins::MouseBtn::ButtonLeft, kc.mouse.btn);
    assert_eq!(1, kc.mouse.col);
    assert_eq!(1, kc.mouse.row);
    assert_eq!(0, kc.kmod.mask);
}

fn Mouse_wheel_down)
{
    twins::decodeInputSeqReset();
    twins::RingBuff<char> inp(rbBuffer);
    twins::KeyCode kc;

    inp.write("\e[Ma$\"");

    decodeInputSeq(inp, kc);
    assert_eq!(Key::MouseEvent, kc.key);
    assert_eq!(twins::MouseBtn::WheelDown, kc.mouse.btn);
    assert_eq!(4, kc.mouse.col);
    assert_eq!(2, kc.mouse.row);
    assert_eq!(0, kc.kmod.mask);
}
 */