//! # RTWins Widget

#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::{Add, Sub};
use std::collections::HashMap;

use crate::input::KeyCode;
use crate::widget_impl::WidgetIter;
use crate::utils::StringListRc;

// ---------------------------------------------------------------------------------------------- //

/// Widget coordinates on screen or on parent widget
#[derive(Clone, Copy, Default)]
pub struct Coord {
    pub col: u8,
    pub row: u8,
}

impl Coord {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Coord { col: 0, row: 0 }
    }

    pub const fn new(c: u8, r: u8) -> Self {
        Coord { col: c, row: r }
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Coord) -> Coord {
        Coord {
            col: self.col.saturating_add(other.col),
            row: self.row.saturating_add(other.row),
        }
    }
}

/// Widget size
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

impl Size {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Size {
        Size { width: 0, height: 0 }
    }

    pub const fn new(w: u8, h: u8) -> Self {
        Size { width: w, height: h }
    }
}

impl Sub for Size {
    type Output = Self;
    fn sub(self, other: Size) -> Size {
        Size {
            width:  self.width.saturating_sub(other.width),
            height: self.height.saturating_sub(other.height),
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
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum ButtonStyle {
    Simple,
    Solid,
    Solid1p5,
}

/// Visual style of Progress Bar
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum PgBarStyle {
    /// #
    Hash,
    ///  ▒
    Shade,
    /// □
    Rectangle,
}

/// Window, panel and page control frame styles
#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

/// Widget static properties
pub mod prop {
    use crate::colors::*;
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
        pub const fn into(self) -> super::Property {
            super::Property::Window(self)
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
        pub const fn into(self) -> super::Property {
            super::Property::Panel(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Label {
        pub title: &'static str,
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
    }

    impl Label {
        pub const fn into(self) -> super::Property {
            super::Property::Label(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct TextEdit {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
    }

    impl TextEdit {
        pub const fn into(self) -> super::Property {
            super::Property::TextEdit(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct CheckBox {
        pub text: &'static str,
        pub fg_color: ColorFG,
    }

    impl CheckBox {
        pub const fn into(self) -> super::Property {
            super::Property::CheckBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Radio {
        pub text: &'static str,
        pub fg_color: ColorFG,
        pub group_id: u16,
        pub radio_id: i16,
    }

    impl Radio {
        pub const fn into(self) -> super::Property {
            super::Property::Radio(self)
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
        pub const fn into(self) -> super::Property {
            super::Property::Button(self)
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
        pub const fn into(self) -> super::Property {
            super::Property::Led(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct PageCtrl {
        pub tab_width: u8,
        pub vert_offs: u8,
    }

    impl PageCtrl {
        pub const fn into(self) -> super::Property {
            super::Property::PageCtrl(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Page {
        pub title: &'static str,
        pub fg_color: ColorFG,
    }

    impl Page {
        pub const fn into(self) -> super::Property {
            super::Property::Page(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ProgressBar {
        pub fg_color: ColorFG,
        pub style: PgBarStyle,
    }

    impl ProgressBar {
        pub const fn into(self) -> super::Property {
            super::Property::ProgressBar(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ListBox {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub no_frame: bool,
    }

    impl ListBox {
        pub const fn into(self) -> super::Property {
            super::Property::ListBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ComboBox {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
        pub drop_down_size: u8,
    }

    impl ComboBox {
        pub const fn into(self) -> super::Property {
            super::Property::ComboBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct CustomWgt {}

    impl CustomWgt {
        pub const fn into(self) -> super::Property {
            super::Property::CustomWgt(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct TextBox {
        pub fg_color: ColorFG,
        pub bg_color: ColorBG,
    }

    impl TextBox {
        pub const fn into(self) -> super::Property {
            super::Property::TextBox(self)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Layer {}

    impl Layer {
        pub const fn into(self) -> super::Property {
            super::Property::Layer(self)
        }
    }
}

/// Widget type with all specific data
#[derive(Copy, Clone)]
pub enum Property {
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

impl fmt::Display for Property {
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

impl Property {
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
    /// coordinates
    pub coord: Coord,
    /// widget size
    pub size: Size,
    /// indexes used after flattening widget tree to array
    pub link: Link,
    /// widget properties defining it's type
    pub prop: Property,
    /// link to children widgets, 2x8B
    pub children: &'static [Widget],
}

impl Widget {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Widget {
            id: WIDGET_ID_NONE,
            link: Link::cdeflt(),
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            prop: Property::cdeflt(),
            children: &[],
        }
    }

    pub fn iter(&self) -> WidgetIter {
        WidgetIter::new(self)
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
    pub children_idx: u16,
    pub children_cnt: u16,
}

impl Link {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Link{ own_idx: 0, parent_idx: 0, children_idx: 0, children_cnt: 0 }
    }
}

// -----------------------------------------------------------------------------------------------

/// Widget RunTime properties
pub mod prop_rt {

#[derive(Default)]
pub struct Chbx {
    pub checked: bool,
}

#[derive(Default)]
pub struct Led {
    pub lit: bool,
}

#[derive(Default)]
pub struct Lbx {
    pub item_idx: i16,
    pub sel_idx: i16,
}

#[derive(Default)]
pub struct Cbbx {
    pub item_idx: i16,
    pub sel_idx: i16,
    pub drop_down: bool,
}

#[derive(Default)]
pub struct Pgbar {
    pub pos: i32,
    pub max: i32,
}

#[derive(Default)]
pub struct Txtbx {
    pub top_line: i16,
}

#[derive(Default)]
pub struct Pgctrl {
    pub page: u8
}

// Implements into() for all properties
macro_rules! impl_into {
    ($($prop: ident)*) => ($(
        impl $prop {
            pub fn into(self) -> State {
                State::$prop(self)
            }
        }
    )*)
}

impl_into!{Chbx Led Lbx Cbbx Pgbar Txtbx Pgctrl}

pub enum State {
    None,
    Chbx(Chbx),
    Led(Led),
    Lbx(Lbx),
    Cbbx(Cbbx),
    Pgbar(Pgbar),
    Txtbx(Txtbx),
    Pgctrl(Pgctrl),
}

impl Default for State {
    fn default() -> State {
        State::None
    }
}

} // mod

/// Contains runtime states for most types of the widgets
#[derive(Default)]
pub struct RuntimeState {
    // widget type state
    states: HashMap<WId, prop_rt::State>,
    // applies to every widget
    enabled: HashMap<WId, bool>,
}

// macro generating similar member functions
macro_rules! impl_as  {
    ($name: ident, $prop: ident) => {
        pub fn $name(&mut self, id: WId) -> &mut prop_rt::$prop {
            let rs = self.states.entry(id).or_insert(
                prop_rt::$prop::default().into());

            if let prop_rt::State::$prop(ref mut stat) = rs {
                return stat;
            }

            panic!("Invalid widget rt state")
        }
    };
}

impl RuntimeState {
    pub fn new() -> Self {
        RuntimeState{states: HashMap::new(), enabled: HashMap::new()}
    }

    pub fn get_enabled_or_default(&self, id: WId) -> bool {
        let en = self.enabled.get(&id).or(Some(&true)).unwrap();
        *en
    }

    pub fn set_enabled(&mut self, id: WId, en: bool) {
        *self.enabled.entry(id).or_insert(true) = en;
    }

    pub fn insert_state(&mut self, id: WId, state: prop_rt::State) {
        self.states.insert(id, state);
    }

    impl_as!(as_chbx, Chbx);
    impl_as!(as_led, Led);
    impl_as!(as_lbx, Lbx);
    impl_as!(as_cbbx, Cbbx);
    impl_as!(as_pgbar, Pgbar);
    impl_as!(as_txtbx, Txtbx);
    impl_as!(as_pgctrl, Pgctrl);
}

// -----------------------------------------------------------------------------------------------

/// Window state and event handler
pub trait WindowState {
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
    fn get_widgets(&self) -> &'static [Widget] { return &[]; }

    /// widget-specific queries; all mutable params are outputs
    fn get_window_coord(&mut self, wgt: &Widget, coord: &mut Coord) {}
    fn get_window_title(&mut self, wgt: &Widget, txt: &mut String) {}
    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool { return false; }
    fn get_label_text(&mut self, wgt: &Widget, txt: &mut String) {}
    fn get_text_edit_text(&mut self, wgt: &Widget, txt: &mut String, edit_mode: bool) {}
    fn get_led_lit(&mut self, wgt: &Widget) -> bool { return false; }
    fn get_led_text(&mut self, wgt: &Widget, txt: &mut String) {}
    fn get_progress_bar_state(&mut self, wgt: &Widget, pos: &mut i32, max: &mut i32) {}
    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> u8 { return 0; }
    fn get_list_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16) {}
    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: i16, txt: &mut String) {}
    fn get_combo_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16, drop_down: &mut bool) {}
    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: i16, txt: &mut String) {}
    fn get_radio_index(&mut self, wgt: &Widget) -> i16 { return -1; }
    fn get_text_box_state(&mut self, wgt: &Widget, lines: &mut StringListRc, top_line: &mut i16) {}
    fn get_button_text(&mut self, wgt: &Widget, txt: &mut String) {}

    /// requests
    fn invalidate(&mut self, wids: &[WId]) {}
    fn invalidate_clear(&mut self) {}
    fn get_invalidated(&mut self) -> Vec<WId> { vec![] }
}

// -----------------------------------------------------------------------------------------------


struct WindowStateStub;

impl WindowStateStub {
    fn new() -> Self {
        WindowStateStub
    }
}

impl WindowState for WindowStateStub {
    //
}
