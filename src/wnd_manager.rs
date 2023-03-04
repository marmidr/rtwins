//! # RTWins Window Manager

use crate::{widget::*};

pub struct WndManager {
    stack: Vec<&'static dyn WindowState>,
}


impl WndManager {
    pub fn new() -> Self {
        WndManager{stack: vec![]}
    }

    /// Show window if not visible
    pub fn show(&mut self, ws: &'static dyn WindowState, bring_to_top: bool) {
        // on the list
        let mut it = self.stack.iter().enumerate();
        if let Some((idx, _)) = it.find(|item| std::ptr::eq(*item.1, ws)) {
            // check if on the top; move onto top if not
            if bring_to_top && (idx+1 < self.len()) {
                let w = self.stack.remove(idx);
                self.stack.push(w);
            }
        }
        else {
            // not on the list
            self.stack.push(ws);
        }
    }

    /// Hide window
    pub fn hide(&mut self, ws: &'static dyn WindowState) {
        let mut it = self.stack.iter().enumerate();
        if let Some((idx, _)) = it.find(|item| std::ptr::eq(*item.1, ws)) {
            self.stack.remove(idx);

            if self.stack.len() > 0 {
                self.redraw_all();
            }
            else {
                // twins::screenClrAll();
                // twins::flushBuffer();
            }
        }
    }

    /** @brief return top window */
    // pub fn topWnd(&self) {
        // twins::IWindowState *
        // assert(mWindows.size());
        // return *mWindows.back();
    // }

    /// Check if given window is on the list
    pub fn visible(&mut self, ws: &'static dyn WindowState) -> bool {
        let mut it = self.stack.iter();
        it.find(|&&item| std::ptr::eq(item, ws)).is_some()
    }

    /// Returns top window widgets
    pub fn top_wnd_widgets(&self) -> &'static [Widget] {
        let top = self.stack.last().expect("at least one window always visible");
        top.get_widgets()
    }

    /// Checks if goven window is visible and on top
    pub fn is_top_wnd(&mut self, ws: &'static dyn WindowState) -> bool {
        let top = self.stack.last().expect("at least one window always visible");
        std::ptr::eq(*top, ws)
    }

    /// Returns number of windows on stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Redraw windows from bottom to top
    pub fn redraw_all(&mut self) {
        for _w in self.stack.iter_mut() {
            // widget_draw::draw_widgets(term, ws, wids)
        }
    }
}
