//! # RTWins Widget

use super::widget::{Widget, Type};

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

pub const fn wgt_is_parent(wgt: &Widget) -> bool {
    match wgt.typ {
        Type::Window(_) |
        Type::Panel(_)  |
        Type::Page(_)   => true,
        _               => false
    }
}

pub fn wgt_get_parent<'a>(wnd: &'a [Widget], wgt: &Widget) -> &'a Widget {
    &wnd[wgt.link.parent_idx as usize]
}

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
