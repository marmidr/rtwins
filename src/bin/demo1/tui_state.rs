//!

use std::collections::HashMap;
use rtwins::widget::*;
use rtwins::input::*;

use super::tui_def;

pub struct DemoWndState {
    wnd: &'static [rtwins::Widget],
    wprop: HashMap<rtwins::WId, WidgetMutProp>,
    focused_id: WId,
}

impl DemoWndState {
    pub fn new(wnd: &'static [rtwins::Widget]) -> Self {
        let mut ws = DemoWndState{wnd,
            wprop: HashMap::new(),
            focused_id: WIDGET_ID_NONE
        };

        use tui_def::Id;
        ws.wprop.insert(Id::Lbl2.into(), WidgetMutProp::new());
        ws.wprop.get_mut(&Id::Lbl2.into()).unwrap().enabled = false;
        return ws;
    }
}

// -----------------------------------------------------------------------------------------------

impl WindowState for DemoWndState {
    fn get_widgets(&self) -> &'static [Widget] {
        self.wnd
    }

    /** events **/

    fn on_button_down(&mut self, wgt: &Widget, kc: &KeyCode) {

    }
    fn on_button_up(&mut self, wgt: &Widget, kc: &KeyCode) {

    }
    fn on_button_click(&mut self, wgt: &Widget, kc: &KeyCode) {

    }
    fn on_text_edit_change(&mut self, wgt: &Widget, txt: &mut String) {

    }
    fn on_text_edit_input_evt(&mut self, wgt: &Widget, kc: &KeyCode, txt: &mut String, cursor_pos: &mut i16) -> bool {
        return false;
    }
    fn on_checkbox_toggle(&mut self, wgt: &Widget) {

    }
    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: u8) {

    }
    fn on_list_box_select(&mut self, wgt: &Widget, sel_idx: i16) {

    }
    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {

    }
    fn on_combo_box_select(&mut self, wgt: &Widget, sel_idx: i16) {

    }
    fn on_combo_box_change(&mut self, wgt: &Widget, new_idx: i16) {

    }
    fn on_combo_box_drop(&mut self, wgt: &Widget, drop_state: bool) {

    }
    fn on_radio_select(&mut self, wgt: &Widget) {

    }
    fn on_text_box_scroll(&mut self, wgt: &Widget, top_line: i16) {

    }
    fn on_custom_widget_draw(&mut self, wgt: &Widget) {

    }
    fn on_custom_widget_input_evt(&mut self, wgt: &Widget, kc: &KeyCode) -> bool {
        return false;
    }
    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, kc: &KeyCode) -> bool {
        return false;
    }

    /** common state queries **/

    fn is_enabled(&mut self, wgt: &Widget) -> bool {
        if let Some(ref p) = self.wprop.get(&wgt.id) {
            return p.enabled;
        }

        true
    }
    fn is_focused(&mut self, wgt: &Widget) -> bool {
        self.focused_id == wgt.id
    }
    fn is_visible(&mut self, wgt: &Widget) -> bool {
        true
    }
    fn get_focused_id(&mut self) -> WId {
        self.focused_id
    }
    fn set_focused_id(&mut self, wid: WId) {
        self.focused_id = wid;
    }

    /** widget-specific queries; all mutable params are outputs **/

    fn get_window_coord(&mut self, wgt: &Widget, coord: &mut Coord) {

    }
    fn get_window_title(&mut self, wgt: &Widget, title: &mut String) {

    }
    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool {
        return false;
    }
    fn get_label_text(&mut self, wgt: &Widget, txt: &mut String) {

    }
    fn get_text_edit_text(&mut self, wgt: &Widget, txt: &mut String, edit_mode: bool) {

    }
    fn get_led_lit(&mut self, wgt: &Widget) -> bool {
        return false;
    }
    fn get_led_text(&mut self, wgt: &Widget, txt: &mut String) {

    }
    fn get_progress_bar_state(&mut self, wgt: &Widget, pos: &mut i32, max: &mut i32) {

    }
    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> i8 {
        return 0;
    }
    fn get_list_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16) {

    }
    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {

    }
    fn get_combo_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16, drop_down: &mut bool) {

    }
    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {

    }
    fn get_radio_index(&mut self, wgt: &Widget) -> i32 {
        return -1;
    }
    fn get_text_box_state(&mut self, wgt: &Widget, lines: &[&str], top_line: &mut i16) {

    }
    fn get_button_text(&mut self, wgt: &Widget, txt: &mut String) {

    }
}
