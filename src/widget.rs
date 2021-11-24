//! # RTWins Widget

#![allow(dead_code)]

/// Widget screen coordinates
#[derive(Clone, Copy)]
pub struct Coord {
    pub col: u8,
    pub row: u8,
}

impl Coord {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Coord {
        Coord { col: 0, row: 0 }
    }
}

/// Widget size
#[derive(Clone, Copy)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

impl Size {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Size {
        Size {
            width: 0,
            height: 0,
        }
    }
}

/// Rectangle area
#[derive(Clone, Copy)]
pub struct Rect {
    coord: Coord,
    size: Size,
}

impl Rect {
    /// Returns default object; can be used in `const` initialization
    const fn cdeflt() -> Rect {
        Rect {
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
        }
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
pub enum ButtonStyle {
    Simple,
    Solid,
    Solid1p5,
}

/// Visual style of Progress Bar
#[derive(Copy, Clone)]
pub enum PgBarStyle {
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
pub const WIDGET_ID_NONE: WId = WId::MIN;
/// Used as a special function parameter
pub const WIDGET_ID_ALL: WId = WId::MAX;

/// Widgets properties
pub mod prop {
    use super::super::colors::*;
    use super::ButtonStyle;
    use super::PgBarStyle;

    #[derive(Copy, Clone)]
    pub struct Window {
        pub title: &'static str,
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
        pub title: &'static str,
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub no_frame: bool,
    }

    impl Panel {
        pub const fn into(self) -> super::Type {
            super::Type::Panel(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Label {
        pub title: &'static str,
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
        pub text: &'static str,
        pub fg_color: ColorFG,
    }

    impl CheckBox {
        pub const fn into(self) -> super::Type {
            super::Type::CheckBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Radio {
        pub text: &'static str,
        pub fg_color: ColorFG,
        pub group_id: u16,
        pub radio_id: u16,
    }

    impl Radio {
        pub const fn into(self) -> super::Type {
            super::Type::Radio(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Button {
        pub text: &'static str,
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub style: ButtonStyle,
    }

    impl Button {
        pub const fn into(self) -> super::Type {
            super::Type::Button(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Led {
        pub text: &'static str,
        pub fg_color: ColorFG,
        pub bg_color_off: ColorBG,
        pub bg_color_on: ColorBG,
    }

    impl Led {
        pub const fn into(self) -> super::Type {
            super::Type::Led(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct PageCtrl {
        pub tab_width: u8,
        pub vert_offs: u8,
    }

    impl PageCtrl {
        pub const fn into(self) -> super::Type {
            super::Type::PageCtrl(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Page {
        pub title: &'static str,
        pub fg_color: ColorFG,
    }

    impl Page {
        pub const fn into(self) -> super::Type {
            super::Type::Page(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ProgressBar {
        pub fg_color: ColorFG,
        pub style: PgBarStyle,
    }

    impl ProgressBar {
        pub const fn into(self) -> super::Type {
            super::Type::ProgressBar(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ListBox {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub no_frame: bool,
    }

    impl ListBox {
        pub const fn into(self) -> super::Type {
            super::Type::ListBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ComboBox {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub drop_down_size: u8,
    }

    impl ComboBox {
        pub const fn into(self) -> super::Type {
            super::Type::ComboBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct CustomWgt {}

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
    pub struct Layer {}

    impl Layer {
        pub const fn into(self) -> super::Type {
            super::Type::Layer(self)
        }
    }
}

/// Widget type with all specific data
#[derive(Copy, Clone)]
pub enum Type {
    NoWgt,
    Window(prop::Window),
    Panel(prop::Panel),
    Label(prop::Label),
    TextEdit(prop::TextEdit),
    CheckBox(prop::CheckBox),
    Radio(prop::Radio),
    Button(prop::Button),
    Led(prop::Led),
    PageCtrl(prop::PageCtrl),
    Page(prop::Page),
    ProgressBar(prop::ProgressBar),
    ListBox(prop::ListBox),
    ComboBox(prop::ComboBox),
    CustomWgt(prop::CustomWgt),
    TextBox(prop::TextBox),
    Layer(prop::Layer),
}

use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::NoWgt         => "NoWgt",
            Self::Window(_)     => "Window",
            Self::Panel(_)      => "Panel",
            Self::Label(_)      => "Label",
            Self::TextEdit(_)   => "TextEdit",
            Self::CheckBox(_)   => "CheckBox",
            Self::Radio(_)      => "Radio",
            Self::Button(_)     => "Button",
            Self::Led(_)        => "Led",
            Self::PageCtrl(_)   => "PageCtrl",
            Self::Page(_)       => "Page",
            Self::ProgressBar(_) => "ProgressBar",
            Self::ListBox(_)    => "ListBox",
            Self::ComboBox(_)   => "ComboBox",
            Self::CustomWgt(_)  => "CustomWgt",
            Self::TextBox(_)    => "TextBox",
            Self::Layer(_)      => "Layer",
        };
        write!(f, "{}", name)
    }
}

impl Type {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Self::NoWgt
    }

    /// Extract window properties from enum
    // pub fn prop_wnd<'a>(&'a self) -> &'a wp::Window {
    pub fn prop_wnd(&self) -> &prop::Window {
        match self {
            Self::Window(ref wp) => wp,
            _ => panic!(),
        }
    }

    /// Extract panel properties from enum
    pub fn prop_pnl(&self) -> &prop::Panel {
        match self {
            Self::Panel(ref wp) => wp,
            _ => panic!(),
        }
    }
}

/// Widget itself
#[derive(Copy, Clone)]
pub struct Widget {
    /// Unique widget ID
    pub id: WId,
    /// indexes used after flattening widget tree to array
    pub link: Link,
    /// coordinates
    pub coord: Coord,
    /// widget size
    pub size: Size,
    /// widget type with properties
    pub typ: Type,
    /// link to children widgets, 2x8B
    pub childs: &'static [Widget],
}

impl Widget {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Widget {
            id: WIDGET_ID_NONE,
            link: Link::cdeflt(),
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: Type::cdeflt(),
            childs: &[],
        }
    }
}

// union IntOrFloat {
//     i: u32,
//     f: f32,
// }

#[derive(Copy, Clone)]
pub struct Link {
    pub own_idx:    u16,
    pub parent_idx: u16,
    pub childs_idx: u16,
    pub childs_cnt: u16,
}

impl Link {
    pub const fn cdeflt() -> Self {
        Link{ own_idx: 0, parent_idx: 0, childs_idx: 0, childs_cnt: 0}
    }
}

/// Widgets dynamic properties
enum DynProp {
    Chbx {
        checked: bool,
    },
    Led {
        lit: bool,
    },
    Lbx {
        item_idx: i16,
        sel_idx: i16,
    },
    Cbx {
        item_idx: i16,
        sel_idx: i16,
        drop_down: bool,
    },
    Pgbar {
        pos: i32,
        max: i32,
    },
    Txtbx {
        top_line: i16,
    },
}

pub struct WidgetDynProp {
    prop: DynProp,
    // applies to every widget
    enabled: bool,
}
