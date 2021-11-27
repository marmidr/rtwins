//! # RTwins demo app

#![allow(unused_variables)]

extern crate rtwins;
use rtwins::TWins;
use rtwins::esc;
use rtwins::widget::*;
use rtwins::input::*;
use std::{io::Write, ops::{DerefMut}};
use std::collections::HashMap;

/// Simple widget-based interface definition as const
#[rustfmt::skip]
mod tui {

    // https://doc.rust-lang.org/cargo/guide/project-layout.html

    use rtwins::colors::{ColorBG, ColorFG};
    use rtwins::{prop, Coord, Size, Widget, Link, ButtonStyle};

    #[allow(dead_code)]
    pub enum Id {
        WndMain = rtwins::WIDGET_ID_NONE as isize + 1,
        Lbl1,
        Lbl2,
        PnlGreen,
        BtnOk,
        BtnCancel,
        PnlYellow,
    }

    /// Easy conversion from enum to Wid
    impl Id {
        pub const fn into(self) -> rtwins::WId {
            self as rtwins::WId
        }
    }

    pub const WINDOW: Widget = Widget {
        id: Id::WndMain.into(),
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 2 },
        size: Size {
            width: 25,
            height: 12,
        },
        typ: prop::Window {
            title: "** DEMO **",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Blue,
            is_popup: false,
        }.into(),
        childs: &[
            Widget {
                id: Id::PnlGreen.into(),
                link: Link::cdeflt(),
                coord: Coord::cdeflt(),
                size: Size::cdeflt(),
                typ: prop::Panel {
                    title: "Panel green",
                    fg_color: ColorFG::White,
                    bg_color: ColorBG::Green,
                    no_frame: false,
                }.into(),
                childs: &[
                    Widget {
                        id: Id::Lbl1.into(),
                        link: Link::cdeflt(),
                        coord: Coord::cdeflt(),
                        size: Size::cdeflt(),
                        typ: prop::Label {
                            title: "Label-1",
                            fg_color: ColorFG::White,
                            bg_color: ColorBG::Blue,
                        }.into(),
                        childs: &[],
                    },
                ],
            },
            Widget {
                id: Id::Lbl2.into(),
                link: Link::cdeflt(),
                coord: Coord::cdeflt(),
                size: Size::cdeflt(),
                typ: prop::Label {
                    title: "Label-2",
                    fg_color: ColorFG::Cyan,
                    bg_color: ColorBG::Black,
                }.into(),
                childs: &[],
            },
            Widget {
                id: Id::PnlYellow.into(),
                link: Link::cdeflt(),
                coord: Coord::cdeflt(),
                size: Size::cdeflt(),
                typ: prop::Panel {
                    title: "Panel yellow",
                    fg_color: ColorFG::Yellow,
                    bg_color: ColorBG::Green,
                    no_frame: false,
                }.into(),
                childs: &[
                    Widget {
                        id: Id::BtnCancel.into(),
                        link: Link::cdeflt(),
                        coord: Coord::cdeflt(),
                        size: Size::cdeflt(),
                        typ: prop::Button {
                            text: "Cancel",
                            fg_color: ColorFG::White,
                            bg_color: ColorBG::Blue,
                            style: ButtonStyle::Solid
                        }.into(),
                        childs: &[],
                    },
                ],
            },
        ],
    };
}

/// Example of const-evaluated and translated Widgets tree into Widgets array
const DEMO_WND: [rtwins::Widget; rtwins::wgt_count(&tui::WINDOW)] = rtwins::wgt_transform_array(&tui::WINDOW);

// -----------------------------------------------------------------------------------------------

struct DemoWndState {
    wnd: &'static [rtwins::Widget],
    wprop: HashMap<rtwins::WId, WidgetMutProp>,
    focused_id: WId,
}

impl DemoWndState {
    pub fn new(wnd: &'static [rtwins::Widget]) -> Self {
        let mut ws = DemoWndState{wnd,
            wprop: HashMap::new(),
            focused_id: WIDGET_ID_NONE
        };

        use tui::Id;
        ws.wprop.insert(Id::Lbl2.into(), WidgetMutProp::new());
        ws.wprop.get_mut(&Id::Lbl2.into()).unwrap().enabled = false;
        return ws;
    }
}

impl WindowState for DemoWndState {
    fn get_widgets(&self) -> &'static [Widget] {
        self.wnd
    }

    /** events **/

    fn on_button_down(&mut self, wgt: &Widget, kc: &KeyCode) {

    }
    fn on_button_up(&mut self, wgt: &Widget, kc: &KeyCode) {

    }
    fn on_button_click(&mut self, wgt: &Widget, kc: &KeyCode) {

    }
    fn on_text_edit_change(&mut self, wgt: &Widget, txt: &mut String) {

    }
    fn on_text_edit_input_evt(&mut self, wgt: &Widget, kc: &KeyCode, txt: &mut String, cursor_pos: &mut i16) -> bool {
        return false;
    }
    fn on_checkbox_toggle(&mut self, wgt: &Widget) {

    }
    fn on_page_control_page_change(&mut self, wgt: &Widget, new_page_idx: u8) {

    }
    fn on_list_box_select(&mut self, wgt: &Widget, sel_idx: i16) {

    }
    fn on_list_box_change(&mut self, wgt: &Widget, new_idx: i16) {

    }
    fn on_combo_box_select(&mut self, wgt: &Widget, sel_idx: i16) {

    }
    fn on_combo_box_change(&mut self, wgt: &Widget, new_idx: i16) {

    }
    fn on_combo_box_drop(&mut self, wgt: &Widget, drop_state: bool) {

    }
    fn on_radio_select(&mut self, wgt: &Widget) {

    }
    fn on_text_box_scroll(&mut self, wgt: &Widget, top_line: i16) {

    }
    fn on_custom_widget_draw(&mut self, wgt: &Widget) {

    }
    fn on_custom_widget_input_evt(&mut self, wgt: &Widget, kc: &KeyCode) -> bool {
        return false;
    }
    fn on_window_unhandled_input_evt(&mut self, wgt: &Widget, kc: &KeyCode) -> bool {
        return false;
    }

    /** common state queries **/

    fn is_enabled(&mut self, wgt: &Widget) -> bool {
        if let Some(ref p) = self.wprop.get(&wgt.id) {
            return p.enabled;
        }

        true
    }
    fn is_focused(&mut self, wgt: &Widget) -> bool {
        self.focused_id == wgt.id
    }
    fn is_visible(&mut self, wgt: &Widget) -> bool {
        true
    }
    fn get_focused_id(&mut self) -> WId {
        self.focused_id
    }
    fn set_focused_id(&mut self, wid: WId) {
        self.focused_id = wid;
    }

    /** widget-specific queries; all mutable params are outputs **/

    fn get_window_coord(&mut self, wgt: &Widget, coord: &mut Coord) {

    }
    fn get_window_title(&mut self, wgt: &Widget, title: &mut String) {

    }
    fn get_checkbox_checked(&mut self, wgt: &Widget) -> bool {
        return false;
    }
    fn get_label_text(&mut self, wgt: &Widget, txt: &mut String) {

    }
    fn get_text_edit_text(&mut self, wgt: &Widget, txt: &mut String, edit_mode: bool) {

    }
    fn get_led_lit(&mut self, wgt: &Widget) -> bool {
        return false;
    }
    fn get_led_text(&mut self, wgt: &Widget, txt: &mut String) {

    }
    fn get_progress_bar_state(&mut self, wgt: &Widget, pos: &mut i32, max: &mut i32) {

    }
    fn get_page_ctrl_page_index(&mut self, wgt: &Widget) -> i8 {
        return 0;
    }
    fn get_list_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16) {

    }
    fn get_list_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {

    }
    fn get_combo_box_state(&mut self, wgt: &Widget, item_idx: &mut i16, sel_idx: &mut i16, items_count: &mut i16, drop_down: &mut bool) {

    }
    fn get_combo_box_item(&mut self, wgt: &Widget, item_idx: &mut i16, txt: &mut String) {

    }
    fn get_radio_index(&mut self, wgt: &Widget) -> i32 {
        return -1;
    }
    fn get_text_box_state(&mut self, wgt: &Widget, lines: &[&str], top_line: &mut i16) {

    }
    fn get_button_text(&mut self, wgt: &Widget, txt: &mut String) {

    }
}

// -----------------------------------------------------------------------------------------------

struct DemoPal {
    line_buff: String,
    logging: bool,
    started_at: std::time::Instant,
}

impl DemoPal {
    fn new() -> Self {
        DemoPal {
            line_buff: String::with_capacity(1000),
            logging: false,
            started_at: std::time::Instant::now(),
        }
    }
}

impl rtwins::pal::Pal for DemoPal {
    fn write_char_n(&mut self, c: char, repeat: i16) {
        for _ in 0..repeat {
            self.line_buff.push(c);
        }
    }

    fn write_str_n(&mut self, s: &str, repeat: i16) {
        self.line_buff.reserve(s.len() * repeat as usize);

        for _ in 0..repeat {
            self.line_buff.push_str(s);
        }
    }

    fn flush_buff(&mut self) {
        std::io::stdout()
            .lock()
            .write(self.line_buff.as_bytes())
            .expect("Error writing to stdout");
        self.line_buff.clear();
    }

    fn set_logging(&mut self, on: bool) {
        self.logging = on;
    }

    fn sleep(&mut self, ms: u16) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }

    fn get_time_stamp(&mut self) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32
    }

    fn get_time_diff(&mut self, prev_timestamp: u32) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32 - prev_timestamp
    }
}

// -----------------------------------------------------------------------------------------------

fn main()
{
    println!(
        "** {}{}{} ** demo; lib v{}{}{}",
        esc::BOLD,
        rtwins::link!("https://github.com/marmidr/rtwins", "RTWins"),
        esc::NORMAL,
        esc::FG_HOT_PINK,
        rtwins::VER,
        esc::FG_DEFAULT
    );
    println!(
        "{}Faint{} {}Bold{} {}Italic{}",
        esc::FAINT,
        esc::NORMAL,
        esc::BOLD,
        esc::NORMAL,
        esc::ITALICS_ON,
        esc::ITALICS_OFF
    );

    {
        let mut ws = DemoWndState::new(&DEMO_WND[..]);

        let mut tw = TWins::new(Box::new(DemoPal::new()));
        let mut ctx = tw.lock();
        use tui::Id::*;
        ctx.invalidate(&DEMO_WND[0],
            &[Lbl1.into(), BtnOk.into(), Lbl2.into()]
        );
        ctx.draw_wnd(&DEMO_WND[0]);

        let c = ctx.deref_mut();
        c.move_to_col(10).log_w("Column 10");
        c.write_char('\n').flush_buff();
    }

    let title = |wgt: &rtwins::Widget| match wgt.typ {
        rtwins::Type::Window(ref wp) => wp.title,
        _ => "<?>",
    };

    for (idx, w) in DEMO_WND.iter().enumerate() {
        let w_par = rtwins::wgt_get_parent(&DEMO_WND, w);
        println!("  {}. {}:{}, idx:{}, chidx:{}, parid {}:{}", idx, w.id, w.typ, w.link.own_idx, w.link.childs_idx, w_par.id, w_par.typ);
    }

    println!("WINDOW title: {}", title(&tui::WINDOW));
    println!("WINDOW title: {}", wnd_prop(&tui::WINDOW).title);
    println!("WINDOW title: {}", tui::WINDOW.typ.prop_wnd().title);
    println!("WINDOW widgets: {}", rtwins::wgt_count(&tui::WINDOW));
    println!(
        "sizeof Widget: {}",
        std::mem::size_of::<rtwins::widget::Widget>()
    );
    println!(
        "sizeof Type: {}",
        std::mem::size_of::<rtwins::widget::Type>()
    );
    println!("sizeof Id: {}", std::mem::size_of::<tui::Id>());

    if let rtwins::Type::Window(ref wp) = tui::WINDOW.typ {
        println!("WINDOW title: {}", wp.title);
    }
}

/// Extract window properties from enum
fn wnd_prop(wgt: &rtwins::Widget) -> &rtwins::widget::prop::Window {
    match wgt.typ {
        rtwins::Type::Window(ref wp) => wp,
        _ => panic!(),
    }
}

