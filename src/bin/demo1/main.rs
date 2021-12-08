//! # RTwins demo app

#![allow(unused_variables)]

extern crate rtwins;
use rtwins::{TWins, widget::WindowState};
use std::{io::Write, ops::DerefMut};

// https://doc.rust-lang.org/cargo/guide/project-layout.html
mod tui_def;
mod tui_state;

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
            logging: true,
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
        if self.logging {
            // write to logs.txt
        }

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

fn main() {
    test_esc_codes();
    test_property_access();

    let mut dws = tui_state::DemoWndState::new(&tui_def::DEMO_WND[..]);
    let mut tw = TWins::new(Box::new(DemoPal::new()));

    {
        let mut ctx = tw.lock();
        ctx.draw_wnd(&mut dws);
    }

    {
        let mut ctx = tw.lock();
        use tui_def::Id::*;
        dws.invalidate(&[Lbl1.into(), BtnOk.into(), Lbl2.into()]);
        ctx.draw_invalidated(&mut dws);
        ctx.flush_buff();
    }

    {
        let mut ctx = tw.lock();
        let c = ctx.deref_mut();
        c.write_str(rtwins::esc::LINE_ERASE_ALL);
        c.move_to_col(10).log_w("Column 10");
        c.write_char('\n').flush_buff();
    }
}

// -----------------------------------------------------------------------------------------------

/// Extract window properties from enum
fn wnd_prop(wgt: &rtwins::widget::Widget) -> &rtwins::widget::prop::Window {
    match wgt.typ {
        rtwins::widget::Type::Window(ref wp) => wp,
        _ => panic!(),
    }
}

fn test_esc_codes() {
    use rtwins::esc;

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
}

fn test_property_access() {
    let title = |wgt: &rtwins::widget::Widget| match wgt.typ {
        rtwins::widget::Type::Window(ref wp) => wp.title,
        _ => "<?>",
    };

    for (idx, w) in tui_def::DEMO_WND.iter().enumerate() {
        let w_par = rtwins::widget_impl::wgt_get_parent(&tui_def::DEMO_WND, w);
        println!("  {}. {}:{}, idx:{}, chidx:{}, parid {}:{}", idx, w.id, w.typ, w.link.own_idx, w.link.childs_idx, w_par.id, w_par.typ);
    }

    println!("WINDOW title: {}", title(&tui_def::WINDOW));
    println!("WINDOW title: {}", wnd_prop(&tui_def::WINDOW).title);
    println!("WINDOW title: {}", tui_def::WINDOW.typ.prop_wnd().title);
    println!("WINDOW widgets: {}", rtwins::widget_impl::wgt_count(&tui_def::WINDOW));
    println!(
        "sizeof Widget: {}",
        std::mem::size_of::<rtwins::widget::Widget>()
    );
    println!(
        "sizeof Type: {}",
        std::mem::size_of::<rtwins::widget::Type>()
    );
    println!("sizeof Id: {}", std::mem::size_of::<tui_def::Id>());

    if let rtwins::widget::Type::Window(ref wp) = tui_def::WINDOW.typ {
        println!("WINDOW title: {}", wp.title);
    }
}
