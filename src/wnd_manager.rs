//! # RTWins Window Manager

use crate::wgt::WindowState;
use crate::wgt::WndId;
use crate::TERM;

use lazy_static::lazy_static;
use std::sync::RwLock;

// ---------------------------------------------------------------------------------------------- //

lazy_static! {
    /// Global instance of Manager
    pub static ref WNDMNGR: RwLock<WndManager> = RwLock::new(
        WndManager::default()
    );
}

type WndMngrItem = Box<dyn WindowState + Send + Sync>;

pub struct WndManager {
    items: Vec<WndMngrItem>,
    visible: Vec<WndId>,
}

impl Default for WndManager {
    fn default() -> Self {
        Self {
            items: Default::default(),
            visible: Default::default(),
        }
    }
}

impl WndManager {
    pub fn push(&mut self, ws: WndMngrItem) -> WndId {
        // id is an index in the items[]
        let wnd_id = self.items.len() as WndId;
        self.items.push(ws);
        wnd_id
    }

    /// Show the window, push on the top (if not main wnd)
    pub fn show(&mut self, wnd_id: WndId) {
        let idx = self
            .visible
            .iter()
            .enumerate()
            .find(|item| *item.1 == wnd_id)
            .map(|item| item.0);

        if let Some(idx) = idx {
            // not main wnd, visible, but under another window;
            if wnd_id > 0 && idx + 1 < self.visible.len() {
                let tmp = self.visible.remove(idx);
                self.visible.push(tmp);
                self.draw_all();
            }
        }
        else {
            // not visible
            self.visible.push(wnd_id);
            self.draw_top();
        }
    }

    /// Hide window
    pub fn hide(&mut self, wnd_id: WndId) {
        let idx = self
            .visible
            .iter()
            .enumerate()
            .find(|item| *item.1 == wnd_id)
            .map(|item| item.0);

        if let Some(idx) = idx {
            self.visible.remove(idx);

            if self.visible.len() > 0 {
                self.draw_all();
            }
            else {
                let mut term_guard = TERM.try_write().unwrap();
                term_guard.screen_clr_all();
                term_guard.flush_buff();
            }
        }
    }

    /// Check if given window is visible
    pub fn is_visible(&self, wnd_id: WndId) -> bool {
        self.visible
            .iter()
            .enumerate()
            .find(|item| *item.1 == wnd_id)
            .map(|item| item.0)
            .is_some()
    }

    /// Returns any window mutable state
    pub fn get_ref(&self, wnd_id: WndId) -> Option<&dyn WindowState> {
        let wnd_id = wnd_id as usize;

        if wnd_id < self.items.len() {
            Some(self.items[wnd_id].as_ref())
        }
        else {
            None
        }
    }

    /// Returns any window mutable state
    pub fn get_mut(&mut self, wnd_id: WndId) -> Option<&mut dyn WindowState> {
        let wnd_id = wnd_id as usize;

        if wnd_id < self.items.len() {
            Some(self.items[wnd_id].as_mut())
        }
        else {
            None
        }
    }

    /// Returns top window state
    pub fn get_top_ref(&self) -> Option<&dyn WindowState> {
        if let Some(wnd_id) = self.visible.last() {
            Some(self.items[*wnd_id as usize].as_ref())
        }
        else {
            None
        }
    }

    /// Returns top window mutable state
    pub fn get_top_mut(&mut self) -> Option<&mut dyn WindowState> {
        if let Some(wnd_id) = self.visible.last_mut() {
            Some(self.items[*wnd_id as usize].as_mut())
        }
        else {
            None
        }
    }

    /// Checks if given window is visible and on the top
    pub fn is_top(&mut self, wnd_id: WndId) -> bool {
        if let Some(top) = self.visible.last() {
            *top == wnd_id
        }
        else {
            false
        }
    }

    /// Draw the top window only
    pub fn draw_top(&mut self) {
        let mut term_guard = TERM.try_write().unwrap();

        if let Some(ws) = self.get_top_mut() {
            term_guard.draw_wnd(ws);
        }
    }

    /// Draw the top window invalidated widgets
    pub fn draw_top_invalidated(&mut self) {
        let mut term_guard = TERM.try_write().unwrap();

        if let Some(ws) = self.get_top_mut() {
            term_guard.draw_invalidated(ws);
        }
    }

    /// Redraw windows from bottom to top
    pub fn draw_all(&mut self) {
        let mut term_guard = TERM.try_write().unwrap();

        for wnd_id in self.visible.iter() {
            let ws = self.items[*wnd_id as usize].as_mut();
            term_guard.draw_wnd(ws);
        }
    }
}
