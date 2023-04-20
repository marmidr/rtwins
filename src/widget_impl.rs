//! # RTWins Widget

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::common::*;
use crate::input::*;
use crate::string_ext::*;
use crate::widget_def::*;
use crate::*; // tr_info

use atomic_once_cell::AtomicLazy;
use try_lock::TryLock;

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

// ---------------------------------------------------------------------------------------------- //

/// State object for current top window.
// using WId instead of references will solve lifetime problems
#[derive(Default)]
pub(crate) struct WidgetState {
    pub focused_wgt: WId,
    pub mouse_down_wgt: WId,
    pub cbx_drop_down: WId,
    pub text_edit_state: TextEditState,
    pub mouse_down_ii: InputInfo,
}

impl WidgetState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[allow(dead_code)]
#[derive(Default, Clone)]
pub(crate) struct TextEditState {
    pub wgt_id: WId,
    pub cursor_pos: i16,
    pub txt: String,
}

pub(crate) static WGT_STATE: AtomicLazy<TryLock<WidgetState>> =
    AtomicLazy::new(|| TryLock::new(WidgetState::default()));

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

    const fn do_transform<const N: usize>(
        mut out: [Widget; N],
        wgt: &Widget,
        out_idx: usize,
        mut next_free_idx: usize,
    ) -> (usize, [Widget; N]) {
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
    matches!(
        wgt.prop,
        Property::Window(_)
            | Property::Panel(_)
            | Property::PageCtrl(_)
            | Property::Page(_)
            | Property::Layer(_)
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
        let wgt = wgt as *const Widget;
        &*wgt.offset(parent_idx_offset)
    }
}

/// Search for Widget with given `id` in transformed widgets array
pub fn find_by_id(wndarray: &[Widget], id: WId) -> Option<&Widget> {
    if id != WIDGET_ID_NONE {
        wndarray.iter().find(|&&item| item.id == id)
    }
    else {
        None
    }
}

/// Finds visible Widget at cursor position `col:row`;
/// Sets `wgt_rect` to found widget screen-based coordinates
pub fn find_at(
    ws: &mut dyn WindowState,
    col: u8,
    row: u8,
    wgt_rect: &mut Rect,
) -> Option<&'static Widget> {
    let mut found_wgt: Option<&'static Widget> = None;
    let mut best_rect = Rect::cdeflt();
    best_rect.set_max();
    let wgts = ws.get_widgets();

    for wgt in wgts.iter() {
        let mut stop_searching = true;
        let mut wgt_screen_rect = Rect::cdeflt();

        wgt_screen_rect.coord = get_screen_coord(ws, wgt);
        wgt_screen_rect.size = wgt.size;

        // eprintln!("{1:2}:{0:} at {2:2}:{3:-2}", wgt.prop, wgt.id, wgt_screen_rect.coord.col, wgt_screen_rect.coord.row);

        // correct the widget size
        match wgt.prop {
            Property::TextEdit(ref _p) => {}
            Property::CheckBox(ref p) => {
                wgt_screen_rect.size.height = 1;
                wgt_screen_rect.size.width = 4 + p.text.displayed_width() as u8;
            }
            Property::Radio(ref p) => {
                wgt_screen_rect.size.height = 1;
                wgt_screen_rect.size.width = 4 + p.text.displayed_width() as u8;
            }
            Property::Button(ref p) => {
                let txt_w = {
                    if !p.text.is_empty() {
                        p.text.displayed_width() as u8
                    }
                    else if wgt.size.width > 0 {
                        wgt.size.width
                    }
                    else {
                        let mut s = String::new();
                        ws.get_button_text(wgt, &mut s);
                        s.displayed_width() as u8
                    }
                };

                match p.style {
                    ButtonStyle::Simple => {
                        wgt_screen_rect.size.height = 1;
                        wgt_screen_rect.size.width = 4 + txt_w;
                    }
                    ButtonStyle::Solid => {
                        wgt_screen_rect.size.height = 1;
                        wgt_screen_rect.size.width = 2 + txt_w;
                    }
                    ButtonStyle::Solid1p5 => {
                        wgt_screen_rect.size.height = 3;
                        wgt_screen_rect.size.width = 2 + txt_w;
                    }
                }
            }
            Property::PageCtrl(ref p) => {
                wgt_screen_rect.size.width = p.tab_width;
            }
            Property::ListBox(ref _p) => {}
            Property::ComboBox(ref _p) => {}
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

pub fn get_screen_coord(ws: &mut dyn WindowState, wgt: &Widget) -> Coord {
    if matches!(wgt.prop, Property::Window(_)) {
        return ws.get_window_coord();
    }

    wgt.iter_parents()
        .skip(1) // because iterator starts at the widget itself
        .fold(wgt.coord, |mut coord, parent| {
            match parent.prop {
                Property::Window(ref prop) => {
                    coord += ws.get_window_coord();
                }
                Property::PageCtrl(ref prop) => {
                    coord += parent.coord;
                    coord.col += prop.tab_width;
                }
                _ => coord += parent.coord,
            }

            coord
        })
}

/// Move cursor to the best position for given type of the widget
pub fn set_cursor_at(term: &mut Term, ws: &mut dyn WindowState, wgt: &Widget) {
    let mut coord = get_screen_coord(ws, wgt);

    match wgt.prop {
        Property::TextEdit(ref _p) => {
            let wgtstate_guard = WGT_STATE.try_lock().unwrap();
            let te_state = &wgtstate_guard.text_edit_state;

            if wgt.id == te_state.wgt_id {
                let max_w = wgt.size.width - 3;
                coord.col += te_state.cursor_pos as u8;
                let mut cursor_pos = te_state.cursor_pos;
                let delta = max_w / 2;

                while cursor_pos >= max_w as i16 - 1 {
                    coord.col -= delta;
                    cursor_pos -= delta as i16;
                }
            }
            else {
                coord.col += wgt.size.width - 2;
            }
        }
        Property::CheckBox(ref _p) => {
            coord.col += 1;
        }
        Property::Radio(ref _p) => {
            coord.col += 1;
        }
        Property::Button(ref p) => match p.style {
            ButtonStyle::Simple => {
                coord.col += 2;
            }
            ButtonStyle::Solid => {
                coord.col += 1;
            }
            ButtonStyle::Solid1p5 => {
                coord.col += 1;
                coord.row += 1;
            }
        },
        Property::PageCtrl(ref p) => {
            coord.row += 1 + p.vert_offs;
            coord.row += ws.get_page_ctrl_page_index(wgt) as u8
        }
        Property::ListBox(ref p) => {
            let mut lbs = Default::default();
            let frame_size = p.no_frame as u8;
            ws.get_list_box_state(wgt, &mut lbs);

            let page_size = wgt.size.height - (frame_size * 2);
            let row = lbs.sel_idx % page_size as i16;

            coord.col += frame_size;
            coord.row += frame_size + row as u8;
        }
        _ => {}
    }

    term.move_to(coord.col as u16, coord.row as u16);
}

pub fn is_visible(ws: &dyn WindowState, wgt: &Widget) -> bool {
    wgt.iter_parents().all(|wgt| ws.is_visible(wgt))
}

pub fn is_enabled(ws: &dyn WindowState, wgt: &Widget) -> bool {
    wgt.iter_parents().all(|wgt| ws.is_enabled(wgt))
}

/// Shall be called eg. on top window change
pub fn reset_internal_state() {
    WGT_STATE.try_lock().unwrap().reset();
}

pub fn process_input(ws: &mut dyn WindowState, ii: &InputInfo) -> bool {
    let mut input_handled;

    // TWINS_LOG_D("---");
    match ii.evnt {
        InputEvent::None => {
            input_handled = true;
        }
        InputEvent::Mouse(_) => {
            input_handled = process_mouse(ws, ii);
        }
        InputEvent::Key(_) | InputEvent::Char(_) => {
            input_handled = process_key(ws, ii);

            if !input_handled && ii.kmod.has_special() {
                let dd_combo_id = WGT_STATE.try_lock().unwrap().cbx_drop_down;

                if let Some(wgt) = find_by_id(ws.get_widgets(), dd_combo_id) {
                    hide_combo_box_dropdown_list(ws, wgt);
                }

                if let InputEvent::Key(ref key) = ii.evnt {
                    match *key {
                        Key::Esc => {
                            let curr_id = ws.get_focused_id();
                            let new_id = get_parent_to_focus(ws, curr_id);
                            input_handled = change_focus_to(ws, new_id);
                        }
                        Key::Tab => {
                            let curr_id = ws.get_focused_id();
                            let new_id = get_next_to_focus(ws, curr_id, !ii.kmod.has_shift());
                            input_handled = change_focus_to(ws, new_id);
                        }
                        _ => {}
                    }
                }
            }

            if !input_handled {
                if let Some(wgt) = ws.get_widgets().first() {
                    input_handled = ws.on_window_unhandled_input_evt(wgt, ii);
                }
            }
        }
    }

    input_handled
}

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGETS HELPER FUNCTIONS ---------------------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

/// Returns given page index on parent PageCtrl
pub fn page_page_idx(page: &Widget) -> Option<i16> {
    if let Property::Page(_) = page.prop {
        let pgctrl = get_parent(page);

        for (idx, pg) in pgctrl.iter_children().enumerate() {
            if page.id == pg.id {
                return Some(idx as i16);
            }
        }
    }

    None
}

/// Returns WId of page at PageCtrl pages index
pub fn pagectrl_page_wid(pgctrl: &Widget, page_idx: i16) -> WId {
    if let Property::PageCtrl(_) = pgctrl.prop {
        if let Some(pg) = pgctrl.iter_children().nth(page_idx as usize) {
            return pg.id;
        }
    }

    WIDGET_ID_NONE
}

/// Returns PageCtrl pages count
pub fn pagectrl_page_count(widgets: &[Widget], pgctrl_id: WId) -> u16 {
    if let Some(pgctrl) = find_by_id(widgets, pgctrl_id) {
        pgctrl.link.children_cnt
    }
    else {
        0
    }
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
                ws.invalidate(pgctrl_id);
                return;
            }
        }
    }

    tr_warn!(
        "Widget Id={} is not PageControl Id={} page",
        page_id,
        pgctrl_id
    );
}

pub fn pagectrl_select_next_page(ws: &mut dyn WindowState, pgctrl_id: WId, next: bool) {
    if let Some(pgctrl) = find_by_id(ws.get_widgets(), pgctrl_id) {
        pagectrl_change_page(ws, pgctrl, next);
    }
}

/// Mark internal clicked widget id
pub fn mark_button_down(btn: &Widget, is_down: bool) {
    WGT_STATE.try_lock().unwrap().mouse_down_wgt = tetrary!(is_down, btn.id, WIDGET_ID_NONE);
}

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGET ITERATORS ------------------------------------------------------------------------ //
// ---------------------------------------------------------------------------------------------- //

/// Iterator over parent-type Widget children
pub struct ChildrenIter<'a> {
    children: &'a Widget,
    children_idx: u16,
    children_cnt: u16,
}

impl<'a> ChildrenIter<'a> {
    /// Creates a new iterator.
    ///
    /// If the given widget happen to be not a parent-type widget,
    /// first iteration will fail anyway as the child counter is 0
    pub fn new(parent_wgt: &'a Widget) -> Self {
        unsafe {
            // SAFETY: see `wgt_get_parent`
            let p_parent = parent_wgt as *const Widget;
            let children_offs =
                parent_wgt.link.children_idx as isize - parent_wgt.link.own_idx as isize;
            let p_children = p_parent.offset(children_offs);

            ChildrenIter {
                children: &*p_children,
                children_idx: 0,
                children_cnt: parent_wgt.link.children_cnt,
            }
        }
    }
}

impl<'a> Iterator for ChildrenIter<'a> {
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
pub struct ParentsIter<'a> {
    wgt: &'a Widget,
    finished: bool,
}

impl<'a> ParentsIter<'a> {
    pub fn new(wgt: &'a Widget) -> Self {
        ParentsIter {
            wgt,
            finished: false,
        }
    }
}

impl<'a> Iterator for ParentsIter<'a> {
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
    if matches!(
        wgt.prop,
        Property::TextEdit(_)
            | Property::CheckBox(_)
            | Property::Radio(_)
            | Property::Button(_)
            | Property::ListBox(_)
            | Property::ComboBox(_)
            | Property::TextBox(_)
    ) {
        return is_enabled(ws, wgt);
    }

    false
}

fn is_focusable_by_id(ws: &mut dyn WindowState, widget_id: WId) -> bool {
    if let Some(wgt) = find_by_id(ws.get_widgets(), widget_id) {
        is_focusable(ws, wgt)
    }
    else {
        false
    }
}

fn get_next_focusable(
    ws: &mut dyn WindowState,
    mut parent: &'static Widget,
    focused_id: WId,
    forward: bool,
    mut first_parent: Option<&'static Widget>,
    break_search: &mut bool,
) -> Option<&'static Widget> {
    if let Some(fp) = first_parent {
        if core::ptr::eq(parent, fp) {
            tr_warn!("full loop detected"); // (pFirstParent id=%d)", pFirstParent?pFirstParent->id:-1);
            *break_search = true;
            return None;
        }
    }

    if parent.id == focused_id {
        return None;
    }

    let children;
    let child_cnt;

    // get childrens and their number
    match parent.prop {
        Property::Window(_) | Property::Panel(_) | Property::Page(_) | Property::Layer(_) => {
            children = &ws.get_widgets()[parent.link.children_idx as usize..];
            child_cnt = parent.link.children_cnt;
        }
        Property::PageCtrl(_) => {
            // get selected page childrens
            let idx = ws.get_page_ctrl_page_index(parent);

            if idx < parent.link.children_cnt as i16 {
                let wgts = ws.get_widgets();
                let idx = parent.link.children_idx as usize + idx as usize;
                parent = &wgts[idx];
                children = &wgts[parent.link.children_idx as usize..];
                child_cnt = parent.link.children_cnt;
            }
            else {
                return None;
            }
        }
        _ => {
            tr_warn!(
                "Widget [{} id:{}] is not a parent type widget",
                parent.prop.to_string(),
                parent.id
            );
            return None;
        }
    }

    if child_cnt == 0 {
        return None;
    }

    match parent.prop {
        Property::Page(_) | Property::Panel(_) | Property::Layer(_) => {
            if first_parent.is_none() {
                // it must be Panel/Page/Layer because while traversing we never step below Page level
                tr_debug!("1st parent[{} id:{}]", parent.prop.to_string(), parent.id);
                first_parent = Some(parent);
            }
        }
        _ => {}
    }

    assert!(!children.is_empty());
    let mut child_idx = 0;
    tr_debug!(
        "parent[{} id:{}] focused_id={}",
        parent.prop.to_string(),
        parent.id,
        focused_id
    );

    if focused_id == WIDGET_ID_NONE {
        // get first/last of the children ID
        child_idx = tetrary!(forward, 0, child_cnt as usize - 1);
        let child = &children[child_idx];

        if is_focusable(ws, child) && is_visible(ws, child) {
            return Some(child);
        }

        if is_parent(child) {
            if let Some(nf) = get_next_focusable(
                ws,
                parent,
                WIDGET_ID_NONE,
                forward,
                first_parent,
                break_search,
            ) {
                return Some(nf);
            }
        }
    }
    else {
        // get pointer to focusedID
        while child_idx < child_cnt as usize && children[child_idx].id != focused_id {
            child_idx += 1;
        }

        if child_idx >= child_cnt as usize {
            tr_warn!(
                "Focused ID={} not found on parent ID={}",
                focused_id,
                parent.id
            );
            return None;
        }
    }

    tr_debug!(
        "Search in [{} id:{} children cnt:{}]",
        parent.prop.to_string(),
        parent.id,
        child_cnt
    );

    // iterate until focusable found or children border reached
    for _ in 0..child_cnt {
        if forward {
            child_idx += 1;
        }
        else {
            child_idx = child_idx.overflowing_sub(1).0;
        }

        if child_idx == child_cnt as usize || child_idx == usize::MAX {
            // border reached: if we are on Panel or Layer, jump to next sibling
            match parent.prop {
                Property::Panel(_) | Property::Layer(_) => {
                    let parents_parent = get_parent(parent);
                    let mut brk = false;

                    return get_next_focusable(
                        ws,
                        parents_parent,
                        parent.id,
                        forward,
                        first_parent,
                        &mut brk,
                    );
                }
                _ => {}
            }

            // make a turn around
            if child_idx >= child_cnt as usize {
                child_idx = 0;
            }
            else {
                child_idx = child_cnt as usize - 1;
            }
        }

        let child = &children[child_idx];
        if is_focusable(ws, child) && is_visible(ws, child) {
            return Some(child);
        }

        if is_parent(&children[child_idx]) {
            let mut brk = false;

            if let Some(nf) = get_next_focusable(
                ws,
                &children[child_idx],
                WIDGET_ID_NONE,
                forward,
                first_parent,
                &mut brk,
            ) {
                return Some(nf);
            }

            if brk {
                break;
            }
        }
    }

    None
}

fn get_next_to_focus(ws: &mut dyn WindowState, focused_id: WId, forward: bool) -> WId {
    let mut focused_wgt = find_by_id(ws.get_widgets(), focused_id);

    if focused_wgt.is_none() {
        // here, fail is possible only if invalid focused_id was passed
        focused_wgt = ws.get_widgets().first();
    }

    if let Some(focused_wgt) = focused_wgt {
        tr_debug!(
            "focused_wgt: {} id={}",
            focused_wgt.prop.to_string(),
            focused_wgt.id
        );

        // use the parent to get next widget
        let focused_wgt_parent = get_parent(focused_wgt);
        let mut brk = false;
        if let Some(nf) =
            get_next_focusable(ws, focused_wgt_parent, focused_id, forward, None, &mut brk)
        {
            return nf.id;
        }
    }

    WIDGET_ID_NONE
}

fn get_parent_to_focus(ws: &mut dyn WindowState, focused_id: WId) -> WId {
    if let Some(focused_wgt) = find_by_id(ws.get_widgets(), focused_id) {
        let wgts = ws.get_widgets();
        let parent_wgt = &wgts[focused_wgt.link.parent_idx as usize];
        return parent_wgt.id;
    }

    return ws.get_widgets().first().unwrap().id;
}

fn change_focus_to(ws: &mut dyn WindowState, new_id: WId) -> bool {
    let curr_id = ws.get_focused_id();
    // tr_debug!("change_focus_to() curr_id={}, new_id={}", curr_id, new_id);

    if new_id != curr_id {
        let prev_id = curr_id;
        ws.set_focused_id(new_id);

        if let Some(new_focused_wgt) = find_by_id(ws.get_widgets(), new_id) {
            // tr_debug!(
            //     "new_focused_wgt: {} id={}",
            //     new_focused_wgt.prop.to_string(),
            //     new_focused_wgt.id
            // );

            if let Property::ListBox(_) = new_focused_wgt.prop {
                let mut lbs = Default::default();
                ws.get_list_box_state(new_focused_wgt, &mut lbs);

                if lbs.item_idx < 0 && lbs.items_cnt > 0 {
                    ws.on_list_box_select(new_focused_wgt, lbs.sel_idx);
                }
            }

            if let Some(mut term_guard) = TERM.try_lock() {
                let term = &mut *term_guard;
                set_cursor_at(term, ws, new_focused_wgt);
            }

            WGT_STATE.try_lock().unwrap().focused_wgt = new_focused_wgt.id;
        }

        if is_focusable_by_id(ws, prev_id) {
            ws.invalidate(prev_id);
        }

        if is_focusable_by_id(ws, new_id) {
            ws.invalidate(new_id);
        }

        return true;
    }

    false
}

// ---------------------------------------------------------------------------------------------- //

fn pagectrl_change_page(ws: &mut dyn WindowState, pgctrl: &Widget, next: bool) {
    // assert(wgt->type == Widget::PageCtrl);

    let pgidx = {
        let mut idx = ws.get_page_ctrl_page_index(pgctrl);
        idx += tetrary!(next, 1, -1);
        if idx < 0 {
            idx = pgctrl.link.children_cnt as i16 - 1;
        }
        if idx >= pgctrl.link.children_cnt as i16 {
            idx = 0;
        }
        idx
    };

    ws.on_page_control_page_change(pgctrl, pgidx);
    ws.invalidate(pgctrl.id);

    // cancel EDIT mode
    WGT_STATE.try_lock().unwrap().text_edit_state.wgt_id = WIDGET_ID_NONE;

    if let Some(focused) = find_by_id(ws.get_widgets(), ws.get_focused_id()) {
        // tr_debug!("focused id={} ({})", focused.id, focused.prop);
        WGT_STATE.try_lock().unwrap().focused_wgt = focused.id;

        if let Some(mut term_guard) = TERM.try_lock() {
            wgt::set_cursor_at(&mut term_guard, ws, focused);
        }
        else {
            tr_debug!("Unable to lock the term");
        }
    }
    else {
        WGT_STATE.try_lock().unwrap().focused_wgt = WIDGET_ID_NONE;
        if let Some(mut term_guard) = TERM.try_lock() {
            term_guard.move_to_home();
        }
    }
}

fn find_main_pg_control(ws: &mut dyn WindowState) -> Option<&'static Widget> {
    let wnd = ws.get_widgets().first().unwrap();

    for child in wnd.iter_children() {
        if let Property::PageCtrl(_) = child.prop {
            return Some(child);
        }
    }

    None
}

fn hide_combo_box_dropdown_list(ws: &mut dyn WindowState, wgt: &Widget) {
    debug_assert!(matches!(wgt.prop, Property::ComboBox(_)));

    ws.on_combo_box_drop(wgt, false);
    // redraw parent to hide list
    let parent = get_parent(wgt);
    ws.invalidate(parent.id);
    WGT_STATE.try_lock().unwrap().cbx_drop_down = WIDGET_ID_NONE;
}

fn invalidate_radio_group(ws: &mut dyn WindowState, wgt: &Widget) {
    if let Property::Radio(ref prop) = wgt.prop {
        let group_id = prop.group_id;
        let wgt_parent = get_parent(wgt);

        for child in wgt_parent.iter_children() {
            if let Property::Radio(ref prop) = child.prop {
                if prop.group_id == group_id {
                    ws.invalidate(child.id)
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGETS KEYBOARD PROCESSING FUNCTIONS --------------------------------------------------- //
// ---------------------------------------------------------------------------------------------- //

fn process_key(ws: &mut dyn WindowState, ii: &InputInfo) -> bool {
    let focused_id = ws.get_focused_id();
    let wgt = find_by_id(ws.get_widgets(), focused_id);
    let mut key_handled = false;

    if let Some(wgt) = wgt {
        if !is_enabled(ws, wgt) {
            return true;
        }

        key_handled = match wgt.prop {
            Property::TextEdit(_) => process_key_text_edit(ws, wgt, ii),
            Property::CheckBox(_) => process_key_check_box(ws, wgt, ii),
            Property::Radio(_) => process_key_radio(ws, wgt, ii),
            Property::Button(_) => process_key_button(ws, wgt, ii),
            Property::PageCtrl(_) => process_key_page_ctrl(ws, wgt, ii),
            Property::ListBox(_) => process_key_list_box(ws, wgt, ii),
            Property::ComboBox(_) => process_key_combo_box(ws, wgt, ii),
            Property::TextBox(_) => process_key_text_box(ws, wgt, ii),
            _ => false,
        };
    }

    key_handled
}

fn process_key_text_edit(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    let mut te_state;

    let handled = {
        let mut wgtstate_guard = WGT_STATE.try_lock().unwrap();
        let testate = &mut wgtstate_guard.text_edit_state;

        if wgt.id == testate.wgt_id {
            // if in edit state, allow user to handle key
            if ws.on_text_edit_input_evt(wgt, ii, &mut testate.txt, &mut testate.cursor_pos) {
                ws.invalidate(wgt.id);
                return true;
            }
            // user let us continue checking the key
        }

        // TODO: improve to avoid cloning
        te_state = testate.clone();
        false
    };

    if handled {
        return true;
    }

    let mut key_handled = false;
    let is_psw_masked = {
        if let Property::TextEdit(ref prop) = wgt.prop {
            prop.psw_mask
        }
        else {
            false
        }
    };

    if te_state.wgt_id != WIDGET_ID_NONE {
        let mut cursor_pos = te_state.cursor_pos as isize;

        if let InputEvent::Key(ref key) = ii.evnt {
            match *key {
                Key::Esc => {
                    // cancel editing
                    te_state.wgt_id = WIDGET_ID_NONE;
                    ws.invalidate(wgt.id);
                    key_handled = true;
                }
                Key::Tab => {
                    if !is_psw_masked {
                        // real TAB may have different widths and require extra processing
                        te_state.txt.insert_str(cursor_pos.max(0) as usize, "    ");
                        cursor_pos += 4;
                        ws.invalidate(wgt.id);
                    }
                    key_handled = true;
                }
                Key::Enter => {
                    // finish editing
                    ws.on_text_edit_change(wgt, &mut te_state.txt);
                    te_state.wgt_id = WIDGET_ID_NONE;
                    ws.invalidate(wgt.id);
                    key_handled = true;
                }
                Key::Backspace => {
                    if cursor_pos > 0 {
                        if ii.kmod.has_ctrl() {
                            te_state.txt.erase_char_range(0, cursor_pos as usize);
                            cursor_pos = 0;
                        }
                        else {
                            te_state
                                .txt
                                .erase_char_range((cursor_pos - 1).max(0) as usize, 1);
                            cursor_pos -= 1;
                        }
                        ws.invalidate(wgt.id);
                    }
                    key_handled = true;
                }
                Key::Delete => {
                    if !is_psw_masked {
                        if ii.kmod.has_ctrl() {
                            te_state.txt.trim_at_char_idx(cursor_pos as usize);
                        }
                        else {
                            te_state.txt.erase_char_range(cursor_pos as usize, 1);
                        }

                        ws.invalidate(wgt.id);
                    }
                    key_handled = true;
                }
                Key::Up | Key::Down => {
                    //
                }
                Key::Left => {
                    if !is_psw_masked && cursor_pos > 0 {
                        cursor_pos -= 1;
                        ws.invalidate(wgt.id);
                    }
                    key_handled = true;
                }
                Key::Right => {
                    if !is_psw_masked && (cursor_pos < te_state.txt.chars().count() as isize) {
                        cursor_pos += 1;
                        ws.invalidate(wgt.id);
                    }
                    key_handled = true;
                }
                Key::Home => {
                    if !is_psw_masked {
                        cursor_pos = 0;
                        ws.invalidate(wgt.id);
                    }
                    key_handled = true;
                }
                Key::End => {
                    if !is_psw_masked {
                        cursor_pos = te_state.txt.chars().count() as isize;
                        ws.invalidate(wgt.id);
                    }
                    key_handled = true;
                }
                _ => {}
            }
        }
        else if let InputEvent::Char(ref cb) = ii.evnt {
            te_state
                .txt
                .insert_str_at_char_idx(cursor_pos as usize, cb.as_str());
            cursor_pos += 1;
            ws.invalidate(wgt.id);
            key_handled = true;
        }

        te_state.cursor_pos = cursor_pos as i16;
    }
    else if let InputEvent::Key(ref key) = ii.evnt {
        if *key == Key::Enter {
            // enter edit mode
            te_state.wgt_id = wgt.id;
            te_state.txt.clear();
            ws.get_text_edit_text(wgt, &mut te_state.txt, true);
            te_state.cursor_pos = te_state.txt.chars().count() as i16;
            ws.invalidate(wgt.id);
            key_handled = true;
        }
    }

    WGT_STATE.try_lock().unwrap().text_edit_state = te_state.clone();
    key_handled
}

fn process_key_check_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    if let InputEvent::Char(ref cb) = ii.evnt {
        if ii.kmod.is_empty() && cb.utf8seq[0] == b' ' {
            ws.on_checkbox_toggle(wgt);
            ws.invalidate(wgt.id);
            return true;
        }
    }

    if let InputEvent::Key(ref key) = ii.evnt {
        if *key == Key::Enter {
            ws.on_checkbox_toggle(wgt);
            ws.invalidate(wgt.id);
            return true;
        }
    }

    false
}

fn process_key_radio(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    if let InputEvent::Char(ref cb) = ii.evnt {
        if ii.kmod.is_empty() && cb.utf8seq[0] == b' ' {
            ws.on_radio_select(wgt);
            invalidate_radio_group(ws, wgt);
            return true;
        }
    }

    if let InputEvent::Key(ref key) = ii.evnt {
        if *key == Key::Enter {
            ws.on_radio_select(wgt);
            invalidate_radio_group(ws, wgt);
            return true;
        }
    }

    false
}

fn process_key_button(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    if ws.on_button_key(wgt, ii) {
        // user handled the keyboard event
        return true;
    }

    if let InputEvent::Key(ref key) = ii.evnt {
        if *key == Key::Enter {
            // pointer may change between onButtonUp and onButtonClick, so remember it
            WGT_STATE.try_lock().unwrap().mouse_down_wgt = wgt.id;
            ws.on_button_down(wgt, ii);
            ws.instant_redraw(wgt.id);
            if let Some(term_guard) = TERM.try_lock() {
                term_guard.pal.sleep(200);
            }
            WGT_STATE.try_lock().unwrap().mouse_down_wgt = WIDGET_ID_NONE;
            ws.on_button_up(wgt, ii);
            ws.on_button_click(wgt, ii);
            ws.invalidate(wgt.id);
            return true;
        }
    }

    false
}

fn process_key_page_ctrl(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    if let InputEvent::Key(ref key) = ii.evnt {
        match *key {
            Key::PgDown | Key::PgUp | Key::F11 | Key::F12 => {
                pagectrl_change_page(ws, wgt, *key == Key::PgDown || *key == Key::F12);
                return true;
            }
            _ => {}
        }
    }

    false
}

fn process_key_list_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    let mut delta = 0;

    if let InputEvent::Key(ref key) = ii.evnt {
        let items_visible = wgt.size.height as i16 - 2;

        match *key {
            Key::Enter => {
                let mut lbs = Default::default();
                ws.get_list_box_state(wgt, &mut lbs);

                if lbs.items_cnt > 0 {
                    if lbs.sel_idx >= 0 && lbs.sel_idx != lbs.item_idx {
                        ws.on_list_box_change(wgt, lbs.sel_idx);
                    }
                    ws.invalidate(wgt.id);
                }

                return true;
            }
            Key::Up => {
                delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, -1, 0);
            }
            Key::Down => {
                delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, 1, 0);
            }
            Key::PgUp => {
                delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, -items_visible, 0);
            }
            Key::PgDown => {
                delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, items_visible, 0);
            }
            _ => {}
        }
    }

    if delta != 0 {
        let mut lbs = Default::default();
        ws.get_list_box_state(wgt, &mut lbs);

        if lbs.items_cnt > 0 {
            lbs.sel_idx += delta;

            if lbs.sel_idx < 0 {
                lbs.sel_idx = lbs.items_cnt - 1;
            }

            if lbs.sel_idx >= lbs.items_cnt {
                lbs.sel_idx = 0;
            }

            ws.on_list_box_select(wgt, lbs.sel_idx);
            ws.invalidate(wgt.id);
        }

        return true;
    }

    false
}

fn process_key_combo_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    let mut cbs = Default::default();
    ws.get_combo_box_state(wgt, &mut cbs);
    let mut input_handled = false;

    if let InputEvent::Char(ref cb) = ii.evnt {
        if cb.utf8seq[0] == b' ' && cbs.items_cnt > 0 {
            cbs.drop_down = !cbs.drop_down;

            if cbs.drop_down {
                ws.on_combo_box_drop(wgt, true);
                WGT_STATE.try_lock().unwrap().cbx_drop_down = wgt.id;
            }
            else {
                hide_combo_box_dropdown_list(ws, wgt);
            }

            input_handled = true;
        }
    }
    else if let InputEvent::Key(ref key) = ii.evnt {
        if *key == Key::Esc {
            hide_combo_box_dropdown_list(ws, wgt);
            input_handled = true;
        }
        else if cbs.drop_down {
            input_handled = true;

            if *key == Key::Up {
                cbs.sel_idx -= 1;
                if cbs.sel_idx < 0 {
                    cbs.sel_idx = cbs.items_cnt - 1;
                }
                ws.on_combo_box_select(wgt, cbs.sel_idx);
            }
            else if *key == Key::Down {
                cbs.sel_idx += 1;
                if cbs.sel_idx >= cbs.items_cnt {
                    cbs.sel_idx = 0;
                }
                ws.on_combo_box_select(wgt, cbs.sel_idx);
            }
            else if *key == Key::PgUp && ii.kmod.mask == KEY_MOD_SPECIAL {
                if let Property::ComboBox(ref prop) = wgt.prop {
                    cbs.sel_idx -= prop.drop_down_size as i16;
                }

                if cbs.sel_idx < 0 {
                    cbs.sel_idx = cbs.items_cnt - 1;
                }
                ws.on_combo_box_select(wgt, cbs.sel_idx);
            }
            else if *key == Key::PgDown && ii.kmod.mask == KEY_MOD_SPECIAL {
                if let Property::ComboBox(ref prop) = wgt.prop {
                    cbs.sel_idx += prop.drop_down_size as i16;
                }

                if cbs.sel_idx >= cbs.items_cnt {
                    cbs.sel_idx = 0;
                }
                ws.on_combo_box_select(wgt, cbs.sel_idx);
            }
            else if *key == Key::Enter {
                ws.on_combo_box_change(wgt, cbs.sel_idx);
                hide_combo_box_dropdown_list(ws, wgt);
            }
            else {
                input_handled = false;
            }
        }
    }

    if input_handled {
        ws.invalidate(wgt.id);
    }

    input_handled
}

fn process_key_text_box(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) -> bool {
    if let InputEvent::Key(ref key) = ii.evnt {
        let mut delta = 0;
        let lines_visible = wgt.size.height as i16 - 2;

        match *key {
            Key::Up => delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, -1, 0),
            Key::Down => delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, 1, 0),
            Key::PgUp => delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, -lines_visible, 0),
            Key::PgDown => delta = tetrary!(ii.kmod.mask == KEY_MOD_SPECIAL, lines_visible, 0),
            _ => {}
        }

        if delta != 0 {
            let mut tbs = Default::default();
            ws.get_text_box_state(wgt, &mut tbs);

            let lines_len = tbs.lines.borrow().len() as i16;
            tbs.top_line += delta;

            if tbs.top_line > lines_len - lines_visible {
                tbs.top_line = lines_len - lines_visible;
            }

            if tbs.top_line < 0 {
                tbs.top_line = 0;
            }

            ws.on_text_box_scroll(wgt, tbs.top_line);
            ws.invalidate(wgt.id);
            return true;
        }
    }

    false
}

// ---------------------------------------------------------------------------------------------- //
// ---- WIDGETS MOUSE PROCESSING FUNCTIONS ------------------------------------------------------ //
// ---------------------------------------------------------------------------------------------- //

fn process_mouse(ws: &mut dyn WindowState, ii: &InputInfo) -> bool {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        if mouse.evt == MouseEvent::ButtonGoBack || mouse.evt == MouseEvent::ButtonGoForward {
            if let Some(main_pg_ctrl) = find_main_pg_control(ws) {
                if is_enabled(ws, main_pg_ctrl) {
                    pagectrl_change_page(
                        ws,
                        main_pg_ctrl,
                        mouse.evt == MouseEvent::ButtonGoForward,
                    );
                }
                return true;
            }
        }

        let mut rct = Rect::cdeflt();

        if let Some(mut wgt) = find_at(ws, mouse.col, mouse.row, &mut rct) {
            let ret = {
                let mouse_down_wgt = WGT_STATE.try_lock().unwrap().mouse_down_wgt;

                if let Some(md_wgt) = find_by_id(ws.get_widgets(), mouse_down_wgt) {
                    // apply only for Button widget
                    if let Property::Button(_) = md_wgt.prop {
                        // mouse button released over another widget - generate Up event for previously clicked button
                        if mouse.evt == MouseEvent::ButtonReleased && md_wgt.id != wgt.id {
                            process_mouse_button_release(ws, md_wgt, ii);
                            return true;
                        }
                    }
                }
                else {
                    // remember clicked widget
                    if mouse.evt >= MouseEvent::ButtonLeft && mouse.evt < MouseEvent::ButtonReleased
                    {
                        let mut wgtstate_guard = WGT_STATE.try_lock().unwrap();
                        wgtstate_guard.mouse_down_wgt = wgt.id;
                        wgtstate_guard.mouse_down_ii = (*ii).clone();
                    }
                }

                false
            };

            // tr_debug!("WidgetAt({:2}:{:2})={} ID:{}", ii.mouse.col, ii.mouse.row, wgt.prop.to_string(), wgt.id);
            let cbx_drop_down = WGT_STATE.try_lock().unwrap().cbx_drop_down;

            // check if drop-down list clicked
            if let Some(cbx) = find_by_id(ws.get_widgets(), cbx_drop_down) {
                if let Property::ComboBox(ref prop) = cbx.prop {
                    let mut dropdownlist_rct = Rect::cdeflt();
                    // rect includes the cbx itself
                    dropdownlist_rct.coord = get_screen_coord(ws, cbx);
                    dropdownlist_rct.size.width = cbx.size.width;
                    dropdownlist_rct.size.height = prop.drop_down_size + 1;

                    if dropdownlist_rct.is_point_within(mouse.col, mouse.row) {
                        // yes -> replace data for processing with g_ds.pDropDownCombo
                        wgt = cbx;
                        rct.coord = get_screen_coord(ws, wgt);
                        rct.size = wgt.size;
                    }
                    else if mouse.evt == MouseEvent::ButtonLeft {
                        hide_combo_box_dropdown_list(ws, cbx);
                    }
                }
            }

            if is_enabled(ws, wgt) {
                match wgt.prop {
                    Property::TextEdit(_) => process_mouse_text_edit(ws, wgt, &rct, ii),
                    Property::CheckBox(_) => process_mouse_check_box(ws, wgt, &rct, ii),
                    Property::Radio(_) => process_mouse_radio(ws, wgt, &rct, ii),
                    Property::Button(_) => process_mouse_button(ws, wgt, &rct, ii),
                    Property::PageCtrl(_) => process_mouse_page_ctrl(ws, wgt, &rct, ii),
                    Property::ListBox(_) => process_mouse_list_box(ws, wgt, &rct, ii),
                    Property::ComboBox(_) => process_mouse_combo_box(ws, wgt, &rct, ii),
                    Property::CustomWgt(_) => process_mouse_custom_wgt(ws, wgt, &rct, ii),
                    Property::TextBox(_) => process_mouse_text_box(ws, wgt, &rct, ii),
                    _ => {
                        if let Some(mut term_guard) = TERM.try_lock() {
                            let term = &mut *term_guard;
                            term.move_to_home();
                        }

                        WGT_STATE.try_lock().unwrap().mouse_down_wgt = WIDGET_ID_NONE;
                        return false;
                    }
                }
            }

            if mouse.evt == MouseEvent::ButtonReleased {
                WGT_STATE.try_lock().unwrap().mouse_down_wgt = WIDGET_ID_NONE;
            }
        }
    }

    true
}

fn process_mouse_text_edit(
    ws: &mut dyn WindowState,
    wgt: &Widget,
    wgt_rect: &Rect,
    ii: &InputInfo,
) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        if mouse.evt == MouseEvent::ButtonLeft {
            change_focus_to(ws, wgt.id);
        }
    }
}

fn process_mouse_check_box(
    ws: &mut dyn WindowState,
    wgt: &Widget,
    wgt_rect: &Rect,
    ii: &InputInfo,
) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        if mouse.evt == MouseEvent::ButtonLeft {
            change_focus_to(ws, wgt.id);
            ws.on_checkbox_toggle(wgt);
            ws.invalidate(wgt.id);
        }
    }
}

fn process_mouse_radio(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        if mouse.evt == MouseEvent::ButtonLeft {
            change_focus_to(ws, wgt.id);
            ws.on_radio_select(wgt);
            invalidate_radio_group(ws, wgt);
        }
    }
}

fn process_mouse_button(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        let mouse_down_wgt = {
            let wgtstate_guard = WGT_STATE.try_lock().unwrap();
            wgtstate_guard.mouse_down_wgt
        };

        // pointer may change between onButtonUp and onButtonClick, so remember it
        if mouse.evt == MouseEvent::ButtonLeft {
            change_focus_to(ws, wgt.id);
            ws.on_button_down(wgt, ii);
            ws.invalidate(wgt.id);
        }
        else if mouse.evt == MouseEvent::ButtonReleased && mouse_down_wgt == wgt.id {
            ws.on_button_up(wgt, ii);
            {
                let mut guard = WGT_STATE.try_lock().unwrap();
                ws.on_button_click(wgt, &guard.mouse_down_ii);
                guard.mouse_down_wgt = WIDGET_ID_NONE;
            }
            ws.invalidate(wgt.id);
        }
        else {
            let mut guard = WGT_STATE.try_lock().unwrap();
            guard.mouse_down_wgt = WIDGET_ID_NONE;
        }
    }
}

fn process_mouse_button_release(ws: &mut dyn WindowState, wgt: &Widget, ii: &InputInfo) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        ws.on_button_up(wgt, ii);
        WGT_STATE.try_lock().unwrap().mouse_down_wgt = WIDGET_ID_NONE;
        ws.invalidate(wgt.id);
    }
}

fn process_mouse_page_ctrl(
    ws: &mut dyn WindowState,
    wgt: &Widget,
    wgt_rect: &Rect,
    ii: &InputInfo,
) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        if mouse.evt == MouseEvent::ButtonLeft {
            change_focus_to(ws, wgt.id);
            let idx = ws.get_page_ctrl_page_index(wgt);
            let vertoffs = if let Property::PageCtrl(ref prop) = wgt.prop {
                prop.vert_offs as i16
            }
            else {
                0
            };

            let new_idx = mouse.row as i16 - wgt_rect.coord.row as i16 - 1 - vertoffs;

            if new_idx != idx && new_idx >= 0 && new_idx < wgt.link.children_cnt as i16 {
                ws.on_page_control_page_change(wgt, new_idx);
                ws.invalidate(wgt.id);
            }
        }
        else if mouse.evt == MouseEvent::WheelUp || mouse.evt == MouseEvent::WheelDown {
            pagectrl_change_page(ws, wgt, mouse.evt == MouseEvent::WheelDown);
        }
    }
}

fn process_mouse_list_box(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        let items_visible = wgt.size.height as i16 - 2;

        if mouse.evt == MouseEvent::ButtonLeft || mouse.evt == MouseEvent::ButtonMid {
            let focus_changed = change_focus_to(ws, wgt.id);
            let mut lbs = Default::default();
            ws.get_list_box_state(wgt, &mut lbs);

            if lbs.items_cnt <= 0 || items_visible == 0 {
                return;
            }

            let page = lbs.sel_idx / items_visible;
            let mut new_selidx = page * items_visible;
            new_selidx += mouse.row as i16 - wgt_rect.coord.row as i16 - 1;

            if mouse.evt == MouseEvent::ButtonLeft {
                if new_selidx < lbs.items_cnt && ((new_selidx != lbs.sel_idx) || focus_changed) {
                    lbs.sel_idx = new_selidx;
                    ws.on_list_box_select(wgt, lbs.sel_idx);
                }
            }
            else if new_selidx < lbs.items_cnt && new_selidx != lbs.item_idx {
                lbs.sel_idx = new_selidx;
                ws.on_list_box_select(wgt, lbs.sel_idx);
                ws.on_list_box_change(wgt, lbs.sel_idx);
            }

            ws.invalidate(wgt.id);
        }
        else if mouse.evt == MouseEvent::WheelUp || mouse.evt == MouseEvent::WheelDown {
            change_focus_to(ws, wgt.id);
            let mut lbs = Default::default();
            ws.get_list_box_state(wgt, &mut lbs);

            if lbs.items_cnt <= 0 {
                return;
            }

            let mut delta = tetrary!(mouse.evt == MouseEvent::WheelUp, -1, 1);
            if ii.kmod.has_ctrl() {
                delta *= items_visible;
            }
            lbs.sel_idx += delta;

            if lbs.sel_idx < 0 {
                lbs.sel_idx = lbs.items_cnt - 1;
            }

            if lbs.sel_idx >= lbs.items_cnt {
                lbs.sel_idx = 0;
            }

            ws.on_list_box_select(wgt, lbs.sel_idx);
            ws.invalidate(wgt.id);
        }
    }
}

fn process_mouse_combo_box(
    ws: &mut dyn WindowState,
    wgt: &Widget,
    wgt_rect: &Rect,
    ii: &InputInfo,
) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        let drop_down_size = if let Property::ComboBox(ref prop) = wgt.prop {
            prop.drop_down_size as i16
        }
        else {
            1
        };

        if mouse.evt == MouseEvent::ButtonLeft {
            change_focus_to(ws, wgt.id);
            let col = mouse.col as i16 - wgt_rect.coord.col as i16;
            let row = mouse.row as i16 - wgt_rect.coord.row as i16 - 1;

            if row >= 0 && row < drop_down_size {
                let mut cbs = Default::default();
                ws.get_combo_box_state(wgt, &mut cbs);

                cbs.sel_idx = (cbs.sel_idx / drop_down_size) * drop_down_size; // top item
                cbs.sel_idx += row;
                if cbs.sel_idx < cbs.items_cnt {
                    ws.on_combo_box_select(wgt, cbs.sel_idx);
                    ws.invalidate(wgt.id);
                }
            }
            else if col >= wgt_rect.size.width as i16 - 3 && col < wgt_rect.size.width as i16 {
                let mut cbs = Default::default();
                ws.get_combo_box_state(wgt, &mut cbs);

                // drop down arrow clicked
                if cbs.items_cnt <= 0 {
                    return;
                }

                cbs.drop_down = !cbs.drop_down;

                if cbs.drop_down {
                    ws.on_combo_box_drop(wgt, true);
                    ws.invalidate(wgt.id);
                    WGT_STATE.try_lock().unwrap().cbx_drop_down = wgt.id;
                }
                else {
                    hide_combo_box_dropdown_list(ws, wgt);
                }
            }
        }
        else if mouse.evt == MouseEvent::WheelUp || mouse.evt == MouseEvent::WheelDown {
            change_focus_to(ws, wgt.id);
            let mut cbs = Default::default();
            ws.get_combo_box_state(wgt, &mut cbs);

            if !cbs.drop_down || cbs.items_cnt <= 0 {
                return;
            }

            let mut delta = tetrary!(mouse.evt == MouseEvent::WheelUp, -1, 1);

            if ii.kmod.has_ctrl() {
                delta *= drop_down_size;
            }

            cbs.sel_idx += delta;

            if cbs.sel_idx < 0 {
                cbs.sel_idx = cbs.items_cnt - 1;
            }

            if cbs.sel_idx >= cbs.items_cnt {
                cbs.sel_idx = 0;
            }

            ws.on_combo_box_select(wgt, cbs.sel_idx);
            ws.invalidate(wgt.id);
        }
        else if mouse.evt == MouseEvent::ButtonMid {
            let evnt_btn_left = InputInfo {
                evnt: InputEvent::Mouse(MouseInfo {
                    evt: MouseEvent::ButtonLeft,
                    col: mouse.col,
                    row: mouse.row,
                }),
                kmod: ii.kmod,
                name: "",
            };
            process_mouse_combo_box(ws, wgt, wgt_rect, &evnt_btn_left);

            let mut cbs = Default::default();
            ws.get_combo_box_state(wgt, &mut cbs);

            if !cbs.drop_down {
                return;
            }

            ws.on_combo_box_change(wgt, cbs.sel_idx);
            hide_combo_box_dropdown_list(ws, wgt);
        }
    }
}

fn process_mouse_custom_wgt(
    ws: &mut dyn WindowState,
    wgt: &Widget,
    wgt_rect: &Rect,
    ii: &InputInfo,
) {
    ws.on_custom_widget_input_evt(wgt, ii);
}

fn process_mouse_text_box(ws: &mut dyn WindowState, wgt: &Widget, wgt_rect: &Rect, ii: &InputInfo) {
    if let InputEvent::Mouse(ref mouse) = ii.evnt {
        change_focus_to(ws, wgt.id);

        if mouse.evt == MouseEvent::WheelUp || mouse.evt == MouseEvent::WheelDown {
            let mut tbs = Default::default();
            ws.get_text_box_state(wgt, &mut tbs);

            let lines = tbs.lines.borrow();

            if !lines.is_empty() {
                let mut delta = tetrary!(mouse.evt == MouseEvent::WheelUp, -1, 1);
                let lines_visible = wgt.size.height as i16 - 2;
                if ii.kmod.has_ctrl() {
                    delta *= lines_visible;
                }
                tbs.top_line += delta;

                if tbs.top_line > lines.len() as i16 - lines_visible {
                    tbs.top_line = lines.len() as i16 - lines_visible;
                }

                if tbs.top_line < 0 {
                    tbs.top_line = 0;
                }

                change_focus_to(ws, wgt.id);
                ws.on_text_box_scroll(wgt, tbs.top_line);
                ws.invalidate(wgt.id);
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------- //
