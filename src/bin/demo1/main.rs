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
    // test_property_access();
    // rtwins::input_decoder::print_seq();

    let mut dws = tui_state::DemoWndState::new(&tui_def::WND_MAIN_ARRAY[..]);
    let mut tw = TWins::new(Box::new(DemoPal::new()));
    tw.lock().write_str(rtwins::esc::TERM_RESET).flush_buff();

    {
        let mut ctx = tw.lock();
        ctx.draw_wnd(&mut dws);
    }

    {
        let mut ctx = tw.lock();
        use tui_def::Id::*;
        dws.invalidate(&[LabelDate.into(), BtnYes.into(), Prgbar3.into()]);
        ctx.draw_invalidated(&mut dws);
    }

    {
        let mut ctx = tw.lock();
        let c = ctx.deref_mut();
        c.write_str(rtwins::esc::LINE_ERASE_ALL);
        c.move_to_col(10).log_w("Column 10");
        c.write_char('\n').flush_buff();
    }

    println!("Press Ctrl-D to quit");
    let mut itty = rtwins::input_tty::InputTty::new(2000);
    let mut ique = rtwins::input_decoder::InputQue::new();
    let mut dec =  rtwins::input_decoder::Decoder::new();
    let mut iinf = rtwins::input::InputInfo::new();

    loop {
        let (inp_seq, q) = itty.read_input();

        if q {
            println!("Quit!");
            break;
        }
        else if inp_seq.len() > 0 {
            for b in inp_seq {
                ique.push_back(*b);
            }

            while dec.decode_input_seq(&mut ique, &mut iinf) > 0 {
                use rtwins::input::InputType;

                match iinf.typ {
                    InputType::Char(ref cb) => {
                       println!("key={}", cb.utf8str());
                    },
                    InputType::Key(ref k) => {
                       println!("key={}", iinf.name);
                    },
                    InputType::Mouse(ref m) => {
                       println!("key={}", iinf.name);
                    },
                    _ => {}
                }
            }
        }
        else {
            println!(" -");
        }
    }
}

// -----------------------------------------------------------------------------------------------

#[allow(dead_code)]
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

#[allow(dead_code)]
fn test_property_access() {
    let title = |wgt: &rtwins::widget::Widget| match wgt.prop {
        rtwins::widget::Property::Window(ref wp) => wp.title,
        _ => "<?>",
    };

    for (idx, w) in tui_def::WND_MAIN_ARRAY.iter().enumerate() {
        let w_par = rtwins::widget_impl::wgt_get_parent(w);
        println!("  {:2}. {:2}:{:10}, idx:{}, chidx:{}, parid {}:{}",
            idx, w.id, w.prop, w.link.own_idx, w.link.children_idx, w_par.id, w_par.prop);
    }

    println!(
        "sizeof Widget: {}",
        std::mem::size_of::<rtwins::widget::Widget>()
    );
    println!(
        "sizeof Type: {}",
        std::mem::size_of::<rtwins::widget::Property>()
    );
    println!("sizeof Id: {}", std::mem::size_of::<tui_def::Id>());
}

