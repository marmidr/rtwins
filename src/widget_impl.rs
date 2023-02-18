//! # RTWins Widget

#![allow(dead_code)]

use crate::string_ext::StrExt;
use crate::widget_def::*;
use crate::common::*;
use crate::input;
use crate::*;

// ---------------------------------------------------------------------------------------------- //

/// State object for current top window.
// using WId instead of references will solve lifetime problems
#[derive(Default)]
struct WidgetState {
    focused_wgt:     WId,
    mouse_down_wgt:  WId,
    drop_down_combo: WId,
    text_edit_state: TextEditState,
    mouse_down_key_code: input::Key,
    // parent_coord: Coord
}

impl WidgetState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct TextEditState {
    wgt_id: WId,
    cursor_pos: i16,
    txt: String,
}

// https://www.sitepoint.com/rust-global-variables/
thread_local!(
    static WGT_STATE: std::cell::RefCell<WidgetState> = std::cell::RefCell::new(WidgetState::default());
);

// ---------------------------------------------------------------------------------------------- //
// ---- UI TRANSFORMATION ----------------------------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

/// Transforming user UI definition into flat working copy
pub mod transform {

    use super::*;

    /// Counts total number of widgets in tree-like definition
    pub const fn tree_wgt_count(wgt: &Widget) -> usize {
        let mut n: usize = 1;
        let mut i: usize = 0;
        while i < wgt.children.len() {
            n += tree_wgt_count(&wgt.children[i]);
            i += 1;
        }
        n
    }

    /// Transforms user tree-like UI definition into flat array of widgets with Link structure filled-in
    pub const fn tree_to_array<const N: usize>(wgt: &Widget) -> [Widget; N] {
        let out: [Widget; N] = [Widget::cdeflt(); N];
        let (_, out) = do_transform(out, wgt, 0, 1);
        out
    }

const fn do_transform<const N: usize>(mut out: [Widget; N], wgt: &Widget, out_idx: usize, mut next_free_idx: usize) -> (usize, [Widget; N]) {
        out[out_idx] = *wgt;
        out[out_idx].link.own_idx = out_idx as u16;
        out[out_idx].children = &[];

        let mut out_child_idx = next_free_idx;

        if !wgt.children.is_empty() {
            out[out_idx].link.children_idx = out_child_idx as u16;
            out[out_idx].link.children_cnt = wgt.children.len() as u16;
            next_free_idx += wgt.children.len();
        }

        let mut ch_idx = 0;
        while ch_idx < wgt.children.len() {
            let (nfidx, o) = do_transform(out, &wgt.children[ch_idx], out_child_idx, next_free_idx);
            out = o;
            out[out_child_idx].link.parent_idx = out_idx as u16;
            next_free_idx = nfidx;

            ch_idx += 1;
            out_child_idx += 1;
        }

        (next_free_idx, out)
    }

} // mod

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGETS GENERAL FUNCTIONS --------------------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

/// Checks if given widget is parent-type
pub const fn is_parent(wgt: &Widget) -> bool {
    matches!(wgt.prop,
        Property::Window(_)   |
        Property::Panel(_)    |
        Property::PageCtrl(_) |
        Property::Page(_)     |
        Property::Layer(_)
    )
}

/// Get `wgt`'s parent, using flat widgets layout produced by `tree_to_array()`
pub fn get_parent(wgt: &Widget) -> &Widget {
    unsafe {
        // SAFETY:
        // it is guaranted thanks to how the `tree_to_array()` places widgets
        // in the contiguous array, thus, having self/parent/children indexes in that array
        // we can safely find any of them having only particular widget handle, without entire array
        let parent_idx_offset = wgt.link.parent_idx as isize - wgt.link.own_idx as isize;
        let p_wgt = wgt as *const Widget;
        &*p_wgt.offset(parent_idx_offset)
    }
}

/// Search for Widget with given `id` in transformed widgets array
pub fn find_by_id(wndarray: &[Widget], id: WId) -> Option<&Widget> {
    wndarray.iter().find(|&&item| item.id == id)
}

/// Finds visible Widget at cursor position `col:row`;
/// Sets `wgt_rect` to found widget screen-based coordinates
pub fn find_at<'a>(ws: &'a mut dyn WindowState, col: u8, row: u8, wgt_rect: &mut Rect) -> Option<&'a Widget> {
    let mut found_wgt: Option<&'a Widget> = None;
    let mut best_rect = Rect::cdeflt();
    best_rect.set_max();
    let wgts = ws.get_widgets();

    for wgt in wgts.iter() {
        let mut stop_searching = true;
        let mut wgt_screen_rect = Rect::cdeflt();

        wgt_screen_rect.coord = get_screen_coord(wgt);
        wgt_screen_rect.size = wgt.size;

        // eprintln!("{1:2}:{0:} at {2:2}:{3:-2}", wgt.prop, wgt.id, wgt_screen_rect.coord.col, wgt_screen_rect.coord.row);

        // correct the widget size
        match wgt.prop {
            Property::TextEdit(ref _p) => {
            },
            Property::CheckBox(ref p) => {
                wgt_screen_rect.size.height = 1;
                wgt_screen_rect.size.width = 4 + p.text.displayed_width() as u8;
            },
            Property::Radio(ref p) => {
                wgt_screen_rect.size.height = 1;
                wgt_screen_rect.size.width = 4 + p.text.displayed_width() as u8;
            },
            Property::Button(ref p) => {
                let txt_w;

                if !p.text.is_empty() {
                    txt_w = p.text.displayed_width() as u8;
                }
                else if wgt.size.width > 0 {
                    txt_w = wgt.size.width;
                }
                else {
                    let mut s = String::new();
                    ws.get_button_text(wgt, &mut s);
                    txt_w = s.displayed_width() as u8;
                }

                match p.style {
                    ButtonStyle::Simple => {
                        wgt_screen_rect.size.height = 1;
                        wgt_screen_rect.size.width = 4 + txt_w;
                    },
                    ButtonStyle::Solid => {
                        wgt_screen_rect.size.height = 1;
                        wgt_screen_rect.size.width = 2 + txt_w;
                    },
                    ButtonStyle::Solid1p5 => {
                        wgt_screen_rect.size.height = 3;
                        wgt_screen_rect.size.width = 2 + txt_w;
                    },
                    }
            },
            Property::PageCtrl(ref p) => {
                wgt_screen_rect.size.width = p.tab_width;
            },
            Property::ListBox(ref _p) => {
            },
            Property::ComboBox(ref _p) => {
            },
            _ => {
                stop_searching = false;
            }
        }

        if wgt_screen_rect.is_point_within(col, row) {
            let is_visible = is_visible(ws, wgt); // controls on tabs? solved

            if is_visible && best_rect.is_rect_within(&wgt_screen_rect) {
                found_wgt = Some(wgt);
                best_rect = wgt_screen_rect;
                *wgt_rect = wgt_screen_rect;

                // visible widget found?
                if stop_searching {
                    break;
                }
            }
        }
    }

    found_wgt
}

pub fn get_screen_coord(wgt: &Widget) -> Coord {
    wgt.iter_parents()
        .skip(1)
        .fold(wgt.coord, |c, parent| {
        let mut c = c;
        if let Property::Window(ref wnd) = parent.prop {
            if wnd.is_popup {
                // TODO: for popups must be centered on parent window
            }
            c = c + parent.coord;
            }
            else {
            c = c + parent.coord;
        }

        if let Property::PageCtrl(ref p) = parent.prop {
            c.col += p.tab_width;
        }

        c
    })
}

/// Move cursor to the best position for given type of the widget
pub fn set_cursor_at(term: &mut Term, ws: &mut dyn WindowState, wgt: &Widget) {
    let mut coord = get_screen_coord(wgt);

    match wgt.prop {
        Property::TextEdit(ref _p) => {
            WGT_STATE.with(|wgtstate|{
                let text_edit_state = &wgtstate.borrow().text_edit_state;

                if wgt.id == text_edit_state.wgt_id {
                    let max_w = wgt.size.width -3;
                    coord.col += text_edit_state.cursor_pos as u8;
                    let mut cursor_pos = text_edit_state.cursor_pos;
                    let delta = max_w/2;

                    while cursor_pos >= max_w as i16 - 1 {
                        coord.col -= delta;
                        cursor_pos -= delta as i16;
                    }
                }
                else {
                    coord.col += wgt.size.width - 2;
                }
            });
        },
        Property::CheckBox(ref _p) => {
            coord.col += 1;
        },
        Property::Radio(ref _p) => {
            coord.col += 1;
        },
        Property::Button(ref p) => {
            match p.style {
            ButtonStyle::Simple => {
                coord.col += 2;
                },
            ButtonStyle::Solid => {
                coord.col += 1;
                },
            ButtonStyle::Solid1p5 => {
                coord.col += 1;
                coord.row += 1;
                },
            }
        },
        Property::PageCtrl(ref p) => {
            coord.row += 1 + p.vert_offs;
            coord.row += ws.get_page_ctrl_page_index(wgt) as u8
        },
        Property::ListBox(ref p) => {
            let mut idx = 0i16;
            let mut selidx = 0i16;
            let mut cnt = 0i16;
            let frame_size = p.no_frame as u8;
            ws.get_list_box_state(wgt, &mut idx, &mut selidx, &mut cnt);

            let page_size = wgt.size.height - (frame_size * 2);
            let row = selidx % page_size as i16;

            coord.col += frame_size;
            coord.row += frame_size + row as u8;
        },
        _ => {}
    }

    term.move_to(coord.col as u16, coord.row as u16);
}

pub fn is_visible(ws: &mut dyn WindowState, wgt: &Widget) -> bool {
    wgt.iter_parents().all(|wgt| ws.is_visible(wgt))
}

pub fn is_enabled(ws: &mut dyn WindowState, wgt: &Widget) -> bool {
    wgt.iter_parents().all(|wgt| ws.is_enabled(wgt))
}

/// Shall be called eg. on top window change
pub fn reset_internal_state() {
    WGT_STATE.with(|wgstate|
        wgstate.borrow_mut().reset()
    );
}

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGETS HELPER FUNCTIONS ---------------------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

/// Returns given page index on parent PageCtrl
pub fn page_page_idx(page: &Widget) -> Option<u8> {
    if let Property::Page(_) = page.prop {
        let pgctrl = get_parent(page);

        for (idx, pg) in pgctrl.iter_children().enumerate() {
            if page.id == pg.id {
                return Some(idx as u8);
            }
        }
    }

    None
}

/// Returns WId of page at PageCtrl pages index
pub fn pagectrl_page_wid(pgctrl: &Widget, page_idx: u8) -> WId {
    if let Property::PageCtrl(_) = pgctrl.prop {
        if let Some(pg) = pgctrl.iter_children().skip(page_idx as usize).next() {
            return pg.id;
        }
    }

    WIDGET_ID_NONE
}

/// checks both `pgctrl` widget type and if `page_id` is one of its pages
pub fn pagectrl_find_page(pgctrl: &Widget, page_id: WId) -> Option<&Widget> {
    if let Property::PageCtrl(_) = pgctrl.prop {
        return pgctrl.iter_children().find(|pg| pg.id == page_id);
    }

    None
}

pub fn pagectrl_select_page(ws: &mut dyn WindowState, pgctrl_id: WId, page_id: WId) {
    if let Some(pgctrl) = find_by_id(ws.get_widgets(), pgctrl_id) {
        if let Some(page) = pagectrl_find_page(pgctrl, page_id) {
            if let Some(pg_idx) = page_page_idx(page) {
                ws.on_page_control_page_change(pgctrl, pg_idx);
                ws.invalidate(&[pgctrl_id]);
                return;
            }
        }
    }

    tr_warn!("Widget Id={} is not PageControl Id={} page", page_id, pgctrl_id);
}

pub fn pagectrl_select_next_page(ws: &mut dyn WindowState, pgctrl_id: WId, next: bool) {
    if let Some(pgctrl) = find_by_id(ws.get_widgets(), pgctrl_id) {
        pagectrl_change_page(ws, pgctrl, next);
    }
}

/// Mark internal clicked widget id
pub fn mark_button_down(btn: &Widget, is_down: bool) {
    WGT_STATE.with(|wgstate|
        wgstate.borrow_mut().mouse_down_wgt = if is_down { btn.id } else { WIDGET_ID_NONE }
    );
}

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGET ITERATORS ------------------------------------------------------------------------ //
// ---------------------------------------------------------------------------------------------- //

/// Iterator over parent-type Widget children
pub struct ChildrenIter <'a> {
    children: &'a Widget,
    children_idx: u16,
    children_cnt: u16
}

impl <'a> ChildrenIter<'a> {
    /// Creates a new iterator.
    ///
    /// If the given widget happen to be not a parent-type widget,
    /// first iteration will fail anyway as the child counter is 0
    pub fn new(parent_wgt: &'a Widget) -> Self {
        unsafe {
            // SAFETY: see `wgt_get_parent`
            let p_parent = parent_wgt as *const Widget;
            let children_offs = parent_wgt.link.children_idx as isize - parent_wgt.link.own_idx as isize;
            let p_children = p_parent.offset(children_offs);

            ChildrenIter { children: &*p_children, children_idx: 0, children_cnt: parent_wgt.link.children_cnt }
        }
    }
}

impl <'a> Iterator for ChildrenIter<'a> {
    type Item = &'a Widget;

    fn next(&mut self) -> Option<Self::Item> {
        if self.children_idx < self.children_cnt {
            unsafe {
                // SAFETY: see `wgt_get_parent`
                let p_child = (self.children as *const Widget).offset(self.children_idx as isize);
                self.children_idx += 1;
                Some(&*p_child)
            }
        }
        else {
            None
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

/// Iterator that traverses over parents hierarchy, starting at `wgt`, up to the root Window
pub struct ParentsIter <'a> {
    wgt: &'a Widget,
    finished: bool
}

impl <'a> ParentsIter<'a> {
    pub fn new(wgt: &'a Widget) -> Self {
        ParentsIter{wgt, finished: false}
    }
}

impl <'a> Iterator for ParentsIter<'a> {
    type Item = &'a Widget;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.finished {
            let ret = Some(self.wgt);
            self.finished = self.wgt.link.own_idx == 0;
            self.wgt = get_parent(self.wgt);
            ret
        }
        else {
            None
        }
    }
}

// ---------------------------------------------------------------------------------------------- //
// ---- PRIVATE FUNCTIONS ----------------------------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

fn pagectrl_change_page(ws: &mut dyn WindowState, pgctrl: &Widget, next: bool) {
    // assert(pWgt->type == Widget::PageCtrl);

    let pgidx = {
        let mut idx = ws.get_page_ctrl_page_index(pgctrl) as i16;
        idx += if next { 1 } else { -1 };
        if idx < 0 {
            idx = pgctrl.link.children_cnt as i16 -1;
        }
        if idx >= pgctrl.link.children_cnt as i16 {
            idx = 0;
        }
        idx as u8
    };

    ws.on_page_control_page_change(pgctrl, pgidx);
    ws.invalidate(&[pgctrl.id]);

    // cancel EDIT mode
    WGT_STATE.with(|wgstate|
        wgstate.borrow_mut().text_edit_state.wgt_id = WIDGET_ID_NONE
    );

    if let Some(focused) = find_by_id(ws.get_widgets(), ws.get_focused_id()) {
        // tr_debug!("focused id={} ({})", focused.id, focused.prop);
        WGT_STATE.with(|wgstate|
            wgstate.borrow_mut().focused_wgt = focused.id
        );

        if let Ok(mut term_lock) = crate::Term::try_lock_write() {
            wgt::set_cursor_at(&mut term_lock, ws, focused);
        }
        else {
            tr_debug!("Unable to lock the term");
        }
    }
    else {
        WGT_STATE.with(|wgstate|
            wgstate.borrow_mut().focused_wgt = WIDGET_ID_NONE
        );

        if let Ok(mut term_lock) = crate::Term::try_lock_write() {
            term_lock.move_to_home();
        }
        else {
            tr_debug!("Unable to lock the term");
        }
    }
}

fn is_focusable(ws: &mut dyn WindowState, wgt: &Widget) -> bool {
    if matches!(wgt.prop,
        Property::TextEdit(_)   |
        Property::CheckBox(_)   |
        Property::Radio(_)      |
        Property::Button(_)     |
        Property::ListBox(_)    |
        Property::ComboBox(_)
    ) {
        return true;
    }

    if let Property::TextBox(_) = wgt.prop {
        return is_enabled(ws, wgt)
    }

    return false;
}

fn is_focusable_by_id(ws: &mut dyn WindowState, widget_id: WId) -> bool {
    if let Some(wgt) = find_by_id(ws.get_widgets(), widget_id) {
        is_focusable(ws, wgt)
    }
    else {
        false
    }
}

fn get_next_focusable(ws: &mut dyn WindowState, mut parent: &'static Widget, focused_id: WId, forward: bool,
    mut first_parent: Option<&'static Widget>, break_search: &mut bool) -> Option<&'static Widget>
{
    if let Some(fp) = first_parent
    {
        if std::ptr::eq(parent, fp) {
            tr_err!("full loop detected");// (pFirstParent id=%d)", pFirstParent?pFirstParent->id:-1);
            *break_search = true;
            return None;
        }
    }

    if parent.id == focused_id {
        return None;
    }

    let mut childrens;
    let child_cnt;

    // get childrens and their number
    match parent.prop {
        Property::Window(_) |
        Property::Panel(_) |
        Property::Page(_) |
        Property::Layer(_) =>  {
            childrens = ws.get_widgets();
            childrens = &childrens[parent.link.children_idx as usize..];
            child_cnt = parent.link.children_cnt;
        }
        Property::PageCtrl(_) => {
            // get selected page childrens
            let idx = ws.get_page_ctrl_page_index(parent);

            if idx < parent.link.children_cnt {
                childrens = ws.get_widgets();
                parent = &childrens[parent.link.children_idx as usize + idx as usize];
                childrens = &childrens[parent.link.children_idx as usize..];
                child_cnt = parent.link.children_cnt;
            }
            else {
                return None;
            }
        }
        _ => {
            tr_err!("Not a parent type widget");
            return None;
        }
    }

    if child_cnt == 0 {
        return None;
    }

    match parent.prop {
        Property::Page(_) |
        Property::Panel(_) |
        Property::Layer(_) => {
            if first_parent.is_none() {
                // it must be Panel/Page/Layer because while traversing we never step below Page level
                // TWINS_LOG_D("1st parent[%s id:%u]", toString(pParent->type), pParent->id);

                first_parent = Some(parent);
            }
        }
        _ => {}
    }

    assert!(!childrens.is_empty());
    let mut wgt_idx = 0;
    tr_debug!("parent[{} id:{}] focused_id={}", parent.prop.to_string(), parent.id, focused_id); //crate::sleepMs(200);

    if focused_id == WIDGET_ID_NONE {
        // get first/last of the children ID
        wgt_idx = if forward { 0 } else { child_cnt as usize -1 };
        let wgt = &childrens[wgt_idx];

        if is_focusable(ws, wgt) && is_visible(ws, wgt) {
            return Some(wgt);
        }

        if is_parent(wgt) {
            if let Some(nf) = get_next_focusable(ws, parent, WIDGET_ID_NONE, forward, first_parent, break_search) {
                return Some(nf);
            }
        }
    }
    else {
        // get pointer to focusedID
        while wgt_idx < child_cnt as usize && childrens[wgt_idx].id != focused_id {
            wgt_idx += 1;
        }

        if wgt_idx >= child_cnt as usize {
            tr_warn!("Focused ID={} not found on parent ID={}", focused_id, parent.id);
            return None;
        }
    }

    tr_debug!("search in [%{} id:{} children:{}]", parent.prop.to_string(), parent.id, child_cnt);

    // iterate until focusable found or children border reached
    for _ in 0..child_cnt {
        if forward { wgt_idx += 1; } else { wgt_idx = wgt_idx.overflowing_sub(1).0; }
        let wgt = &childrens[wgt_idx];

        if wgt_idx == child_cnt as usize || wgt_idx == usize::MAX {
            // border reached: if we are on Panel or Layer, jump to next sibling
            match parent.prop {
                Property::Panel(_) |
                Property::Layer(_) => {
                    let p = get_parent(parent);
                    let mut brk = false;
                    return get_next_focusable(ws, p, parent.id, forward, first_parent, &mut brk);
                }
                _ => {
                    // make a turn around
                    if wgt_idx >= child_cnt as usize {
                        wgt_idx = 0;
                    }
                    else {
                        wgt_idx = child_cnt as usize;
                    }
                }
            }
        }

        if is_focusable(ws, wgt) && is_visible(ws, wgt) {
            return Some(wgt);
        }

        if is_parent(&childrens[wgt_idx]) {
            let mut brk = false;

            if let Some(nf) = get_next_focusable(ws, &childrens[wgt_idx], WIDGET_ID_NONE,
                    forward, first_parent, &mut brk) {
                return Some(nf);
            }

            if brk {
                break;
            }
        }
    }

    return None;
}

fn get_next_to_focus(ws: &mut dyn WindowState, focused_id: WId, forward: bool) -> WId {
    let mut focused_wgt = find_by_id(ws.get_widgets(), focused_id);

    if focused_wgt.is_none() {
        // here, find may fail only if invalid focusedID was given
        focused_wgt = ws.get_widgets().first();
    }

    if let Some(focused_wgt) = focused_wgt {
        let mut brk = false;
        if let Some(nf) = get_next_focusable(ws, focused_wgt, WIDGET_ID_NONE,
                forward, None, &mut brk) {
            return nf.id;
        }
    }

    return WIDGET_ID_NONE;
}

fn get_parent_to_focus(ws: &mut dyn WindowState, focused_id: WId) -> WId {
    if focused_id != WIDGET_ID_NONE {
        let focused_wgt = find_by_id(ws.get_widgets(), focused_id);

        if let Some(focused_wgt) = focused_wgt {
            let wgts = ws.get_widgets();
            let parent_wgt = &wgts[focused_wgt.link.parent_idx as usize];
            return parent_wgt.id;
        }
    }

    return ws.get_widgets().first().unwrap().id;
}

fn change_focus_to(ws: &mut dyn WindowState, new_id: WId) -> bool {
    let curr_id = ws.get_focused_id();
    tr_debug!("curr_id={}, new_id={}", curr_id, new_id);

    if new_id != curr_id {
        let prev_id = curr_id;
        ws.set_focused_id(new_id);
        let new_focused_wgt = find_by_id(ws.get_widgets(), new_id);

        if let Some(new_focused_wgt) = new_focused_wgt {
            if let Property::ListBox(_) = new_focused_wgt.prop {
                let mut idx = 0;
                let mut selidx = 0;
                let mut cnt = 0;
                ws.get_list_box_state(new_focused_wgt, &mut idx, &mut selidx, &mut cnt);

                if idx < 0 && cnt > 0 {
                    ws.on_list_box_select(new_focused_wgt, selidx);
                }
            }

            if let Ok(mut term_lock) = crate::Term::try_lock_write() {
                let term = &mut *term_lock;
                set_cursor_at(term, ws, new_focused_wgt);
            }

            WGT_STATE.with(|wgtstate|{
                wgtstate.borrow_mut().focused_wgt = new_focused_wgt.id;
            });
        }

        if is_focusable_by_id(ws, prev_id) {
            ws.invalidate(&[prev_id]);
        }

        if is_focusable_by_id(ws, new_id) {
            ws.invalidate(&[new_id]);
        }

        return true;
    }

    return false;
}

fn find_main_pg_control(ws: &mut dyn WindowState) -> Option<&'static Widget> {
    let wnd = &ws.get_widgets()[0];

    for i in 0..wnd.link.children_cnt {
        let idx = (wnd.link.children_idx + i) as usize;
        let wgt = &ws.get_widgets()[idx];

        if let Property::PageCtrl(_) = wgt.prop {
            return Some(wgt);
        }
    }

    return None;
}

fn combo_box_hide_list(ws: &mut dyn WindowState, wgt: &Widget) {
    assert!(matches!(wgt.prop, Property::ComboBox(_)));

    ws.on_combo_box_drop(wgt, false);
    // redraw parent to hide list
    let parent = get_parent(wgt);
    ws.invalidate(&[parent.id]);

    WGT_STATE.with(|wgtstate|{
        wgtstate.borrow_mut().drop_down_combo = WIDGET_ID_NONE;
    });
}
