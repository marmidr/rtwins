//! # RTWins Widget drawing

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;

use crate::colors::*;
use crate::common::*;
use crate::esc;
use crate::string_ext::*;
use crate::wgt;
use crate::wgt::*;
use crate::*; // tr_info

// ---------------------------------------------------------------------------------------------- //

struct DrawCtx<'a> {
    /// Reference to a drawer instance
    term_cell: RefCell<&'a mut Term>,
    /// Reference to a widget to be drawn
    wgt: &'a Widget,
    /// Reference to window state that relates to the widget
    wnd_state: &'a mut dyn WindowState,
    /// Current widget's parent left-top position
    parent_coord: Coord,
    /// Reference to the widgets array
    wnd_widgets: &'a [Widget],
    /// Common string buffer for entire drawing session
    strbuff: String,
}

/// Draw `wids` widgets of the given window.
/// If `wids` contains only `WIDGET_ID_ALL`, draw entire window
pub fn draw_widgets(term: &mut Term, ws: &mut dyn WindowState, wids: &[WId]) {
    if wids.is_empty() {
        return;
    }

    let mut fm = FontMementoManual::from_term(term);
    let focused_id = ws.get_focused_id();
    WGT_STATE.try_write().unwrap().focused_wgt = focused_id;
    term.cursor_hide();
    term.flush_buff();

    if wids.len() == 1 && wids[0] == WIDGET_ID_ALL {
        let wnd_widgets = ws.get_widgets();
        // window is at index 0
        if let Some(wnd) = wnd_widgets.first() {
            let mut dctx = DrawCtx {
                term_cell: RefCell::new(term),
                wgt: wnd,
                wnd_state: ws,
                parent_coord: Coord::cdeflt(),
                wnd_widgets,
                strbuff: String::with_capacity(200),
            };
            draw_widget_internal(&mut dctx);
        }
    }
    else {
        let wnd_widgets = ws.get_widgets();

        for id in wids {
            if let Some(wgt) = wgt::find_by_id(wnd_widgets, *id) {
                if wgt::is_visible(ws, wgt) {
                    // set parent's background color
                    term.push_cl_bg(get_widget_bg_color(wgt));

                    {
                        let wgt_parent = wgt::get_parent(wgt);
                        let mut dctx = DrawCtx {
                            term_cell: RefCell::new(term),
                            wgt,
                            wnd_state: ws,
                            parent_coord: wgt::get_screen_coord(wgt_parent),
                            wnd_widgets,
                            strbuff: String::with_capacity(200),
                        };

                        draw_widget_internal(&mut dctx);
                    }

                    // restore background color
                    term.pop_cl_bg();
                }
            }
        }
    }

    term.reset_attr();
    term.reset_cl_bg();
    term.reset_cl_fg();

    if let Some(wgt) = wgt::find_by_id(ws.get_widgets(), focused_id) {
        wgt::set_cursor_at(term, ws, wgt)
    }
    term.cursor_show();
    fm.restore(term);
    term.flush_buff();
}

// ---------------------------------------------------------------------------------------------- //

fn draw_widget_internal(dctx: &mut DrawCtx) {
    if !wgt::is_visible(dctx.wnd_state, dctx.wgt) {
        return;
    }

    let attr_stack_len_before = dctx.term_cell.borrow().attr_stack_len();
    let en = wgt::is_enabled(dctx.wnd_state, dctx.wgt);
    if !en {
        dctx.term_cell.borrow_mut().push_attr(FontAttrib::Faint);
    }

    // tr_debug!("drawing {} id:{}", dctx.wgt.prop, dctx.wgt.id);
    dctx.strbuff.clear();

    match dctx.wgt.prop {
        Property::Window(ref p) => draw_window(dctx, p),
        Property::Panel(ref p) => draw_panel(dctx, p),
        Property::Label(ref p) => draw_label(dctx, p),
        Property::TextEdit(ref p) => draw_text_edit(dctx, p),
        Property::CheckBox(ref p) => draw_checkbox(dctx, p),
        Property::Radio(ref p) => draw_radio(dctx, p),
        Property::Button(ref p) => draw_button(dctx, p),
        Property::Led(ref p) => draw_led(dctx, p),
        Property::PageCtrl(ref p) => draw_page_control(dctx, p),
        Property::Page(ref p) => draw_page(dctx, p, true),
        Property::ProgressBar(ref p) => draw_progress_bar(dctx, p),
        Property::ListBox(ref p) => draw_list_box(dctx, p),
        Property::ComboBox(ref p) => draw_combo_box(dctx, p),
        Property::CustomWgt(ref p) => draw_custom_wgt(dctx, p),
        Property::TextBox(ref p) => draw_text_box(dctx, p),
        Property::Layer(ref p) => draw_layer(dctx, p),
        _ => {}
    }

    {
        let mut term = dctx.term_cell.borrow_mut();
        if !en {
            term.pop_attr();
        }

        // check if widget drawing procedure cleaned-up environment
        if term.attr_stack_len() != attr_stack_len_before {
            tr_warn!(
                "drawn {} id:{} - attr stack left before: {} after: {}",
                dctx.wgt.prop,
                dctx.wgt.id,
                attr_stack_len_before,
                term.attr_stack_len()
            );
            term.reset_attr();
        }
        term.flush_buff();
    }
}

// ---------------------------------------------------------------------------------------------- //

fn draw_window(dctx: &mut DrawCtx, prp: &prop::Window) {
    let wnd_coord = dctx.wnd_state.get_window_coord();
    draw_area(
        &mut dctx.term_cell.borrow_mut(),
        wnd_coord,
        dctx.wgt.size,
        prp.bg_color,
        prp.fg_color,
        FrameStyle::Double,
        true,
        prp.is_popup,
    );

    // title
    let mut wnd_title = String::new();

    if !prp.title.is_empty() {
        wnd_title = prp.title.to_string();
    }
    else {
        dctx.wnd_state.get_window_title(dctx.wgt, &mut wnd_title);
    }

    if !wnd_title.is_empty() {
        let title_width = wnd_title.as_str().displayed_width() as u16 + 4;
        let mut term = dctx.term_cell.borrow_mut();
        term.move_to(
            wnd_coord.col as u16 + (dctx.wgt.size.width as u16 - title_width) / 2,
            wnd_coord.row as u16,
        );
        term.push_attr(FontAttrib::Bold);
        term.write_str(format!("╡ {} ╞", wnd_title.as_str()).as_str());
        term.pop_attr();
    }

    dctx.term_cell.borrow_mut().flush_buff();
    dctx.parent_coord = wnd_coord;

    {
        let wnd = dctx.wgt;

        for wgt in wnd.iter_children() {
            dctx.wgt = wgt;
            draw_widget_internal(dctx);
        }

        dctx.wgt = wnd;
    }

    // reset colors set by frame drawer
    {
        let mut term = dctx.term_cell.borrow_mut();
        term.pop_cl_bg();
        term.pop_cl_fg();
        term.move_to(0, wnd_coord.row as u16 + dctx.wgt.size.height as u16);
    }
}

fn draw_panel(dctx: &mut DrawCtx, prp: &prop::Panel) {
    let mut fm = FontMementoManual::from_term(&dctx.term_cell.borrow());
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    draw_area(
        &mut dctx.term_cell.borrow_mut(),
        my_coord,
        dctx.wgt.size,
        prp.bg_color,
        prp.fg_color,
        if prp.no_frame {
            FrameStyle::None
        }
        else {
            FrameStyle::Single
        },
        true,
        false,
    );

    // title
    if !prp.title.is_empty() {
        let title_width = prp.title.displayed_width() as u16;
        let mut term = dctx.term_cell.borrow_mut();
        term.move_to(
            my_coord.col as u16 + (dctx.wgt.size.width as u16 - title_width - 2) / 2,
            my_coord.row as u16,
        );
        term.push_attr(FontAttrib::Bold);
        term.write_char(' ').write_str(prp.title).write_char(' ');
        term.pop_attr();
    }

    dctx.term_cell.borrow_mut().flush_buff();

    // draw childrens
    {
        let coord_bkp = dctx.parent_coord;
        dctx.parent_coord = my_coord;
        let pnl = dctx.wgt;

        for wgt in pnl.iter_children() {
            dctx.wgt = wgt;
            draw_widget_internal(dctx);
        }

        dctx.wgt = pnl;
        dctx.parent_coord = coord_bkp;
    }

    // dctx.term.borrow_mut().pop_cl_bg();
    fm.restore(&mut dctx.term_cell.borrow_mut());
}

fn draw_label(dctx: &mut DrawCtx, prp: &prop::Label) {
    // label text
    let mut title = String::new();
    if !prp.title.is_empty() {
        title = prp.title.into();
    }
    else {
        dctx.wnd_state.get_label_text(dctx.wgt, &mut title);
    }

    let _fm = FontMemento::new(&dctx.term_cell);
    let mut term = dctx.term_cell.borrow_mut();

    // setup colors
    term.push_cl_fg(get_widget_fg_color(dctx.wgt));
    term.push_cl_bg(get_widget_bg_color(dctx.wgt));

    // print all lines
    let col = dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16;
    let row = dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16;
    term.move_to(col, row);

    let max_lines = if dctx.wgt.size.height > 0 {
        dctx.wgt.size.height
    }
    else {
        50
    };
    let line_width = dctx.wgt.size.width;

    for (line, s) in title.lines().enumerate() {
        dctx.strbuff.clear();
        dctx.strbuff.push_str(s);

        if line_width > 0 {
            dctx.strbuff.set_displayed_width(line_width as i16);
        }

        term.write_str(dctx.strbuff.as_str());
        term.move_to(col, row + 1 + line as u16);
        term.flush_buff();
        if line as u8 == max_lines {
            break;
        }
    }
}

fn draw_text_edit(dctx: &mut DrawCtx, prp: &prop::TextEdit) {
    let mut display_pos = 0;
    let max_w = dctx.wgt.size.width as i16 - 3;

    {
        let wgtstate_guard = WGT_STATE.try_read().unwrap();
        let te_state = &wgtstate_guard.text_edit_state;

        if dctx.wgt.id == te_state.wgt_id {
            // in edit mode; similar calculation in setCursorAt()
            dctx.strbuff = te_state.txt.clone();
            let mut cursor_pos = te_state.cursor_pos;
            let delta = max_w / 2;

            while cursor_pos >= max_w - 1 {
                cursor_pos -= delta;
                display_pos += delta;
            }
        }
        else {
            dctx.strbuff.clear();
            dctx.wnd_state
                .get_text_edit_text(dctx.wgt, &mut dctx.strbuff, false);
        }
        // guard dropped
    }

    let txt_width = dctx.strbuff.displayed_width() as i16;

    if display_pos > 0 {
        let txt_to_display = dctx.strbuff.split_at_char_idx(display_pos as usize + 1);
        let mut s = String::with_capacity(txt_to_display.len() + 4);
        s.append("◁");
        s.append(txt_to_display);
        dctx.strbuff = s;
    }

    if display_pos + max_w <= txt_width {
        dctx.strbuff
            .set_displayed_width(dctx.wgt.size.width as i16 - 3 - 1);
        dctx.strbuff.append("▷");
    }
    else {
        dctx.strbuff
            .set_displayed_width(dctx.wgt.size.width as i16 - 3);
    }
    dctx.strbuff.append("[^]");

    let focused = dctx.wnd_state.is_focused(dctx.wgt);
    let clbg = get_widget_bg_color(dctx.wgt).intensify_if(focused);
    let _fm = FontMemento::new(&dctx.term_cell);
    let mut term = dctx.term_cell.borrow_mut();
    term.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
    );
    term.push_cl_bg(clbg);
    term.push_cl_fg(get_widget_fg_color(dctx.wgt));
    term.write_str(dctx.strbuff.as_str());
}

fn draw_led(dctx: &mut DrawCtx, prp: &prop::Led) {
    let clbg = if dctx.wnd_state.get_led_lit(dctx.wgt) {
        prp.bg_color_on
    }
    else {
        prp.bg_color_off
    };

    if !prp.text.is_empty() {
        dctx.strbuff.push_str(prp.text);
    }
    else {
        dctx.wnd_state.get_led_text(dctx.wgt, &mut dctx.strbuff);
    }

    // led text
    let _fm = FontMemento::new(&dctx.term_cell);

    let mut term = dctx.term_cell.borrow_mut();
    term.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
    );
    term.push_cl_bg(clbg);
    term.push_cl_fg(get_widget_fg_color(dctx.wgt));
    term.write_str(dctx.strbuff.as_str());
}

fn draw_checkbox(dctx: &mut DrawCtx, prp: &prop::CheckBox) {
    let chk_state = if dctx.wnd_state.get_checkbox_checked(dctx.wgt) {
        "[■] "
    }
    else {
        "[ ] "
    };
    let focused = dctx.wnd_state.is_focused(dctx.wgt);
    let clfg = get_widget_fg_color(dctx.wgt).intensify_if(focused);
    let _fm = FontMemento::new(&dctx.term_cell);
    let mut term = dctx.term_cell.borrow_mut();
    term.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
    );
    if focused {
        term.push_attr(FontAttrib::Bold);
    }
    term.push_cl_fg(clfg);
    term.write_str(chk_state);
    term.write_str(prp.text);
}

fn draw_radio(dctx: &mut DrawCtx, prp: &prop::Radio) {
    let radio_state = {
        let ridx = dctx.wnd_state.get_radio_index(dctx.wgt);
        if prp.radio_id == ridx {
            "(●) "
        }
        else {
            "( ) "
        }
    };

    let focused = dctx.wnd_state.is_focused(dctx.wgt);
    let clfg = get_widget_fg_color(dctx.wgt).intensify_if(focused);
    let _fm = FontMemento::new(&dctx.term_cell);
    let mut term = dctx.term_cell.borrow_mut();
    term.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
    );
    if focused {
        term.push_attr(FontAttrib::Bold);
    }
    term.push_cl_fg(clfg);
    term.write_str(radio_state);
    term.write_str(prp.text);
}

fn draw_button(dctx: &mut DrawCtx, prp: &prop::Button) {
    let focused = dctx.wnd_state.is_focused(dctx.wgt);
    let pressed = dctx.wgt.id == WGT_STATE.try_read().unwrap().mouse_down_wgt;
    let clfg = get_widget_fg_color(dctx.wgt).intensify_if(focused);
    let mut txt = String::new();

    if !prp.text.is_empty() {
        txt.push_str(prp.text);
    }
    else {
        dctx.wnd_state.get_button_text(dctx.wgt, &mut txt);
    }

    if prp.style == ButtonStyle::Simple {
        let _fm = FontMemento::new(&dctx.term_cell);

        {
            let mut term = dctx.term_cell.borrow_mut();

            dctx.strbuff.push_str("[ ");
            dctx.strbuff.push_str(txt.as_str());
            dctx.strbuff.push_str(" ]");

            term.move_to(
                dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
                dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
            );

            if focused {
                term.push_attr(FontAttrib::Bold);
            }
            if pressed {
                term.push_attr(FontAttrib::Inverse);
            }
            let clbg = if pressed {
                get_widget_bg_color(dctx.wgt)
            }
            else {
                get_widget_bg_color(wgt::get_parent(dctx.wgt))
            };
            term.push_cl_bg(clbg);
            term.push_cl_fg(clfg);
            term.write_str(dctx.strbuff.as_str());
        }
    }
    else if prp.style == ButtonStyle::Solid {
        {
            let _fm = FontMemento::new(&dctx.term_cell);

            {
                let mut term = dctx.term_cell.borrow_mut();
                dctx.strbuff.push(' ');
                dctx.strbuff.push_str(txt.as_str());
                dctx.strbuff.push(' ');

                term.move_to(
                    dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
                    dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
                );

                if focused {
                    term.push_attr(FontAttrib::Bold);
                }
                if pressed {
                    term.push_attr(FontAttrib::Inverse);
                }
                let clbg = get_widget_bg_color(dctx.wgt);
                term.push_cl_bg(clbg);
                term.push_cl_fg(clfg);
                term.write_str(dctx.strbuff.as_str());
            }
        }

        let shadow_len = 2 + txt.displayed_width() as i16;

        if pressed {
            // erase trailing shadow
            let mut term = dctx.term_cell.borrow_mut();

            term.push_cl_bg(get_widget_bg_color(wgt::get_parent(dctx.wgt)));
            term.write_char(' ');

            // erase shadow below
            term.move_to(
                dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16 + 1,
                dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16 + 1,
            );

            term.write_char_n(' ', shadow_len);
            term.pop_cl_bg();
        }
        else {
            let _fm = FontMemento::new(&dctx.term_cell);
            // trailing shadow
            {
                let mut term = dctx.term_cell.borrow_mut();

                term.push_cl_bg(get_widget_bg_color(wgt::get_parent(dctx.wgt)));
                term.write_str(crate::fg_color!(233));
                term.write_char('▄');
                // shadow below
                term.move_to(
                    dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16 + 1,
                    dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16 + 1,
                );
                term.write_char_n('▀', shadow_len);
            }
        }
    }
    else if prp.style == ButtonStyle::Solid1p5 {
        let _fm = FontMemento::new(&dctx.term_cell);
        dctx.strbuff.push(' ');
        dctx.strbuff.push_str(txt.as_str());
        dctx.strbuff.push(' ');

        let clbg = get_widget_bg_color(dctx.wgt);
        let clparbg = get_widget_bg_color(wgt::get_parent(dctx.wgt));
        let bnt_len = 2 + txt.displayed_width() as i16;
        let scl_shadow = crate::bg_color!(233);
        let scl_bg2fg = clbg.transcode_2_fg();

        {
            let mut term = dctx.term_cell.borrow_mut();

            // upper half line
            term.move_to(
                dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
                dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
            );

            term.push_cl_bg(clparbg);
            if pressed {
                term.push_cl_fg(clfg);
            }
            else {
                term.write_str(scl_bg2fg.as_str());
            }

            term.write_char_n('▄', bnt_len);

            // middle line - text
            term.move_by(-bnt_len, 1);
            term.push_cl_bg(clbg);
            term.push_cl_fg(clfg);
            if pressed {
                term.push_attr(FontAttrib::Inverse);
            }
            if focused {
                term.push_attr(FontAttrib::Bold);
            }
            term.write_str(dctx.strbuff.as_str());
            if focused {
                term.pop_attr();
            }
            if pressed {
                term.pop_attr();
            }

            // middle-shadow
            if pressed {
                term.push_cl_bg(clparbg);
            }
            else {
                term.write_str(scl_shadow);
            }
            term.write_char(' ');

            // lower half-line
            term.move_by(-bnt_len - 1, 1);

            if pressed {
                term.push_cl_fg(clfg);
                term.push_cl_bg(clparbg);
                term.write_char('▀');
                term.push_cl_bg(clparbg);
            }
            else {
                term.write_str(scl_bg2fg.as_str());
                term.push_cl_bg(clparbg);
                term.write_char('▀');
                term.write_str(scl_shadow);
            }
            term.write_char_n('▀', bnt_len - 1);

            // trailing shadow
            term.write_char(' ');
        }
    }
}

fn draw_page_control(dctx: &mut DrawCtx, prp: &prop::PageCtrl) {
    let mut fm = FontMementoManual::from_term(&dctx.term_cell.borrow());
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    dctx.term_cell
        .borrow_mut()
        .push_cl_bg(get_widget_bg_color(dctx.wgt));
    dctx.term_cell
        .borrow_mut()
        .push_cl_fg(get_widget_fg_color(dctx.wgt));

    draw_area(
        &mut dctx.term_cell.borrow_mut(),
        my_coord + Coord::new(prp.tab_width, 0),
        dctx.wgt.size - Size::new(prp.tab_width, 0),
        ColorBg::Inherit,
        ColorFg::Inherit,
        FrameStyle::PgControl,
        true,
        false,
    );

    dctx.term_cell.borrow_mut().flush_buff();

    let coord_bkp = dctx.parent_coord;
    dctx.parent_coord = my_coord;

    // tabs title
    {
        dctx.strbuff.push_n(' ', (prp.tab_width as i16 - 8) / 2);
        dctx.strbuff.push_str("≡ MENU ≡");
        dctx.strbuff.set_displayed_width(prp.tab_width as i16);

        let mut term = dctx.term_cell.borrow_mut();
        term.move_to(
            my_coord.col as u16,
            my_coord.row as u16 + prp.vert_offs as u16,
        );
        term.push_attr(FontAttrib::Inverse);
        term.write_str(dctx.strbuff.as_str());
        term.pop_attr();
    }

    dctx.term_cell.borrow_mut().flush_buff();

    // draw tabs and pages
    {
        let pgctrl = dctx.wgt;
        let cur_pg_idx = dctx.wnd_state.get_page_ctrl_page_index(pgctrl) as usize;
        let focused = dctx.wnd_state.is_focused(pgctrl);

        for (idx, page) in pgctrl.iter_children().enumerate() {
            // check if page is below lower border
            if idx as i16 == pgctrl.size.height as i16 - 1 - prp.vert_offs as i16 {
                break;
            }

            let page_prp = match page.prop {
                Property::Page(ref p) => p,
                _ => panic!(),
            };

            // draw tab title
            dctx.strbuff.clear();

            if idx == cur_pg_idx {
                dctx.strbuff.push('►');
            }
            else {
                dctx.strbuff.push(' ');
            }

            dctx.strbuff.push_str(page_prp.title);
            dctx.strbuff.set_displayed_width(prp.tab_width as i16);

            // for Page we do not want inherit after it's title color
            {
                let mut clfg = page_prp.fg_color;
                if clfg == ColorFg::Inherit {
                    clfg = get_widget_fg_color(page);
                }
                let mut term = dctx.term_cell.borrow_mut();
                term.move_to(
                    my_coord.col as u16,
                    my_coord.row as u16 + prp.vert_offs as u16 + idx as u16 + 1,
                );
                term.push_cl_fg(clfg);
                if idx == cur_pg_idx {
                    term.push_attr(FontAttrib::Inverse);
                }
                term.write_str(dctx.strbuff.as_str());
                if idx == cur_pg_idx {
                    term.pop_attr();
                }
                term.pop_cl_fg();
            }

            // when checking if page is visible, here we can call the wnd_state directly
            if idx == cur_pg_idx && dctx.wnd_state.is_visible(page) {
                dctx.term_cell.borrow_mut().flush_buff();
                dctx.wgt = page;
                draw_page(dctx, page_prp, false);
            }
        }

        dctx.wgt = pgctrl;
        dctx.parent_coord = coord_bkp;
    }

    fm.restore(&mut dctx.term_cell.borrow_mut());
}

fn draw_page(dctx: &mut DrawCtx, prp: &prop::Page, erase_bg: bool /*=false*/) {
    let pgctrl = wgt::get_parent(dctx.wgt);
    let mut my_coord = dctx.parent_coord + dctx.wgt.coord;
    let tab_width;

    if let Property::PageCtrl(ref pgctrl_prp) = pgctrl.prop {
        tab_width = pgctrl_prp.tab_width;
        my_coord.col += tab_width;
        dctx.parent_coord = my_coord;
    }
    else {
        return;
    }

    if erase_bg {
        let my_size = pgctrl.size - Size::new(tab_width, 0);

        draw_area(
            &mut dctx.term_cell.borrow_mut(),
            my_coord,
            my_size,
            ColorBg::Inherit,
            ColorFg::Inherit,
            FrameStyle::PgControl,
            true,
            false,
        );
    }

    // draw childrens
    {
        let page = dctx.wgt;

        for wgt in page.iter_children() {
            dctx.wgt = wgt;
            draw_widget_internal(dctx);
        }
    }

    dctx.strbuff.clear();
}

fn draw_progress_bar(dctx: &mut DrawCtx, prp: &prop::ProgressBar) {
    const STYLE_DATA: [[char; 2]; 3] = [['#', '.'], ['█', '▒'], ['■', '□']];

    let mut pbs = Default::default();
    dctx.wnd_state.get_progress_bar_state(dctx.wgt, &mut pbs);

    if pbs.max <= 0 {
        pbs.max = 1;
    }
    if pbs.pos > pbs.max {
        pbs.pos = pbs.max;
    }

    let style = prp.style as usize;
    let mut term = dctx.term_cell.borrow_mut();
    term.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16,
    );

    let fill_len = (pbs.pos * dctx.wgt.size.width as i32 / pbs.max) as i16;
    dctx.strbuff.push_n(STYLE_DATA[style][0], fill_len);
    dctx.strbuff
        .push_n(STYLE_DATA[style][1], dctx.wgt.size.width as i16 - fill_len);

    term.push_cl_fg(get_widget_fg_color(dctx.wgt));
    term.write_str(dctx.strbuff.as_str());
    term.pop_cl_fg();

    // ████░░░░░░░░░░░
    // [####.........]
    // [■■■■□□□□□□□□□]
    //  ▁▂▃▄▅▆▇█ - for vertical ▂▄▆█
}

fn draw_list_box(dctx: &mut DrawCtx, prp: &prop::ListBox) {
    let mut fm = FontMementoManual::from_term(&dctx.term_cell.borrow());
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    draw_area(
        &mut dctx.term_cell.borrow_mut(),
        my_coord,
        dctx.wgt.size,
        prp.bg_color,
        prp.fg_color,
        if prp.no_frame {
            FrameStyle::None
        }
        else {
            FrameStyle::ListBox
        },
        false,
        false,
    );

    if dctx.wgt.size.height < 3 {
        return;
    }

    let mut dlp = DrawListParams {
        coord: my_coord,
        ..Default::default()
    };

    let mut lbs = Default::default();
    dctx.wnd_state.get_list_box_state(dctx.wgt, &mut lbs);
    dlp.item_idx = lbs.item_idx;
    dlp.sel_idx = lbs.sel_idx;
    dlp.items_cnt = lbs.items_cnt;
    dlp.frame_size = !prp.no_frame as u8;
    dlp.items_visible = dctx.wgt.size.height as i16 - (dlp.frame_size as i16 * 2);
    dlp.top_item = (dlp.sel_idx / dlp.items_visible) * dlp.items_visible;
    dlp.focused = dctx.wnd_state.is_focused(dctx.wgt);
    dlp.wgt_width = dctx.wgt.size.width;

    // destructure dctx so the closure will capture local variables, not entire struct
    let wgt = dctx.wgt;
    let ws = &mut dctx.wnd_state;
    let mut term = dctx.term_cell.borrow_mut();

    let getitem_cb = |idx, out: &mut String| {
        ws.get_list_box_item(wgt, idx, out);
    };

    draw_list(&mut term, &dlp, getitem_cb);
    fm.restore(&mut term);
}

fn draw_combo_box(dctx: &mut DrawCtx, prp: &prop::ComboBox) {
    let _fm = FontMemento::new(&dctx.term_cell);
    let my_coord = dctx.parent_coord + dctx.wgt.coord;
    let focused = dctx.wnd_state.is_focused(dctx.wgt);

    let mut cbs = Default::default();
    dctx.wnd_state.get_combo_box_state(dctx.wgt, &mut cbs);

    {
        dctx.strbuff.clear();
        dctx.wnd_state
            .get_combo_box_item(dctx.wgt, cbs.item_idx, &mut dctx.strbuff);
        dctx.strbuff.insert(0, ' ');
        dctx.strbuff
            .set_displayed_width(dctx.wgt.size.width as i16 - 4); //, true);

        dctx.strbuff
            .push_str(tetrary!(cbs.drop_down, " [▲]", " [▼]"));

        let mut term = dctx.term_cell.borrow_mut();
        term.move_to(my_coord.col as u16, my_coord.row as u16);
        term.push_cl_fg(get_widget_fg_color(dctx.wgt));
        term.push_cl_bg(get_widget_bg_color(dctx.wgt));

        if focused && !cbs.drop_down {
            term.push_attr(FontAttrib::Inverse);
        }
        if cbs.drop_down {
            term.push_attr(FontAttrib::Underline);
        }
        if focused {
            term.push_attr(FontAttrib::Bold);
        }
        term.write_str(dctx.strbuff.as_str());
        if focused {
            term.pop_attr();
        }
        if cbs.drop_down {
            term.pop_attr();
        }
    }

    if cbs.drop_down {
        let mut dlp = DrawListParams {
            coord: my_coord,
            item_idx: cbs.item_idx,
            sel_idx: cbs.sel_idx,
            items_cnt: cbs.items_cnt,
            items_visible: prp.drop_down_size as i16,
            focused,
            wgt_width: dctx.wgt.size.width,
            ..DrawListParams::default()
        };

        dlp.top_item = (dlp.sel_idx / dlp.items_visible) * dlp.items_visible;
        dlp.coord.row += 1;

        // in 2021 edition, we can use entire struct in the closure
        // and see no borrowchecker error
        let getitem_cb = |idx, out: &mut String| {
            dctx.wnd_state.get_combo_box_item(dctx.wgt, idx, out);
        };

        let mut term = dctx.term_cell.borrow_mut();
        draw_list(&mut term, &dlp, getitem_cb);
    }
}

fn draw_custom_wgt(dctx: &mut DrawCtx, _: &prop::CustomWgt) {
    dctx.wnd_state
        .on_custom_widget_draw(dctx.wgt, &dctx.term_cell);
}

fn draw_text_box(dctx: &mut DrawCtx, prp: &prop::TextBox) {
    let _fm = FontMemento::new(&dctx.term_cell);
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    draw_area(
        &mut dctx.term_cell.borrow_mut(),
        my_coord,
        dctx.wgt.size,
        prp.bg_color,
        prp.fg_color,
        FrameStyle::ListBox,
        false,
        false,
    );

    if dctx.wgt.size.height < 3 {
        return;
    }

    let lines_visible = dctx.wgt.size.height as i16 - 2;
    let mut tbs = Default::default();
    dctx.wnd_state.get_text_box_state(dctx.wgt, &mut tbs);
    if std::sync::Arc::strong_count(&tbs.lines) > 0 {
        let lines = tbs.lines.as_ref().borrow();
        if tbs.top_line > lines.len() as i16 {
            tbs.top_line = lines.len() as i16 - lines_visible;
            dctx.wnd_state.on_text_box_scroll(dctx.wgt, tbs.top_line);
        }

        if tbs.top_line < 0 {
            dctx.wnd_state.on_text_box_scroll(dctx.wgt, tbs.top_line);
            tbs.top_line = 0;
        }

        let mut term = dctx.term_cell.borrow_mut();

        draw_list_scroll_bar_v(
            &mut term,
            my_coord + Coord::new(dctx.wgt.size.width - 1, 1),
            lines_visible,
            lines.len() as i16 - lines_visible,
            tbs.top_line,
        );

        term.flush_buff();
        dctx.strbuff.clear();

        // scan invisible lines (because scrooled down) for ESC sequences: colors, font attributes
        for i in 0..tbs.top_line as usize {
            if let Some(line) = lines.get(i) {
                // iterate over all ESC sequences in the line
                for it in line.bytes().enumerate() {
                    if it.1 == crate::esc::ESC_U8 {
                        let esc = line.as_str();
                        // get the sequence and push it to the output stream
                        let esc = &esc[it.0..esc.len()];
                        let esclen = esc.esc_seq_len();
                        let sequence = &esc[0..esclen];
                        dctx.strbuff.push_str(sequence);
                    }
                }
            }
        }

        term.write_str(dctx.strbuff.as_str());

        // draw lines
        for i in 0..lines_visible as usize {
            dctx.strbuff.clear();

            if tbs.top_line as usize + i < lines.len() {
                if let Some(line) = lines.get(tbs.top_line as usize + i) {
                    dctx.strbuff.push_str(line);
                }
            }
            dctx.strbuff
                .set_displayed_width(dctx.wgt.size.width as i16 - 2); //, true);
            term.move_to(my_coord.col as u16 + 1, my_coord.row as u16 + i as u16 + 1);
            term.write_str(dctx.strbuff.as_str());
        }

        term.flush_buff();
    }
}

fn draw_layer(dctx: &mut DrawCtx, _: &prop::Layer) {
    // draw only childrens; to erase, redraw layer's parent
    let layer = dctx.wgt;

    for wgt in layer.iter_children() {
        dctx.wgt = wgt;
        draw_widget_internal(dctx);
    }

    dctx.wgt = layer;
}

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const FRAME_NONE: [char; 9] = [
    ' ', ' ', ' ',
    ' ', ' ', ' ',
    ' ', ' ', ' '
];

#[rustfmt::skip]
const FRAME_SINGLE: [char; 9] = [
    '┌', '─', '┐',
    '│', ' ', '│',
    '└', '─', '┘'
];

#[rustfmt::skip]
const FRAME_LISTBOX: [char; 9] = [
    '┌', '─', '┐',
    '│', ' ', '▒',
    '└', '─', '┘'
];

#[rustfmt::skip]
const FRAME_PGCONTROL: [char; 9] = [
    '├', '─', '┐',
    '│', ' ', '│',
    '├', '─', '┘'
];

#[rustfmt::skip]
const FRAME_DOUBLE: [char; 9] = [
    '╔', '═', '╗',
    '║', ' ', '║',
    '╚', '═', '╝'
];

#[allow(clippy::too_many_arguments)]
fn draw_area(
    term: &mut Term,
    coord: Coord,
    size: Size,
    cl_bg: ColorBg,
    cl_fg: ColorFg,
    style: FrameStyle,
    filled: bool,
    shadow: bool,
) {
    term.move_to(coord.col.into(), coord.row.into());

    let frame = match style {
        FrameStyle::Single => &FRAME_SINGLE,
        FrameStyle::Double => &FRAME_DOUBLE,
        FrameStyle::PgControl => &FRAME_PGCONTROL,
        FrameStyle::ListBox => &FRAME_LISTBOX,
        _ => &FRAME_NONE,
    };

    // background and frame color
    if cl_bg != ColorBg::Inherit {
        term.push_cl_bg(cl_bg);
    }
    if cl_fg != ColorFg::Inherit {
        term.push_cl_fg(cl_fg);
    }

    let mut strbuff = String::with_capacity(500);

    // top line
    strbuff.push(frame[0]);
    draw_line(&mut strbuff, frame[1], size.width);
    strbuff.push(frame[2]);

    term.write_str(strbuff.as_str());
    term.move_by(-(size.width as i16), 1);
    term.flush_buff();

    // lines in the middle
    strbuff.clear();
    strbuff.push(frame[3]);

    if filled {
        draw_line(&mut strbuff, frame[4], size.width);
    }
    else {
        strbuff.push_esc_fmt(esc::CURSOR_FORWARD_FMT, size.width as i16 - 2);
    }
    strbuff.push(frame[5]);

    if shadow {
        // trailing shadow
        strbuff.push_str(esc::FG_BLACK);
        strbuff.push('█');
        strbuff.push_str(cl_fg.encode());
    }

    for r in coord.row + 1..coord.row + size.height - 1 {
        term.write_str(strbuff.as_str());
        term.move_by(-(size.width as i16 + shadow as i16), 1);
        term.flush_buff();
    }

    // bottom line
    strbuff.clear();
    strbuff.push(frame[6]);
    draw_line(&mut strbuff, frame[7], size.width);
    strbuff.push(frame[8]);

    if shadow {
        // trailing shadow
        strbuff.push_str(esc::FG_BLACK);
        strbuff.push('█');
    }

    term.write_str(strbuff.as_str());
    term.flush_buff();

    if shadow {
        term.move_by(-(size.width as i16), 1);
        strbuff.clear();
        // trailing shadow
        draw_line(&mut strbuff, '█', size.width + 2);
        term.write_str(strbuff.as_str());
        term.write_str(cl_fg.encode());
        term.flush_buff();
    }

    // here the Fg and Bg colors are not restored
}

fn draw_line(strbuff: &mut String, c: char, len: u8) {
    if cfg!(fast_line) {
        strbuff.push(c);
        strbuff.push_esc_fmt(esc::CHAR_REPEAT_LAST_FMT, len as i16 - 3);
    }
    else {
        // in case the code is not supported
        strbuff.push_n(c, len as i16 - 2);
    }
}

fn draw_list_scroll_bar_v(term: &mut Term, coord: Coord, height: i16, pos_max: i16, pos: i16) {
    if pos > pos_max {
        tr_debug!("W: pos ({}) > max ({})", pos, pos_max);
        return;
    }

    let slider_at = ((height - 1) * pos) / pos_max;
    // "▲▴ ▼▾ ◄◂ ►▸ ◘ █";

    for i in 0..height {
        term.move_to(coord.col.into(), (coord.row as u16) + i as u16);
        term.write_char(if i == slider_at { '◘' } else { '▒' });
    }
}

#[derive(Default)]
struct DrawListParams {
    coord: Coord,
    item_idx: i16,
    sel_idx: i16,
    items_cnt: i16,
    items_visible: i16,
    top_item: i16,
    focused: bool,
    wgt_width: u8,
    frame_size: u8,
}

fn draw_list<F>(term: &mut Term, dlp: &DrawListParams, mut get_item: F)
where
    F: FnMut(i16, &mut String),
{
    if dlp.items_cnt > dlp.items_visible {
        draw_list_scroll_bar_v(
            term,
            dlp.coord + Coord::new(dlp.wgt_width - 1, dlp.frame_size),
            dlp.items_visible,
            dlp.items_cnt - 1,
            dlp.sel_idx,
        );
    }

    let mut get_list_item = |idx: i16, strbuff: &mut String| {
        get_item(idx, strbuff);
    };

    term.flush_buff();
    let mut strbuff = String::with_capacity(50);

    for i in 0..dlp.items_visible {
        let is_current_item = tetrary!(dlp.items_cnt > 0, dlp.top_item + i == dlp.item_idx, false);
        let is_sel_item = dlp.top_item + i == dlp.sel_idx;

        term.move_to(
            dlp.coord.col as u16 + dlp.frame_size as u16,
            dlp.coord.row as u16 + i as u16 + dlp.frame_size as u16,
        );

        strbuff.clear();

        if dlp.top_item + i < dlp.items_cnt {
            strbuff.push(tetrary!(is_current_item, '►', ' '));
            get_list_item(dlp.top_item + i, &mut strbuff);
            strbuff.set_displayed_width(dlp.wgt_width as i16 - 1 - dlp.frame_size as i16);
        }
        else {
            // empty string - to erase old content
            strbuff.set_displayed_width(dlp.wgt_width as i16 - 1 - dlp.frame_size as i16);
        }

        if dlp.focused && is_sel_item {
            term.push_attr(FontAttrib::Inverse);
        }
        if is_current_item {
            term.push_attr(FontAttrib::Underline);
        }

        term.write_str(strbuff.as_str());

        if is_current_item {
            term.pop_attr();
        }
        if dlp.focused && is_sel_item {
            term.pop_attr();
        }
    }
}

fn get_widget_bg_color(wgt: &Widget) -> ColorBg {
    let mut cl = match wgt.prop {
        Property::Window(ref p) => p.bg_color,
        Property::Panel(ref p) => p.bg_color,
        Property::Label(ref p) => p.bg_color,
        Property::TextEdit(ref p) => p.bg_color,
        Property::Button(ref p) => p.bg_color,
        Property::ListBox(ref p) => p.bg_color,
        Property::ComboBox(ref p) => p.bg_color,
        Property::TextBox(ref p) => p.bg_color,
        _ => ColorBg::Inherit,
    };

    if cl == ColorBg::Inherit {
        let parent = wgt::get_parent(wgt);
        if parent.id != wgt.id {
            cl = get_widget_bg_color(parent);
        }
    }

    cl
}

fn get_widget_fg_color(wgt: &Widget) -> ColorFg {
    let mut cl = match wgt.prop {
        Property::Window(ref p) => p.fg_color,
        Property::Panel(ref p) => p.fg_color,
        Property::Label(ref p) => p.fg_color,
        Property::TextEdit(ref p) => p.fg_color,
        Property::CheckBox(ref p) => p.fg_color,
        Property::Radio(ref p) => p.fg_color,
        Property::Button(ref p) => p.fg_color,
        Property::Led(ref p) => p.fg_color,
        Property::Page(ref p) => p.fg_color,
        Property::ProgressBar(ref p) => p.fg_color,
        Property::ListBox(ref p) => p.fg_color,
        Property::ComboBox(ref p) => p.fg_color,
        Property::TextBox(ref p) => p.fg_color,
        _ => ColorFg::Inherit,
    };

    if cl == ColorFg::Inherit {
        let parent = wgt::get_parent(wgt);
        if parent.id != wgt.id {
            cl = get_widget_fg_color(parent);
        }
    }

    cl
}
