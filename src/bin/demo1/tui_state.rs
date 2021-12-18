//! Demo - window state

use std::collections::HashMap;
use rtwins::widget;
use rtwins::esc;
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
    /// list of widgets to redraw
    invalidated: Vec<widget::WId>,
    //
    radiogrp1_idx: i16
}

impl DemoWndState {
    pub fn new(widgets: &'static [Widget]) -> Self {
        let mut ws = DemoWndState{widgets,
            wstate: HashMap::new(),
            focused_id: WIDGET_ID_NONE,
            text_edit_txt: String::new(),
            invalidated: vec![],
            radiogrp1_idx: 1
        };

        use tui_def::Id;
        ws.wstate.insert(Id::LabelFwVersion.into(), WidgetState::new());
        ws.wstate.get_mut(&Id::LabelFwVersion.into()).unwrap().enabled = false;

        ws.wstate.insert(Id::Prgbar1.into(), WidgetState{state: RuntimeState::Pgbar{ pos:5, max: 10 }, enabled: true});
        ws.wstate.insert(Id::Prgbar2.into(), WidgetState{state: RuntimeState::Pgbar{ pos:2, max: 10 }, enabled: true});
        ws.wstate.insert(Id::Prgbar3.into(), WidgetState{state: RuntimeState::Pgbar{ pos:8, max: 10 }, enabled: true});

        ws.wstate.insert(Id::LedLock.into(), WidgetState{state: RuntimeState::Led{ lit: true }, enabled: true});

        ws.wstate.insert(Id::ChbxEnbl.into(), WidgetState{state: RuntimeState::Chbx{ checked: true }, enabled: true});

        ws.wstate.insert(Id::PgControl.into(), WidgetState{state: RuntimeState::Pgctrl{ page: 1 }, enabled: true});
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
        // TODO: self.radiogrp1_idx =
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
        *coord = wgt.coord;
    }

    fn get_window_title(&mut self, wgt: &Widget, txt: &mut String) {
        txt.push_str(esc::BOLD);
        txt.push_str("** Service Menu **");
        txt.push_str(esc::NORMAL);
        txt.push_str(esc::UNDERLINE_ON);
        txt.push_str(" (Ctrl+D quit)");
        txt.push_str(esc::UNDERLINE_OFF);
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
        if wgt.id == tui_def::Id::LabelDate.into() {
            txt.push_str(format!("Date•{}",
                "<datetime>").as_str()
            );
        }

        if wgt.id == tui_def::Id::LabelAbout.into() {
            use rtwins::link;
            txt.push_str(link!("https://bitbucket.org/marmidr/twins", "About..."));
        }

        if wgt.id == tui_def::Id::LabelMultiFmt.into() {
            // TODO: extend String with << operator, extraxt StringExt to separate unit
            txt.push_str("  ▫▫▫▫▫ ");
            txt.push_str(esc::INVERSE_ON);
            txt.push_str("ListBox");
            txt.push_str(esc::INVERSE_OFF);
            txt.push_str(" ▫▫▫▫▫\n");
            txt.push_str("• ");
            txt.push_str(esc::UNDERLINE_ON);
            txt.push_str("Up/Down");
            txt.push_str(esc::UNDERLINE_OFF);
            txt.push_str(" -> change item\n");
            txt.push_str("• ");
            txt.push_str(esc::UNDERLINE_ON);
            txt.push_str("PgUp/PgDown");
            txt.push_str(esc::UNDERLINE_OFF);
            txt.push_str(" -> scroll page\n");
            txt.push_str("• ");
            txt.push_str(esc::UNDERLINE_ON);
            txt.push_str("Enter");
            txt.push_str(esc::UNDERLINE_OFF);
            txt.push_str(" -> select the item");
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

    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> u8 {
        if let Some(ws) = self.wstate.get(&wgt.id) {
            if let widget::RuntimeState::Pgctrl { page } = ws.state {
                return page;
            }
        }
        0
    }

    fn get_list_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16) {

    }

    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {

    }

    fn get_combo_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16, drop_down: &mut bool) {

    }

    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {

    }

    fn get_radio_index(&mut self, wgt: &Widget) -> i16 {
        return self.radiogrp1_idx;
    }

    fn get_text_box_state(&mut self, wgt: &Widget, lines: &[&str], top_line: &mut i16) {

    }

    fn get_button_text(&mut self, wgt: &Widget, txt: &mut String) {

    }

    /* */

    fn invalidate(&mut self, wids: &[widget::WId]) {
        self.invalidated.extend(wids.iter());
        // self.invalidated.sort();
        // self.invalidated.dedup();
    }

    fn invalidate_clear(&mut self) {
        self.invalidated.clear();
    }

    fn get_invalidated(&mut self) -> Vec<WId> {
        let mut ret = vec![];
        ret.reserve(8);
        std::mem::swap(&mut self.invalidated, &mut ret);
        ret
    }
}
