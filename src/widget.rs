//! # RTWins Widget

#![allow(dead_code)]
#![allow(unused_variables)]

use super::input::*;

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

/// Window, panel and page control frame styles
pub enum FrameStyle {
    None,
    Single,
    Double,
    PgControl,
    ListBox
}

// -----------------------------------------------------------------------------------------------

/// Widget unique identifier
pub type WId = u16;

/// Convenient, default value that points to nothing
pub const WIDGET_ID_NONE: WId = WId::MIN;
/// Used as a special function parameter
pub const WIDGET_ID_ALL: WId = WId::MAX;

// -----------------------------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------------------------

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

/// Structure used in flattened widgets structure navigation.
/// Filled up in compile time
#[derive(Copy, Clone)]
pub struct Link {
    pub own_idx:    u16,
    pub parent_idx: u16,
    pub childs_idx: u16,
    pub childs_cnt: u16,
}

impl Link {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Link{ own_idx: 0, parent_idx: 0, childs_idx: 0, childs_cnt: 0 }
    }
}

// -----------------------------------------------------------------------------------------------

/// Widget runtime state
pub enum RuntimeState {
    None,
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
    Pgctrl {
        page: u8
    }
}

pub struct WidgetState {
    // particular widget type state
    pub state: RuntimeState,
    // applies to every widget
    pub enabled: bool,
}

impl WidgetState {
    pub fn new() -> Self {
        WidgetState{state: RuntimeState::None, enabled: true}
    }
}

// -----------------------------------------------------------------------------------------------

/// Window state and event handler
pub trait WindowState {
    fn init(&self, wgts: &[Widget]) {}
    fn get_widgets(&self) -> &[Widget] { return &[]; }

    /// events
    fn on_button_down(&mut self, wgt: &Widget, kc: &KeyCode) {}
    fn on_button_up(&mut self, wgt: &Widget, kc: &KeyCode) {}
    fn on_button_click(&mut self, wgt: &Widget, kc: &KeyCode) {}
    fn on_text_edit_change(&mut self, wgt: &Widget, txt: &mut String) {}
    fn on_text_edit_input_evt(&mut self, wgt: &Widget, kc: &KeyCode, txt: &mut String, cursor_pos: &mut i16) -> bool { return false; }
    fn on_checkbox_toggle(&mut self, wgt: &Widget) {}
    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: u8) {}
    fn on_list_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {}
    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {}
    fn on_combo_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {}
    fn on_combo_box_change(&mut self, wgt: &Widget, new_idx: i16) {}
    fn on_combo_box_drop(&mut self, wgt: &Widget, drop_state: bool) {}
    fn on_radio_select(&mut self, wgt: &Widget) {}
    fn on_text_box_scroll(&mut self, wgt: &Widget, new_top_line: i16) {}
    fn on_custom_widget_draw(&mut self, wgt: &Widget) {}
    fn on_custom_widget_input_evt(&mut self, wgt: &Widget, kc: &KeyCode) -> bool { return false; }
    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, kc: &KeyCode) -> bool { return false; }

    /// common state queries
    fn is_enabled(&mut self, wgt: &Widget) -> bool { return true; }
    fn is_focused(&mut self, wgt: &Widget) -> bool { return false; }
    fn is_visible(&mut self, wgt: &Widget) -> bool { return true; }
    fn get_focused_id(&mut self) -> WId { return WIDGET_ID_NONE; }
    fn set_focused_id(&mut self, wid: WId) {}

    /// widget-specific queries; all mutable params are outputs
    fn get_window_coord(&mut self, wgt: &Widget, coord: &mut Coord) {}
    fn get_window_title(&mut self, wgt: &Widget, txt: &mut String) {}
    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool { return false; }
    fn get_label_text(&mut self, wgt: &Widget, txt: &mut String) {}
    fn get_text_edit_text(&mut self, wgt: &Widget, txt: &mut String, edit_mode: bool) {}
    fn get_led_lit(&mut self, wgt: &Widget) -> bool { return false; }
    fn get_led_text(&mut self, wgt: &Widget, txt: &mut String) {}
    fn get_progress_bar_state(&mut self, wgt: &Widget, pos: &mut i32, max: &mut i32) {}
    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> i8 { return 0; }
    fn get_list_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16) {}
    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {}
    fn get_combo_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16, drop_down: &mut bool) {}
    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {}
    fn get_radio_index(&mut self, wgt: &Widget) -> i32 { return -1; }
    fn get_text_box_state(&mut self, wgt: &Widget, lines: &[&str], top_line: &mut i16) {}
    fn get_button_text(&mut self, wgt: &Widget, txt: &mut String) {}

    /// requests
    fn invalidate(&self, id: WId, instantly: bool) { self.invalidate_impl(&[id], instantly); }
    // fn invalidate_many(const std::initializer_list<twins::WID> &ids, bool instantly = false) { invalidate_impl(ids.begin(), ids.size(), instantly); }

    // private fn
    fn invalidate_impl(&self, ids: &[WId], instantly: bool) {}
}
