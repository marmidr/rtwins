//! # RTwins demo app

#![allow(unused_variables)]

extern crate rtwins;
use rtwins::{TWins, widget::WindowState};
use std::io::Write;

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
    // test_esc_codes();
    // test_property_access();
    // rtwins::input_decoder::print_seq();

    gui();
}

fn gui() {
    let mut dws = tui_state::DemoWndState::new(&tui_def::WND_MAIN_ARRAY[..]);
    let mut tw = TWins::new(Box::new(DemoPal::new()));
    let mut mouse_on = true;

    tw.lock().write_str(rtwins::esc::TERM_RESET);
    tw.lock().draw_wnd(&mut dws);
    tw.lock().mouse_mode(rtwins::MouseMode::M2);
    tw.lock().flush_buff();

    {
        let mut twl = tw.lock();
        use tui_def::Id::*;
        dws.invalidate(&[LabelDate.into(), BtnYes.into(), Prgbar3.into()]);
        twl.draw_invalidated(&mut dws);
    }

    println!("Press Ctrl-D to quit");
    let mut itty = rtwins::input_tty::InputTty::new(2000);
    let mut ique = rtwins::input_decoder::InputQue::new();
    let mut dec =  rtwins::input_decoder::Decoder::new();
    let mut inp = rtwins::input::InputInfo::new();

    loop {
        let (inp_seq, q) = itty.read_input();

        if q {
            tw.lock().log_w("Quit!");
            break;
        }
        else if inp_seq.len() > 0 {
            for b in inp_seq {
                ique.push_back(*b);
            }

            // print raw sequence
            if false {
                let mut s = String::with_capacity(10);
                for b in inp_seq {
                    if *b == 0 { break; }
                    if *b < b' ' { s.push('�') } else { s.push(*b as char) };
                }
                tw.lock().log_d(format!("seq={}", s).as_str());
            }

            while dec.decode_input_seq(&mut ique, &mut inp) > 0 {
                use rtwins::input::InputType;
                use rtwins::input::Key;

                // input debug info
                match inp.typ {
                    InputType::Char(ref cb) => {
                       tw.lock().log_d(format!("char={}", cb.utf8str()).as_str());
                    },
                    InputType::Key(ref k) => {
                       tw.lock().log_d(format!("key={}", inp.name).as_str());
                    },
                    InputType::Mouse(ref m) => {
                       tw.lock().log_d(format!("mouse={:?} at {}:{}", m.evt, m.col, m.row).as_str());
                    },
                    _ => {}
                }

                // input processing
                if let InputType::Key(ref k) = inp.typ {
                    let mut twl = tw.lock();

                    if *k == Key::F2 {
                        // wndMain.wndEnabled = !wndMain.wndEnabled;
                        // wndMain.invalidate(ID_WND);
                    }
                    else if *k == Key::F4 {
                        mouse_on = !mouse_on;
                        twl.log_i(format!("Mouse {}", if mouse_on {"ON"} else {"OFF"}).as_str());
                        twl.mouse_mode( if mouse_on {rtwins::MouseMode::M2} else {rtwins::MouseMode::Off});
                        twl.flush_buff();
                    }
                    else if *k == Key::F5 {
                        twl.screen_clr_all();

                        // draw windows from bottom to top
                        twl.draw_invalidated(&mut dws);
                        twl.flush_buff();
                        // twins::glob::wMngr.redrawAll();
                    }
                    else if *k == Key::F6 {
                        twl.cursor_save_pos();
                        // twins::moveTo(0, twins::glob::pal.getLogsRow());
                        twl.screen_clr_below();
                        twl.cursor_restore_pos();
                    }
                    else if inp.kmod.has_ctrl() && (*k == Key::PgUp || *k == Key::PgDown) {
                        // if (twins::glob::wMngr.topWnd() == &wndMain)
                        //     twins::wgt::selectNextPage(wndMain.getWidgets(), ID_PGCONTROL, kc.key == twins::Key::PgDown);
                    }
                    else if *k == Key::F9 || *k == Key::F10 {
                        // if (twins::glob::wMngr.topWnd() == &wndMain)
                        //     twins::wgt::selectNextPage(wndMain.getWidgets(), ID_PGCONTROL, kc.key == twins::Key::F10);
                    }
                }

                tw.lock().flush_buff();
            } // decode_input_seq
        }
        else {
            tw.lock().log_d(" -");
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

