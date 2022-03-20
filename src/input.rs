//! # RTWins terminal input constants and structures

#![allow(dead_code)]

/// ANSI control codes
#[derive(PartialEq)]
pub enum AnsiCodes {
    NUL = 0x00,  // Null
    SOH = 0x01,  // Start of Header
    STX = 0x02,  // Start of Text
    ETX = 0x03,  // End of Text
    EOT = 0x04,  // End of Transmission
    ENQ = 0x05,  // Enquiry
    ACK = 0x06,  // Acknowledgment
    BEL = 0x07,  // Bell
    BS  = 0x08,  // Backspace
    HT  = 0x09,  // Horizontal Tab
    LF  = 0x0A,  // Line Feed
    VT  = 0x0B,  // Vertical Tab
    FF  = 0x0C,  // Form Feed
    CR  = 0x0D,  // Carriage Return
    SO  = 0x0E,  // Shift Out
    SI  = 0x0F,  // Shift In
    DLE = 0x10,  // Data Link Escape
    DC1 = 0x11,  // XONDevice Control 1
    DC2 = 0x12,  // Device Control 2
    DC3 = 0x13,  // XOFFDevice Control 3
    DC4 = 0x14,  // Device Control 4
    NAK = 0x15,  // Negative Ack.
    SYN = 0x16,  // Synchronous Idle
    ETB = 0x17,  // End of Trans. Block
    CAN = 0x18,  // Cancel
    EM  = 0x19,  // End of Medium
    SUB = 0x1A,  // Substitute
    ESC = 0x1B,  // Escape
    FS  = 0x1C,  // File Separator
    GS  = 0x1D,  // Group Separator
    RS  = 0x1E,  // Record Separator
    US  = 0x1F,  // Unit Separator
    DEL = 0x7F   // Delete
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
    MouseEvent
}

/// Mouse button click events
#[derive(PartialEq)]
pub enum MouseBtn {
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

/// Mouse event representation
pub struct Mouse {
    // button or wheel event
    pub btn: MouseBtn,
    // 1:1 based terminal coordinates of the event
    pub col: u8,
    pub row: u8
}

impl Mouse {
    fn new() -> Self {
        Self{btn: MouseBtn::None, col: 0, row: 0}
    }
}

/// Key modifiers
pub const KEY_MOD_NONE: u8 = 0b0;
pub const KEY_MOD_CTRL: u8 = 0b1;
pub const KEY_MOD_ALT: u8 = 0b10;
pub const KEY_MOD_SHIFT: u8 = 0b100;
pub const KEY_MOD_SPECIAL: u8 = 0b1000;

/// Representation of key modifiers coded on bits
pub struct KeyMod {
    pub mask: u8
}

impl KeyMod {
    pub fn new() -> Self { Self {mask: 0 } }
    pub fn is_none(&self) ->    bool { self.mask == 0 }
    pub fn has_ctrl(&self) ->   bool { self.mask & KEY_MOD_CTRL != 0 }
    pub fn has_alt(&self) ->    bool { self.mask & KEY_MOD_ALT != 0 }
    pub fn has_shift(&self) ->  bool { self.mask & KEY_MOD_SHIFT != 0 }
    pub fn has_special(&self) -> bool { self.mask & KEY_MOD_SPECIAL != 0 }

    pub fn set_ctrl(&mut self)  { self.mask |= KEY_MOD_CTRL; }
    pub fn set_alt(&mut self)   { self.mask |= KEY_MOD_ALT; }
    pub fn set_shift(&mut self) { self.mask |= KEY_MOD_SHIFT; }
}

/// Decoded terminal key representation
pub struct KeyCode {
/*
    union
    {
        /** used for regular text input */
        char    utf8[5];    // NUL terminated UTF-8 code: 'a', '4', 'Ł'
        /** used for special keys */
        Key     key = {};   // 'F1', 'Enter'
        /** used for mouse events (when key == Key::MouseClick) */
        struct
        {
            // same as key above
            Key      key;
            /** button or wheel event */
            MouseBtn btn;
            /** 1:1 based terminal coordinates of the event */
            uint8_t  col;
            uint8_t  row;
        } mouse;
    };
 */

    pub utf8seq: [u8; 4],   // UTF-8 code: 'a', '4', 'Ł'
    pub utf8sl: u8,         // length of utf8seq seq
    pub key:    Key,        // 'F1', 'Enter'
    pub kmod:   KeyMod,     // Ctrl/Alt/Shift
    pub mouse:  Mouse,
    pub name:   &'static str
}

impl KeyCode {
    pub fn new() -> Self {
        Self{utf8seq: [0; 4], utf8sl: 0, key: Key::None, kmod: KeyMod::new(), mouse: Mouse::new(), name: ""}
    }

    /// Reset all fields to initial state
    pub fn reset(&mut self) {
        self.utf8seq[0] = b'\0';
        self.utf8sl = 0;
        self.key = Key::None;
        self.kmod.mask = 0;
        self.mouse.btn = MouseBtn::None;
        self.name = "";
    }

    /// Returns proper UTF-8 slice from self.utf8seq
    pub fn utf8slice<'a>(&'a self) -> &'a str {
        let res = std::str::from_utf8(self.utf8seq.split_at(self.utf8sl as usize).0);
        if let Ok(s) = res {
            s
        }
        else {
            ""
        }
    }
}