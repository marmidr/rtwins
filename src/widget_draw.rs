//! # RTWins Widget drawing

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;

use crate::{FontMementoManual, FontMemento, FontAttrib, colors};
use crate::widget_impl::*;
use crate::widget::*;
use crate::colors::*;
use crate::Ctx;
use crate::string_ext::*;
use crate::esc;

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
    /// Reference to the widgets array
    wnd_widgets: &'a [Widget],
    /// Common string buffer for entire drawing session
    strbuff: String
}

/// Draw `wids` widgets of the given window.
/// If `wids` contains only `WIDGET_ID_ALL`, draw entire window
pub fn draw_widgets(ctx: &mut Ctx, ws: &mut dyn WindowState, wids: &[WId])
{
    if wids.len() == 0 {
        return;
    }

    let mut fm = FontMementoManual::from_ctx(ctx);
    // TODO: g_ws.pFocusedWgt = getWidgetByWID(dctx, ctx.stat.getFocusedID());

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
    // TODO: setCursorAt(dctx, g_ws.pFocusedWgt);
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
        let title_width = wnd_title.as_str().ansi_displayed_width() as u16 + 4;
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

        for wgt in wnd.iter() {
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
    let mut fm = FontMementoManual::from_ctx(&dctx.ctx.borrow());
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
        let title_width = prp.title.ansi_displayed_width() as u16;
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

        for wgt in pnl.iter() {
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
    let col = dctx.parent_coord.col as u16 + dctx.wgt.coord.col as u16;
    let row = dctx.parent_coord.row as u16 + dctx.wgt.coord.row as u16;
    ctx.move_to(col, row);

    let max_lines = if dctx.wgt.size.height > 0 { dctx.wgt.size.height } else { 50 };
    let line_width = dctx.wgt.size.width;

    for (line, s) in title.lines().enumerate() {
        dctx.strbuff.clear();
        dctx.strbuff.push_str(s);

        if line_width > 0 {
            dctx.strbuff.set_displayed_width(line_width as i16);
        }

        ctx.write_str(dctx.strbuff.as_str());
        ctx.move_to(col, row + 1 + line as u16);
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

        let shadow_len = 2 + txt.ansi_displayed_width() as i16;

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
        let _fm = FontMemento::new(&dctx.ctx);
        dctx.strbuff.push_str(" ");
        dctx.strbuff.push_str(txt.as_str());
        dctx.strbuff.push_str(" ");

        let clbg = get_widget_bg_color(dctx.wgt);
        let clparbg = get_widget_bg_color(wgt_get_parent(dctx.wgt));
        let bnt_len = 2 + txt.ansi_displayed_width() as i16;
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
    let mut fm = FontMementoManual::from_ctx(&dctx.ctx.borrow());
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
        dctx.strbuff.set_displayed_width(prp.tab_width as i16);

        let mut ctx = dctx.ctx.borrow_mut();
        ctx.move_to(my_coord.col as u16, my_coord.row as u16 + prp.vert_offs as u16);
        ctx.push_attr(FontAttrib::Inverse);
        ctx.write_str(dctx.strbuff.as_str());
        ctx.pop_attr();
    }

    dctx.ctx.borrow_mut().flush_buff();

    // draw tabs and pages
    {
        let pgctrl = dctx.wgt;
        let cur_pg_idx = dctx.wnd_state.get_page_ctrl_page_index(pgctrl) as usize;
        let focused = dctx.wnd_state.is_focused(pgctrl);
        dctx.parent_coord.col += prp.tab_width;

        for (idx, page) in pgctrl.iter().enumerate() {
            // check if page is below lower border
            if idx as i16 == pgctrl.size.height as i16 - 1 - prp.vert_offs as i16 {
                break;
            }

            let page_prp = match page.prop {
                Property::Page(ref p) => p,
                _ => panic!()
            };

            // draw tab title
            dctx.strbuff.clear();

            if idx == cur_pg_idx {
                dctx.strbuff.push_str("►");
            }
            else {
                dctx.strbuff.push_str(" ");
            }

            dctx.strbuff.push_str(page_prp.title);
            dctx.strbuff.set_displayed_width(prp.tab_width as i16);

            // for Page we do not want inherit after it's title color
            {
                let mut clfg = page_prp.fg_color;
                if clfg == ColorFG::Inherit { clfg = get_widget_fg_color(page); }
                let mut ctx = dctx.ctx.borrow_mut();
                ctx.move_to(
                    my_coord.col as u16,
                    my_coord.row as u16 + prp.vert_offs as u16 + idx as u16 + 1);
                ctx.push_cl_fg(clfg);
                if idx == cur_pg_idx { ctx.push_attr(FontAttrib::Inverse); }
                ctx.write_str(dctx.strbuff.as_str());
                if idx == cur_pg_idx { ctx.pop_attr(); }
                ctx.pop_cl_fg();
            }

            if idx == cur_pg_idx && dctx.wnd_state.is_visible(page) {
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

        if let Property::PageCtrl(ref pgctrl_prp) = pgctrl.prop {
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

        for wgt in page.iter() {
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

fn draw_list_box(dctx: &mut DrawCtx, prp: &prop::ListBox)
{
    let mut fm = FontMementoManual::from_ctx(&dctx.ctx.borrow());
    let my_coord = dctx.parent_coord + dctx.wgt.coord;

    draw_area(&mut dctx.ctx.borrow_mut(),
        my_coord,
        dctx.wgt.size,
        prp.bg_color, prp.fg_color,
        if prp.no_frame {FrameStyle::None} else {FrameStyle::ListBox},
        false, false);

    if dctx.wgt.size.height < 3 {
        return;
    }

    let mut dlp = DrawListParams::default();
    dlp.coord = my_coord;
    dctx.wnd_state.get_list_box_state(dctx.wgt,
        &mut dlp.item_idx, &mut dlp.sel_idx, &mut dlp.items_cnt);
    dlp.frame_size = !prp.no_frame as u8;
    dlp.items_visible = dctx.wgt.size.height as i16 - (dlp.frame_size as i16 * 2);
    dlp.top_item = (dlp.sel_idx / dlp.items_visible) * dlp.items_visible;
    dlp.focused = dctx.wnd_state.is_focused(dctx.wgt);
    dlp.wgt_width = dctx.wgt.size.width;

    // destructure dctx so the closure will capture local variables, not entire struct
    let wgt = dctx.wgt;
    let ws = &mut dctx.wnd_state;
    let mut ctx = dctx.ctx.borrow_mut();

    let getitem_cb = |idx, out: &mut String| {
        ws.get_list_box_item(wgt, idx, out);
    };

    draw_list(&mut ctx, &dlp, getitem_cb);
    fm.restore(&mut ctx);
}

fn draw_combo_box(dctx: &mut DrawCtx, prp: &prop::ComboBox)
{
    let _fm = FontMemento::new(&dctx.ctx);
    let my_coord = dctx.parent_coord + dctx.wgt.coord;
    let focused = dctx.wnd_state.is_focused(dctx.wgt);

    let mut item_idx = 0i16;
    let mut sel_idx = 0i16;
    let mut items_count = 0i16;
    let mut drop_down = false;
    dctx.wnd_state.get_combo_box_state(dctx.wgt, &mut item_idx, &mut sel_idx, &mut items_count, &mut drop_down);

    {
        dctx.strbuff.clear();
        dctx.wnd_state.get_combo_box_item(dctx.wgt, item_idx, &mut dctx.strbuff);
        dctx.strbuff.insert(0, ' ');
        dctx.strbuff.set_displayed_width(dctx.wgt.size.width as i16 - 4);//, true);
        dctx.strbuff.push_str(" [▼]");

        let mut ctx = dctx.ctx.borrow_mut();
        ctx.move_to(my_coord.col as u16, my_coord.row as u16);
        ctx.push_cl_fg(get_widget_fg_color(dctx.wgt));
        ctx.push_cl_bg(get_widget_bg_color(dctx.wgt));
        if focused && !drop_down { ctx.push_attr(FontAttrib::Inverse); }
        if drop_down { ctx.push_attr(FontAttrib::Underline); }
        if focused { ctx.push_attr(FontAttrib::Bold); }
        ctx.write_str(dctx.strbuff.as_str());
        if focused { ctx.pop_attr(); }
        if drop_down { ctx.pop_attr(); }
    }

    if drop_down {
        let mut dlp = DrawListParams::default();
        dlp.coord = my_coord;
        dlp.coord.row += 1;
        dlp.item_idx = item_idx;
        dlp.sel_idx = sel_idx;
        dlp.item_idx = items_count;
        dlp.frame_size = 0;
        dlp.items_cnt = items_count;
        dlp.items_visible = prp.drop_down_size as i16;
        dlp.top_item = (dlp.sel_idx / dlp.items_visible) * dlp.items_visible;
        dlp.focused = focused;
        dlp.wgt_width = dctx.wgt.size.width;

        // destructure dctx so the closure will capture local variables, not entire struct
        let wgt = dctx.wgt;
        let ws = &mut dctx.wnd_state;
        let mut ctx = dctx.ctx.borrow_mut();

        let getitem_cb = |idx, out: &mut String| {
            ws.get_combo_box_item(wgt, idx, out);
        };

        draw_list(&mut ctx, &dlp, getitem_cb);
    }
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

    let lines_visible = dctx.wgt.size.height as i16 - 2;
    let mut top_line = 0i16;
    let mut lines_opt_rc: Option<std::rc::Rc<Vec<String>>> = None;
    dctx.wnd_state.get_text_box_state(dctx.wgt, &mut lines_opt_rc, &mut top_line);

    if let Some(ref lines_rc) = lines_opt_rc {
        if top_line > lines_rc.len() as i16 {
            top_line = lines_rc.len() as i16 - lines_visible;
            dctx.wnd_state.on_text_box_scroll(dctx.wgt, top_line);
        }

        if top_line < 0 {
            dctx.wnd_state.on_text_box_scroll(dctx.wgt, top_line);
            top_line = 0;
        }

        let mut ctx = dctx.ctx.borrow_mut();

        draw_list_scroll_bar_v(&mut ctx,
            my_coord + Coord::new(dctx.wgt.size.width-1, 1),
            lines_visible, lines_rc.len() as i16 - lines_visible, top_line);

        ctx.flush_buff();

        // scan invisible lines for ESC sequences: colors, font attributes
        dctx.strbuff.clear();

        for i in 0..top_line {
            // TODO:
            // let line = lines_rc.get(i as usize).unwrap();
            // while (const char *esc = twins::util::strnchr(sr.data, sr.size, '\e'))
            // {
            //     auto esclen = String::escLen(esc, sr.data + sr.size);
            //     dctx.strbuff. .appendLen(esc, esclen);

            //     sr.size -= esc - sr.data + 1;
            //     sr.data = esc + 1;
            // }
        }

        ctx.write_str(dctx.strbuff.as_str());

        // draw lines
        for i in 0..lines_visible {
            dctx.strbuff.clear();

            if top_line + i < lines_rc.len() as i16 {
                // TODO:
                // let &sr = (*lines_rc)[top_line + i];
                // dctx.strbuff.push_str() .appendLen(sr.data, sr.size);
            }
            dctx.strbuff.set_displayed_width(dctx.wgt.size.width as i16 - 2);//, true);
            ctx.move_to(my_coord.col as u16 + 1, my_coord.row as u16 + i as u16 + 1);
            ctx.write_str(dctx.strbuff.as_str());
        }

        ctx.flush_buff();
    }
}

fn draw_layer(dctx: &mut DrawCtx, _: &prop::Layer)
{
    // draw only childrens; to erase, redraw layer's parent
    let layer = dctx.wgt;

    for wgt in layer.iter() {
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

fn draw_list_scroll_bar_v(ctx: &mut Ctx, coord: Coord, height: i16, pos_max: i16, pos: i16) {
    if pos > pos_max {
        // ctx.log_d(format!("pos ({}) > max ({})", pos, pos_max).as_str());
        return;
    }

    let slider_at = ((height-1) * pos) / pos_max;
    // "▲▴ ▼▾ ◄◂ ►▸ ◘ █";

    for i in 0..height {
        ctx.move_to(coord.col.into(), (coord.row as u16) + i as u16);
        ctx.write_char(if i == slider_at {'◘'} else {'▒'});
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

fn draw_list<Cb: FnMut(i16, &mut String)>(ctx: &mut Ctx, dlp: &DrawListParams, mut get_item: Cb)
{
    if dlp.items_cnt > dlp.items_visible {
        draw_list_scroll_bar_v(ctx,
            dlp.coord + Coord::new(dlp.wgt_width-1, dlp.frame_size),
            dlp.items_visible, dlp.items_cnt -1, dlp.sel_idx);
    }

    ctx.flush_buff();
    let mut strbuff = String::with_capacity(50);

    for i in 0..dlp.items_visible {
        let is_current_item = if dlp.items_cnt > 0 { dlp.top_item + i == dlp.item_idx } else { false };
        let is_sel_item = dlp.top_item + i == dlp.sel_idx;
        ctx.move_to(
            dlp.coord.col as u16 + dlp.frame_size as u16,
            dlp.coord.row as u16 + i as u16 + dlp.frame_size as u16);

        strbuff.clear();
        if dlp.top_item + i < dlp.items_cnt {
            get_item(dlp.top_item + i, &mut strbuff);
            strbuff.insert(0, if is_current_item {'►'} else {' '});
            strbuff.set_displayed_width(dlp.wgt_width as i16 - 1 - dlp.frame_size as i16);
        }
        else {
            // empty string - to erase old content
            strbuff.set_displayed_width(dlp.wgt_width as i16 - 1 - dlp.frame_size as i16);
        }

        if dlp.focused && is_sel_item { ctx.push_attr(FontAttrib::Inverse); }
        if is_current_item { ctx.push_attr(FontAttrib::Underline); }
        ctx.write_str(strbuff.as_str());
        if is_current_item { ctx.pop_attr(); }
        if dlp.focused && is_sel_item { ctx.pop_attr(); }
    }
}

fn get_widget_bg_color(wgt: &Widget) -> ColorBG {
    let mut cl = match wgt.prop {
        Property::Window(ref p) => p.bg_color,
        Property::Panel(ref p) => p.bg_color,
        Property::Label(ref p) => p.bg_color,
        Property::TextEdit(ref p) => p.bg_color,
        Property::Button(ref p) => p.bg_color,
        Property::ListBox(ref p) => p.bg_color,
        Property::ComboBox(ref p) => p.bg_color,
        Property::TextBox(ref p) => p.bg_color,
        _ => ColorBG::Inherit
    };

    if cl == ColorBG::Inherit {
        let parent = wgt_get_parent(wgt);
        if parent.id != wgt.id {
            cl = get_widget_bg_color(parent);
        }
    }

    return cl;
}

fn get_widget_fg_color(wgt: &Widget) -> ColorFG {
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
        _ => ColorFG::Inherit
    };

    if cl == ColorFG::Inherit {
        let parent = wgt_get_parent(wgt);
        if parent.id != wgt.id {
            cl = get_widget_fg_color(parent);
        }
    }

    return cl;
}
