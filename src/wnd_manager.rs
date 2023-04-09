//! # RTWins Window Manager

use crate::wgt::WindowState;
use crate::TERM;

extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------------------------- //

pub trait WindowManager {
    // pub trait WindowManager<IT> {
    // where IT: Iterator {
    /// Returns any window state
    fn get_ref(&self, wnd_idx: usize) -> Option<&dyn WindowState>;

    /// Returns any window mutable state
    fn get_mut(&mut self, wnd_idx: usize) -> Option<&mut dyn WindowState>;

    /// Returns vector of visible windows
    fn get_visible(&self) -> &[usize];

    /// Returns mutable vector of visible windows
    fn get_visible_mut(&mut self) -> &mut Vec<usize>;

    // fn iter(&self) -> IT;

    /// Show the window, push on the top (if not main wnd)
    fn show(&mut self, wnd_idx: usize) {
        let visible = self.get_visible_mut();

        let idx = visible
            .iter()
            .enumerate()
            .find(|item| *item.1 == wnd_idx)
            .map(|item| item.0);

        if let Some(idx) = idx {
            // not main wnd, visible, but under another window;
            if wnd_idx > 0 && idx + 1 < visible.len() {
                let tmp = visible.remove(idx);
                visible.push(tmp);
                self.draw_all();
            }
        }
        else {
            // not visible
            visible.push(wnd_idx);
            self.draw_top();
        }
    }

    /// Hide window
    fn hide(&mut self, wnd_idx: usize) {
        let visible = self.get_visible_mut();

        let idx = visible
            .iter()
            .enumerate()
            .find(|item| *item.1 == wnd_idx)
            .map(|item| item.0);

        if let Some(idx) = idx {
            visible.remove(idx);

            if !visible.is_empty() {
                self.draw_all();
            }
            else if let Some(mut term_guard) = TERM.try_lock() {
                term_guard.screen_clr_all();
                term_guard.flush_buff();
            }
        }
    }

    /// Check if given window is visible
    fn is_visible(&self, wnd_idx: usize) -> bool {
        self.get_visible()
            .iter()
            .enumerate()
            .find(|item| *item.1 == wnd_idx)
            .map(|item| item.0)
            .is_some()
    }

    /// Returns top window state
    fn get_top_ref(&self) -> Option<&dyn WindowState> {
        let visible = self.get_visible();

        if let Some(wnd_idx) = visible.last() {
            self.get_ref(*wnd_idx)
        }
        else {
            None
        }
    }

    /// Returns top window mutable state
    fn get_top_mut(&mut self) -> Option<&mut dyn WindowState> {
        let visible = self.get_visible();

        if let Some(wnd_idx) = visible.last() {
            self.get_mut(*wnd_idx)
        }
        else {
            None
        }
    }

    /// Checks if given window is visible and on the top
    fn is_top(&self, wnd_idx: usize) -> bool {
        let visible = self.get_visible();

        if let Some(top) = visible.last() {
            *top == wnd_idx
        }
        else {
            false
        }
    }

    /// Draw the top window only
    fn draw_top(&mut self) {
        if let Some(mut term_guard) = TERM.try_lock() {
            if let Some(ws) = self.get_top_mut() {
                term_guard.draw_wnd(ws);
            }
        }
    }

    /// Draw the top window invalidated widgets
    fn draw_top_invalidated(&mut self) {
        if let Some(mut term_guard) = TERM.try_lock() {
            if let Some(ws) = self.get_top_mut() {
                term_guard.draw_invalidated(ws);
            }
        }
    }

    /// Redraw windows from bottom to top
    fn draw_all(&mut self) {
        if let Some(mut term_guard) = TERM.try_lock() {
            let visible = self.get_visible().to_owned();

            for wnd_idx in 0..99 {
                if let Some(ws) = self.get_mut(wnd_idx) {
                    if visible.contains(&wnd_idx) {
                        term_guard.draw_wnd(ws);
                    }
                }
                else {
                    break;
                }
            }
        }
    }
}
