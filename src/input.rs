//! # RTWins terminal input constants and structures

#![allow(dead_code)]

/// ANSI control codes
#[derive(PartialEq)]
pub enum AnsiCodes {
    NUL = 0x00, // Null
    SOH = 0x01, // Start of Header
    STX = 0x02, // Start of Text
    ETX = 0x03, // End of Text
    EOT = 0x04, // End of Transmission
    ENQ = 0x05, // Enquiry
    ACK = 0x06, // Acknowledgment
    BEL = 0x07, // Bell
    BS = 0x08,  // Backspace
    HT = 0x09,  // Horizontal Tab
    LF = 0x0A,  // Line Feed
    VT = 0x0B,  // Vertical Tab
    FF = 0x0C,  // Form Feed
    CR = 0x0D,  // Carriage Return
    SO = 0x0E,  // Shift Out
    SI = 0x0F,  // Shift In
    DLE = 0x10, // Data Link Escape
    DC1 = 0x11, // XONDevice Control 1
    DC2 = 0x12, // Device Control 2
    DC3 = 0x13, // XOFFDevice Control 3
    DC4 = 0x14, // Device Control 4
    NAK = 0x15, // Negative Ack.
    SYN = 0x16, // Synchronous Idle
    ETB = 0x17, // End of Trans. Block
    CAN = 0x18, // Cancel
    EM = 0x19,  // End of Medium
    SUB = 0x1A, // Substitute
    ESC = 0x1B, // Escape
    FS = 0x1C,  // File Separator
    GS = 0x1D,  // Group Separator
    RS = 0x1E,  // Record Separator
    US = 0x1F,  // Unit Separator
    DEL = 0x7F, // Delete
}

/// Special keyboard keys
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Key {
    None,
    Esc,
    Tab,
    Enter,
    Backspace,
    Pause,
    //
    Up,
    Down,
    Left,
    Right,
    //
    Insert,
    Delete,
    Home,
    End,
    PgUp,
    PgDown,
    //
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    //
}

impl Default for Key {
    fn default() -> Self {
        Key::None
    }
}

/// Mouse button click events
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum MouseEvent {
    None,
    ButtonLeft,
    ButtonMid,
    ButtonRight,
    ButtonGoBack,
    ButtonGoForward,
    ButtonReleased,
    WheelUp,
    WheelDown,
}

impl Default for MouseEvent {
    fn default() -> Self {
        MouseEvent::None
    }
}

/// Mouse event representation
#[derive(Debug, Clone)]
pub struct MouseInfo {
    // button or wheel event
    pub evt: MouseEvent,
    // 1:1 based terminal coordinates of the event
    pub col: u8,
    pub row: u8,
}

impl Default for MouseInfo {
    fn default() -> Self {
        Self {
            evt: MouseEvent::None,
            col: 0,
            row: 0,
        }
    }
}

/// Key modifiers
pub const KEY_MOD_NONE: u8 = 0b0000;
pub const KEY_MOD_CTRL: u8 = 0b0001;
pub const KEY_MOD_ALT: u8 = 0b0010;
pub const KEY_MOD_SHIFT: u8 = 0b0100;
pub const KEY_MOD_SPECIAL: u8 = 0b1000;

/// Representation of key modifiers coded on bits
#[derive(Debug, Copy, Clone, Default)]
pub struct KeyMod {
    pub mask: u8,
}

impl KeyMod {
    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }
    pub fn has_ctrl(&self) -> bool {
        self.mask & KEY_MOD_CTRL != 0
    }
    pub fn has_alt(&self) -> bool {
        self.mask & KEY_MOD_ALT != 0
    }
    pub fn has_shift(&self) -> bool {
        self.mask & KEY_MOD_SHIFT != 0
    }
    pub fn has_special(&self) -> bool {
        self.mask & KEY_MOD_SPECIAL != 0
    }

    pub fn set_ctrl(&mut self) {
        self.mask |= KEY_MOD_CTRL;
    }
    pub fn set_alt(&mut self) {
        self.mask |= KEY_MOD_ALT;
    }
    pub fn set_shift(&mut self) {
        self.mask |= KEY_MOD_SHIFT;
    }
}

/// Buffer for UTF-8 sequence
#[derive(Debug, Default, Clone)]
pub struct CharBuff {
    pub utf8seq: [u8; 4], // UTF-8 code: 'a', '4', 'Ł'
    pub utf8sl: u8,       // length of utf8seq seq
}

impl CharBuff {
    /// Reset all fields to initial state
    pub fn reset(&mut self) {
        self.utf8seq = [0; 4];
        self.utf8sl = 0;
    }

    /// Returns proper UTF-8 slice from self.utf8seq or empty slice in case of invalid sequence
    pub fn utf8str(&self) -> &str {
        let leading_seq = self.utf8seq.split_at(self.utf8sl as usize).0;
        let res = std::str::from_utf8(leading_seq);
        if let Ok(s) = res {
            s
        }
        else {
            ""
        }
    }
}

/// Decoded input type
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// No event decoded
    None,
    /// Single UTF-8 character
    Char(CharBuff),
    /// Special key, like F1, Home
    Key(Key),
    /// Mouse event
    Mouse(MouseInfo),
}

impl Default for InputEvent {
    fn default() -> Self {
        InputEvent::None
    }
}

/// Describes decoded input
#[derive(Debug, Clone)]
pub struct InputInfo {
    /// Input type with details
    pub evnt: InputEvent,
    /// Ctrl/Alt/Shift
    pub kmod: KeyMod,
    /// Human readable event description
    pub name: &'static str,
}

impl InputInfo {
    /// Reset input to None
    pub fn reset(&mut self) {
        self.evnt = InputEvent::None;
        self.kmod.mask = 0;
        self.name = "";
    }
}

impl Default for InputInfo {
    fn default() -> Self {
        Self {
            evnt: InputEvent::None,
            kmod: KeyMod::default(),
            name: "",
        }
    }
}
