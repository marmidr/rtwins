//! Demo - window state

#![allow(unused_variables)]
#![allow(dead_code)]

use rtwins::common::*;
use rtwins::input;
use rtwins::input::*;
use rtwins::utils;
use rtwins::wgt::{self, WId, Widget, WIDGET_ID_NONE};
use rtwins::TERM;

use super::tui_def_msgbox::idmb;

// ---------------------------------------------------------------------------------------------- //

/// State of all the DemoWindow widget dynamic properties
pub struct MsgBoxState {
    /// all window widgets, starting with the window widget itself
    widgets: &'static [wgt::Widget],
    /// widgets runtime state
    pub rs: wgt::RuntimeStates,
    /// currently focused widget
    focused_id: WId,
    /// list of widgets to redraw
    invalidated: Vec<WId>,
    /// button click handler
    on_button: Box<dyn Fn(WId)>,
    wnd_title: String,
    wnd_message: String,
    buttons: &'static str,
}

impl MsgBoxState {
    pub fn new(widgets: &'static [Widget]) -> Self {
        let wnd_state = MsgBoxState {
            widgets,
            rs: wgt::RuntimeStates::new(),
            focused_id: WIDGET_ID_NONE,
            invalidated: vec![],
            on_button: Box::new(|_id: WId| {}),
            wnd_title: String::new(),
            wnd_message: String::new(),
            buttons: "ynoc",
        };

        wnd_state
    }

    /// Shows the MessageBox
    ///
    /// buttons: string of 'ynoc' defining visibility of Yes/No/Ok/Cancel buttons
    pub fn show(
        &mut self,
        title: String,
        message: String,
        on_button: Box<dyn Fn(WId)>,
        buttons: &'static str,
    ) {
        self.wnd_title = title;
        if let Some(lbl) = wgt::find_by_id(self.widgets, idmb::LBL_MSG) {
            self.wnd_message = utils::word_wrap(lbl.size.width as usize, &message)
                .take()
                .join("\n");
        }
        self.on_button = on_button;
        self.buttons = buttons;

        // twins::glob::wMngr.show(getWndPopup());
    }
}

// -----------------------------------------------------------------------------------------------

impl rtwins::wgt::WindowState for MsgBoxState {
    /** events **/

    fn on_button_click(&mut self, wgt: &Widget, ii: &InputInfo) {
        rtwins::tr_debug!("BTN_CLICK");
        self.on_button.as_ref()(wgt.id);
        // twins::glob::wMngr.hide(this);
    }

    fn on_button_key(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        false
    }

    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, ii: &InputInfo) -> bool {
        rtwins::tr_debug!("onWindowUnhandledInputEvt={}", ii.name);
        if let InputEvent::Key(key) = ii.evnt {
            if key == input::Key::Esc {
                // twins::glob::wMngr.hide(this);
                return true;
            }
        }

        false
    }

    /** common state queries **/

    fn is_enabled(&self, wgt: &Widget) -> bool {
        self.rs.get_enabled_or_default(wgt.id)
    }

    fn is_focused(&self, wgt: &Widget) -> bool {
        self.focused_id == wgt.id
    }

    fn is_visible(&self, wgt: &Widget) -> bool {
        match wgt.id {
            idmb::WND_MSGBOX => false, // TODO return twins::glob::wMngr.topWnd() == this;
            idmb::BTN_YES => self.buttons.as_bytes().contains(&b'y'),
            idmb::BTN_NO => self.buttons.as_bytes().contains(&b'n'),
            idmb::BTN_CANCEL => self.buttons.as_bytes().contains(&b'c'),
            idmb::BTN_OK => self.buttons.as_bytes().contains(&b'o'),
            _ => true,
        }
    }

    fn is_desktop(&self) -> bool {
        false
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
        // TODO: const auto *p_wnd = twins::glob::pMainWindowWgts;
        // calc location on the main window center
        // coord.col = (p_wnd->size.width - pWgt->size.width) / 2;
        // coord.col += p_wnd->coord.col;
        // coord.row = (p_wnd->size.height - pWgt->size.height) / 2;
        // coord.row += p_wnd->coord.row;

        Coord::cdeflt()
    }

    fn get_window_size(&mut self) -> Size {
        self.widgets.first().unwrap().size
    }

    fn get_window_title(&mut self, wgt: &Widget, txt: &mut String) {
        txt.push_str(&self.wnd_title)
    }

    fn get_label_text(&mut self, wgt: &Widget, txt: &mut String) {
        if wgt.id == idmb::LBL_MSG {
            // let wrapped = utils::word_wrap(wgt.size.width as usize, &self.wnd_message);
            // txt.push_str(wrapped.take().join("\n").as_str());
            txt.push_str(&self.wnd_message);
        }
    }

    /* */

    fn invalidate_many(&mut self, wids: &[WId]) {
        self.invalidated.extend(wids.iter());
    }

    fn instant_redraw(&mut self, wid: WId) {
        if let Ok(mut term_guard) = TERM.try_write() {
            term_guard.draw(self, &[wid]);
            term_guard.flush_buff();
        }
        else {
            rtwins::tr_warn!("Cannot lock the term");
        }
    }

    fn invalidate_clear(&mut self) {
        self.invalidated.clear();
    }

    fn get_invalidated(&mut self) -> Vec<WId> {
        let mut ret = vec![];
        std::mem::swap(&mut self.invalidated, &mut ret);
        ret
    }
}
