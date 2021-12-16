//! # RTWins Widget drawing

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::fmt::Write;
use unicode_width::UnicodeWidthStr;

use crate::{FontMementoManual, FontMemento, FontAttrib, colors};
use crate::widget_impl::*;
use crate::widget::*;
use crate::colors::*;
use crate::Ctx;
use crate::esc;

// ---------------------------------------------------------------------------------------------- //

/// Trait extending base `String` functionality
trait StrExt {
    /// Push ANSI escape sequence, replacing `{0}` with the `val`
    fn push_esc_fmt(&mut self, escfmt: &str, val: i16);
    /// Push `repeat` copies of `c`
    fn push_n(&mut self, c: char, n: i16);
    /// Set displayed width to `w` according to Unicode Standard
    fn set_width(&mut self, w: i16);
}

impl StrExt for String {
    fn push_esc_fmt(&mut self, escfmt: &str, val: i16) {
        if let Some((a, b)) = escfmt.split_once("{0}") {
            self.write_fmt(format_args!("{}{}{}", a, val, b)).unwrap_or_default();
        }
    }

    fn push_n(&mut self, c: char, repeat: i16) {
        for i in 0..repeat {
            self.push(c);
        }
    }

    fn set_width(&mut self, w: i16) {
        let n = UnicodeWidthStr::width(self.as_str());
        self.push_n(' ', w - n as i16);
    }
}

// ---------------------------------------------------------------------------------------------- //

struct DrawCtx<'a>
{
    /// Reference to a drawer instance
    ctx: RefCell<&'a mut Ctx>,
    /// Reference to a widget to be drawn
    wgt: &'a Widget,
    /// Reference to window state that relates to the widget
    wnd_state: &'a mut dyn WindowState,
    /// Current widget's parent left-top position
    parent_coord: Coord,
    ///
    wnd_widgets: &'a [Widget],
    ///
    strbuff: String
}

/// Draw `wids` widgets of the given window.
/// If `wids` contains only `WIDGET_ID_ALL`, draw entire window
pub fn draw_widgets(ctx: &mut Ctx, ws: &mut dyn WindowState, wids: &[WId])
{
    if wids.len() == 0 {
        return;
    }

    let mut fm = FontMementoManual::new();
    fm.store(ctx);

/*
    g_ws.pFocusedWgt = getWidgetByWID(dctx, ctx.stat.getFocusedID());
*/
    ctx.cursor_hide();
    ctx.flush_buff();

    if wids.len() == 1 && wids[0] == WIDGET_ID_ALL {
        let wgts = ws.get_widgets();
        let wgt = wgts.get(0).unwrap(); // window is at index 0
        let mut dctx = DrawCtx{ ctx: RefCell::new(ctx),
            wgt, wnd_state: ws, parent_coord: Coord::cdeflt(), wnd_widgets: wgts,
            strbuff: String::with_capacity(200) };
        draw_widget_internal(&mut dctx);
    }
    else {
        for i in 0..wids.len() {
            let _wss = WidgetSearchStruct::new(wids[i]);
 /*
            if (getWidgetWSS(dctx, wss) && wss.isVisible)
            {
                dctx.parentCoord = wss.parentCoord;
                // set parent's background color
                pushClBg(getWidgetBgColor(wss.pWidget));
                drawWidgetInternal(dctx, wss.pWidget);
                popClBg();
            }
  */
        }
    }

    ctx.reset_attr();
    ctx.reset_cl_bg();
    ctx.reset_cl_fg();
/*
    setCursorAt(dctx, g_ws.pFocusedWgt);
*/
    ctx.cursor_show();
    fm.restore(ctx);
    ctx.flush_buff();
}

// -----------------------------------------------------------------------------------------------

fn draw_widget_internal(dctx: &mut DrawCtx)
{
    if !dctx.wnd_state.is_visible(dctx.wgt) {
        return;
    }

    let en = dctx.wnd_state.is_enabled(dctx.wgt);
    if !en { dctx.ctx.borrow_mut().push_attr(FontAttrib::Faint); }

    // dctx.ctx.borrow_mut().log_d(format!("drawing {}", dctx.wgt.typ).as_str());
    // println!("drawing {}", dctx.wgt.typ);
    dctx.strbuff.clear();

    match dctx.wgt.typ {
        Type::Window(ref p) => draw_window(dctx, p),
        Type::Panel(ref p) => draw_panel(dctx, p),
        Type::Label(ref p) => draw_label(dctx, p),
        Type::TextEdit(ref p) => draw_text_edit(dctx, p),
        Type::CheckBox(ref p) => draw_checkbox(dctx, p),
        Type::Radio(ref p) => draw_radio(dctx, p),
        Type::Button(ref p) => draw_button(dctx, p),
        Type::Led(ref p) => draw_led(dctx, p),
        Type::PageCtrl(ref p) => draw_page_control(dctx, p),
        Type::Page(ref p) => draw_page(dctx, p, true),
        Type::ProgressBar(ref p) => draw_progress_bar(dctx, p),
        Type::ListBox(ref p) => draw_list_box(dctx, p),
        Type::ComboBox(ref p) => draw_combo_box(dctx, p),
        Type::CustomWgt(ref p) => draw_custom_wgt(dctx, p),
        Type::TextBox(ref p) => draw_text_box(dctx, p),
        Type::Layer(ref p) => draw_layer(dctx, p),
        _ => {}
    }

    if !en { dctx.ctx.borrow_mut().pop_attr(); }
    dctx.ctx.borrow_mut().flush_buff();
}

// -----------------------------------------------------------------------------------------------

fn draw_window(dctx: &mut DrawCtx, prp: &prop::Window)
{
    let mut wnd_coord = Coord::cdeflt();
    dctx.wnd_state.get_window_coord(dctx.wgt, &mut wnd_coord);

    draw_area(&mut dctx.ctx.borrow_mut(), wnd_coord, dctx.wgt.size,
            prp.bg_color, prp.fg_color, FrameStyle::Double, true, prp.is_popup);

    // title
    let mut wnd_title = String::new();

    if !prp.title.is_empty() {
        wnd_title = prp.title.to_string();
    }
    else {
        dctx.wnd_state.get_window_title(&dctx.wgt, &mut wnd_title);
    }

    if !wnd_title.is_empty() {
        let title_width = UnicodeWidthStr::width(wnd_title.as_str()) as u16 + 4;
        let mut ctx = dctx.ctx.borrow_mut();
        ctx.move_to(
            wnd_coord.col as u16 + (dctx.wgt.size.width as u16 - title_width) / 2,
            wnd_coord.row as u16);
        ctx.push_attr(FontAttrib::Bold);
        ctx.write_str(format!("╡ {} ╞", wnd_title.as_str()).as_str());
        ctx.pop_attr();
    }

    dctx.ctx.borrow_mut().flush_buff();
    dctx.parent_coord = wnd_coord;

    {
        let wnd = dctx.wgt;

        for wgt in WidgetIter::new(wnd) {
            dctx.wgt = wgt;
            draw_widget_internal(dctx);
        }

        dctx.wgt = wnd;
    }

    // reset colors set by frame drawer
    {
        let mut ctx = dctx.ctx.borrow_mut();
        ctx.pop_cl_bg();
        ctx.pop_cl_fg();
        ctx.move_to(0, wnd_coord.row as u16 + dctx.wgt.size.height as u16);
    }
}

fn draw_panel(dctx: &mut DrawCtx, prp: &prop::Panel)
{
    let mut fm = FontMementoManual::new();
    fm.store(&dctx.ctx.borrow());
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    draw_area(&mut dctx.ctx.borrow_mut(),
        my_coord,
        dctx.wgt.size,
        prp.bg_color,
        prp.fg_color,
        if prp.no_frame { FrameStyle::None } else { FrameStyle::Single },
        true, false);

    // title
    if !prp.title.is_empty() {
        let title_width = UnicodeWidthStr::width(prp.title) as u16;
        let mut ctx = dctx.ctx.borrow_mut();
        ctx.move_to(
            my_coord.col as u16 + (dctx.wgt.size.width as u16 - title_width - 2)/2,
            my_coord.row as u16);
        ctx.push_attr(FontAttrib::Bold);
        ctx.write_char(' ').write_str(prp.title).write_char(' ');
        ctx.pop_attr();
    }

    dctx.ctx.borrow_mut().flush_buff();

    // draw childrens
    {
        let coord_bkp = dctx.parent_coord;
        dctx.parent_coord = my_coord;
        let pnl = dctx.wgt;

        for wgt in WidgetIter::new(pnl) {
            dctx.wgt = wgt;
            draw_widget_internal(dctx);
        }

        dctx.wgt = pnl;
        dctx.parent_coord = coord_bkp;
    }

    // dctx.ctx.borrow_mut().pop_cl_bg();
    fm.restore(&mut dctx.ctx.borrow_mut());
}

fn draw_label(dctx: &mut DrawCtx, prp: &prop::Label)
{
    // label text
    let mut title = String::new();
    if !prp.title.is_empty() {
        title = prp.title.into();
    }
    else {
        dctx.wnd_state.get_label_text(dctx.wgt, &mut title);
    }

    let _fm = FontMemento::new(&dctx.ctx);
    let mut ctx = dctx.ctx.borrow_mut();

    // setup colors
    ctx.push_cl_fg(get_widget_fg_color(dctx.wgt));
    ctx.push_cl_bg(get_widget_bg_color(dctx.wgt));

    // print all lines
    ctx.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);

    let max_lines = if dctx.wgt.size.height > 0 { dctx.wgt.size.height } else { 50 };
    let line_width = dctx.wgt.size.width;

    for (line, s) in title.lines().enumerate() {
        dctx.strbuff.push_str(s);

        if line_width > 0 {
            dctx.strbuff.set_width(line_width as i16);
        }

        ctx.write_str(dctx.strbuff.as_str());
        let w = UnicodeWidthStr::width(dctx.strbuff.as_str()) as i16;
        ctx.move_by(-w, 1);
        ctx.flush_buff();
        if line as u8 == max_lines {
            break;
        }
    }
}

fn draw_text_edit(dctx: &mut DrawCtx, prp: &prop::TextEdit)
{
    /*
    g_w s.str.clear();
    int16_t display_pos = 0;
    const int16_t max_w = dctx.wgt->size.width-3;

    if (dctx.wgt == g_ws.textEditState.dctx.wgt)
    {
        // in edit mode; similar calculation in setCursorAt()
        g_ws.str = g_ws.textEditState.str;
        auto cursor_pos = g_ws.textEditState.cursorPos;
        auto delta = (max_w/2);

        while (cursor_pos >= max_w-1)
        {
            cursor_pos -= delta;
            display_pos += delta;
        }
    }
    else
    {
        dctx.pState->getTextEditText(dctx.wgt, g_ws.str);
    }

    const int txt_width = g_ws.str.width();

    if (display_pos > 0)
    {
        auto *str_beg = String::u8skip(g_ws.str.cstr(), display_pos + 1);
        String s("◁");
        s << str_beg;
        g_ws.str = std::move(s);
    }

    if (display_pos + max_w <= txt_width)
    {
        g_ws.str.setWidth(dctx.wgt->size.width-3-1);
        g_ws.str.append("▷");
    }
    else
    {
        g_ws.str.setWidth(dctx.wgt->size.width-3);
    }
    g_ws.str.append("[^]");

    bool focused = dctx.pState->isFocused(dctx.wgt);
    auto clbg = getWidgetBgColor(dctx.wgt);
    intensifyClIf(focused, clbg);

    let _fm = FontMemento::new(&dctx.ctx);
    moveTo(dctx.parentCoord.col + dctx.wgt->coord.col, dctx.parentCoord.row + dctx.wgt->coord.row);
    pushClBg(clbg);
    pushClFg(getWidgetFgColor(dctx.wgt));
    ctx.write_str(dctx.strbuff.as_str()); */
}

fn draw_led(dctx: &mut DrawCtx, prp: &prop::Led)
{
    let clbg = if dctx.wnd_state.get_led_lit(dctx.wgt) { prp.bg_color_on } else { prp.bg_color_off };

    if !prp.text.is_empty() {
        dctx.strbuff.push_str(prp.text);
    }
    else {
        dctx.wnd_state.get_led_text(dctx.wgt, &mut dctx.strbuff);
    }

    // led text
    let _fm = FontMemento::new(&dctx.ctx);

    let mut ctx = dctx.ctx.borrow_mut();
    ctx.move_to(dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);
    ctx.push_cl_bg(clbg);
    ctx.push_cl_fg(get_widget_fg_color(dctx.wgt));
    ctx.write_str(dctx.strbuff.as_str());
}

fn draw_checkbox(dctx: &mut DrawCtx, prp: &prop::CheckBox)
{
    let chk_state = if dctx.wnd_state.get_checkbox_checked(dctx.wgt) { "[■] " } else { "[ ] " };
    let focused = dctx.wnd_state.is_focused(dctx.wgt);
    let clfg = {
        let cl = get_widget_fg_color(dctx.wgt);
        if focused { intensify_cl_fg(cl) } else { cl }
    };

    let _fm = FontMemento::new(&dctx.ctx);
    let mut ctx = dctx.ctx.borrow_mut();
    ctx.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);
    if focused {
        ctx.push_attr(FontAttrib::Bold);
    }
    ctx.push_cl_fg(clfg);
    ctx.write_str(chk_state);
    ctx.write_str(prp.text);
}

fn draw_radio(dctx: &mut DrawCtx, prp: &prop::Radio)
{
    let radio_state = {
        let ridx = dctx.wnd_state.get_radio_index(dctx.wgt);
        if prp.radio_id == ridx { "(●) " } else { "( ) " }
    };

    let focused = dctx.wnd_state.is_focused(dctx.wgt);
    let clfg = {
        let cl = get_widget_fg_color(dctx.wgt);
        if focused { colors::intensify_cl_fg(cl) } else { cl }
    };

    let _fm = FontMemento::new(&dctx.ctx);
    let mut ctx = dctx.ctx.borrow_mut();
    ctx.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);
    if focused {
        ctx.push_attr(FontAttrib::Bold);
    }
    ctx.push_cl_fg(clfg);
    ctx.write_str(radio_state);
    ctx.write_str(prp.text);
}

fn draw_button(dctx: &mut DrawCtx, prp: &prop::Button)
{
    let focused = dctx.wnd_state.is_focused(dctx.wgt);
    let pressed = false; // TODO:dctx.wgt == g_ws.pMouseDownWgt;
    let clfg = {
        let cl = get_widget_fg_color(dctx.wgt);
        if focused { intensify_cl_fg(cl) } else { cl }
    };

    let mut txt= String::new();
    if !prp.text.is_empty() {
        txt.push_str(prp.text);
    }
    else {
        dctx.wnd_state.get_button_text(dctx.wgt, &mut txt);
    }

    if prp.style == ButtonStyle::Simple {
        let _fm = FontMemento::new(&dctx.ctx);

        {
            let mut ctx = dctx.ctx.borrow_mut();

            dctx.strbuff.push_str("[ ");
            dctx.strbuff.push_str(txt.as_str());
            dctx.strbuff.push_str(" ]");

            ctx.move_to(
                dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
                dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);

            if focused { ctx.push_attr(FontAttrib::Bold); }
            if pressed { ctx.push_attr(FontAttrib::Inverse); }
            let clbg = if pressed { get_widget_bg_color(dctx.wgt) } else { get_widget_bg_color(wgt_get_parent(dctx.wgt)) };
            ctx.push_cl_bg(clbg);
            ctx.push_cl_fg(clfg);
            ctx.write_str(dctx.strbuff.as_str());
        }
    }
    else if prp.style == ButtonStyle::Solid {
        {
            let _fm = FontMemento::new(&dctx.ctx);

            {
                let mut ctx = dctx.ctx.borrow_mut();
                dctx.strbuff.push_str(" ");
                dctx.strbuff.push_str(txt.as_str());
                dctx.strbuff.push_str(" ");

                ctx.move_to(
                    dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
                    dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);

                if focused { ctx.push_attr(FontAttrib::Bold); }
                if pressed { ctx.push_attr(FontAttrib::Inverse); }
                let clbg = get_widget_bg_color(dctx.wgt);
                ctx.push_cl_bg(clbg);
                ctx.push_cl_fg(clfg);
                ctx.write_str(dctx.strbuff.as_str());
            }
        }

        let shadow_len = 2 + txt.width() as i16;

        if pressed {
            // erase trailing shadow
            let mut ctx = dctx.ctx.borrow_mut();

            ctx.push_cl_bg(get_widget_bg_color(wgt_get_parent(dctx.wgt)));
            ctx.write_char(' ');

            // erase shadow below
            ctx.move_to(
                dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16 + 1,
                dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16 + 1);

            ctx.write_char_n(' ', shadow_len);
            ctx.pop_cl_bg();
        }
        else {
            let _fm = FontMemento::new(&dctx.ctx);
            // trailing shadow
            {
                let mut ctx = dctx.ctx.borrow_mut();

                ctx.push_cl_bg(get_widget_bg_color(wgt_get_parent(dctx.wgt)));
                ctx.write_str(crate::fg_color!(233));
                ctx.write_char('▄');
                // shadow below
                ctx.move_to(
                    dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16 + 1,
                    dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16 + 1);
                ctx.write_char_n('▀', shadow_len);
            }
        }
    }
    else if prp.style == ButtonStyle::Solid1p5 {
        // dctx.strbuff.clear();
        let _fm = FontMemento::new(&dctx.ctx);
        dctx.strbuff.push_str(" ");
        dctx.strbuff.push_str(txt.as_str());
        dctx.strbuff.push_str(" ");

        let clbg = get_widget_bg_color(dctx.wgt);
        let clparbg = get_widget_bg_color(wgt_get_parent(dctx.wgt));
        let bnt_len = 2 + txt.width() as i16;
        let scl_shadow = crate::bg_color!(233);
        let scl_bg2fg = transcode_cl_bg_2_fg(encode_cl_bg(clbg));

        {
            let mut ctx = dctx.ctx.borrow_mut();

            // upper half line
            ctx.move_to(
                dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
                dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);

            ctx.push_cl_bg(clparbg);
            if pressed {
                ctx.push_cl_fg(clfg);
            }
            else {
                ctx.write_str(scl_bg2fg.as_str());
            }

            ctx.write_char_n('▄', bnt_len);

            // middle line - text
            ctx.move_by(-bnt_len, 1);
            ctx.push_cl_bg(clbg);
            ctx.push_cl_fg(clfg);
            if pressed { ctx.push_attr(FontAttrib::Inverse); }
            if focused { ctx.push_attr(FontAttrib::Bold); }
            ctx.write_str(dctx.strbuff.as_str());
            if focused { ctx.pop_attr(); }
            if pressed { ctx.pop_attr(); }

            // middle-shadow
            if pressed {
                ctx.push_cl_bg(clparbg);
            }
            else {
                ctx.write_str(scl_shadow);
            }
            ctx.write_char(' ');

            // lower half-line
            ctx.move_by(-bnt_len-1, 1);

            if pressed {
                ctx.push_cl_fg(clfg);
                ctx.push_cl_bg(clparbg);
                ctx.write_char('▀');
                ctx.push_cl_bg(clparbg);
            }
            else {
                ctx.write_str(scl_bg2fg.as_str());
                ctx.push_cl_bg(clparbg);
                ctx.write_char('▀');
                ctx.write_str(scl_shadow);
            }
            ctx.write_char_n('▀', bnt_len-1);

            // trailing shadow
            ctx.write_char(' ');
        }
    }
}

fn draw_page_control(dctx: &mut DrawCtx, prp: &prop::PageCtrl)
{
    let mut fm = FontMementoManual::new();
    fm.store(&dctx.ctx.borrow());
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    dctx.ctx.borrow_mut().push_cl_bg(get_widget_bg_color(dctx.wgt));
    dctx.ctx.borrow_mut().push_cl_fg(get_widget_fg_color(dctx.wgt));

    draw_area(&mut dctx.ctx.borrow_mut(),
        my_coord + Coord::new(prp.tab_width, 0),
        dctx.wgt.size - Size::new(prp.tab_width, 0),
        ColorBG::Inherit,
        ColorFG::Inherit,
        FrameStyle::PgControl,
        true, false);

    dctx.ctx.borrow_mut().flush_buff();

    let coord_bkp = dctx.parent_coord;
    dctx.parent_coord = my_coord;

    // tabs title
    {
        dctx.strbuff.push_n(' ', (prp.tab_width as i16 -8) / 2);
        dctx.strbuff.push_str("≡ MENU ≡");
        dctx.strbuff.set_width(prp.tab_width as i16);

        let mut ctx = dctx.ctx.borrow_mut();
        ctx.move_to(my_coord.col as u16, my_coord.row as u16 + prp.vert_offs as u16);
        ctx.push_attr(FontAttrib::Inverse);
        ctx.write_str(dctx.strbuff.as_str());
        ctx.pop_attr();
    }

    // moveTo(dctx.parentCoord.col + dctx.wgt->coord.col, dctx.parentCoord.row + dctx.wgt->coord.row);
    dctx.ctx.borrow_mut().flush_buff();

    // draw tabs and pages
    {
        let pgctrl = dctx.wgt;
        let pg_idx = dctx.wnd_state.get_page_ctrl_page_index(pgctrl);
        let focused = dctx.wnd_state.is_focused(pgctrl);
        dctx.parent_coord.col += prp.tab_width;

        for (idx, page) in WidgetIter::new(pgctrl).enumerate() {
            // check if page is below lower border
            if idx as i16 == pgctrl.size.height as i16 - 1 - prp.vert_offs as i16 {
                break;
            }

            let page_prp = match page.typ {
                Type::Page(ref p) => p,
                _ => panic!()
            };

            // draw tab title
            dctx.strbuff.clear();

            if idx as i8 == pg_idx {
                dctx.strbuff.push_str("►");
            }
            else {
                dctx.strbuff.push_str(" ");
            }

            dctx.strbuff.push_str(page_prp.title);
            dctx.strbuff.set_width(prp.tab_width as i16);

            // for Page we do not want inherit after it's title color
            {
                let mut clfg = page_prp.fg_color;
                if clfg == ColorFG::Inherit { clfg = get_widget_fg_color(page); }
                let mut ctx = dctx.ctx.borrow_mut();
                ctx.move_to(
                    my_coord.col as u16,
                    my_coord.row as u16 + prp.vert_offs as u16 + idx as u16 + 1);
                ctx.push_cl_fg(clfg);
                if idx as i8 == pg_idx { ctx.push_attr(FontAttrib::Inverse); }
                ctx.write_str(dctx.strbuff.as_str());
                if idx as i8 == pg_idx { ctx.pop_attr(); }
                ctx.pop_cl_fg();
            }

            if idx as i8 == pg_idx && dctx.wnd_state.is_visible(page) {
                dctx.ctx.borrow_mut().flush_buff();
                dctx.wgt = page;
                draw_page(dctx, page_prp, false);
            }
        }

        dctx.wgt = pgctrl;
        dctx.parent_coord = coord_bkp;
    }

    fm.restore(&mut dctx.ctx.borrow_mut());
}

fn draw_page(dctx: &mut DrawCtx, prp: &prop::Page, erase_bg: bool /*=false*/)
{
    if erase_bg {
        let pgctrl = wgt_get_parent(dctx.wgt);

        if let Type::PageCtrl(ref pgctrl_prp) = pgctrl.typ {
            let page_coord = wgt_get_screen_coord(dctx.wgt);
            let page_size = pgctrl.size - Size::new(pgctrl_prp.tab_width, 0);

            draw_area(&mut dctx.ctx.borrow_mut(),
                page_coord,
                page_size,
                ColorBG::Inherit,
                ColorFG::Inherit,
                FrameStyle::PgControl,
                true, false);
        }
    }

    // draw childrens
    {
        let page = dctx.wgt;

        for wgt in WidgetIter::new(page) {
            dctx.wgt = wgt;
            draw_widget_internal(dctx);
        }
    }

    dctx.strbuff.clear();
}

fn draw_progress_bar(dctx: &mut DrawCtx, prp: &prop::ProgressBar)
{
    const STYLE_DATA: [[char;2];3] = [
        ['#', '.'],
        ['█', '▒'],
        ['■', '□']
    ];

    let mut pos = 0i32;
    let mut max = 1i32;
    let style = prp.style as usize;
    dctx.wnd_state.get_progress_bar_state(&dctx.wgt, &mut pos, &mut max);

    if max <= 0 { max = 1; }
    if pos > max { pos = max; }

    let mut ctx = dctx.ctx.borrow_mut();
    ctx.move_to(
        dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16,
        dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16);

    let fill_len = (pos * dctx.wgt.size.width as i32 / max) as i16;
    dctx.strbuff.push_n(STYLE_DATA[style][0], fill_len);
    dctx.strbuff.push_n(STYLE_DATA[style][1], dctx.wgt.size.width as i16 - fill_len);

    ctx.push_cl_fg(get_widget_fg_color(dctx.wgt));
    ctx.write_str(dctx.strbuff.as_str());
    ctx.pop_cl_fg();

    // ████░░░░░░░░░░░
    // [####.........]
    // [■■■■□□□□□□□□□]
    //  ▁▂▃▄▅▆▇█ - for vertical ▂▄▆█
}

struct DrawListParams
{
    // Coord coord;
    // int16_t item_idx;
    // int16_t sel_idx;
    // int16_t items_cnt;
    // uint16_t items_visible;
    // uint16_t top_item;
    // bool focused;
    // uint8_t wgt_width;
    // uint8_t frame_size;
    // std::function<void(int16_t idx, String &out)> getItem;
}

fn draw_list(p: &DrawListParams)
{
  /*   if (p.items_cnt > p.items_visible)
    {
        drawListScrollBarV(p.coord + Coord{uint8_t(p.wgt_width-1), p.frame_size},
            p.items_visible, p.items_cnt-1, p.sel_idx);
    }

    flushBuffer();

    for (int i = 0; i < p.items_visible; i++)
    {
        bool is_current_item = p.items_cnt ? (p.top_item + i == p.item_idx) : false;
        bool is_sel_item = p.top_item + i == p.sel_idx;
        moveTo(p.coord.col + p.frame_size, p.coord.row + i + p.frame_size);

        g_ws.str.clear();

        if (p.top_item + i < p.items_cnt)
        {
            p.getItem(p.top_item + i, g_ws.str);
            g_ws.str.insert(0, is_current_item ? "►" : " ");
            g_ws.str.setWidth(p.wgt_width - 1 - p.frame_size, true);
        }
        else
        {
            // empty string - to erase old content
            g_ws.str.setWidth(p.wgt_width - 1 - p.frame_size);
        }

        if (p.focused && is_sel_item) pushAttr(FontAttrib::Inverse);
        if (is_current_item) pushAttr(FontAttrib::Underline);
        ctx.write_str(dctx.strbuff.as_str());
        if (is_current_item) popAttr();
        if (p.focused && is_sel_item) popAttr();
    } */
}

fn draw_list_box(dctx: &mut DrawCtx, prp: &prop::ListBox)
{
  /*   let _fm = FontMemento::new(&dctx.ctx);
    const auto my_coord = dctx.parentCoord + dctx.wgt->coord;
    drawArea(my_coord, dctx.wgt->size,
        dctx.wgt->listbox.bgColor, dctx.wgt->listbox.fgColor,
        dctx.wgt->listbox.noFrame ? FrameStyle::None : FrameStyle::ListBox, false);

    if (dctx.wgt->size.height < 3)
        return;

    DrawListParams dlp = {};
    dlp.coord = my_coord;
    dctx.pState->getListBoxState(dctx.wgt, dlp.item_idx, dlp.sel_idx, dlp.items_cnt);
    dlp.frame_size = !dctx.wgt->listbox.noFrame;
    dlp.items_visible = dctx.wgt->size.height - (dlp.frame_size * 2);
    dlp.top_item = (dlp.sel_idx / dlp.items_visible) * dlp.items_visible;
    dlp.focused = dctx.pState->isFocused(dctx.wgt);
    dlp.wgt_width = dctx.wgt->size.width;
    dlp.getItem = [dctx.wgt, &dctx](int16_t idx, String &out) { dctx.pState->getListBoxItem(dctx.wgt, idx, out); };
    drawList(dlp); */
}

fn draw_combo_box(dctx: &mut DrawCtx, prp: &prop::ComboBox)
{
/*     let _fm = FontMemento::new(&dctx.ctx);
    const auto my_coord = dctx.parentCoord + dctx.wgt->coord;
    const bool focused = dctx.pState->isFocused(dctx.wgt);

    int16_t item_idx = 0; int16_t sel_idx = 0; int16_t items_count; bool drop_down = false;
    dctx.pState->getComboBoxState(dctx.wgt, item_idx, sel_idx, items_count, drop_down);

    {
        g_ws.str.clear();
        dctx.pState->getComboBoxItem(dctx.wgt, item_idx, g_ws.str);
        g_ws.str.insert(0, " ");
        g_ws.str.setWidth(dctx.wgt->size.width - 4, true);
        g_ws.str << " [▼]";

        moveTo(my_coord.col, my_coord.row);
        pushClFg(getWidgetFgColor(dctx.wgt));
        pushClBg(getWidgetBgColor(dctx.wgt));
        if (focused && !drop_down) pushAttr(FontAttrib::Inverse);
        if (drop_down) pushAttr(FontAttrib::Underline);
        if (focused) pushAttr(FontAttrib::Bold);
        ctx.write_str(dctx.strbuff.as_str());
        if (focused) popAttr();
        if (drop_down) popAttr();
    }

    if (drop_down)
    {
        DrawListParams dlp = {};
        dlp.coord.col = my_coord.col;
        dlp.coord.row = my_coord.row+1;
        dlp.item_idx = item_idx;
        dlp.sel_idx = sel_idx;
        dlp.items_cnt = items_count;
        dlp.frame_size = 0;
        dlp.items_visible = dctx.wgt->combobox.dropDownSize;
        dlp.top_item = (dlp.sel_idx / dlp.items_visible) * dlp.items_visible;
        dlp.focused = focused;
        dlp.wgt_width = dctx.wgt->size.width;
        dlp.getItem = [dctx.wgt, &dctx](int16_t idx, String &out) { dctx.pState->getComboBoxItem(dctx.wgt, idx, out); };
        drawList(dlp);
    } */
}

fn draw_custom_wgt(dctx: &mut DrawCtx, _: &prop::CustomWgt)
{
    dctx.wnd_state.on_custom_widget_draw(dctx.wgt);
}

fn draw_text_box(dctx: &mut DrawCtx, prp: &prop::TextBox)
{
    let _fm = FontMemento::new(&dctx.ctx);
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    draw_area(&mut dctx.ctx.borrow_mut(), my_coord, dctx.wgt.size,
        prp.bg_color, prp.fg_color,
        FrameStyle::ListBox, false, false);

    if dctx.wgt.size.height < 3 {
        return;
    }

/*
    const uint8_t lines_visible = dctx.wgt->size.height - 2;
    const twins::Vector<twins::CStrView> *p_lines = nullptr;
    int16_t top_line = 0;

    dctx.pState->getTextBoxState(dctx.wgt, &p_lines, top_line);

    if (!p_lines || !p_lines->size())
        return;

    if (top_line > (int)p_lines->size())
    {
        top_line = p_lines->size() - lines_visible;
        dctx.pState->onTextBoxScroll(dctx.wgt, top_line);
    }

    if (top_line < 0)
    {
        dctx.pState->onTextBoxScroll(dctx.wgt, top_line);
        top_line = 0;
    }

    drawListScrollBarV(my_coord + Coord{uint8_t(dctx.wgt->size.width-1), 1},
        lines_visible, p_lines->size() - lines_visible, top_line);

    flushBuffer();

    // scan invisible lines for ESC sequences: colors, font attributes
    g_ws.str.clear();
    for (int i = 0; i < top_line; i++)
    {
        auto sr = (*p_lines)[i];
        while (const char *esc = twins::util::strnchr(sr.data, sr.size, '\e'))
        {
            auto esclen = String::escLen(esc, sr.data + sr.size);
            g_ws.str.appendLen(esc, esclen);

            sr.size -= esc - sr.data + 1;
            sr.data = esc + 1;
        }
    }
    ctx.write_str(dctx.strbuff.as_str());

    // draw lines
    for (int i = 0; i < lines_visible; i++)
    {
        g_ws.str.clear();
        if (top_line + i < (int)p_lines->size())
        {
            const auto &sr = (*p_lines)[top_line + i];
            g_ws.str.appendLen(sr.data, sr.size);
        }
        g_ws.str.setWidth(dctx.wgt->size.width - 2, true);
        moveTo(my_coord.col + 1, my_coord.row + i + 1);
        ctx.write_str(dctx.strbuff.as_str());
    }

    // flushBuffer();
    */
}

fn draw_layer(dctx: &mut DrawCtx, _: &prop::Layer)
{
    // draw only childrens; to erase, redraw layer's parent
    let layer = dctx.wgt;

    for wgt in WidgetIter::new(layer) {
        dctx.wgt = wgt;
        draw_widget_internal(dctx);
    }

    dctx.wgt = layer;
}

// -----------------------------------------------------------------------------------------------

const FRAME_NONE: [char; 9] =
[
    ' ', ' ', ' ',
    ' ', ' ', ' ',
    ' ', ' ', ' ',
];

const FRAME_SINGLE: [char; 9] =
[
    '┌', '─', '┐',
    '│', ' ', '│',
    '└', '─', '┘',
];

const FRAME_LISTBOX: [char; 9] =
[
    '┌', '─', '┐',
    '│', ' ', '▒',
    '└', '─', '┘',
];

const FRAME_PGCONTROL: [char; 9] =
[
    '├', '─', '┐',
    '│', ' ', '│',
    '├', '─', '┘',
];

const FRAME_DOUBLE: [char; 9] =
[
    '╔', '═', '╗',
    '║', ' ', '║',
    '╚', '═', '╝',
];

fn draw_area(ctx: &mut Ctx, coord: Coord, size: Size, cl_bg: ColorBG, cl_fg: ColorFG, style: FrameStyle, filled: bool, shadow: bool)
{
    ctx.move_to(coord.col.into(), coord.row.into());

    let frame = match style {
        FrameStyle::Single => &FRAME_SINGLE,
        FrameStyle::Double => &FRAME_DOUBLE,
        FrameStyle::PgControl => &FRAME_PGCONTROL,
        FrameStyle::ListBox => &FRAME_LISTBOX,
        _ => &FRAME_NONE
    };

    // background and frame color
    if cl_bg != ColorBG::Inherit { ctx.push_cl_bg(cl_bg); }
    if cl_fg != ColorFG::Inherit { ctx.push_cl_fg(cl_fg); }

    let mut strbuff = String::with_capacity(500);

    // top line
    strbuff.push(frame[0]);
    draw_line(&mut strbuff, frame[1], size.width);
    strbuff.push(frame[2]);

    ctx.write_str(strbuff.as_str());
    ctx.move_by(-(size.width as i16), 1);
    ctx.flush_buff();

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
        strbuff.push_str("█");
        strbuff.push_str(colors::encode_cl_fg(cl_fg));
    }

    for r in coord.row + 1 .. coord.row + size.height - 1 {
        ctx.write_str(strbuff.as_str());
        ctx.move_by(-(size.width as i16 + shadow as i16), 1);
        ctx.flush_buff();
    }

    // bottom line
    strbuff.clear();
    strbuff.push(frame[6]);
    draw_line(&mut strbuff, frame[7], size.width);
    strbuff.push(frame[8]);

    if shadow {
        // trailing shadow
        strbuff.push_str(esc::FG_BLACK);
        strbuff.push_str("█");
    }

    ctx.write_str(strbuff.as_str());
    ctx.flush_buff();

    if shadow {
        ctx.move_by(-(size.width as i16), 1);
        strbuff.clear();
        // trailing shadow
        draw_line(&mut strbuff, '█', size.width + 2);
        ctx.write_str(strbuff.as_str());
        ctx.write_str(colors::encode_cl_fg(cl_fg));
        ctx.flush_buff();
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

fn draw_list_scroll_bar_v(ctx: &mut Ctx, coord: Coord, height: i8, max: i32, pos: i32) {
    if pos > max {
        ctx.log_d(format!("pos ({}) > max ({})", pos, max).as_str());
        return;
    }

    let slider_at = (((height-1) as i32) * pos) / max;
    // "▲▴ ▼▾ ◄◂ ►▸ ◘ █";

    for i in 0..height {
        ctx.move_to(coord.col.into(), (coord.row as u16) + i as u16);
        ctx.write_str(if i as i32 == slider_at {"◘"} else {"▒"});
    }
}

fn get_widget_bg_color(wgt: &Widget) -> ColorBG {
    let mut cl = match wgt.typ {
        Type::Window(ref p) => p.bg_color,
        Type::Panel(ref p) => p.bg_color,
        Type::Label(ref p) => p.bg_color,
        Type::TextEdit(ref p) => p.bg_color,
        Type::Button(ref p) => p.bg_color,
        Type::ListBox(ref p) => p.bg_color,
        Type::ComboBox(ref p) => p.bg_color,
        Type::TextBox(ref p) => p.bg_color,
        _ => ColorBG::Inherit
    };

    if cl == ColorBG::Inherit {
        let parent = wgt_get_parent(wgt);
        cl = get_widget_bg_color(parent);
    }

    return cl;
}

fn get_widget_fg_color(wgt: &Widget) -> ColorFG {
    let mut cl = match wgt.typ {
        Type::Window(ref p) => p.fg_color,
        Type::Panel(ref p) => p.fg_color,
        Type::Label(ref p) => p.fg_color,
        Type::TextEdit(ref p) => p.fg_color,
        Type::CheckBox(ref p) => p.fg_color,
        Type::Radio(ref p) => p.fg_color,
        Type::Button(ref p) => p.fg_color,
        Type::Led(ref p) => p.fg_color,
        Type::Page(ref p) => p.fg_color,
        Type::ProgressBar(ref p) => p.fg_color,
        Type::ListBox(ref p) => p.fg_color,
        Type::ComboBox(ref p) => p.fg_color,
        Type::TextBox(ref p) => p.fg_color,
        _ => ColorFG::Inherit
    };

    if cl == ColorFG::Inherit {
        let parent = wgt_get_parent(wgt);
        cl = get_widget_fg_color(parent);
    }

    return cl;
}
