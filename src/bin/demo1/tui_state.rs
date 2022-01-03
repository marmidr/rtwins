//! Demo - window state

use rtwins::widget;
use rtwins::utils;
use rtwins::esc;
use rtwins::string_ext::*;
use rtwins::widget::*;
use rtwins::input::*;

use std::cell::RefCell;
use std::rc::Rc;

use super::tui_def;

/// State of all the DemoWindow widget dynamic properties
pub struct DemoWndState {
    /// all window widgets, starting with the window widget itself
    widgets: &'static [Widget],
    /// widgets runtime state
    rs: RuntimeState,
    /// currently focused widget
    focused_id: WId,
    /// text of focused text edit widget
    text_edit_txt: String,
    /// list of widgets to redraw
    invalidated: Vec<widget::WId>,
    //
    radiogrp1_idx: i16,
    //
    lbx_items: Vec<&'static str>,
    // text box raw source string and source splitted into rows
    tbx_text: String,
    tbx_wide_lines: Rc<RefCell<Vec<String>>>,
    tbx_narrow_lines: Rc<RefCell<Vec<String>>>
}

impl DemoWndState {
    pub fn new(widgets: &'static [Widget]) -> Self {
        let mut wnd_state = DemoWndState{widgets,
            rs: RuntimeState::new(),
            focused_id: WIDGET_ID_NONE,
            text_edit_txt: String::new(),
            invalidated: vec![],
            radiogrp1_idx: 1,
            lbx_items: vec![],
            tbx_text: String::with_capacity(400),
            tbx_wide_lines: Rc::new(RefCell::new(vec![])),
            tbx_narrow_lines: Rc::new(RefCell::new(vec![])),
        };

        wnd_state.lbx_items.extend_from_slice(&[
            "Black",
            "BlackIntense",
            "Red",
            "RedIntense",
            "Green",
            "GreenIntense",
            "Yellow",
            "YellowIntense",
            "Blue",
            "BlueIntense",
            "Magenta",
            "MagentaIntense",
            "Cyan",
            "CyanIntense",
            "White",
            "WhiteIntense",
        ]);

        // prepare text box content
        let _ = wnd_state.tbx_text.stream()
            << esc::BOLD
            << "🔶 Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam arcu magna, placerat sit amet libero at, aliquam fermentum augue.\n"
            << esc::NORMAL
            << esc::FG_GOLD
            << " Morbi egestas consectetur malesuada. Mauris vehicula, libero eget tempus ullamcorper, nisi lorem efficitur velit, vel bibendum augue eros vel lorem. Duis vestibulum magna a ornare bibendum. Curabitur eleifend dictum odio, eu ultricies nunc eleifend et.\n"
            << "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.\n"
            << esc::FG_GREEN_YELLOW
            << "🔷 Interdum et malesuada fames ac ante ipsum primis in faucibus. Aenean malesuada lacus leo, a eleifend lorem suscipit sed.\n"
            << "▄";

            // << esc::BOLD
            // << "🔶 Lorem ipsum \ndolor sit\n amet, consectetur adipiscing elit. Nam arcu magna, placerat sit amet libero at, aliquam fermentum augue. \n"
            // << esc::NORMAL
            // << "▄";

        // setup some widgets initial properties
        use tui_def::Id;
        use prop_rt::*;

        wnd_state.rs.set_enabled(Id::LabelFwVersion.into(), false);

        wnd_state.rs.insert_state(Id::Prgbar1.into(),   Pgbar{ pos:5, max: 10 }.into());
        wnd_state.rs.insert_state(Id::Prgbar2.into(),   Pgbar{ pos:2, max: 10 }.into());
        wnd_state.rs.insert_state(Id::Prgbar3.into(),   Pgbar{ pos:8, max: 10 }.into());
        wnd_state.rs.insert_state(Id::LedLock.into(),   Led{ lit: true }.into());
        wnd_state.rs.insert_state(Id::ChbxEnbl.into(),  Chbx{ checked: true }.into());
        wnd_state.rs.insert_state(Id::PgControl.into(), Pgctrl{ page: 4 }.into());
        wnd_state.rs.insert_state(Id::TbxWide.into(),   Txtbx{ top_line: 9 }.into());
        return wnd_state;
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
        let rs = self.rs.as_chbx(wgt.id);
        rs.checked = !rs.checked;
    }

    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: u8) {
        let rs = self.rs.as_pgctrl(wgt.id);
        rs.page = new_page_idx;
    }

    fn on_list_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {
        let rs = self.rs.as_lbx(wgt.id);
        rs.sel_idx = new_sel_idx;
    }

    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {
        let rs = self.rs.as_lbx(wgt.id);
        rs.item_idx = new_idx;
    }

    fn on_combo_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {
        let rs = self.rs.as_cbbx(wgt.id);
        rs.sel_idx = new_sel_idx;
    }

    fn on_combo_box_change(&mut self, wgt: &Widget, new_idx: i16) {
        let rs = self.rs.as_cbbx(wgt.id);
        rs.item_idx = new_idx;
    }

    fn on_combo_box_drop(&mut self, wgt: &Widget, drop_state: bool) {
        let rs = self.rs.as_cbbx(wgt.id);
        rs.drop_down = drop_state;
    }

    fn on_radio_select(&mut self, wgt: &Widget) {
        // TODO: self.radiogrp1_idx =
    }

    fn on_text_box_scroll(&mut self, wgt: &Widget, top_line: i16) {
        let rs = self.rs.as_txtbx(wgt.id);
        rs.top_line = top_line;
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
        self.rs.get_enabled_or_default(wgt.id)
    }

    fn is_focused(&mut self, wgt: &Widget) -> bool {
        self.focused_id == wgt.id
    }

    fn is_visible(&mut self, wgt: &Widget) -> bool {
        // if wgt.id == tui_def::Id::LbxUnderoptions.into() { return false; }
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
        let _ = txt.stream()
            << esc::BOLD
            << "** Service Menu **"
            << esc::NORMAL
            << esc::UNDERLINE_ON
            << " (Ctrl+D quit)"
            << esc::UNDERLINE_OFF
            ;
    }

    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool {
        let rs = self.rs.as_chbx(wgt.id);
        rs.checked
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
            let _ = txt.stream()
                << "  ▫▫▫▫▫ "
                << esc::INVERSE_ON
                << "ListBox"
                << esc::INVERSE_OFF
                << " ▫▫▫▫▫\n"
                << "• "
                << esc::UNDERLINE_ON
                << "Up/Down"
                << esc::UNDERLINE_OFF
                << " -> change item\n"
                << "• "
                << esc::UNDERLINE_ON
                << "PgUp/PgDown"
                << esc::UNDERLINE_OFF
                << " -> scroll page\n"
                << "• "
                << esc::UNDERLINE_ON
                << "Enter"
                << esc::UNDERLINE_OFF
                << " -> select the item"
                ;
        }
    }

    fn get_text_edit_text(&mut self, wgt: &Widget, txt: &mut String, edit_mode: bool) {
        *txt = self.text_edit_txt.clone();
    }

    fn get_led_lit(&mut self, wgt: &Widget) -> bool {
        let rs = self.rs.as_led(wgt.id);
        rs.lit
    }

    fn get_led_text(&mut self, wgt: &Widget, txt: &mut String) {
        *txt = "led-text".to_string();
    }

    fn get_progress_bar_state(&mut self, wgt: &Widget, pos: &mut i32, max: &mut i32) {
        let rs = self.rs.as_pgbar(wgt.id);
        *max = rs.max;
        *pos = rs.pos
    }

    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> u8 {
        let rs = self.rs.as_pgctrl(wgt.id);
        rs.page
    }

    fn get_list_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16) {
        let rs = self.rs.as_lbx(wgt.id);
        *item_idx = rs.item_idx;
        *sel_idx = rs.sel_idx;
        *items_count = self.lbx_items.len() as i16;
    }

    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: i16, txt: &mut String) {
        txt.push_str(format!("{:2}. {}", item_idx, self.lbx_items[item_idx as usize]).as_str());
    }

    fn get_combo_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16, drop_down: &mut bool) {
        let rs = self.rs.as_cbbx(wgt.id);
        *item_idx = rs.item_idx;
        *sel_idx = rs.sel_idx;
        *items_count = self.lbx_items.len() as i16;
        // *drop_down = rs.drop_down;
        *drop_down = true;
    }

    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: i16, txt: &mut String) {
        txt.push_str(self.lbx_items[item_idx as usize]);
    }

    fn get_radio_index(&mut self, wgt: &Widget) -> i16 {
        return self.radiogrp1_idx;
    }

    fn get_text_box_state(&mut self, wgt: &Widget, lines: &mut utils::StringListRc, top_line: &mut i16) {
        let rs = self.rs.as_txtbx(wgt.id);
        *top_line = rs.top_line;

        if wgt.id == tui_def::Id::TbxWide.into() {
            if self.tbx_wide_lines.borrow().len() == 0 {
                self.tbx_wide_lines = utils::word_wrap(wgt.size.width as usize-2, &self.tbx_text);
            }
            *lines = Rc::clone(&self.tbx_wide_lines);
        }
        else if wgt.id == tui_def::Id::TbxNarrow.into() {
            if self.tbx_narrow_lines.borrow().len() == 0 {
                self.tbx_narrow_lines = utils::word_wrap(wgt.size.width as usize-2, &self.tbx_text);
            }
            *lines = Rc::clone(&self.tbx_narrow_lines);
        }
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
