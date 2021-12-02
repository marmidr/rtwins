//! Demo - window state

use std::collections::HashMap;
use rtwins::widget;
use rtwins::widget::*;
use rtwins::input::*;

use super::tui_def;

/// State of all the DemoWindow widget dynamic properties
pub struct DemoWndState {
    /// all window widgets, starting with the window widget itself
    widgets: &'static [Widget],
    /// widgets state
    wstate: HashMap<WId, WidgetState>,
    /// currently focused widget
    focused_id: WId,
    /// text of focused text edit widget
    text_edit_txt: String,
}

impl DemoWndState {
    pub fn new(widgets: &'static [Widget]) -> Self {
        let mut ws = DemoWndState{widgets,
            wstate: HashMap::new(),
            focused_id: WIDGET_ID_NONE,
            text_edit_txt: String::new(),
        };

        use tui_def::Id;
        ws.wstate.insert(Id::Lbl2.into(), WidgetState::new());
        ws.wstate.get_mut(&Id::Lbl2.into()).unwrap().enabled = false;
        return ws;
    }
}

// -----------------------------------------------------------------------------------------------

impl WindowState for DemoWndState {
    /** events **/

    fn on_button_down(&mut self, wgt: &Widget, kc: &KeyCode) {
        // self.ctx.flush_buff();
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
        let rs = WidgetState{state: RuntimeState::Chbx{ checked: false }, enabled: true };
        let ws = self.wstate.entry(wgt.id).or_insert(rs);

        if let RuntimeState::Chbx { ref mut checked } = ws.state {
            *checked = !*checked;
        }
    }

    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: u8) {
        if let Some(ws) = self.wstate.get_mut(&wgt.id) {
            if let widget::RuntimeState::Pgctrl { ref mut page } = ws.state {
                *page = new_page_idx;
            }
        }
    }

    fn on_list_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {
        if let Some(ws) = self.wstate.get_mut(&wgt.id) {
            if let widget::RuntimeState::Lbx { ref mut sel_idx, .. } = ws.state {
                *sel_idx = new_sel_idx;
            }
        }
    }

    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {
        if let Some(ws) = self.wstate.get_mut(&wgt.id) {
            if let widget::RuntimeState::Lbx { ref mut item_idx, .. } = ws.state {
                *item_idx = new_idx;
            }
        }
    }

    fn on_combo_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {

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
        match self.wstate.get(&wgt.id) {
            Some(p) => p.enabled,
            None => true
        }
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

    fn get_widgets(&self) -> &'static [Widget] {
        self.widgets
    }

    /** widget-specific queries; all mutable params are outputs **/

    fn get_window_coord(&mut self, wgt: &Widget, coord: &mut Coord) {

    }

    fn get_window_title(&mut self, wgt: &Widget, txt: &mut String) {
        if let widget::Type::Window(ref p) = wgt.typ {
            *txt = p.title.to_string();
        }
    }

    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool {
        if let Some(ws) = self.wstate.get(&wgt.id) {
            if let widget::RuntimeState::Chbx { checked } = ws.state {
                return checked;
            }
        }
        return false;
    }

    fn get_label_text(&mut self, wgt: &Widget, txt: &mut String) {
        if let widget::Type::Label(ref p) = wgt.typ {
            *txt = p.title.to_string();
        }
    }

    fn get_text_edit_text(&mut self, wgt: &Widget, txt: &mut String, edit_mode: bool) {
        *txt = self.text_edit_txt.clone();
    }

    fn get_led_lit(&mut self, wgt: &Widget) -> bool {
        if let Some(ws) = self.wstate.get(&wgt.id) {
            if let widget::RuntimeState::Led { ref lit } = ws.state {
                return *lit;
            }
        }
        return false;
    }

    fn get_led_text(&mut self, wgt: &Widget, txt: &mut String) {
        *txt = "led-text".to_string();
    }

    fn get_progress_bar_state(&mut self, wgt: &Widget, pos: &mut i32, max: &mut i32) {
        if let Some(ws) = self.wstate.get(&wgt.id) {
            if let widget::RuntimeState::Pgbar { max: m, pos: p } = ws.state {
                *pos = p;
                *max = m;
            }
        }
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
