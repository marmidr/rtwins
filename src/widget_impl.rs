//! # RTWins Widget

use crate::string_ext::StrExt;
use crate::widget_def::*;
use crate::common::*;
use crate::input;
use crate::*;

// ---------------------------------------------------------------------------------------------- //

/// State object for current top window.
// using WId instead of references will solve lifetime problems
#[allow(dead_code)]
#[derive(Default)]
struct WidgetState {
    focused_wgt:     WId,
    mouse_down_wgt:  WId,
    drop_down_combo: WId,
    text_edit_state: TextEditState,
    mouse_down_key_code: input::Key,
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
pub fn get_parent<'a>(wgt: &'a Widget) -> &'a Widget {
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
pub fn find_by_id<'a>(wndarray: &'a [Widget], id: WId) -> Option<&'a Widget> {
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
    wgt.iter_parents().skip(1).fold(wgt.coord,
        |c, parent| {
            let mut c = c;
            if let Property::Window(_) = parent.prop {
                // TODO: for popups must be centered on parent window
                c = c + parent.coord;
            }
            else {
                c = c + parent.coord;
            }

            if let Property::PageCtrl(ref p) = parent.prop {
                c.col += p.tab_width;
            }

            c
        }
    )
}

/// Move cursor to the best position for given type of the widget
pub fn set_cursor_at(ctx: &mut Ctx, ws: &mut dyn WindowState, wgt: &Widget) {
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
            coord.row += ws.get_page_ctrl_page_index(wgt)
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

    ctx.move_to(coord.col as u16, coord.row as u16);
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
pub fn pagectrl_find_page<'a>(pgctrl: &'a Widget, page_id: WId) -> Option<&'a Widget> {
    if let Property::PageCtrl(_) = pgctrl.prop {
        return pgctrl.iter_children().find(|pg| pg.id == page_id);
    }

    None
}

pub fn pagectrl_select_page(ws: &mut dyn WindowState, pgctrl_id: WId, page_id: WId) {
    if let Some(pgctrl) = find_by_id(ws.get_widgets(), pgctrl_id) {
        if let Some(page) = pagectrl_find_page(pgctrl, page_id) {
            if let Some(pg_idx) = page_page_idx(&page) {
                ws.on_page_control_page_change(pgctrl, pg_idx);
                ws.invalidate(&[pgctrl_id]);
                return;
            }
        }
    }

    // TWINS_LOG_W("Widget Id=%d is not PageControl Id=%d page", pageID, pageControlID);
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
        // TWINS_LOG_D("focused id=%d (%s)", p_wgt->id, toString(p_wgt->type));
        WGT_STATE.with(|wgstate|
            wgstate.borrow_mut().focused_wgt = focused.id
        );
        // setCursorAt(ctx, p_wgt); TODO:
    }
    else {
        WGT_STATE.with(|wgstate|
            wgstate.borrow_mut().focused_wgt = WIDGET_ID_NONE
        );
        // moveToHome(); TODO
    }
}
