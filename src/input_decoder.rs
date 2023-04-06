//! # RTWins decoder of terminal ESC sequences to key/mouse codes

#![allow(dead_code)]

use crate::input::*;
use std::cmp::Ordering;

pub type InputQue = Vec<u8>;

// -----------------------------------------------------------------------------

/// ESC sequence definition
#[derive(Copy, Clone)]
struct SeqMap {
    // ESC sequence
    seq: &'static str,
    // keyboard key name mapped to sequence
    name: &'static str,
    // keyboard special key code
    key: Key,
    // key modifiers, like KEY_MOD_CTRL
    kmod: u8,
}

impl SeqMap {
    const fn cdeflt() -> Self {
        Self {
            seq: "",
            name: "",
            key: Key::None,
            kmod: 0,
        }
    }
}

/// CTRL key definition
#[derive(Copy, Clone)]
struct LetterMap {
    // letter A..Z
    code: u8,
    // key code
    key: u8,
    // key modifiers, like KEY_MOD_CTRL
    kmod: u8,
    // key name
    name: &'static str,
}

impl LetterMap {
    const fn cdeflt() -> Self {
        Self {
            code: 0,
            key: 0,
            kmod: 0,
            name: "",
        }
    }
}

/// CTRL key definition
#[derive(Copy, Clone)]
struct SpecialMap {
    // 0x01..
    code: u8,
    // key code
    key: Key,
    // key modifiers, like KEY_MOD_CTRL
    kmod: u8,
    // key name
    name: &'static str,
}

impl SpecialMap {
    const fn cdeflt() -> Self {
        Self {
            code: 0,
            key: Key::None,
            kmod: 0,
            name: "",
        }
    }
}

// -----------------------------------------------------------------------------

macro_rules! seq_def {
    ($s:literal, $n:literal, $k:expr, $m:expr) => {
        SeqMap {
            seq: $s,
            name: $n,
            key: $k,
            kmod: $m,
        }
    };
}

#[rustfmt::skip]
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
    seq_def!("[6~",      "PgDown",       Key::PgDown,    KEY_MOD_SPECIAL),   // vt
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

const ESC_KEYS_MAP_SORTED: [SeqMap; ESC_KEYS_MAP_UNSORTED.len()] = sort_seq(&ESC_KEYS_MAP_UNSORTED);

macro_rules! ctrl_def {
    ($c:literal, $n:literal, $k:literal, $m:expr) => {
        LetterMap {
            code: $c,
            name: $n,
            key: $k,
            kmod: $m,
        }
    };
}

#[rustfmt::skip]
const CTRL_KEYS_MAP_SORTED : [LetterMap; 26] = [
    // letter_def!( 0, "C-2", '2', KEY_MOD_CTRL)
    ctrl_def!(0x01, "C-A", b'A', KEY_MOD_CTRL),
    ctrl_def!(0x02, "C-B", b'B', KEY_MOD_CTRL),
    ctrl_def!(0x03, "C-C", b'C', KEY_MOD_CTRL),
    ctrl_def!(0x04, "C-D", b'D', KEY_MOD_CTRL),
    ctrl_def!(0x05, "C-E", b'E', KEY_MOD_CTRL),
    ctrl_def!(0x06, "C-F", b'F', KEY_MOD_CTRL),
    ctrl_def!(0x07, "C-G", b'G', KEY_MOD_CTRL),
    ctrl_def!(0x08, "C-H", b'H', KEY_MOD_CTRL), // BS
    ctrl_def!(0x09, "C-I", b'I', KEY_MOD_CTRL), // HT
    ctrl_def!(0x0A, "C-J", b'J', KEY_MOD_CTRL), // LF
    ctrl_def!(0x0B, "C-K", b'K', KEY_MOD_CTRL),
    ctrl_def!(0x0C, "C-L", b'L', KEY_MOD_CTRL),
    ctrl_def!(0x0D, "C-M", b'M', KEY_MOD_CTRL), // CR
    ctrl_def!(0x0E, "C-N", b'N', KEY_MOD_CTRL),
    ctrl_def!(0x0F, "C-O", b'O', KEY_MOD_CTRL),
    ctrl_def!(0x10, "C-P", b'P', KEY_MOD_CTRL),
    ctrl_def!(0x11, "C-Q", b'Q', KEY_MOD_CTRL),
    ctrl_def!(0x12, "C-R", b'R', KEY_MOD_CTRL),
    ctrl_def!(0x13, "C-S", b'S', KEY_MOD_CTRL),
    ctrl_def!(0x14, "C-T", b'T', KEY_MOD_CTRL),
    ctrl_def!(0x15, "C-U", b'U', KEY_MOD_CTRL),
    ctrl_def!(0x16, "C-V", b'V', KEY_MOD_CTRL),
    ctrl_def!(0x17, "C-W", b'W', KEY_MOD_CTRL),
    ctrl_def!(0x18, "C-X", b'X', KEY_MOD_CTRL),
    ctrl_def!(0x19, "C-Y", b'Y', KEY_MOD_CTRL),
    ctrl_def!(0x1A, "C-Z", b'Z', KEY_MOD_CTRL),
];

macro_rules! special_def {
    ($c:expr, $n:literal, $k:expr, $m:expr) => {
        SpecialMap {
            code: $c as u8,
            key: $k,
            kmod: $m,
            name: $n,
        }
    };
}

#[rustfmt::skip]
const SPECIAL_KEYS_MAP_UNSORTED : [SpecialMap; 8] = [
    special_def!(AnsiCodes::DEL,   "Backspace",  Key::Backspace, KEY_MOD_SPECIAL),
    special_def!(AnsiCodes::HT,    "Tab",        Key::Tab,       KEY_MOD_SPECIAL),
    special_def!(AnsiCodes::LF,    "Enter",      Key::Enter,     KEY_MOD_SPECIAL),
    special_def!(AnsiCodes::CR,    "Enter",      Key::Enter,     KEY_MOD_SPECIAL),
    special_def!(AnsiCodes::ESC,   "Esc",        Key::Esc,       KEY_MOD_SPECIAL),
    special_def!(AnsiCodes::GS,    "Pause",      Key::Pause,     KEY_MOD_SPECIAL),
    // special_def!(0x17,             "C-Bspc",     Key::Backspace, KEY_MOD_SPECIAL | KEY_MOD_CTRL), // VSCode
    // special_def!(0x08,             "S-Bspc",     Key::Backspace, KEY_MOD_SPECIAL | KEY_MOD_SHIFT), // VSCode
    special_def!(AnsiCodes::RS,    "C-Enter",    Key::Enter,     KEY_MOD_SPECIAL | KEY_MOD_CTRL), // mintty
    special_def!(AnsiCodes::US,    "C-Bspc",     Key::Backspace, KEY_MOD_SPECIAL | KEY_MOD_CTRL), // mintty
];

// -----------------------------------------------------------------------------

/// Compile time sorting of sequence array that allows fast searching
const fn sort_seq<const N: usize>(input: &[SeqMap]) -> [SeqMap; N] {
    let mut out = [SeqMap::cdeflt(); N];
    let mut i = 0usize;

    while i < input.len() {
        out[i] = input[i];
        i += 1;
    }

    out = sort(out);
    out
}

/// bubble-sort
/// q-sort cannot be used due to the stack limit
const fn sort<const N: usize>(mut array: [SeqMap; N]) -> [SeqMap; N] {
    if array.len() > 1 {
        let mut l = 0usize;

        while l < array.len() - 1 {
            let mut r = l + 1;

            while r < array.len() {
                #[allow(clippy::manual_swap)]
                if seqmap_cmp(&array[l], &array[r]).is_gt() {
                    // manual swap; array.swap() not available in const function
                    let tmp = array[l];
                    array[l] = array[r];
                    array[r] = tmp;
                }
                r += 1;
            }
            l += 1;
        }
    }

    array
}

/// Compares two SeqMap using method similar to strcmp
const fn seqmap_cmp(left: &SeqMap, right: &SeqMap) -> Ordering {
    seq_cmp(left.seq.as_bytes(), right.seq.as_bytes()).1
}

/// Compares two byte slices using method similar to strcmp
/// .0 == true - left begins with right, but latter they may differ
const fn seq_cmp(left: &[u8], right: &[u8]) -> (bool, Ordering) {
    let commn_len = if left.len() < right.len() {
        left.len()
    }
    else {
        right.len()
    };
    let mut i = 0usize;

    while i < commn_len {
        if left[i] != right[i] {
            if left[i] < right[i] {
                return (false, Ordering::Less);
            }
            else {
                return (false, Ordering::Greater);
            }
        }
        i += 1;
    }

    if left.len() == right.len() {
        (true, Ordering::Equal)
    }
    else if left.len() < right.len() {
        (true, Ordering::Less)
    }
    else {
        (true, Ordering::Greater)
    }
}

// -----------------------------------------------------------------------------

pub fn print_seq() {
    for s in ESC_KEYS_MAP_SORTED.iter() {
        println!("{:7} -> {}", s.seq, s.name);
    }
}

/// Fast binary search of key-sequence `sequence` in sorted `map`.
/// Returns Some(SeqMap) if found, None otherwise.
fn seq_binary_search(sequence: &[u8], map: &'static [SeqMap]) -> Option<&'static SeqMap> {
    if sequence.len() < 2 || sequence[0] == 0 {
        return None;
    }

    let mut lo = 0isize;
    let mut hi = map.len() as isize - 1;
    let mut mid = (hi - lo) / 2;

    loop {
        // map[mid].seq must not necessary be equal to seq, but be at the beginning of it
        let (startswith, cmp) = seq_cmp(sequence, map[mid as usize].seq.as_bytes());

        if startswith {
            return Some(&map[mid as usize]);
        }
        else if cmp.is_gt() {
            lo = mid + 1;
        }
        else {
            hi = mid - 1;
        }

        if hi < lo {
            break;
        }
        mid = lo + ((hi - lo) / 2);
    }

    // seq not found
    None
}

// -----------------------------------------------------------------------------

/// ESC sequence into Key description decoder
pub struct Decoder {
    decode_fail_ctr: u8,
    prev_cr: u8,
    prev_esc_ignored: bool,
}

impl Decoder {
    /// Reset the state; for testing purposes
    #[cfg(test)]
    pub fn reset_state(&mut self) {
        self.decode_fail_ctr = 0;
        self.prev_cr = 0;
        self.prev_esc_ignored = false;
    }

    /// Decodes input ESC sequence fetching bytes from queue;
    /// fills the output with decoded key/mouse event,
    /// Returns number of bytes consumed from the queue; 0 if no valid data found
    pub fn decode_input_seq(&mut self, input: &mut InputQue, inp_info: &mut InputInfo) -> u8 {
        inp_info.reset();

        if input.is_empty() {
            return 0;
        }

        let read_seq_from_queue = |inp: &InputQue, out: &mut [u8]| -> usize {
            // out.copy_from_slice() impossible as Deque is noncontiguous
            let count = (out.len() - 1).min(inp.len());
            let mut it = inp.iter();
            #[allow(clippy::needless_range_loop)]
            for i in 0..count {
                out[i] = *it.next().unwrap_or(&0);
            }

            out[count] = 0; // NUL-term
            count
        };

        let mut seq = [0u8; crate::esc::SEQ_MAX_LENGTH];

        while !input.is_empty() {
            let seq_sz = read_seq_from_queue(input, &mut seq);
            self.prev_cr >>= 1; // set = 2 and then shift is faster than: if(prevCR) prevCR--;

            // 1. ANSI escape sequence
            //    check for two following ESC characters to avoid lock
            if seq_sz > 1 && seq[0] == AnsiCodes::ESC as u8 && seq[1] != AnsiCodes::ESC as u8 {
                if seq_sz < 3 {
                    // sequence too short
                    return 0;
                }

                self.prev_esc_ignored = false;

                // check mouse code
                if seq_sz >= 6 && seq[1] == b'[' && seq[2] == b'M' {
                    let mouse_btn = seq[3] - b' ';
                    let mut mi = MouseInfo::default();
                    match mouse_btn & 0xE3 {
                        0x00 => mi.evt = MouseEvent::ButtonLeft,
                        0x01 => mi.evt = MouseEvent::ButtonMid,
                        0x02 => mi.evt = MouseEvent::ButtonRight,
                        0x03 => mi.evt = MouseEvent::ButtonReleased,
                        0x80 => mi.evt = MouseEvent::ButtonGoBack,
                        0x81 => mi.evt = MouseEvent::ButtonGoForward,
                        0x40 => mi.evt = MouseEvent::WheelUp,
                        0x41 => mi.evt = MouseEvent::WheelDown,
                        _ => mi.evt = MouseEvent::None,
                    }

                    if mouse_btn & 0x04 != 0 {
                        inp_info.kmod.set_shift();
                    }
                    if mouse_btn & 0x08 != 0 {
                        inp_info.kmod.set_alt();
                    }
                    if mouse_btn & 0x10 != 0 {
                        inp_info.kmod.set_ctrl();
                    }

                    mi.col = seq[4] - b' ';
                    mi.row = seq[5] - b' ';
                    inp_info.evnt = InputEvent::Mouse(mi);
                    inp_info.name = "MouseEvent";
                    input.drain(..6);
                    return 6;
                }

                // binary search: find key map in max 7 steps
                if let Some(km) = seq_binary_search(&seq[1..seq_sz], &ESC_KEYS_MAP_SORTED) {
                    inp_info.evnt = InputEvent::Key(km.key);
                    inp_info.kmod.mask = km.kmod;
                    inp_info.name = km.name;
                    input.drain(..1 + km.seq.len()); // +1 for ESC
                    return 1 + km.seq.len() as u8;
                }

                // ESC sequence invalid or unknown?
                if seq_sz > 3 {
                    // 3 is mimimum ESC seq len
                    let mut esc_found = false;
                    // data is long enough to store ESC sequence
                    #[allow(clippy::needless_range_loop)]
                    for i in 1..seq_sz {
                        if seq[i] == AnsiCodes::ESC as u8 {
                            esc_found = true;
                            // found next ESC, current seq is unknown
                            input.drain(..i);
                            //dbg!("found at ", i);
                            break;
                        }
                    }

                    if esc_found {
                        continue;
                    }
                }

                self.decode_fail_ctr += 1;
                if self.decode_fail_ctr == 3 {
                    // invalid sequence; drop entire buffer
                    self.decode_fail_ctr = 0;
                    input.clear();
                }

                return 0;
            }
            else {
                // single character
                if seq[0] == AnsiCodes::ESC as u8 && seq_sz == 1 && !self.prev_esc_ignored {
                    // avoid situations where ESC not followed by another code
                    // is decoded as freestanding Esc key
                    self.prev_esc_ignored = true;
                    return 0;
                }

                self.prev_esc_ignored = false;
                let mut skip = false;

                // 2. check for special key
                // note: it conflicts with ctrl_keys_map[] but has higher priority
                for km in SPECIAL_KEYS_MAP_UNSORTED.iter() {
                    if seq[0] == km.code {
                        if seq[0] == AnsiCodes::CR as u8 {
                            // CR   -> treat as LF
                            // CRLF -> ignore LF
                            //   LF -> LF
                            self.prev_cr = 2;
                        }
                        else if seq[0] == AnsiCodes::LF as u8 && self.prev_cr > 0 {
                            input.drain(..1);
                            self.prev_cr = 0;
                            skip = true;
                            break;
                        }

                        inp_info.evnt = InputEvent::Key(km.key);
                        inp_info.kmod.mask = km.kmod;
                        inp_info.name = km.name;
                        input.drain(..1);
                        return 1;
                    }
                }

                if skip {
                    continue;
                }

                // 3. check for one of Ctrl+[A..Z]
                if let Some(km) = CTRL_KEYS_MAP_SORTED.iter().find(|&&x| x.code == seq[0]) {
                    let mut cb = CharBuff::default();
                    cb.utf8seq[0] = km.key;
                    cb.utf8sl = 1;
                    inp_info.evnt = InputEvent::Char(cb);
                    inp_info.kmod.mask = km.kmod;
                    inp_info.name = km.name;
                    input.drain(..1);
                    return 1;
                }

                // 4. regular ASCII character or UTF-8 sequence
                let utf8seqlen = crate::utils::utf8_char_width(seq[0]); // 0..4
                                                                        // dbg!(seq, seq_sz, utf8seqlen);

                if utf8seqlen > 0 && utf8seqlen <= seq_sz {
                    // copy valid UTF-8 seq
                    let mut cb = CharBuff::default();
                    cb.utf8seq[0] = seq[0];
                    cb.utf8seq[1] = seq[1];
                    cb.utf8seq[2] = seq[2];
                    cb.utf8seq[3] = seq[3];
                    cb.utf8sl = utf8seqlen as u8;
                    inp_info.evnt = InputEvent::Char(cb);
                    inp_info.name = "<.>";

                    input.drain(..utf8seqlen);
                    return utf8seqlen as u8;
                }
                else {
                    // invalid/incomplete sequence?
                    if utf8seqlen > 0 {
                        // try next time with more data
                        return 0;
                    }
                    else {
                        // invalid byte
                        input.drain(..1);
                    }
                }
            }
        }

        0
    }
}

impl Default for Decoder {
    /// Creates a new decoder instance
    fn default() -> Self {
        Self {
            decode_fail_ctr: 0,
            prev_cr: 0,
            prev_esc_ignored: false,
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // tests of local stuff only
    use super::*;

    #[test]
    fn test_seq_cmp() {
        {
            // same length, equal
            let l = SeqMap {
                seq: "AAA",
                ..SeqMap::cdeflt()
            };
            let r = SeqMap {
                seq: "AAA",
                ..SeqMap::cdeflt()
            };
            assert_eq!(Ordering::Equal, seqmap_cmp(&l, &r));
        }

        {
            // same length, inequal
            let l = SeqMap {
                seq: "AAA",
                ..SeqMap::cdeflt()
            };
            let r = SeqMap {
                seq: "AAB",
                ..SeqMap::cdeflt()
            };
            assert_eq!(Ordering::Less, seqmap_cmp(&l, &r));
            assert_eq!(Ordering::Greater, seqmap_cmp(&r, &l));
        }

        {
            // similar, diferent lenght
            let l = SeqMap {
                seq: "AAA",
                ..SeqMap::cdeflt()
            };
            let r = SeqMap {
                seq: "AAAA",
                ..SeqMap::cdeflt()
            };
            assert_eq!(Ordering::Less, seqmap_cmp(&l, &r));
            assert_eq!(Ordering::Greater, seqmap_cmp(&r, &l));
        }

        {
            // different content and length
            let l = SeqMap {
                seq: "AAB",
                ..SeqMap::cdeflt()
            };
            let r = SeqMap {
                seq: "AAAA",
                ..SeqMap::cdeflt()
            };
            assert_eq!(Ordering::Greater, seqmap_cmp(&l, &r));
            assert_eq!(Ordering::Less, seqmap_cmp(&r, &l));
        }
    }

    #[test]
    fn test_seq_binary_search() {
        {
            // empy input
            let opt = seq_binary_search(b"", &ESC_KEYS_MAP_SORTED);
            assert!(opt.is_none());
        }

        {
            // input too short
            let opt = seq_binary_search(b"[", &ESC_KEYS_MAP_SORTED);
            assert!(opt.is_none());
        }

        {
            // input unknown
            let opt = seq_binary_search(b"[abc", &ESC_KEYS_MAP_SORTED);
            assert!(opt.is_none());
        }

        {
            // input followed by "BC"
            let opt = seq_binary_search(b"[ABC", &ESC_KEYS_MAP_SORTED);
            assert!(opt.is_some());
            if let Some(km) = opt {
                assert_eq!(Key::Up, km.key);
            }
        }

        {
            // valid input
            let opt = seq_binary_search(b"[H", &ESC_KEYS_MAP_SORTED);
            assert!(opt.is_some());
            if let Some(km) = opt {
                assert_eq!(Key::Home, km.key);
            }
        }

        {
            // valid input
            let opt = seq_binary_search(b"[24;5~", &ESC_KEYS_MAP_SORTED);
            assert!(opt.is_some());
            if let Some(km) = opt {
                assert_eq!(Key::F12, km.key);
            }
        }

        {
            // valid input
            let opt = seq_binary_search(b"[24;6~", &ESC_KEYS_MAP_SORTED);
            assert!(opt.is_some());
            if let Some(km) = opt {
                assert_eq!(Key::F12, km.key);
                assert_eq!(KEY_MOD_CTRL | KEY_MOD_SHIFT | KEY_MOD_SPECIAL, km.kmod);
            }
        }
    }
} // mod tests
