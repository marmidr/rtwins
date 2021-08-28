//! # RTwins demo app

extern crate rtwins;
use rtwins::{esc, TWins};
use std::{io::Write, ops::DerefMut};

/// Simple widget-based interface definition as const
#[rustfmt::skip]
mod tui {
    use rtwins::colors::{ColorBG, ColorFG};
    use rtwins::widget::WIDGET_ID_NONE;
    use rtwins::{wp, Coord, Size, Type, Widget};

    #[allow(dead_code)]
    pub enum Id {
        WndMain = rtwins::WIDGET_ID_NONE as isize + 1,
        Lbl1,
        Lbl2,
        PnlGreen,
        BtnOk,
        BtnCancel,
        PnlWhite,
    }

    /// Easy conversion from enum to Wid
    impl Id {
        const fn into(self) -> rtwins::WId {
            self as rtwins::WId
        }
    }

    pub const NO_CHILDS: [Widget; 0] = [];

    pub const WINDOW: Widget = Widget {
        id: Id::WndMain.into(),
        parent: WIDGET_ID_NONE,
        coord: Coord { col: 1, row: 2 },
        size: Size {
            width: 25,
            height: 12,
        },
        typ: wp::Window {
            title: "** DEMO **",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Blue,
            is_popup: false,
        }.into(),
        // link: &[]
        link: &[
            Widget {
                id: Id::Lbl1.into(),
                parent: rtwins::WIDGET_ID_NONE,
                coord: Coord::cdeflt(),
                size: Size::cdeflt(),
                typ: wp::Label {
                    title: "Name",
                    fg_color: ColorFG::White,
                    bg_color: ColorBG::Blue,
                }.into(),
                link: &NO_CHILDS,
            },
            Widget {
                id: 2,
                parent: rtwins::WIDGET_ID_NONE,
                coord: Coord::cdeflt(),
                size: Size::cdeflt(),
                typ: Type::NoWgt,
                link: &[],
            },
        ],
    };
}

/// Example of const-evaluated and translated Widgets tree into Widgets array
const DEMO_WND: [rtwins::Widget; rtwins::wgt_count(&tui::WINDOW)] = rtwins::wgt_translate(&tui::WINDOW);


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
    fn write_char(&mut self, c: char) {
        self.line_buff.push(c);
    }

    fn write_char_n(&mut self, c: char, repeat: i16) {
        for _ in 0..repeat {
            self.line_buff.push(c);
        }
    }

    fn write_str(&mut self, s: &str) {
        self.line_buff.push_str(s);
    }

    fn write_str_n(&mut self, s: &str, repeat: i16) {
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

    fn get_logs_row(&mut self) -> u16 {
        0
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
        let to_invalidate = [1, 2, 3];
        let mut ctx = tw.lock();
        ctx.invalidate(&DEMO_WND[0], &to_invalidate);
        ctx.invalidate(&DEMO_WND[0], &[1, 2, 3]);
        ctx.draw_wnd(&DEMO_WND[0]);

        let c = ctx.deref_mut();
        c.move_to_col(20);
        c.log_w("Collumn 20");
        c.pal.write_str("\n");
        c.pal.flush_buff();
    }

    let w_none = rtwins::Widget {
        id: 0,
        parent: rtwins::WIDGET_ID_NONE,
        coord: rtwins::Coord::cdeflt(),
        size: rtwins::Size::cdeflt(),
        typ: rtwins::Type::NoWgt,
        link: &tui::NO_CHILDS,
    };

    println!("w_none childs: {}", w_none.link.len());
    println!("w_none widgets: {}", rtwins::wgt_count(&w_none));
    println!("WINDOW childs: {}", tui::WINDOW.link.len());

    let title = |wgt: &rtwins::Widget| match wgt.typ {
        rtwins::Type::Window(ref wp) => wp.title,
        _ => "<?>",
    };

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
fn wnd_prop(wgt: &rtwins::Widget) -> &rtwins::widget::wp::Window {
    match wgt.typ {
        rtwins::Type::Window(ref wp) => wp,
        _ => panic!(),
    }
}

