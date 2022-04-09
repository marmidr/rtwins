//! # RTWins Widget

use crate::string_ext::StrExt;
use crate::widget_def::*;
use crate::common::*;

// ---------------------------------------------------------------------------------------------- //

/// Widget drawing state object
#[allow(dead_code)]
struct WidgetState {
    // p_focused_wgt: Option<&Widget>,
    // p_mouse_down_wgt: Option<&Widget>,
    // p_drop_down_combo: Option<&Widget>,
    // mouse_down_key_code: input::KeyCode,
    // struct                              // state of Edit being modified
    // {
    //     const Widget *pWgt = nullptr;
    //     int16_t cursorPos = 0;
    //     String  str;
    // } textEditState;
}

#[allow(dead_code)]
pub struct WidgetSearchStruct {
    searched_id: WId,       // given
    parent_coord: Coord,    // expected
    is_visible: bool,       // expected
    // p_widget: &Widget    // expected
}

impl WidgetSearchStruct {
    pub fn new(searched_id: WId) -> Self {
        WidgetSearchStruct{searched_id, parent_coord: Coord::cdeflt(), is_visible: true}
    }
}

// ---------------------------------------------------------------------------------------------- //

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

/// Flattens tree-like TUI definition into array of widgets
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

/// Checks if given widget is parent-type
pub const fn is_parent(wgt: &Widget) -> bool {
    matches!(wgt.prop,
        Property::Window(_) |
        Property::Panel(_)  |
        Property::Page(_))
}

///
pub fn wgt_get_wss(/* CallCtx &ctx,*/ wss: &mut WidgetSearchStruct) -> bool {
    if wss.searched_id == WIDGET_ID_NONE {
        return false;
    }

/*
    const Widget *p_wgt = ctx.pWidgets;

    for (;; p_wgt++)
    {
        if (p_wgt->id == wss.searchedID)
            break;

        // pWndArray is terminated by empty entry
        if (p_wgt->id == WIDGET_ID_NONE)
            return false;
    }

    wss.pWidget = p_wgt;
    wss.isVisible = ctx.pState->isVisible(p_wgt);

    // go up the widgets hierarchy
    int parent_idx = p_wgt->link.parentIdx;

    for (;;)
    {
        const auto *p_parent = ctx.pWidgets + parent_idx;
        wss.isVisible &= ctx.pState->isVisible(p_parent);

        Coord coord = p_parent->coord;
        if (p_parent->type == Widget::Type::Window)
            ctx.pState->getWindowCoord(p_parent, coord);
        wss.parentCoord += coord;

        if (p_parent->type == Widget::Type::PageCtrl)
            wss.parentCoord.col += p_parent->pagectrl.tabWidth;

        if (parent_idx == 0)
            break;

        parent_idx = p_parent->link.parentIdx;
    }
     true;
 */
    false
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

/// Search for Widget with given `id` in window array
pub fn find_by_id<'a>(id: WId, wndarray: &'a [Widget]) -> Option<&'a Widget> {
    wndarray.iter().find(|&&item| item.id == id)
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

pub fn is_visible(ws: &mut dyn WindowState, wgt: &Widget) -> bool {
    wgt.iter_parents().all(|wgt| ws.is_visible(wgt))
}

pub fn is_enabled(ws: &mut dyn WindowState, wgt: &Widget) -> bool {
    wgt.iter_parents().all(|wgt| ws.is_enabled(wgt))
}

pub fn at<'a>(ws: &'a mut dyn WindowState, col: u8, row: u8, wgt_rect: &mut Rect) -> Option<&'a Widget> {
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

pub fn page_pageno(wgt: &Widget) -> Option<u8> {
    if let Property::Page(_) = wgt.prop {
        let pgctrl = get_parent(wgt);

        for (idx, page) in pgctrl.iter_children().enumerate() {
            if page.id == wgt.id {
                return Some(idx as u8);
            }
        }
    }

    None
}

// -----------------------------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------------------------
// ---- TWINS PRIVATE FUNCTIONS ------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------
