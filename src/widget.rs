//! RTWins Widget

#![allow(dead_code)]

/// Screen coordinates
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct Rect
{
    coord: Coord,
    size : Size
}

impl Rect
{
    /// Returns default object; can be used in `const` initialization
    const fn cdeflt() -> Rect {
        Rect{coord: Coord::cdeflt(), size: Size::cdeflt()}
    }

    pub fn set_max(&mut self) {
        self.coord.col = 1;
        self.coord.row = 1;
        self.size.width = u8::MAX;
        self.size.height = u8::MAX;
    }
}

/// Visual style of button
#[derive(Copy, Clone)]
pub enum ButtonStyle
{
    Simple,
    Solid,
    Solid1p5,
}

/// Visual style of Progress Bar
#[derive(Copy, Clone)]
pub enum PgBarStyle
{
    /// #
    Hash,
    ///  ▒
    Shade,
    /// □
    Rectangle,
}


/// Widget unique identifier
pub type WId = u16;

/// Convenient, default value that points to nothing
pub const WIDGET_ID_NONE: WId = u16::MIN;
/// special function parameter
pub const WIDGET_ID_ALL: WId = u16::MAX;

/// Widgets properties
pub mod wp
{
    use super::super::colors::*;
    use super::ButtonStyle;
    use super::PgBarStyle;

    #[derive(Copy, Clone)]
    pub struct Window {
        pub title   : &'static str,
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub is_popup: bool,
        // get_state: fn() -> &IWindowState
    }

    impl Window {
        pub const fn into(self) -> super::Type {
            super::Type::Window(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Panel {
        pub title   : &'static str,
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub no_frame: bool
    }

    impl Panel {
        pub const fn into(self) -> super::Type {
            super::Type::Panel(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Label {
        pub title   : &'static str,
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
    }

    impl Label {
        pub const fn into(self) -> super::Type {
            super::Type::Label(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct TextEdit {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
    }

    impl TextEdit {
        pub const fn into(self) -> super::Type {
            super::Type::TextEdit(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct CheckBox {
        pub text    : &'static str,
        pub fg_color: ColorFG,
    }

    impl CheckBox {
        pub const fn into(self) -> super::Type {
            super::Type::CheckBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Radio {
        pub text    : &'static str,
        pub fg_color: ColorFG,
        pub group_id: u16,
        pub radio_id: u16
    }

    impl Radio {
        pub const fn into(self) -> super::Type {
            super::Type::Radio(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Button {
        pub text    : &'static str,
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub style   : ButtonStyle
    }

    impl Button {
        pub const fn into(self) -> super::Type {
            super::Type::Button(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Led {
        pub text        : &'static str,
        pub fg_color    : ColorFG,
        pub bg_color_off: ColorBG,
        pub bg_color_on : ColorBG
    }

    impl Led {
        pub const fn into(self) -> super::Type {
            super::Type::Led(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct PageCtrl {
        pub tab_width   : u8,
        pub vert_offs   : u8
    }

    impl PageCtrl {
        pub const fn into(self) -> super::Type {
            super::Type::PageCtrl(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Page {
        pub title       : &'static str,
        pub fg_color    : ColorFG,
    }

    impl Page {
        pub const fn into(self) -> super::Type {
            super::Type::Page(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ProgressBar {
        pub fg_color    : ColorFG,
        pub style       : PgBarStyle
    }

    impl ProgressBar {
        pub const fn into(self) -> super::Type {
            super::Type::ProgressBar(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ListBox {
        pub fg_color    : ColorFG,
        pub bg_color    : ColorBG,
        pub no_frame    : bool
    }

    impl ListBox {
        pub const fn into(self) -> super::Type {
            super::Type::ListBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ComboBox {
        pub fg_color        : ColorFG,
        pub bg_color        : ColorBG,
        pub drop_down_size  : u8
    }

    impl ComboBox {
        pub const fn into(self) -> super::Type {
            super::Type::ComboBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct CustomWgt {
    }

    impl CustomWgt {
        pub const fn into(self) -> super::Type {
            super::Type::CustomWgt(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct TextBox {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
    }

    impl TextBox {
        pub const fn into(self) -> super::Type {
            super::Type::TextBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Layer {
    }

    impl Layer {
        pub const fn into(self) -> super::Type {
            super::Type::Layer(self)
        }
    }
}

/// Widget type with all specific data
#[derive(Copy, Clone)]
pub enum Type
{
    NoWgt,
    Window(wp::Window),
    Panel(wp::Panel),
    Label(wp::Label),
    TextEdit(wp::TextEdit),
    CheckBox(wp::CheckBox),
    Radio(wp::Radio),
    Button(wp::Button),
    Led(wp::Led),
    PageCtrl(wp::PageCtrl),
    Page(wp::Page),
    ProgressBar(wp::ProgressBar),
    ListBox(wp::ListBox),
    ComboBox(wp::ComboBox),
    CustomWgt(wp::CustomWgt),
    TextBox(wp::TextBox),
    Layer(wp::Layer),
}

impl Type {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt()  -> Self {
        Self::NoWgt
    }

    /// Extract window properties from enum
    // pub fn prop_wnd<'a>(&'a self) -> &'a wp::Window {
    pub fn prop_wnd(&self) -> &wp::Window {
        match self {
            Self::Window(ref wp) => wp,
            _ => panic!()
        }
    }

    /// Extract panel properties from enum
    pub fn prop_pnl(&self) -> &wp::Panel {
        match self {
            Self::Panel(ref wp) => wp,
            _ => panic!()
        }
    }
}

/// Widget itself
#[derive(Copy, Clone)]
pub struct Widget
{
    /// Unique widget ID
    pub id      : WId,
    pub parent  : WId,
    /// coordinates
    pub coord   : Coord,
    /// widget size
    pub size    : Size,
    /// widget type with properties
    pub typ     : Type,
    /// link to children widgets, 2x8B
    pub link    : &'static[Widget]
}

impl Widget {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt()  -> Self {
        Widget{
            id      : WIDGET_ID_NONE,
            parent  : WIDGET_ID_NONE,
            coord   : Coord::cdeflt(),
            size    : Size::cdeflt(),
            typ     : Type::cdeflt(),
            link    : &[]
        }
    }
}

// union IntOrFloat {
//     i: u32,
//     f: f32,
// }

#[derive(Copy, Clone)]
struct Idx
{
    own_idx     : u16,
    parent_idx  : u16,
    childs_idx  : u16,
    childs_cnt  : u16,
}

union Link
{
    addr: u32,
    idx : Idx
}

/// Widgets dynamic properties
enum DynProp
{
    Chbx {
        checked: bool
    },
    Led {
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

pub struct WidgetDynProp
{
    prop    : DynProp,
    // applies to every widget
    enabled : bool
}
