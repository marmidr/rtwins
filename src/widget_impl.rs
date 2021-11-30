//! # RTWins Widget

use crate::input;
use crate::{WId, Coord, WIDGET_ID_NONE};
use crate::widget::{Widget, Type};

pub struct WidgetSearchStruct
{
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

/** Widget drawing state object */
struct WidgetState
{
    buff: String,                        // common string buff for widget drawers
    // p_focused_wgt: Option<&Widget>,
    // p_mouse_down_wgt: Option<&Widget>,
    // p_drop_down_combo: Option<&Widget>,
    mouse_down_key_code: input::KeyCode,
    // struct                              // state of Edit being modified
    // {
    //     const Widget *pWgt = nullptr;
    //     int16_t cursorPos = 0;
    //     String  str;
    // } textEditState;
}

// -----------------------------------------------------------------------------------------------

/// Counts total number of widgets in tree-like definition
pub const fn wgt_count(wgt: &Widget) -> usize {
    let mut n: usize = 1;
    let mut i: usize = 0;
    while i < wgt.childs.len() {
        n += wgt_count(&wgt.childs[i]);
        i += 1;
    }
    n
}

/// Checks if given widget is parent-type
pub const fn wgt_is_parent(wgt: &Widget) -> bool {
    match wgt.typ {
        Type::Window(_) |
        Type::Panel(_)  |
        Type::Page(_)   => true,
        _               => false
    }
}

/// Flattens tree-like TUI definition into array of widgets
pub const fn wgt_transform_array<const N: usize>(wgt: &Widget) -> [Widget; N] {
    let out: [Widget; N] = [Widget::cdeflt(); N];
    let (_, out) = wgt_transform(out, wgt, 0, 1);
    out
}

const fn wgt_transform<const N: usize>(mut out: [Widget; N], wgt: &Widget, out_idx: usize, mut next_free_idx: usize) -> (usize, [Widget; N]) {
    out[out_idx] = *wgt;
    out[out_idx].link.own_idx = out_idx as u16;
    out[out_idx].childs = &[];

    let mut out_child_idx = next_free_idx;

    if wgt.childs.len() > 0 {
        out[out_idx].link.childs_idx = out_child_idx as u16;
        out[out_idx].link.childs_cnt = wgt.childs.len() as u16;
        next_free_idx += wgt.childs.len();
    }

    let mut ch_idx = 0;
    while ch_idx < wgt.childs.len() {
        let (nfidx, o) = wgt_transform(out, &wgt.childs[ch_idx], out_child_idx, next_free_idx);
        out = o;
        out[out_child_idx].link.parent_idx = out_idx as u16;
        next_free_idx = nfidx;

        ch_idx += 1;
        out_child_idx += 1;
    }

    (next_free_idx, out)
}

///
pub fn wgt_get_wss(/* CallCtx &ctx,*/ wss: &mut WidgetSearchStruct) -> bool
{
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

/// Get `wgt` parent in `wnd` array; for Window, return window itself
pub fn wgt_get_parent<'a>(wnd: &'a [Widget], wgt: &Widget) -> &'a Widget {
    &wnd[wgt.link.parent_idx as usize]
}
