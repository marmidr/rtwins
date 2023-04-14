//! # RTWins Widget

#![allow(dead_code)]
#![allow(unused_variables)]
// #![feature(trace_macros)]

use crate::common::*;
use crate::input::*;
use crate::wgt;

use core::fmt;

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------------------------- //

/// Widget unique identifier
pub type WId = u16;

/// Convenient, default value that points to nothing
pub const WIDGET_ID_NONE: WId = WId::MIN;
/// Used as a special function parameter
pub const WIDGET_ID_ALL: WId = WId::MAX;

// This macro uses TT munchers technique:
// https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
#[macro_export]
macro_rules! generate_ids {
    // 5+ items
    // expanding 5 items at a time, the recursion limit of 255 is unlikely to occure
    ($INIT: expr;  $ID1: ident $ID2: ident $ID3: ident $ID4: ident $ID5: ident $($TAIL: tt)+) => {
        pub const $ID1: WId = $INIT;
        pub const $ID2: WId = $INIT+1;
        pub const $ID3: WId = $INIT+2;
        pub const $ID4: WId = $INIT+3;
        pub const $ID5: WId = $INIT+4;
        $crate::generate_ids!{$INIT+5; $($TAIL)+}
    };

    // 2+ items
    ($INIT: expr; $ID: ident $($TAIL: tt)+) => {
        pub const $ID: WId = $INIT;
        $crate::generate_ids!{$INIT+1; $($TAIL)+}
    };

    // 1 item - final
    ($INIT: expr; $ID: ident) => {
        pub const $ID: WId = $INIT;
    };

    // public matcher without initializer
    ($($IDS: tt)+) => {
        // ID's must be > WIDGET_ID_NONE
        $crate::generate_ids!{WIDGET_ID_NONE + 1; $($IDS)+}
    };
}

// ---------------------------------------------------------------------------------------------- //

/// Widget static properties
pub mod prop {
    use super::ButtonStyle;
    use super::PgBarStyle;
    use crate::colors::*;
    use core::prelude::rust_2021::*;

    #[derive(Copy, Clone)]
    pub struct Window {
        pub title: &'static str,
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
        pub is_popup: bool,
    }

    #[derive(Copy, Clone)]
    pub struct Panel {
        pub title: &'static str,
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
        pub no_frame: bool,
    }

    #[derive(Copy, Clone)]
    pub struct Label {
        pub title: &'static str,
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
    }

    #[derive(Copy, Clone)]
    pub struct TextEdit {
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
        pub psw_mask: bool,
    }

    #[derive(Copy, Clone)]
    pub struct CheckBox {
        pub text: &'static str,
        pub fg_color: ColorFg,
    }

    #[derive(Copy, Clone)]
    pub struct Radio {
        pub text: &'static str,
        pub fg_color: ColorFg,
        pub group_id: u16,
        pub radio_id: i16,
    }

    #[derive(Copy, Clone)]
    pub struct Button {
        pub text: &'static str,
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
        pub style: ButtonStyle,
    }

    #[derive(Copy, Clone)]
    pub struct Led {
        pub text: &'static str,
        pub fg_color: ColorFg,
        pub bg_color_off: ColorBg,
        pub bg_color_on: ColorBg,
    }

    #[derive(Copy, Clone)]
    pub struct PageCtrl {
        pub tab_width: u8,
        pub vert_offs: u8,
    }

    #[derive(Copy, Clone)]
    pub struct Page {
        pub title: &'static str,
        pub fg_color: ColorFg,
    }

    #[derive(Copy, Clone)]
    pub struct ProgressBar {
        pub fg_color: ColorFg,
        pub style: PgBarStyle,
    }

    #[derive(Copy, Clone)]
    pub struct ListBox {
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
        pub no_frame: bool,
    }

    #[derive(Copy, Clone)]
    pub struct ComboBox {
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
        pub drop_down_size: u8,
    }

    #[derive(Copy, Clone)]
    pub struct CustomWgt {}

    #[derive(Copy, Clone)]
    pub struct TextBox {
        pub fg_color: ColorFg,
        pub bg_color: ColorBg,
    }

    #[derive(Copy, Clone)]
    pub struct Layer {}

    // Implements into() for all properties
    macro_rules! impl_into {
        ($($WGT: ident)*) => (
            $(
                impl $WGT {
                    pub const fn into(self) -> super::Property {
                        super::Property::$WGT(self)
                    }
                }
            )*
        )
    }

    impl_into! {
        Window Panel Label TextEdit CheckBox Radio Button Led PageCtrl
        Page ProgressBar ListBox ComboBox CustomWgt TextBox Layer
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

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::NoWgt => "NoWgt",
            Self::Window(_) => "Window",
            Self::Panel(_) => "Panel",
            Self::Label(_) => "Label",
            Self::TextEdit(_) => "TextEdit",
            Self::CheckBox(_) => "CheckBox",
            Self::Radio(_) => "Radio",
            Self::Button(_) => "Button",
            Self::Led(_) => "Led",
            Self::PageCtrl(_) => "PageCtrl",
            Self::Page(_) => "Page",
            Self::ProgressBar(_) => "ProgressBar",
            Self::ListBox(_) => "ListBox",
            Self::ComboBox(_) => "ComboBox",
            Self::CustomWgt(_) => "CustomWgt",
            Self::TextBox(_) => "TextBox",
            Self::Layer(_) => "Layer",
        };
        write!(f, "{name}")
    }
}

impl Property {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Self::NoWgt
    }

    //     /// Extract window properties from enum
    //     // pub fn prop_wnd<'a>(&'a self) -> &'a wp::Window {
    //     pub fn prop_wnd(&self) -> &prop::Window {
    //         match self {
    //             Self::Window(ref wp) => wp,
    //             _ => panic!(),
    //         }
    //     }

    //     /// Extract panel properties from enum
    //     pub fn prop_pnl(&self) -> &prop::Panel {
    //         match self {
    //             Self::Panel(ref wp) => wp,
    //             _ => panic!(),
    //         }
    //     }
}

// ---------------------------------------------------------------------------------------------- //

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
    ListBox,
}

// ---------------------------------------------------------------------------------------------- //

/// Widget itself, used for static UI definition;
/// contains only static, readonly properties
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
    /// link to children widgets, 2x8B; empty after processing with `tree_to_array()`
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

    /// Returns iterator going through widget children
    pub fn iter_children(&self) -> wgt::ChildrenIter {
        wgt::ChildrenIter::new(self)
    }

    /// Returns iterator going up the parents hierarchy, but starting at the widget itself
    pub fn iter_parents(&self) -> wgt::ParentsIter {
        wgt::ParentsIter::new(self)
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
    pub own_idx: u16,
    pub parent_idx: u16,
    pub children_idx: u16,
    pub children_cnt: u16,
}

impl Link {
    /// Returns default object; can be used in `const` initialization
    pub const fn cdeflt() -> Self {
        Link {
            own_idx: 0,
            parent_idx: 0,
            children_idx: 0,
            children_cnt: 0,
        }
    }
}

// ---------------------------------------------------------------------------------------------- //
// ---- RUNTIME STATE --------------------------------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

/// Widget RunTime state
pub mod rstate {
    use crate::utils::StringListRc;

    extern crate alloc;
    use alloc::string::String;

    /// CheckBox
    #[derive(Default, Clone, Copy)]
    pub struct ChbxState {
        pub checked: bool,
    }

    /// Led
    #[derive(Default, Clone, Copy)]
    pub struct LedState {
        pub lit: bool,
    }

    /// Label
    #[derive(Default, Clone)]
    pub struct LblState {
        pub txt: String,
    }

    /// TextEdit
    #[derive(Default, Clone)]
    pub struct TxteState {
        pub txt: String,
    }

    /// ListBox
    #[derive(Default, Clone, Copy)]
    pub struct LbxState {
        pub item_idx: i16,
        // used only when returning current state
        pub items_cnt: i16,
        pub sel_idx: i16,
    }

    /// ComboBox
    #[derive(Default, Clone, Copy)]
    pub struct CbbxState {
        pub item_idx: i16,
        // used only when returning current state
        pub items_cnt: i16,
        pub sel_idx: i16,
        pub drop_down: bool,
    }

    /// ProgressBar
    #[derive(Default, Clone, Copy)]
    pub struct PgbarState {
        pub pos: i32,
        pub max: i32,
    }

    /// TextBox
    #[derive(Default)]
    pub struct TxtbxState {
        pub top_line: i16,
        // used only when returning current state
        pub lines: StringListRc,
    }

    /// PageControl
    #[derive(Default, Clone, Copy)]
    pub struct PgctrlState {
        pub page: i16,
    }
} // mod

/// Contains runtime states for most types of the widgets
#[derive(Default)]
pub struct RuntimeStates {
    // state for each widget type
    pub chbx: BTreeMap<WId, rstate::ChbxState>,
    pub led: BTreeMap<WId, rstate::LedState>,
    pub lbl: BTreeMap<WId, rstate::LblState>,
    pub lbx: BTreeMap<WId, rstate::LbxState>,
    pub cbbx: BTreeMap<WId, rstate::CbbxState>,
    pub pgbar: BTreeMap<WId, rstate::PgbarState>,
    pub txtbx: BTreeMap<WId, rstate::TxtbxState>,
    pub pgctrl: BTreeMap<WId, rstate::PgctrlState>,
    pub txte: BTreeMap<WId, rstate::TxteState>,
    // applies to every widget
    pub enabled: BTreeMap<WId, bool>,
}

impl RuntimeStates {
    pub fn get_enabled_or_default(&self, id: WId) -> bool {
        let en = self.enabled.get(&id).unwrap_or(&true);
        *en
    }

    pub fn set_enabled(&mut self, id: WId, en: bool) {
        *self.enabled.entry(id).or_insert(true) = en;
    }
}

// ---------------------------------------------------------------------------------------------- //
// ---- WINDOW STATE TRAIT ---------------------------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

/// Window state and event handler
pub trait WindowState {
    /// events
    fn on_button_down(&mut self, wgt: &Widget, ii: &InputInfo) {}
    fn on_button_up(&mut self, wgt: &Widget, ii: &InputInfo) {}
    fn on_button_click(&mut self, wgt: &Widget, ii: &InputInfo) {}
    fn on_button_key(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        false
    }
    fn on_text_edit_change(&mut self, wgt: &Widget, txt: &mut String) {}
    fn on_text_edit_input_evt(
        &mut self,
        wgt: &Widget,
        ii: &InputInfo,
        txt: &mut String,
        cursor_pos: &mut i16,
    ) -> bool {
        false
    }
    fn on_checkbox_toggle(&mut self, wgt: &Widget) {}
    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: i16) {}
    fn on_list_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {}
    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {}
    fn on_combo_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {}
    fn on_combo_box_change(&mut self, wgt: &Widget, new_idx: i16) {}
    fn on_combo_box_drop(&mut self, wgt: &Widget, drop_state: bool) {}
    fn on_radio_select(&mut self, wgt: &Widget) {}
    fn on_text_box_scroll(&mut self, wgt: &Widget, new_top_line: i16) {}
    fn on_custom_widget_draw(
        &mut self,
        wgt: &Widget,
        term: &core::cell::RefCell<&mut crate::terminal::Term>,
    ) {
    }
    fn on_custom_widget_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        false
    }
    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        false
    }

    /// common state queries
    fn is_enabled(&self, wgt: &Widget) -> bool {
        true
    }
    fn is_focused(&self, wgt: &Widget) -> bool {
        false
    }
    fn is_visible(&self, wgt: &Widget) -> bool {
        true
    }
    fn is_desktop(&self) -> bool {
        false
    }
    fn get_focused_id(&mut self) -> WId {
        WIDGET_ID_NONE
    }

    fn get_widgets(&self) -> &'static [Widget] {
        &[]
    }
    fn get_rstate(&mut self) -> Option<&mut wgt::RuntimeStates> {
        None
    }

    /// widget-specific queries; all mutable params are outputs
    fn get_window_coord(&mut self) -> Coord {
        Coord::cdeflt()
    }
    fn get_window_size(&mut self) -> Size {
        Size::cdeflt()
    }
    fn get_window_title(&mut self, wgt: &Widget, out: &mut String) {}
    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool {
        false
    }
    fn get_label_text(&mut self, wgt: &Widget, out: &mut String) {}
    fn get_text_edit_text(&mut self, wgt: &Widget, out: &mut String, edit_mode: bool) {}
    fn get_led_lit(&mut self, wgt: &Widget) -> bool {
        false
    }
    fn get_led_text(&mut self, wgt: &Widget, out: &mut String) {}
    fn get_progress_bar_state(&mut self, wgt: &Widget, out: &mut rstate::PgbarState) {}
    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> i16 {
        0
    }
    fn get_list_box_state(&mut self, wgt: &Widget, out: &mut rstate::LbxState) {}
    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: i16, out: &mut String) {}
    fn get_combo_box_state(&mut self, wgt: &Widget, out: &mut rstate::CbbxState) {}
    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: i16, out: &mut String) {}
    fn get_radio_index(&mut self, wgt: &Widget) -> i16 {
        -1
    }
    fn get_text_box_state(&mut self, wgt: &Widget, out: &mut rstate::TxtbxState) {}
    fn get_button_text(&mut self, wgt: &Widget, out: &mut String) {}

    /// requests
    fn set_focused_id(&mut self, wid: WId) {}
    #[inline]
    fn invalidate(&mut self, wid: WId) {
        self.invalidate_many(&[wid]);
    }
    fn instant_redraw(&mut self, wid: WId) {}
    fn invalidate_many(&mut self, wids: &[WId]) {}
    fn clear_invalidated(&mut self) {}
    fn get_invalidated(&mut self, out: &mut Vec<WId>) {}
}

// ---------------------------------------------------------------------------------------------- //

struct WindowStateStub;

impl WindowStateStub {
    fn new() -> Self {
        WindowStateStub
    }
}

impl WindowState for WindowStateStub {
    //
}

// ---------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::{WId, WIDGET_ID_NONE};

    #[test]
    fn check_widget_ids_generator() {
        generate_ids!(POPUP_EMPTY);
        assert_eq!(1, POPUP_EMPTY);

        generate_ids!(
            POPUP_WARN
                BTN_OK
        );
        assert_eq!(1, POPUP_WARN);
        assert_eq!(2, BTN_OK);

        // enable printing of the recursive macro arguments
        // (uncomment trace_macros on top of the file)

        // trace_macros!(true);
        generate_ids!(
            POPUP_YESNOCANCEL
            POPUP_CAPTION
                BTN_YES
                BTN_NO
                BTN_CANCEL
                LBL_MESSAGE
        );
        // trace_macros!(false);
        assert_eq!(6, LBL_MESSAGE);
    }
}
