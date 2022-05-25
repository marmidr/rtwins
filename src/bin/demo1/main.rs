//! # RTWins demo app

#![allow(unused_variables)]

extern crate rtwins;
use rtwins::*;
use rtwins::WindowState; // to use trait implementation

use std::io::Write;

// https://doc.rust-lang.org/cargo/guide/project-layout.html
mod tui_def;
mod tui_state;

// -----------------------------------------------------------------------------------------------

struct DemoPal {
    line_buff: String,
    writing_logs: bool,
    started_at: std::time::Instant,
}

impl DemoPal {
    fn new() -> Self {
        DemoPal {
            line_buff: String::with_capacity(500),
            writing_logs: false,
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

        std::io::stdout()
            .lock()
            .flush()
            .expect("Error flushing stdout");

        self.line_buff.clear();
        // self.sleep(100); // helpful when debugging drawing process
    }

    fn mark_logging(&mut self, active: bool) {
        if self.writing_logs && !active {
            // write to logs.txt
        }

        self.writing_logs = active;
    }

    fn sleep(&mut self, ms: u16) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }

    fn get_timestamp(&mut self) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32
    }

    fn get_timespan_ms(&mut self, prev_timestamp: u32) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32 - prev_timestamp
    }

    fn get_logs_timestr(&self) -> String {
        let local_time = chrono::Local::now();
        local_time.format("%H:%M:%S%.3f ").to_string()
    }
}

// -----------------------------------------------------------------------------------------------

fn main() {
    // test_esc_codes();
    // test_property_access();
    // rtwins::input_decoder::print_seq();

    tui_demo();
}

fn tui_demo() {
    // create Demo window state
    let mut dws = tui_state::DemoWndState::new(&tui_def::WND_MAIN_ARRAY[..]);
    // replace default PAL with our own:
    rtwins::Term::lock_write().pal = Box::new(DemoPal::new());
    let mut mouse_on = true;
    // let mut wm = WndManager::new();

    {
        let mut term = rtwins::Term::lock_write();
        term.logs_row = {
            let coord = dws.get_window_coord();
            let sz = dws.get_window_size();
            coord.row as u16 + sz.height as u16 + 1
        };
        term.write_str(rtwins::esc::TERM_RESET);
        term.draw_wnd(&mut dws);
        term.mouse_mode(rtwins::MouseMode::M2);
        term.flush_buff();
    }

    rtwins::tr_info!("Press Ctrl-D to quit");
    rtwins::tr_warn!("WARN MACRO 1");
    rtwins::tr_err!("ERR MACRO 1");
    rtwins::tr_debug!("DEBUG MACRO: X={} N={}", 42, "Warduna");
    rtwins::tr_flush!(&mut rtwins::Term::lock_write());

    let mut itty = rtwins::input_tty::InputTty::new(1000);
    let mut ique = rtwins::input_decoder::InputQue::new();
    let mut dec =  rtwins::input_decoder::Decoder::new();
    let mut inp = rtwins::input::InputInfo::new();

    loop {
        let (inp_seq, q) = itty.read_input();
        // TODO: detect that application was sent to background and restore terminal config

        if q {
            rtwins::tr_warn!("Exit requested");
            break;
        }
        else if inp_seq.len() > 0 {
            for b in inp_seq {
                ique.push_back(*b);
            }

            let mut term = rtwins::Term::lock_write();

            // print raw sequence
            if false {
                let mut s = String::with_capacity(10);
                for b in inp_seq {
                    if *b == 0 { break; }
                    if *b < b' ' { s.push('�') } else { s.push(*b as char) };
                }
                term.log_d(format!("seq={}", s).as_str());
            }

            while dec.decode_input_seq(&mut ique, &mut inp) > 0 {
                use rtwins::input::InputType;
                use rtwins::input::Key;

                // input debug info
                match inp.typ {
                    InputType::Char(ref cb) => {
                        term.log_d(format!("char={}", cb.utf8str()).as_str());
                    },
                    InputType::Key(ref k) => {
                        term.log_d(format!("key={}", inp.name).as_str());
                    },
                    InputType::Mouse(ref m) => {
                        let mut r = rtwins::Rect::cdeflt();
                        let wgt_opt = wgt::find_at(&mut dws, m.col, m.row, &mut r);
                        if let Some(w) = wgt_opt {
                            term.log_d(format!("mouse={:?} at {}:{} ({})", m.evt, m.col, m.row, w.prop).as_str());
                        }
                        else {
                            term.log_d(format!("mouse={:?} at {}:{}", m.evt, m.col, m.row).as_str());
                        }
                    },
                    InputType::None => {
                    },
                }

                // input processing
                if let InputType::Key(ref k) = inp.typ {
                    use tui_def::Id;

                    if *k == Key::F2 {
                        dws.rs.set_enabled(Id::WndMain.into(),
                            !dws.rs.get_enabled_or_default(Id::WndMain.into()));
                        dws.invalidate(&[rtwins::WIDGET_ID_ALL]);
                    }
                    else if *k == Key::F4 {
                        mouse_on = !mouse_on;
                        term.log_i(format!("Mouse {}", if mouse_on {"ON"} else {"OFF"}).as_str());
                        term.mouse_mode(if mouse_on {rtwins::MouseMode::M2} else {rtwins::MouseMode::Off});
                        term.flush_buff();
                    }
                    else if *k == Key::F5 {
                        term.screen_clr_all();
                        // draw windows from bottom to top
                        // TODO: wm.redraw_all();
                        term.draw_wnd(&mut dws);
                        term.flush_buff();
                    }
                    else if *k == Key::F6 {
                        term.log_clear();
                    }
                    else if inp.kmod.has_ctrl() && (*k == Key::PgUp || *k == Key::PgDown) {
                        // if wm.is_top_wnd(&dws) {
                            rtwins::wgt::pagectrl_select_next_page(&mut dws, Id::PgControl.into(), *k == Key::PgDown);
                            dws.invalidate(&[Id::PgControl.into()]);
                        // }
                    }
                    else if *k == Key::F9 || *k == Key::F10 {
                        // if wm.is_top_wnd(&dws) {
                            rtwins::wgt::pagectrl_select_next_page(&mut dws, Id::PgControl.into(), *k == Key::F10);
                            dws.invalidate(&[Id::PgControl.into()]);
                        // }
                    }
                }

                term.draw_invalidated(&mut dws);
            } // decode_input_seq
        }

        // flush the trace logs on every loop
        rtwins::tr_flush!(&mut rtwins::Term::lock_write());
    }

    // epilogue
    {
        let mut term = rtwins::Term::lock_write();
        rtwins::tr_flush!(&mut term);
        term.mouse_mode(rtwins::MouseMode::Off);
        term.log_clear();
        let logs_row = term.logs_row;
        term.move_to(0, logs_row);
        term.flush_buff();
    }
}

// -----------------------------------------------------------------------------------------------

#[allow(dead_code)]
fn test_esc_codes() {
    println!(
        "** {}{}{} ** demo; lib v{}{}{}",
        esc::BOLD,
        rtwins::url_link!("https://github.com/marmidr/rtwins", "RTWins"),
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
    for (idx, w) in tui_def::WND_MAIN_ARRAY.iter().enumerate() {
        let w_par = wgt::get_parent(w);
        println!("  {:2}. {:2}:{:10}, idx:{}, chidx:{}, parid {}:{}",
            idx, w.id, w.prop, w.link.own_idx, w.link.children_idx, w_par.id, w_par.prop);
    }

    println!(
        "sizeof Widget: {}",
        std::mem::size_of::<rtwins::Widget>()
    );
    println!(
        "sizeof Type: {}",
        std::mem::size_of::<rtwins::Property>()
    );
    println!("sizeof Id: {}", std::mem::size_of::<tui_def::Id>());
}

