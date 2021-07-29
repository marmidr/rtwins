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
    pub const fn cdeflt()  -> Coord {
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
    pub const fn cdeflt() -> Size {
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
        self.size.width = u8::MAX;
        self.size.height = u8::MAX;
    }
}

impl Rect
{
    /// Returns default object; can be used in `const` initialization
    const fn cdeflt() -> Rect {
        Rect{coord: Coord::cdeflt(), size: Size::cdeflt()}
    }
}

/// Widget unique identifier
pub type WID = u16;
/// convenient; default value points to nothing
pub const WIDGET_ID_NONE: WID = u16::MIN;
/// special function parameter
pub const WIDGET_ID_ALL: WID = u16::MAX;

/// Widget type with all specific data
pub enum Type
{
    None,
    Window {
        title   : &'static str,
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
    /// Unique widget ID
    pub id: WID,
    /// coordinates
    pub coord: Coord,
    /// widget size
    pub size: Size,
    /// widget type with
    pub typ: Type,
    /// link to children widgets
    pub link: &'static[Widget]
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
