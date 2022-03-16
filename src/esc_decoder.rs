//! # RTWins decoder of terminal ESC sequences to key/mouse codes


#![allow(dead_code)]

use crate::input::*;

// -----------------------------------------------------------------------------

/// ESC sequence definition
#[derive(Copy, Clone)]
struct SeqMap
{
    // ESC sequence
    seq:    &'static str,
    // keyboard key name mapped to sequence
    name:   &'static str,
    // keyboard special key code
    key:    Key,
    // key modifiers, like KEY_MOD_CTRL
    modif:  u8,
    // seq.len()
    seqlen: u8
}

impl SeqMap {
    const fn cdeflt() -> Self {
        Self{seq: "", name: "", key: Key::None, modif: 0, seqlen: 0}
    }
}

/// CTRL key definition
#[derive(Copy, Clone)]
struct LetterMap
{
    // letter A..Z
    code:   u8,
    // key name
    name:   &'static str,
    // key code
    key:    u8,
    // key modifiers, like KEY_MOD_CTRL
    modif:  u8,
    // =1
    seqlen: u8
}

impl LetterMap {
    const fn cdeflt() -> Self {
        Self{code: 0, name: "", key: 0, modif: 0, seqlen: 0}
    }
}

/// CTRL key definition
#[derive(Copy, Clone)]
struct CtrlMap
{
    // 0x01..
    code:   u8,
    // key name
    name:   &'static str,
    // key code
    key:    Key,
    // key modifiers, like KEY_MOD_CTRL
    modif:  u8,
    // =1
    seqlen: u8
}

impl CtrlMap {
    const fn cdeflt() -> Self {
        Self{code: 0, name: "", key: Key::None, modif: 0, seqlen: 0}
    }
}

// -----------------------------------------------------------------------------

macro_rules! seq_def {
    ($s:literal, $n:literal, $k:expr, $m:expr) => {
        SeqMap{seq: $s, name: $n, key: $k, modif: $m, seqlen: $s.len() as u8}
    };
}

const ESC_KEYS_MAP_UNSORTED : [SeqMap; 155] = [
    seq_def!("[A",       "Up",           Key::Up,        KEY_MOD_SPECIAL),   // xterm
    seq_def!("[B",       "Down",         Key::Down,      KEY_MOD_SPECIAL),   // xterm
    seq_def!("[C",       "Right",        Key::Right,     KEY_MOD_SPECIAL),   // xterm
    seq_def!("[D",       "Left",         Key::Left,      KEY_MOD_SPECIAL),   // xterm
    seq_def!("[F",       "End",          Key::End,       KEY_MOD_SPECIAL),   // xterm
    seq_def!("[H",       "Home",         Key::Home,      KEY_MOD_SPECIAL),   // xterm
    seq_def!("[1~",      "Home",         Key::Home,      KEY_MOD_SPECIAL),   // vt
    seq_def!("[2~",      "Ins",          Key::Insert,    KEY_MOD_SPECIAL),   // vt
    seq_def!("[3~",      "Del",          Key::Delete,    KEY_MOD_SPECIAL),   // vt
    seq_def!("[4~",      "End",          Key::End,       KEY_MOD_SPECIAL),   // vt
    seq_def!("[5~",      "PgUp",         Key::PgUp,      KEY_MOD_SPECIAL),   // vt
    seq_def!("[6~",      "PdDown",       Key::PgDown,    KEY_MOD_SPECIAL),   // vt
    seq_def!("[7~",      "Home",         Key::Home,      KEY_MOD_SPECIAL),   // vt
    seq_def!("[8~",      "End",          Key::End,       KEY_MOD_SPECIAL),   // vt
    seq_def!("OP",       "F1",           Key::F1,        KEY_MOD_SPECIAL),
    seq_def!("OQ",       "F2",           Key::F2,        KEY_MOD_SPECIAL),
    seq_def!("OR",       "F3",           Key::F3,        KEY_MOD_SPECIAL),
    seq_def!("OS",       "F4",           Key::F4,        KEY_MOD_SPECIAL),
    seq_def!("[11~",     "F1",           Key::F1,        KEY_MOD_SPECIAL),
    seq_def!("[12~",     "F2",           Key::F2,        KEY_MOD_SPECIAL),
    seq_def!("[13~",     "F3",           Key::F3,        KEY_MOD_SPECIAL),
    seq_def!("[14~",     "F4",           Key::F4,        KEY_MOD_SPECIAL),
    seq_def!("[15~",     "F5",           Key::F5,        KEY_MOD_SPECIAL),
    seq_def!("[17~",     "F6",           Key::F6,        KEY_MOD_SPECIAL),
    seq_def!("[18~",     "F7",           Key::F7,        KEY_MOD_SPECIAL),
    seq_def!("[19~",     "F8",           Key::F8,        KEY_MOD_SPECIAL),
    seq_def!("[20~",     "F9",           Key::F9,        KEY_MOD_SPECIAL),
    seq_def!("[21~",     "F10",          Key::F10,       KEY_MOD_SPECIAL),
    seq_def!("[23~",     "F11",          Key::F11,       KEY_MOD_SPECIAL),
    seq_def!("[24~",     "F12",          Key::F12,       KEY_MOD_SPECIAL),
    // + Shift
    seq_def!("[1;2A",    "S-Up",         Key::Up,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2B",    "S-Down",       Key::Down,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2C",    "S-Right",      Key::Right,     KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2D",    "S-Left",       Key::Left,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2F",    "S-End",        Key::End,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2H",    "S-Home",       Key::Home,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2~",    "S-Home",       Key::Home,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[2;2~",    "S-Ins",        Key::Insert,    KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[3;2~",    "S-Del",        Key::Delete,    KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[4;2~",    "S-End",        Key::End,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[5;2~",    "S-PgUp",       Key::PgUp,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[6;2~",    "S-PdDown",     Key::PgDown,    KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2P",    "S-F1",         Key::F1,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2Q",    "S-F2",         Key::F2,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2R",    "S-F3",         Key::F3,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[1;2S",    "S-F4",         Key::F4,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[15;2~",   "S-F5",         Key::F5,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[17;2~",   "S-F6",         Key::F6,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[18;2~",   "S-F7",         Key::F7,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[19;2~",   "S-F8",         Key::F8,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[20;2~",   "S-F9",         Key::F9,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[21;2~",   "S-F10",        Key::F10,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[23;2~",   "S-F11",        Key::F11,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[24;2~",   "S-F12",        Key::F12,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    seq_def!("[Z",       "S-Tab",        Key::Tab,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT),
    // + Alt
    seq_def!("[1;3A",    "M-Up",         Key::Up,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3B",    "M-Down",       Key::Down,      KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3C",    "M-Right",      Key::Right,     KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3D",    "M-Left",       Key::Left,      KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3F",    "M-End",        Key::End,       KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3H",    "M-Home",       Key::Home,      KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[2;3~",    "M-Ins",        Key::Insert,    KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[3;3~",    "M-Del",        Key::Delete,    KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[5;3~",    "M-PgUp",       Key::PgUp,      KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[6;3~",    "M-PdDown",     Key::PgDown,    KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3P",    "M-F1",         Key::F1,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3Q",    "M-F2",         Key::F2,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3R",    "M-F3",         Key::F3,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[1;3S",    "M-F4",         Key::F4,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[15;3~",   "M-F5",         Key::F5,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[17;3~",   "M-F6",         Key::F6,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[18;3~",   "M-F7",         Key::F7,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[19;3~",   "M-F8",         Key::F8,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[20;3~",   "M-F9",         Key::F9,        KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[21;3~",   "M-F10",        Key::F10,       KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[23;3~",   "M-F11",        Key::F11,       KEY_MOD_SPECIAL | KEY_MOD_ALT),
    seq_def!("[24;3~",   "M-F12",        Key::F12,       KEY_MOD_SPECIAL | KEY_MOD_ALT),
    // + Ctrl
    seq_def!("[1;5A",    "C-Up",         Key::Up,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5B",    "C-Down",       Key::Down,      KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5C",    "C-Right",      Key::Right,     KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5D",    "C-Left",       Key::Left,      KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5F",    "C-End",        Key::End,       KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5H",    "C-Home",       Key::Home,      KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[2;5~",    "C-Ins",        Key::Insert,    KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[3;5~",    "C-Del",        Key::Delete,    KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[5;5~",    "C-PgUp",       Key::PgUp,      KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[6;5~",    "C-PdDown",     Key::PgDown,    KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5P",    "C-F1",         Key::F1,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5Q",    "C-F2",         Key::F2,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5R",    "C-F3",         Key::F3,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[1;5S",    "C-F4",         Key::F4,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[15;5~",   "C-F5",         Key::F5,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[17;5~",   "C-F6",         Key::F6,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[18;5~",   "C-F7",         Key::F7,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[19;5~",   "C-F8",         Key::F8,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[20;5~",   "C-F9",         Key::F9,        KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[21;5~",   "C-F10",        Key::F10,       KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[23;5~",   "C-F11",        Key::F11,       KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    seq_def!("[24;5~",   "C-F12",        Key::F12,       KEY_MOD_SPECIAL | KEY_MOD_CTRL),
    // + Shit + Ctrl
    seq_def!("[1;6A",    "S-C-Up",       Key::Up,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6B",    "S-C-Down",     Key::Down,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6C",    "S-C-Right",    Key::Right,     KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6D",    "S-C-Left",     Key::Left,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6F",    "S-C-End",      Key::End,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6H",    "S-C-Home",     Key::Home,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[2;6~",    "S-C-Ins",      Key::Insert,    KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[3;6~",    "S-C-Del",      Key::Delete,    KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[5;6~",    "S-C-PgUp",     Key::PgUp,      KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[6;6~",    "S-C-PdDown",   Key::PgDown,    KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6P",    "S-C-F1",       Key::F1,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6Q",    "S-C-F2",       Key::F2,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6R",    "S-C-F3",       Key::F3,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[1;6S",    "S-C-F4",       Key::F4,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[15;6~",   "S-C-F5",       Key::F5,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[17;6~",   "S-C-F6",       Key::F6,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[18;6~",   "S-C-F7",       Key::F7,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[19;6~",   "S-C-F8",       Key::F8,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[20;6~",   "S-C-F9",       Key::F9,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[21;6~",   "S-C-F10",      Key::F10,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[23;6~",   "S-C-F11",      Key::F11,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[24;6~",   "S-C-F12",      Key::F12,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[23^",     "S-C-F1",       Key::F1,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[24^",     "S-C-F2",       Key::F2,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[25^",     "S-C-F3",       Key::F3,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[26^",     "S-C-F4",       Key::F4,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[28^",     "S-C-F5",       Key::F5,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[29^",     "S-C-F6",       Key::F6,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[31^",     "S-C-F7",       Key::F7,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[32^",     "S-C-F8",       Key::F8,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[33^",     "S-C-F9",       Key::F9,        KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[34^",     "S-C-F10",      Key::F10,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[23@",     "S-C-F11",      Key::F11,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    seq_def!("[24@",     "S-C-F12",      Key::F12,       KEY_MOD_SPECIAL | KEY_MOD_SHIFT | KEY_MOD_CTRL),
    // + Ctrl + Alt
    seq_def!("[1;7A",    "C-M-Up",       Key::Up,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7B",    "C-M-Down",     Key::Down,      KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7C",    "C-M-Right",    Key::Right,     KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7D",    "C-M-Left",     Key::Left,      KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7F",    "C-M-End",      Key::End,       KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7H",    "C-M-Home",     Key::Home,      KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[2;7~",    "C-M-Ins",      Key::Insert,    KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[3;7~",    "C-M-Del",      Key::Delete,    KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[5;7~",    "C-M-PgUp",     Key::PgUp,      KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[6;7~",    "C-M-PdDown",   Key::PgDown,    KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7P",    "C-M-F1",       Key::F1,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7Q",    "C-M-F2",       Key::F2,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7R",    "C-M-F3",       Key::F3,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[1;7S",    "C-M-F4",       Key::F4,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[15;7~",   "C-M-F5",       Key::F5,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[17;7~",   "C-M-F6",       Key::F6,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[18;7~",   "C-M-F7",       Key::F7,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[19;7~",   "C-M-F8",       Key::F8,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[20;7~",   "C-M-F9",       Key::F9,        KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[21;7~",   "C-M-F10",      Key::F10,       KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[23;7~",   "C-M-F11",      Key::F11,       KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    seq_def!("[24;7~",   "C-M-F12",      Key::F12,       KEY_MOD_SPECIAL | KEY_MOD_CTRL | KEY_MOD_ALT),
    // + Shift + Alt
];

macro_rules! letter_def {
    ($c:literal, $n:literal, $k:literal, $m:expr) => {
        LetterMap{code: $c, name: $n, key: $k, modif: $m, seqlen: 1}
    };
}

const CTRL_KEYS_MAP_UNSORTED : [LetterMap; 26] = [
    // letter_def!( 0, "C-2", '2', KEY_MOD_CTRL)
    letter_def!(0x01, "C-A", b'A', KEY_MOD_CTRL),
    letter_def!(0x02, "C-B", b'B', KEY_MOD_CTRL),
    letter_def!(0x03, "C-C", b'C', KEY_MOD_CTRL),
    letter_def!(0x04, "C-D", b'D', KEY_MOD_CTRL),
    letter_def!(0x05, "C-E", b'E', KEY_MOD_CTRL),
    letter_def!(0x06, "C-F", b'F', KEY_MOD_CTRL),
    letter_def!(0x07, "C-G", b'G', KEY_MOD_CTRL),
    letter_def!(0x08, "C-H", b'H', KEY_MOD_CTRL), // BS
    letter_def!(0x09, "C-I", b'I', KEY_MOD_CTRL), // HT
    letter_def!(0x0A, "C-J", b'J', KEY_MOD_CTRL), // LF
    letter_def!(0x0B, "C-K", b'K', KEY_MOD_CTRL),
    letter_def!(0x0C, "C-L", b'L', KEY_MOD_CTRL),
    letter_def!(0x0D, "C-M", b'M', KEY_MOD_CTRL), // CR
    letter_def!(0x0E, "C-N", b'N', KEY_MOD_CTRL),
    letter_def!(0x0F, "C-O", b'O', KEY_MOD_CTRL),
    letter_def!(0x10, "C-P", b'P', KEY_MOD_CTRL),
    letter_def!(0x11, "C-Q", b'Q', KEY_MOD_CTRL),
    letter_def!(0x12, "C-R", b'R', KEY_MOD_CTRL),
    letter_def!(0x13, "C-S", b'S', KEY_MOD_CTRL),
    letter_def!(0x14, "C-T", b'T', KEY_MOD_CTRL),
    letter_def!(0x15, "C-U", b'U', KEY_MOD_CTRL),
    letter_def!(0x16, "C-V", b'V', KEY_MOD_CTRL),
    letter_def!(0x17, "C-W", b'W', KEY_MOD_CTRL),
    letter_def!(0x18, "C-X", b'X', KEY_MOD_CTRL),
    letter_def!(0x19, "C-Y", b'Y', KEY_MOD_CTRL),
    letter_def!(0x1A, "C-Z", b'Z', KEY_MOD_CTRL),
];

macro_rules! ctrl_def {
    ($c:expr, $n:literal, $k:expr, $m:expr) => {
        CtrlMap{code: $c as u8, name: $n, key: $k, modif: $m, seqlen: 1}
    };
}

const SPECIAL_KEYS_MAP_UNSORTED : [CtrlMap; 8] = [
    ctrl_def!(AnsiCodes::DEL,   "Backspace",  Key::Backspace, KEY_MOD_SPECIAL),
    ctrl_def!(AnsiCodes::HT,    "Tab",        Key::Tab,       KEY_MOD_SPECIAL),
    ctrl_def!(AnsiCodes::LF,    "Enter",      Key::Enter,     KEY_MOD_SPECIAL),
    ctrl_def!(AnsiCodes::CR,    "Enter",      Key::Enter,     KEY_MOD_SPECIAL),
    ctrl_def!(AnsiCodes::ESC,   "Esc",        Key::Esc,       KEY_MOD_SPECIAL),
    ctrl_def!(AnsiCodes::GS,    "Pause",      Key::Pause,     KEY_MOD_SPECIAL),
    // ctrl_def!(0x17,             "C-Bspc",     Key::Backspace, KEY_MOD_SPECIAL | KEY_MOD_CTRL), // VSCode
    // ctrl_def!(0x08,             "S-Bspc",     Key::Backspace, KEY_MOD_SPECIAL | KEY_MOD_SHIFT), // VSCode
    ctrl_def!(AnsiCodes::RS,    "C-Enter",    Key::Enter,     KEY_MOD_SPECIAL | KEY_MOD_CTRL), // mintty
    ctrl_def!(AnsiCodes::US,    "C-Bspc",     Key::Backspace, KEY_MOD_SPECIAL | KEY_MOD_CTRL), // mintty
];

// -----------------------------------------------------------------------------

/*
/// @brief constexpr comparison operator needed for sort function
constexpr bool operator <(const SeqMap &left, const SeqMap &right)
{
    const char *pl = left.seq;
    const char *pr = right.seq;

    while (*pl == *pr)
        pl++, pr++;
    return *pl < *pr;
}

/// @brief constexpr swap
template<class T>
constexpr void cex_swap(T& lho, T& rho)
{
    T tmp = std::move(lho);
    lho = std::move(rho);
    rho = std::move(tmp);
}

/// @brief constexpr sort
template <typename T, unsigned N>
constexpr void cex_sort_impl(Array<T, N> &array, unsigned left, unsigned right)
{
    if (left < right)
    {
        unsigned m = left;

        for (unsigned i = left + 1; i < right; i++)
            if (array[i] < array[left])
                cex_swap(array[++m], array[i]);

        cex_swap(array[left], array[m]);

        cex_sort_impl(array, left, m);
        cex_sort_impl(array, m + 1, right);
    }
}

/// @brief returns constexpr sorted array
template <typename T, unsigned N>
constexpr Array<T, N> cex_sort_arr(Array<T, N> array)
{
    auto sorted_array = array;
    cex_sort_impl(sorted_array, 0, N);
    return sorted_array;
}

*/


const fn sort_keys<const N: usize>(input: &[CtrlMap]) -> [CtrlMap; N] {
    let mut out = [CtrlMap::cdeflt(); N];
    let mut i = 0usize;

    while i < input.len() {
        out[i] = input[i];
        i += 1;
    }

    out
}

const SPECIAL_KEYS_MAP_SORTED : [CtrlMap; SPECIAL_KEYS_MAP_UNSORTED.len()] = sort_keys(&SPECIAL_KEYS_MAP_UNSORTED);
