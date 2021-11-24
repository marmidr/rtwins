//! # RTwins demo app

extern crate rtwins;
use rtwins::{esc, TWins};
use std::{io::Write, ops::DerefMut};

/// Simple widget-based interface definition as const
#[rustfmt::skip]
mod tui {
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
        let mut tw = TWins::new(Box::new(DemoPal::new()));
        let mut ctx = tw.lock();
        {
            use tui::Id::*;
            ctx.invalidate(&DEMO_WND[0],
                &[Lbl1.into(), BtnOk.into(), Lbl2.into()]
            );
        }
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

