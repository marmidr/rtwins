//! RTWins Widget

#![allow(dead_code)]

use super::colors::*;

/// Screen coordinates
pub struct Coord
{
    pub col: u8,
    pub row: u8
}

impl Coord
{
    /// Returns default object; can be used in `const` initialization
    pub const fn dflt()  -> Coord {
        Coord{col: 0, row: 0}
    }
}

/// Widget size
pub struct Size
{
    pub width: u8,
    pub height: u8
}

impl Size {
    /// Returns default object; can be used in `const` initialization
    pub const fn dflt() -> Size {
        Size{width: 0, height: 0}
    }
}

/// Rectangle area
pub struct Rect
{
    coord: Coord,
    size : Size
}

impl Rect
{
    pub fn set_max(&mut self) {
        self.coord.col = 1;
        self.coord.row = 1;
        self.size.width = 0xff;
        self.size.height = 0xff;
    }
}

impl Rect
{
    /// Returns default object; can be used in `const` initialization
    const fn dflt() -> Rect {
        Rect{coord: Coord::dflt(), size: Size::dflt()}
    }
}

/// Widget unique identifier
pub type WID = u16;
pub const WIDGET_ID_NONE: WID = 0;    // convenient; default value points to nothing
pub const WIDGET_ID_ALL: WID = u16::MAX;

/// Widget type with all specific data
pub enum Type
{
    None,
    Window {
        title: &'static str,
        fg_color: ColorFG,
        bg_color: ColorBG,
        is_popup: bool,
        // get_state: fn() -> &IWindowState
    },
    Panel,
    Label,
    TextEdit,
    CheckBox,
    Radio,
    Button,
    Led,
    PageCtrl,
    Page,
    ProgressBar,
    ListBox,
    ComboBox,
    CustomWgt,
    TextBox,
    Layer,
}

/// Widget itself
pub struct Widget
{
    pub id: WID,
    pub coord: Coord,
    pub size: Size,
    pub typ: Type,
}

enum Prop {
    Chbx {
        checked: bool
    },
    Led {
        txt: &'static str,
        lit: bool
    },
    Lbx {
        item_idx: i16,
        sel_idx: i16
    },
    Cbx {
        item_idx: i16,
        sel_idx: i16,
        drop_down: bool
    },
    Pgbar {
        pos: i32,
        max: i32
    },
    Txtbx {
        top_line: i16
    }
}

pub struct WidgetProp
{
    prop: Prop,
    // applies to every widget
    enabled: bool
}
