//! RTwins demo app

extern crate rtwins;
use rtwins::esc;
use std::io::Write;

/// Simple widget-based interface definition as const
mod tui
{
use rtwins::{wp, Coord, Size, Type, Widget};
use rtwins::colors::{ColorBG, ColorFG};
use rtwins::widget::WIDGET_ID_NONE;

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
    const fn into(self) -> rtwins::WId { self as rtwins::WId }
}

pub const NO_CHILDS: [Widget; 0] = [];

pub const WINDOW: Widget = Widget {
    id : Id::WndMain.into(),
    parent: WIDGET_ID_NONE,
    coord: Coord{col: 1, row: 2},
    size: Size{width: 25, height: 12},
    typ: wp::Window {
        title   : "** DEMO **",
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
                title   : "Name",
                fg_color: ColorFG::White,
                bg_color: ColorBG::Blue,
            }.into(),
            link: &NO_CHILDS
        },
        Widget {
            id: 2,
            parent: rtwins::WIDGET_ID_NONE,
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: Type::NoWgt,
            link: &[]
        },
    ]
};

}

struct Pal {
    line_buff: String,
    logging: bool,
    mtx: std::sync::Mutex<()>,
    started_at: std::time::Instant,
}

impl Pal {
    fn new() -> Self {
        Pal{line_buff: String::with_capacity(1000),
            logging: false,
            mtx: std::sync::Mutex::default(),
            started_at: std::time::Instant::now(),
        }
    }
}

impl rtwins::pal::Pal for Pal
{
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
        std::io::stdout().lock().write(self.line_buff.as_bytes()).expect("Error writing to stdout");
        self.line_buff.clear();
    }

    fn set_logging(&mut self, on: bool) {
        self.logging = on;
    }

    fn sleep(&mut self, ms: u16) {
        std::thread::sleep( std::time::Duration::from_millis(ms as u64));
    }

    fn get_logs_row(&mut self, ) -> u16 {
        0
    }

    fn get_time_stamp(&mut self, ) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32
    }

    fn get_time_diff(&mut self, prev_timestamp: u32) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32 - prev_timestamp
    }

    fn lock(&mut self, wait: bool) -> bool {
        if wait {
            let lr = self.mtx.lock();
            lr.is_ok()
        }
        else {
            self.mtx.try_lock().is_ok()
        }
    }
}

// -----------------------------------------------------------------------------------------------

fn main()
{
    let _pal = Pal::new();
    rtwins::init();
    println!("RTWins demo; lib v{}", rtwins::VER);
    println!("Normal {}Bold{} {}Italic{}", esc::BOLD, esc::NORMAL, esc::ITALICS_ON, esc::ITALICS_OFF);

    let w_none = rtwins::Widget {
        id      : 0,
        parent  : rtwins::WIDGET_ID_NONE,
        coord   : rtwins::Coord::cdeflt(),
        size    : rtwins::Size::cdeflt(),
        typ     : rtwins::Type::NoWgt,
        link    : &tui::NO_CHILDS
    };

    println!("w_none childs: {}", w_none.link.len() );
    println!("w_none widgets: {}", rtwins::wgt_count(&w_none));
    println!("WINDOW childs: {}", tui::WINDOW.link.len() );

    let title = |wgt: &rtwins::Widget| match wgt.typ {
        rtwins::Type::Window(ref wp) => wp.title,
        _                            => "<?>"
    };

    println!("WINDOW title: {}", title(&tui::WINDOW) );
    println!("WINDOW title: {}", wnd_prop(&tui::WINDOW).title );
    println!("WINDOW title: {}", tui::WINDOW.typ.prop_wnd().title );
    println!("WINDOW widgets: {}", rtwins::wgt_count(&tui::WINDOW) );
    println!("sizeof Widget: {}", std::mem::size_of::<rtwins::widget::Widget>());
    println!("sizeof Type: {}", std::mem::size_of::<rtwins::widget::Type>());
    println!("sizeof Id: {}", std::mem::size_of::<tui::Id>());

    if let rtwins::Type::Window(ref wp) = tui::WINDOW.typ {
        println!("WINDOW title: {}", wp.title );
    }
}

/// Extract window properties from enum
fn wnd_prop(wgt: &rtwins::Widget) -> &rtwins::widget::wp::Window {
    match wgt.typ {
        rtwins::Type::Window(ref wp) => wp,
        _ => panic!()
    }
}

/// Example of const-evaluated and translated Widgets tree into Widgets array
const _W: [rtwins::Widget; rtwins::wgt_count(&tui::WINDOW)] = rtwins::wgt_translate(&tui::WINDOW);
