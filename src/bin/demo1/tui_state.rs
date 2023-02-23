//! Demo - window state

use rtwins::esc;
use rtwins::input::*;
use rtwins::string_ext::*;
use rtwins::utils;
use rtwins::wgt::*;
use rtwins::*;

use std::cell::RefCell;
use std::rc::Rc;

use super::tui_def::Id;

/// State of all the DemoWindow widget dynamic properties
pub struct DemoWndState {
    /// all window widgets, starting with the window widget itself
    widgets: &'static [Widget],
    /// widgets runtime state
    pub rs: RuntimeStates,
    /// currently focused widget
    focused_id: WId,
    /// text of focused text edit widget
    text_edit1_txt: String,
    text_edit2_txt: String,
    /// list of widgets to redraw
    invalidated: Vec<WId>,
    //
    radiogrp1_idx: i16,
    //
    lbx_items: Vec<&'static str>,
    // text box raw source string and source splitted into rows
    tbx_text: String,
    tbx_wide_lines: Rc<RefCell<Vec<String>>>,
    tbx_narrow_lines: Rc<RefCell<Vec<String>>>,
}

trait StatesById {
    fn get_state_by_id(&self, id: Id) -> Option<&state_rt::State>;
    fn get_enabled_or_default_by_id(&self, id: Id) -> bool;
    fn set_enabled_by_id(&mut self, id: Id, en: bool);
    fn insert_state_by_id(&mut self, id: Id, state: state_rt::State);
}

/// Helper to avoid using `.into()`
impl StatesById for RuntimeStates {
    #[inline]
    fn get_state_by_id(&self, id: Id) -> Option<&state_rt::State> {
        self.get_state(id.into())
    }

    #[inline]
    fn get_enabled_or_default_by_id(&self, id: Id) -> bool {
        self.get_enabled_or_default(id.into())
    }

    #[inline]
    fn set_enabled_by_id(&mut self, id: Id, en: bool) {
        self.set_enabled(id.into(), en);
    }

    #[inline]
    fn insert_state_by_id(&mut self, id: Id, state: state_rt::State) {
        self.insert_state(id.into(), state);
    }
}

impl DemoWndState {
    pub fn new(widgets: &'static [Widget]) -> Self {
        let mut wnd_state = DemoWndState {
            widgets,
            rs: RuntimeStates::new(),
            focused_id: WIDGET_ID_NONE,
            text_edit1_txt: String::new(),
            text_edit2_txt: String::new(),
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
        use state_rt::*;

        wnd_state.rs.set_enabled_by_id(Id::LabelFwVersion, false);

        wnd_state
            .rs
            .insert_state_by_id(Id::Prgbar1, PgbarState { pos: 5, max: 10 }.into());
        wnd_state
            .rs
            .insert_state_by_id(Id::Prgbar2, PgbarState { pos: 2, max: 10 }.into());
        wnd_state
            .rs
            .insert_state_by_id(Id::Prgbar3, PgbarState { pos: 8, max: 10 }.into());
        wnd_state
            .rs
            .insert_state_by_id(Id::LedLock, LedState { lit: true }.into());
        wnd_state
            .rs
            .insert_state_by_id(Id::ChbxEnbl, ChbxState { checked: true }.into());
        wnd_state
            .rs
            .insert_state_by_id(Id::PgControl, PgctrlState { page: 0 }.into());
        wnd_state.rs.insert_state_by_id(
            Id::TbxWide,
            TxtbxState {
                top_line: 9,
                lines: Default::default(),
            }
            .into(),
        );
        wnd_state
            .rs
            .insert_state_by_id(Id::ChbxL1, ChbxState { checked: true }.into());
        wnd_state
            .rs
            .insert_state_by_id(Id::ChbxL2, ChbxState { checked: true }.into());

        wnd_state
    }
}

// -----------------------------------------------------------------------------------------------

impl rtwins::WindowState for DemoWndState {
    /** events **/

    fn on_button_down(&mut self, wgt: &Widget, ii: &InputInfo) {
        if wgt.id == Id::BtnYes {
            tr_debug!("▼ BTN_YES");
        }
        if wgt.id == Id::BtnNo {
            tr_warn!("▼ BTN_NO");
        }
        if wgt.id == Id::BtnPopup {
            tr_info!("▼ BTN_POPUP");
        }
    }

    fn on_button_up(&mut self, wgt: &Widget, ii: &InputInfo) {
        if wgt.id == Id::BtnYes {
            tr_debug!("▲ BTN_YES");
        }
        if wgt.id == Id::BtnNo {
            tr_warn!("▲ BTN_NO");
        }
        if wgt.id == Id::BtnPopup {
            tr_info!("▲ BTN_POPUP");
        }
        if wgt.id == Id::BtnSayNo {
            self.rs.set_enabled_by_id(
                Id::BtnSayYes,
                !self.rs.get_enabled_or_default_by_id(Id::BtnSayYes),
            );

            self.invalidate(Id::BtnSayYes.into());

            self.rs.set_enabled_by_id(
                Id::Btn1p5,
                !self.rs.get_enabled_or_default_by_id(Id::Btn1p5),
            );

            self.invalidate(Id::Btn1p5.into());
        }
    }

    fn on_button_click(&mut self, wgt: &Widget, ii: &InputInfo) {
        tr_debug!("BTN_CLICK");

        if wgt.id == Id::BtnPopup {
            /* TODO:
            showPopup("Lorem Titlum",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. "
                "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
                [](twins::WID btnID) { TWINS_LOG_D(ESC_BG_DarkGreen "Choice: %d" ESC_BG_DEFAULT, btnID); },
                "ync"
            );
            */
        }

        if wgt.id == Id::BtnYes {
            // TODO:
            // rtwins::wgt::pagectrl_select_page(ws, Id::PgControl, Id::PageTextbox);
        }
    }

    fn on_button_key(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        if wgt.id == Id::Btn1p5 {
            tr_debug!("BTN_ON_KEY");

            if let InputEvent::Char(ref ch) = ii.evnt {
                if ch.utf8seq[0] == b' ' {
                    rtwins::wgt::mark_button_down(wgt, true);
                    self.invalidate_now(wgt.id);
                    // wait and unpress the button
                    //TODO: twins::glob::pal.sleep(500);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    rtwins::wgt::mark_button_down(wgt, false);
                    self.invalidate(wgt.id);
                    // clear input queue as it may be full of Keyboar key events;
                    // ... rbKeybInput is out of reach from here 🙁
                    return true;
                }
            }

            rtwins::wgt::mark_button_down(wgt, false);
            self.invalidate(wgt.id);
        }

        false
    }

    fn on_text_edit_change(&mut self, wgt: &Widget, txt: &mut String) {
        if wgt.id == Id::Edt1 {
            self.text_edit1_txt = std::mem::take(txt);
        }
        else if wgt.id == Id::Edt2 {
            self.text_edit2_txt = std::mem::take(txt);
        }

        tr_debug!("value: {}", txt);
    }

    fn on_text_edit_input_evt(
        &mut self,
        wgt: &Widget,
        ii: &InputInfo,
        txt: &mut String,
        cursor_pos: &mut i16,
    ) -> bool {
        /* TODO:
        if wgt.id == Id::EDT2 {
            return rtwins::util::numEditInputEvt(kc, str, cursorPos);
        }
        */

        // false means continue default key handling
        false
    }

    fn on_checkbox_toggle(&mut self, wgt: &Widget) {
        let rs = self.rs.as_chbx(wgt.id);
        rs.checked = !rs.checked;

        if wgt.id == Id::ChbxEnbl {
            tr_debug!("CHBX_ENBL");
        }
        else if wgt.id == Id::ChbxLock {
            tr_debug!("CHBX_LOCK");
        }
        else if wgt.id == Id::ChbxL1 || wgt.id == Id::ChbxL2 {
            self.invalidate(Id::PageServ.into());
        }
        else {
            tr_debug!("CHBX");
        }
    }

    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: i16) {
        tr_info!("NewPageIdx={}", new_page_idx);
        let rs = self.rs.as_pgctrl(wgt.id);
        rs.page = new_page_idx;
    }

    fn on_list_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {
        tr_debug!("LISTBOX_SELECT={}", new_sel_idx);
        let rs = self.rs.as_lbx(wgt.id);
        rs.sel_idx = new_sel_idx;
    }

    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {
        tr_debug!("LISTBOX_CHANGE={}", new_idx);
        let rs = self.rs.as_lbx(wgt.id);
        rs.item_idx = new_idx;
    }

    fn on_combo_box_select(&mut self, wgt: &Widget, new_sel_idx: i16) {
        tr_debug!("COMBOBOX_SELECT={}", new_sel_idx);
        let rs = self.rs.as_cbbx(wgt.id);
        rs.sel_idx = new_sel_idx;
    }

    fn on_combo_box_change(&mut self, wgt: &Widget, new_idx: i16) {
        tr_debug!("COMBOBOX_CHANGE={}", new_idx);
        let rs = self.rs.as_cbbx(wgt.id);
        rs.item_idx = new_idx;
    }

    fn on_combo_box_drop(&mut self, wgt: &Widget, drop_state: bool) {
        tr_debug!("COMBOBOX_DROP={}", drop_state);
        let rs = self.rs.as_cbbx(wgt.id);
        rs.drop_down = drop_state;
    }

    fn on_radio_select(&mut self, wgt: &Widget) {
        if let Property::Radio(ref p) = wgt.prop {
            tr_debug!("RADIO_SELECT.radio.id={}", p.radio_id);
            self.radiogrp1_idx = p.radio_id;
        }
    }

    fn on_text_box_scroll(&mut self, wgt: &Widget, top_line: i16) {
        let rs = self.rs.as_txtbx(wgt.id);
        rs.top_line = top_line;
    }

    fn on_custom_widget_draw(
        &mut self,
        wgt: &Widget,
        term_cell: &std::cell::RefCell<&mut rtwins::Term>,
    ) {
        let coord = rtwins::wgt::get_screen_coord(wgt);
        let sz = &wgt.size;
        let mut term = term_cell.borrow_mut();

        term.move_to(coord.col as u16, coord.row as u16);
        term.write_char_n('-', sz.width as i16);
        term.move_to(coord.col as u16, coord.row as u16 + sz.height as u16);
        term.write_char_n('-', sz.width as i16);
    }

    fn on_custom_widget_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        if let InputEvent::Mouse(ref mouse) = ii.evnt {
            if let Ok(mut term_lock) = rtwins::Term::try_lock_write() {
                let term = &mut *term_lock;
                term.move_to(mouse.col as u16, mouse.row as u16);
                let mark = mouse.evt.as_mark();
                term.write_char(mark);
            }
            else {
                tr_warn!("Cannot lock the term");
            }
        }
        true
    }

    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        tr_debug!("onWindowUnhandledInputEvt={}", ii.name);
        false
    }

    /** common state queries **/

    fn is_enabled(&self, wgt: &Widget) -> bool {
        if wgt.id == Id::ChbxC {
            false
        }
        else {
            self.rs.get_enabled_or_default(wgt.id)
        }
    }

    fn is_focused(&self, wgt: &Widget) -> bool {
        self.focused_id == wgt.id
    }

    fn is_visible(&self, wgt: &Widget) -> bool {
        if let Property::Page(_) = wgt.prop {
            let pgctrl = get_parent(wgt);

            if let Some(stat) = self.rs.get_state(pgctrl.id) {
                if let Some(pg_idx) = page_page_idx(wgt) {
                    if let state_rt::State::Pgctrl(ref pgctrl) = stat {
                        return pg_idx == pgctrl.page;
                    }
                }
            }
        }

        if wgt.id == Id::Layer1 {
            if let Some(stat) = self.rs.get_state_by_id(Id::ChbxL1) {
                if let state_rt::State::Chbx(ref cbx) = stat {
                    return cbx.checked;
                }
            }
        }

        if wgt.id == Id::Layer2 {
            if let Some(stat) = self.rs.get_state_by_id(Id::ChbxL2) {
                if let state_rt::State::Chbx(ref cbx) = stat {
                    return cbx.checked;
                }
            }
        }

        true
    }

    fn is_desktop(&self) -> bool {
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

    fn get_window_coord(&mut self) -> Coord {
        if let Some(w) = self.widgets.first() {
            w.coord
        }
        else {
            Coord::cdeflt()
        }
    }

    fn get_window_size(&mut self) -> Size {
        if let Some(w) = self.widgets.first() {
            w.size
        }
        else {
            Size::cdeflt()
        }
    }

    fn get_window_title(&mut self, wgt: &Widget, txt: &mut String) {
        let _ = txt.stream()
            << esc::BOLD
            << "** Service Menu **"
            << esc::NORMAL
            << esc::UNDERLINE_ON
            << " (Ctrl+D quit)"
            << esc::UNDERLINE_OFF;
    }

    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool {
        let rs = self.rs.as_chbx(wgt.id);
        rs.checked
    }

    fn get_label_text(&mut self, wgt: &Widget, txt: &mut String) {
        if wgt.id == Id::LabelDate {
            txt.push_str(format!("Date•{}", "<datetime>").as_str());
        }

        if wgt.id == Id::LabelAbout {
            txt.push_str(url_link!("https://github.com/marmidr/rtwins", "About..."));
        }

        if wgt.id == Id::LabelMultiFmt {
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
                << " -> select the item";
        }
    }

    fn get_text_edit_text(&mut self, wgt: &Widget, txt: &mut String, edit_mode: bool) {
        if wgt.id == Id::Edt1 {
            *txt = self.text_edit1_txt.clone();
        }
        else if wgt.id == Id::Edt2 {
            *txt = self.text_edit2_txt.clone();
        }
    }

    fn get_led_lit(&mut self, wgt: &Widget) -> bool {
        let rs = self.rs.as_led(wgt.id);
        rs.lit
    }

    fn get_led_text(&mut self, wgt: &Widget, txt: &mut String) {
        *txt = "led-text".to_string();
    }

    fn get_progress_bar_state(&mut self, wgt: &Widget, state: &mut state_rt::PgbarState) {
        let rs = self.rs.as_pgbar(wgt.id);
        *state = *rs;
    }

    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> i16 {
        let rs = self.rs.as_pgctrl(wgt.id);
        rs.page
    }

    fn get_list_box_state(&mut self, wgt: &Widget, state: &mut state_rt::LbxState) {
        let rs = self.rs.as_lbx(wgt.id);
        *state = *rs;
        state.items_cnt = self.lbx_items.len() as i16
    }

    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: i16, txt: &mut String) {
        let plants = ['🌷', '🌱', '🌲', '🌻'];

        if wgt.id == Id::ListBox {
            if item_idx == 3 {
                txt.push_str(esc::BOLD);
                txt.push_str("Item");
                txt.push_str(esc::NORMAL);
                txt.push_str(" 0034567890123456789*");
            }
            else {
                txt.push_str(&format!(
                    "{}Item{} {:03} {}",
                    esc::FG_BLACK,
                    esc::FG_BLUE,
                    item_idx,
                    plants[item_idx as usize & 0x03]
                ));
            }
        }
    }

    fn get_combo_box_state(&mut self, wgt: &Widget, state: &mut state_rt::CbbxState) {
        let rs = self.rs.as_cbbx(wgt.id);
        *state = *rs;
        state.items_cnt = self.lbx_items.len() as i16;
    }

    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: i16, txt: &mut String) {
        txt.push_str(self.lbx_items[item_idx as usize]);
    }

    fn get_radio_index(&mut self, wgt: &Widget) -> i16 {
        self.radiogrp1_idx
    }

    fn get_text_box_state(&mut self, wgt: &Widget, state: &mut state_rt::TxtbxState) {
        let rs = self.rs.as_txtbx(wgt.id);
        state.top_line = rs.top_line;

        if wgt.id == Id::TbxWide {
            if self.tbx_wide_lines.borrow().len() == 0 {
                self.tbx_wide_lines = utils::word_wrap(wgt.size.width as usize - 2, &self.tbx_text);
            }
            state.lines = Rc::clone(&self.tbx_wide_lines);
        }
        else if wgt.id == Id::TbxNarrow {
            if self.tbx_narrow_lines.borrow().len() == 0 {
                self.tbx_narrow_lines =
                    utils::word_wrap(wgt.size.width as usize - 2, &self.tbx_text);
            }
            state.lines = Rc::clone(&self.tbx_narrow_lines);
        }
    }

    fn get_button_text(&mut self, wgt: &Widget, txt: &mut String) {}

    /* */

    fn invalidate_many(&mut self, wids: &[WId]) {
        self.invalidated.extend(wids.iter());
    }

    fn invalidate_now(&mut self, wid: WId) {
        // TODO:
        self.invalidate(wid);
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
