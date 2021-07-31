//! # RTWins Widget

use super::widget::Widget;

/// Counts total number of widgets in tree-like definition
pub const fn wgt_count(wgt: &Widget) -> usize {
    let mut n: usize = 1;
    let mut i: usize = 0;
    while i < wgt.link.len() {
        n += wgt_count(&wgt.link[i]);
        i += 1;
    }
    n
}

// TODO: not finished
pub const fn wgt_translate<const N: usize>(wgt: &Widget) -> [Widget; N] {
    let mut result: [Widget; N] = [Widget::cdeflt(); N];
    let mut i: usize = 0;
    while i < wgt.link.len() {
        result[i] = *wgt;
        i += 1;
    }
    result
}
