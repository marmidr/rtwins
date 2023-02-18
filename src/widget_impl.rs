//! # RTWins Widget

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::string_ext::StrExt;
use crate::widget_def::*;
use crate::common::*;
use crate::input;
use crate::input::*;
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

pub fn process_input(ws: &mut dyn WindowState, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    CallCtx ctx(pWindowWidgets);
    bool key_processed = false;

    if (kc.key == Key::None)
        return true;

    // TWINS_LOG_D("---");

    if (kc.key == Key::MouseEvent)
    {
        key_processed = processMouse(ctx, kc);
    }
    else
    {
        key_processed = processKey(ctx, kc);

        if (!key_processed && kc.m_spec)
        {
            if (g_ws.pDropDownCombo)
            {
                comboBoxHideList(ctx, g_ws.pDropDownCombo);
            }

            switch (kc.key)
            {
            case Key::Esc:
            {
                auto curr_id = ctx.pState->getFocusedID();
                auto new_id = getParentToFocus(ctx, curr_id);
                key_processed = changeFocusTo(ctx, new_id);
                break;
            }
            case Key::Tab:
            {
                auto curr_id = ctx.pState->getFocusedID();
                auto new_id = getNextToFocus(ctx, curr_id, !kc.m_shift);
                key_processed = changeFocusTo(ctx, new_id);
                break;
            }
            default:
                break;
            }
        }

        if (!key_processed)
            key_processed = ctx.pState->onWindowUnhandledInputEvt(ctx.pWidgets, kc);
    }

    return key_processed;
    */
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

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGETS KEYBOARD PROCESSING FUNCTIONS --------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

fn process_key(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    auto focused_id = ctx.pState->getFocusedID();
    const Widget* p_wgt = getWidgetByWID(ctx, focused_id);
    bool key_handled = false;

    if (!p_wgt)
        return false;

    if (!isEnabled(ctx, p_wgt))
        return true;

    switch (p_wgt->type)
    {
    case Widget::TextEdit:
        key_handled = processKey_TextEdit(ctx, p_wgt, kc);
        break;
    case Widget::CheckBox:
        key_handled = processKey_CheckBox(ctx, p_wgt, kc);
        break;
    case Widget::Radio:
        key_handled = processKey_Radio(ctx, p_wgt, kc);
        break;
    case Widget::Button:
        key_handled = processKey_Button(ctx, p_wgt, kc);
        break;
    case Widget::PageCtrl:
        key_handled = processKey_PageCtrl(ctx, p_wgt, kc);
        break;
    case Widget::ListBox:
        key_handled = processKey_ListBox(ctx, p_wgt, kc);
        break;
    case Widget::ComboBox:
        key_handled = processKey_ComboBox(ctx, p_wgt, kc);
        break;
    case Widget::TextBox:
        key_handled = processKey_TextBox(ctx, p_wgt, kc);
        break;
    default:
        break;
    }

    return key_handled;
    */
}


fn process_key_text_edit(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    if (pWgt == g_ws.textEditState.pWgt)
    {
        // if in edit state, allow user to handle key
        if (ctx.pState->onTextEditInputEvt(pWgt, kc, g_ws.textEditState.str, g_ws.textEditState.cursorPos))
        {
            ctx.pState->invalidate(pWgt->id);
            return true;
        }
        // user let us continue checking the key
    }

    bool key_handled = false;

    if (g_ws.textEditState.pWgt)
    {
        auto cursor_pos = g_ws.textEditState.cursorPos;

        if (kc.m_spec)
        {
            switch (kc.key)
            {
            case Key::Esc:
                // cancel editing
                g_ws.textEditState.pWgt = nullptr;
                ctx.pState->invalidate(pWgt->id);
                key_handled = true;
                break;
            case Key::Tab:
                // real TAB may have different widths and require extra processing
                g_ws.textEditState.str.insert(cursor_pos, "    ");
                cursor_pos += 4;
                ctx.pState->invalidate(pWgt->id);
                key_handled = true;
                break;
            case Key::Enter:
                // finish editing
                ctx.pState->onTextEditChange(pWgt, std::move(g_ws.textEditState.str));
                g_ws.textEditState.pWgt = nullptr;
                ctx.pState->invalidate(pWgt->id);
                key_handled = true;
                break;
            case Key::Backspace:
                if (cursor_pos > 0)
                {
                    if (kc.m_ctrl)
                    {
                        g_ws.textEditState.str.erase(0, cursor_pos);
                        cursor_pos = 0;
                    }
                    else
                    {
                        g_ws.textEditState.str.erase(cursor_pos-1);
                        cursor_pos--;
                    }
                    ctx.pState->invalidate(pWgt->id);
                }
                key_handled = true;
                break;
            case Key::Delete:
                if (kc.m_ctrl)
                    g_ws.textEditState.str.trim(cursor_pos);
                else
                    g_ws.textEditState.str.erase(cursor_pos);

                key_handled = true;
                ctx.pState->invalidate(pWgt->id);
                break;
            case Key::Up:
            case Key::Down:
                break;
            case Key::Left:
                if (cursor_pos > 0)
                {
                    cursor_pos --;
                    ctx.pState->invalidate(pWgt->id);
                }
                key_handled = true;
                break;
            case Key::Right:
                if (cursor_pos < (signed)g_ws.textEditState.str.u8len())
                {
                    cursor_pos++;
                    ctx.pState->invalidate(pWgt->id);
                }
                key_handled = true;
                break;
            case Key::Home:
                cursor_pos = 0;
                ctx.pState->invalidate(pWgt->id);
                key_handled = true;
                break;
            case Key::End:
                cursor_pos = g_ws.textEditState.str.u8len();
                ctx.pState->invalidate(pWgt->id);
                key_handled = true;
                break;
            default:
                break;
            }
        }
        else
        {
            g_ws.textEditState.str.insert(cursor_pos, kc.utf8);
            cursor_pos++;
            ctx.pState->invalidate(pWgt->id);
            key_handled = true;
        }

        g_ws.textEditState.cursorPos = cursor_pos;
    }
    else if (kc.key == Key::Enter)
    {
        // enter edit mode
        g_ws.textEditState.pWgt = pWgt;
        g_ws.textEditState.str.clear();
        ctx.pState->getTextEditText(pWgt, g_ws.textEditState.str, true);
        g_ws.textEditState.cursorPos = g_ws.textEditState.str.u8len();
        ctx.pState->invalidate(pWgt->id);
        key_handled = true;
    }

    return key_handled;
 */
}

fn process_key_check_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    if (kc.mod_all == KEY_MOD_NONE && kc.utf8[0] == ' ')
    {
        ctx.pState->onCheckboxToggle(pWgt);
        ctx.pState->invalidate(pWgt->id);
        return true;
    }

    if (kc.key == Key::Enter)
    {
        ctx.pState->onCheckboxToggle(pWgt);
        ctx.pState->invalidate(pWgt->id);
        return true;
    }
    return false;
*/
}

fn process_key_radio(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    if (kc.mod_all == KEY_MOD_NONE && kc.utf8[0] == ' ')
    {
        ctx.pState->onRadioSelect(pWgt);
        invalidateRadioGroup(ctx, pWgt);
        return true;
    }

    if (kc.key == Key::Enter)
    {
        ctx.pState->onRadioSelect(pWgt);
        invalidateRadioGroup(ctx, pWgt);
        return true;
    }
    return false;
*/
}

fn process_key_button(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    auto *p_wstate = ctx.pState;

    if (p_wstate->onButtonKey(pWgt, kc))
    {
        // user handled the keyboard event
        return true;
    }

    if (kc.key == Key::Enter)
    {
        // pointer may change between onButtonUp and onButtonClick, so remember it
        g_ws.pMouseDownWgt = pWgt;
        p_wstate->onButtonDown(pWgt, kc);
        p_wstate->invalidate(pWgt->id, true);
        sleepMs(50);
        g_ws.pMouseDownWgt = nullptr;
        p_wstate->onButtonUp(pWgt, kc);
        p_wstate->onButtonClick(pWgt, kc);
        p_wstate->invalidate(pWgt->id);
        return true;
    }
    return false;
*/
}

fn process_key_page_ctrl(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    if (kc.key == Key::PgDown || kc.key == Key::PgUp ||
        kc.key == Key::F11 || kc.key == Key::F12)
    {
        pgControlChangePage(ctx, pWgt, kc.key == Key::PgDown || kc.key == Key::F12);
        return true;
    }
    return false;
*/
}

fn process_key_list_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    int delta = 0;
    const uint16_t items_visible = pWgt->size.height-2;

    switch (kc.key)
    {
    case Key::Enter:
    {
        int16_t idx = 0, selidx = 0, cnt = 0;
        ctx.pState->getListBoxState(pWgt, idx, selidx, cnt);
        if (cnt > 0)
        {
            if (selidx >= 0 && selidx != idx)
                ctx.pState->onListBoxChange(pWgt, selidx);
            ctx.pState->invalidate(pWgt->id);
        }
        return true;
    }
    case Key::Up:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? -1 : 0;
        break;
    case Key::Down:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? 1 : 0;
        break;
    case Key::PgUp:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? -items_visible : 0;
        break;
    case Key::PgDown:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? items_visible : 0;
        break;
    default:
        break;
    }

    if (delta != 0)
    {
        int16_t idx = 0, selidx = 0, cnt = 0;
        ctx.pState->getListBoxState(pWgt, idx, selidx, cnt);

        if (cnt > 0)
        {
            selidx += delta;

            if (selidx < 0)
                selidx = cnt - 1;

            if (selidx >= cnt)
                selidx = 0;

            ctx.pState->onListBoxSelect(pWgt, selidx);
            ctx.pState->invalidate(pWgt->id);
        }
        return true;
    }
    return false;
*/
}

fn process_key_combo_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    int16_t idx = 0, selidx = 0, cnt = 0; bool drop_down = false;
    ctx.pState->getComboBoxState(pWgt, idx, selidx, cnt, drop_down);

    if (kc.utf8[0] == ' ')
    {
        if (cnt > 0)
        {
            drop_down = !drop_down;

            if (drop_down)
            {
                ctx.pState->onComboBoxDrop(pWgt, true);
                g_ws.pDropDownCombo = pWgt;
            }
            else
            {
                comboBoxHideList(ctx, pWgt);
            }
        }
    }
    else if (kc.key == Key::Esc)
    {
        comboBoxHideList(ctx, pWgt);
    }
    else if (drop_down)
    {
        if (kc.key == Key::Up)
        {
            if (--selidx < 0) selidx = cnt-1;
            ctx.pState->onComboBoxSelect(pWgt, selidx);
        }
        else if (kc.key == Key::Down)
        {
            if (++selidx >= cnt) selidx = 0;
            ctx.pState->onComboBoxSelect(pWgt, selidx);
        }
        else if (kc.key == Key::PgUp && kc.mod_all == KEY_MOD_SPECIAL)
        {
            selidx -= pWgt->combobox.dropDownSize;
            if (selidx < 0) selidx = cnt-1;
            ctx.pState->onComboBoxSelect(pWgt, selidx);
        }
        else if (kc.key == Key::PgDown && kc.mod_all == KEY_MOD_SPECIAL)
        {
            selidx += pWgt->combobox.dropDownSize;
            if (selidx >= cnt) selidx = 0;
            ctx.pState->onComboBoxSelect(pWgt, selidx);
        }
        else if (kc.key == Key::Enter)
        {
            ctx.pState->onComboBoxChange(pWgt, selidx);
            comboBoxHideList(ctx, pWgt);
        }
        else
        {
            return false;
        }
    }
    else
    {
        return false;
    }

    ctx.pState->invalidate(pWgt->id);
    return true;
    */
}

fn process_key_text_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    unimplemented!();
/*
    int delta = 0;
    const uint16_t lines_visible = pWgt->size.height - 2;

    switch (kc.key)
    {
    case Key::Up:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? -1 : 0;
        break;
    case Key::Down:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? 1 : 0;
        break;
    case Key::PgUp:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? -lines_visible : 0;
        break;
    case Key::PgDown:
        delta = kc.mod_all == KEY_MOD_SPECIAL ? lines_visible : 0;
        break;
    default:
        break;
    }

    if (delta != 0)
    {
        const twins::Vector<twins::CStrView> *p_lines = nullptr;
        int16_t top_line = 0;

        ctx.pState->getTextBoxState(pWgt, &p_lines, top_line);

        if (p_lines)
        {
            top_line += delta;

            if (top_line > (int)p_lines->size() - lines_visible)
                top_line = p_lines->size() - lines_visible;

            if (top_line < 0)
                top_line = 0;

            ctx.pState->onTextBoxScroll(pWgt, top_line);
            ctx.pState->invalidate(pWgt->id);
        }
        return true;
    }
    return false;
*/
}

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGETS MOUSE PROCESSING FUNCTIONS ------------------------------------------------------ //
// ---------------------------------------------------------------------------------------------- //

fn process_mouse(ws: &mut dyn WindowState, ii: &InputInfo) -> bool
{
    unimplemented!();
/*
    if (kc.mouse.btn == MouseBtn::ButtonGoBack || kc.mouse.btn == MouseBtn::ButtonGoForward)
    {
        if (const auto *p_wgt = findMainPgControl(ctx))
        {
            if (isEnabled(ctx, p_wgt))
                pgControlChangePage(ctx, p_wgt, kc.mouse.btn == MouseBtn::ButtonGoForward);
            return true;
        }
    }

    Rect rct;
    const Widget *p_wgt = getWidgetAt(ctx, kc.mouse.col, kc.mouse.row, rct);

    if (g_ws.pMouseDownWgt)
    {
        // apply only for Button widget
        if (g_ws.pMouseDownWgt->type == Widget::Button)
        {
            // mouse button released over another widget - generate Up event for previously clicked button
            if (kc.mouse.btn == MouseBtn::ButtonReleased && g_ws.pMouseDownWgt != p_wgt)
            {
                processMouse_Button_Release(ctx, g_ws.pMouseDownWgt, kc);
                return true;
            }
        }
    }
    else if (p_wgt)
    {
        // remember clicked widget
        if (kc.mouse.btn >= MouseBtn::ButtonLeft && kc.mouse.btn < MouseBtn::ButtonReleased)
        {
            g_ws.pMouseDownWgt = p_wgt;
            g_ws.mouseDownKeyCode = kc;
        }
    }

    if (!p_wgt)
        return false;

    // TWINS_LOG_D("WidgetAt(%2d:%2d)=%s ID:%u", kc.mouse.col, kc.mouse.row, toString(p_wgt->type), p_wgt->id);

    if (g_ws.pDropDownCombo)
    {
        // check if drop-down list clicked
        Rect dropdownlist_rct;
        dropdownlist_rct.coord = getScreenCoord(g_ws.pDropDownCombo);
        dropdownlist_rct.coord.row++;
        dropdownlist_rct.size.width = g_ws.pDropDownCombo->size.width;
        dropdownlist_rct.size.height = g_ws.pDropDownCombo->combobox.dropDownSize;

        if (isPointWithin(kc.mouse.col, kc.mouse.row, dropdownlist_rct))
        {
            // yes -> replace data for processing with g_ds.pDropDownCombo
            p_wgt = g_ws.pDropDownCombo;
            rct.coord = getScreenCoord(g_ws.pDropDownCombo);
            rct.size = g_ws.pDropDownCombo->size;
        }
        else
        {
            if (kc.mouse.btn == MouseBtn::ButtonLeft)
                comboBoxHideList(ctx, g_ws.pDropDownCombo);
        }
    }

    if (isEnabled(ctx, p_wgt))
    {
        switch (p_wgt->type)
        {
        case Widget::TextEdit:
            processMouse_TextEdit(ctx, p_wgt, rct, kc);
            break;
        case Widget::CheckBox:
            processMouse_CheckBox(ctx, p_wgt, rct, kc);
            break;
        case Widget::Radio:
            processMouse_Radio(ctx, p_wgt, rct, kc);
            break;
        case Widget::Button:
            processMouse_Button(ctx, p_wgt, rct, kc);
            break;
        case Widget::PageCtrl:
            processMouse_PageCtrl(ctx, p_wgt, rct, kc);
            break;
        case Widget::ListBox:
            processMouse_ListBox(ctx, p_wgt, rct, kc);
            break;
        case Widget::ComboBox:
            processMouse_ComboBox(ctx, p_wgt, rct, kc);
            break;
        case Widget::CustomWgt:
            processMouse_CustomWgt(ctx, p_wgt, rct, kc);
            break;
        case Widget::TextBox:
            processMouse_TextBox(ctx, p_wgt, rct, kc);
            break;
        default:
            moveToHome();
            g_ws.pMouseDownWgt = nullptr;
            return false;
        }
    }

    if (kc.mouse.btn == MouseBtn::ButtonReleased)
        g_ws.pMouseDownWgt = nullptr;

    return true;
*/
}

fn process_mouse_text_edit(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    if (kc.mouse.btn == MouseBtn::ButtonLeft)
    {
        changeFocusTo(ctx, pWgt->id);
    }
*/
}

fn process_mouse_check_box(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    if (kc.mouse.btn == MouseBtn::ButtonLeft)
    {
        changeFocusTo(ctx, pWgt->id);
        ctx.pState->onCheckboxToggle(pWgt);
        ctx.pState->invalidate(pWgt->id);
    }
*/
}

fn process_mouse_radio(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    if (kc.mouse.btn == MouseBtn::ButtonLeft)
    {
        changeFocusTo(ctx, pWgt->id);
        ctx.pState->onRadioSelect(pWgt);
        invalidateRadioGroup(ctx, pWgt);
    }
*/
}

fn process_mouse_button(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    // pointer may change between onButtonUp and onButtonClick, so remember it
    auto *p_wstate = ctx.pState;

    if (kc.mouse.btn == MouseBtn::ButtonLeft)
    {
        changeFocusTo(ctx, pWgt->id);
        p_wstate->onButtonDown(pWgt, kc);
        p_wstate->invalidate(pWgt->id);
    }
    else if (kc.mouse.btn == MouseBtn::ButtonReleased && g_ws.pMouseDownWgt == pWgt)
    {
        p_wstate->onButtonUp(pWgt, kc);
        p_wstate->onButtonClick(pWgt, g_ws.mouseDownKeyCode);
        g_ws.pMouseDownWgt = nullptr;
        p_wstate->invalidate(pWgt->id);
    }
    else
    {
        g_ws.pMouseDownWgt = nullptr;
    }
*/
}

fn process_mouse_button_release(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo)
{
    unimplemented!();
/*
    auto *p_wstate = ctx.pState;

    p_wstate->onButtonUp(pWgt, kc);
    g_ws.pMouseDownWgt = nullptr;
    p_wstate->invalidate(pWgt->id);
*/
}

fn process_mouse_page_ctrl(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    if (kc.mouse.btn == MouseBtn::ButtonLeft)
    {
        changeFocusTo(ctx, pWgt->id);
        int idx = ctx.pState->getPageCtrlPageIndex(pWgt);
        int new_idx = kc.mouse.row - wgtRect.coord.row - 1 - pWgt->pagectrl.vertOffs;

        if (new_idx != idx && new_idx >= 0 && new_idx < pWgt->link.childrenCnt)
        {
            ctx.pState->onPageControlPageChange(pWgt, new_idx);
            ctx.pState->invalidate(pWgt->id);
        }
    }
    else if (kc.mouse.btn == MouseBtn::WheelUp || kc.mouse.btn == MouseBtn::WheelDown)
    {
        pgControlChangePage(ctx, pWgt, kc.mouse.btn == MouseBtn::WheelDown);
    }
*/
}

fn process_mouse_list_box(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    const uint16_t items_visible = pWgt->size.height-2;

    if (kc.mouse.btn == MouseBtn::ButtonLeft || kc.mouse.btn == MouseBtn::ButtonMid)
    {
        bool focus_changed = changeFocusTo(ctx, pWgt->id);

        int16_t idx = 0, selidx = 0, cnt = 0;
        ctx.pState->getListBoxState(pWgt, idx, selidx, cnt);

        if (cnt <= 0)
            return;

        int page = selidx / items_visible;
        unsigned new_selidx = page * items_visible;
        new_selidx += (int)kc.mouse.row - wgtRect.coord.row - 1;

        if (kc.mouse.btn == MouseBtn::ButtonLeft)
        {
            if (new_selidx < (unsigned)cnt && (((signed)new_selidx != selidx) || focus_changed))
            {
                selidx = new_selidx;
                ctx.pState->onListBoxSelect(pWgt, selidx);
            }
        }
        else
        {
            if (new_selidx < (unsigned)cnt && new_selidx != (unsigned)idx)
            {
                selidx = new_selidx;
                ctx.pState->onListBoxSelect(pWgt, selidx);
                ctx.pState->onListBoxChange(pWgt, selidx);
            }
        }

        ctx.pState->invalidate(pWgt->id);
    }
    else if (kc.mouse.btn == MouseBtn::WheelUp || kc.mouse.btn == MouseBtn::WheelDown)
    {
        changeFocusTo(ctx, pWgt->id);

        int16_t idx = 0, selidx = 0, cnt = 0;
        ctx.pState->getListBoxState(pWgt, idx, selidx, cnt);

        if (cnt <= 0)
            return;

        int delta = kc.mouse.btn == MouseBtn::WheelUp ? -1 : 1;
        if (kc.m_ctrl) delta *= items_visible;
        selidx += delta;

        if (selidx < 0)
            selidx = cnt - 1;

        if (selidx >= cnt)
            selidx = 0;

        ctx.pState->onListBoxSelect(pWgt, selidx);
        ctx.pState->invalidate(pWgt->id);
    }

*/
}

fn process_mouse_combo_box(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    if (kc.mouse.btn == MouseBtn::ButtonLeft)
    {
        changeFocusTo(ctx, pWgt->id);

        auto col = kc.mouse.col - wgtRect.coord.col;
        auto row = kc.mouse.row - wgtRect.coord.row - 1;

        if (row >= 0 && row < pWgt->combobox.dropDownSize)
        {
            int16_t idx = 0, selidx = 0, cnt = 0; bool drop_down = false;
            ctx.pState->getComboBoxState(pWgt, idx, selidx, cnt, drop_down);
            selidx = (selidx / pWgt->combobox.dropDownSize) * pWgt->combobox.dropDownSize; // top item
            selidx += row;
            if (selidx < cnt)
            {
                ctx.pState->onComboBoxSelect(pWgt, selidx);
                ctx.pState->invalidate(pWgt->id);
            }
        }
        else if (col >= wgtRect.size.width - 3 && col <= wgtRect.size.width - 1)
        {
            // drop down arrow clicked
            int16_t _, cnt = 0; bool drop_down = false;
            ctx.pState->getComboBoxState(pWgt, _, _, cnt, drop_down);

            if (cnt <= 0)
                return;

            drop_down = !drop_down;

            if (drop_down)
            {
                ctx.pState->onComboBoxDrop(pWgt, true);
                ctx.pState->invalidate(pWgt->id);
                g_ws.pDropDownCombo = pWgt;
            }
            else
            {
                comboBoxHideList(ctx, pWgt);
            }
        }
    }
    else if (kc.mouse.btn == MouseBtn::WheelUp || kc.mouse.btn == MouseBtn::WheelDown)
    {
        changeFocusTo(ctx, pWgt->id);

        int16_t idx = 0, selidx = 0, cnt = 0; bool drop_down = false;
        ctx.pState->getComboBoxState(pWgt, idx, selidx, cnt, drop_down);

        if (!drop_down || cnt <= 0)
            return;

        int delta = kc.mouse.btn == MouseBtn::WheelUp ? -1 : 1;
        if (kc.m_ctrl) delta *= pWgt->combobox.dropDownSize;
        selidx += delta;

        if (selidx < 0)
            selidx = cnt - 1;

        if (selidx >= cnt)
            selidx = 0;

        ctx.pState->onComboBoxSelect(pWgt, selidx);
        ctx.pState->invalidate(pWgt->id);
    }
    else if (kc.mouse.btn == MouseBtn::ButtonMid)
    {
        twins::KeyCode key_left = kc;
        key_left.mouse.btn = MouseBtn::ButtonLeft;
        processMouse_ComboBox(ctx, pWgt, wgtRect, key_left);

        int16_t _, selidx = 0; bool drop_down = false;
        ctx.pState->getComboBoxState(pWgt, _, selidx, _, drop_down);

        if (!drop_down)
            return;

        ctx.pState->onComboBoxChange(pWgt, selidx);
        comboBoxHideList(ctx, pWgt);
    }

*/
}

fn process_mouse_custom_wgt(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    ctx.pState->onCustomWidgetInputEvt(pWgt, kc);
*/
}

fn process_mouse_text_box(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo)
{
    unimplemented!();
/*
    changeFocusTo(ctx, pWgt->id);

    if (kc.mouse.btn == MouseBtn::WheelUp || kc.mouse.btn == MouseBtn::WheelDown)
    {
        const twins::Vector<twins::CStrView> *p_lines = nullptr;
        int16_t top_line = 0;

        ctx.pState->getTextBoxState(pWgt, &p_lines, top_line);

        if (p_lines && p_lines->size())
        {
            int delta = kc.mouse.btn == MouseBtn::WheelUp ? -1 : 1;
            const uint16_t lines_visible = pWgt->size.height - 2;
            if (kc.m_ctrl) delta *= lines_visible;

            top_line += delta;

            if (top_line > (int)p_lines->size() - lines_visible)
                top_line = p_lines->size() - lines_visible;

            if (top_line < 0)
                top_line = 0;

            changeFocusTo(ctx, pWgt->id);
            ctx.pState->onTextBoxScroll(pWgt, top_line);
            ctx.pState->invalidate(pWgt->id);
        }
    }

*/
}

// ---------------------------------------------------------------------------------------------- //
