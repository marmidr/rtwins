//! # RTWins full demo app

use rtwins::wgt::WindowState;
use rtwins::wnd_manager::WindowManager;
use rtwins::{tetrary, wgt};
use rtwins::{tr_info, TERM};

use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use crate::tui_commands::Command;

// https://doc.rust-lang.org/cargo/guide/project-layout.html
mod tui_colors;
mod tui_commands;
mod tui_main_def;
mod tui_main_state;
mod tui_msgbox_def;
mod tui_msgbox_state;

// ---------------------------------------------------------------------------------------------- //

struct DemoFullPal {
    line_buff: String,
    writing_logs: bool,
    started_at: std::time::Instant,
}

impl DemoFullPal {
    fn new() -> Self {
        DemoFullPal {
            line_buff: String::with_capacity(500),
            writing_logs: false,
            started_at: std::time::Instant::now(),
        }
    }
}

impl rtwins::pal::Pal for DemoFullPal {
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
            .write_all(self.line_buff.as_bytes())
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

    fn sleep(&self, ms: u16) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }

    fn get_timestamp_ms(&self) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32
    }

    fn get_timespan_ms(&self, prev_timestamp: u32) -> u32 {
        let dif = std::time::Instant::now() - self.started_at;
        dif.as_millis() as u32 - prev_timestamp
    }
}

// ---------------------------------------------------------------------------------------------- //

struct WndMngr {
    cmdque: Rc<RefCell<tui_commands::CommandsQueue>>,
    visible: Vec<usize>,
    main: tui_main_state::MainWndState,
    msgbox: tui_msgbox_state::MsgBoxState,
}

impl WndMngr {
    fn new() -> Self {
        let cmdque = Rc::new(RefCell::new(tui_commands::CommandsQueue::default()));

        let mut ret = Self {
            cmdque: Rc::clone(&cmdque),
            visible: vec![],
            // all UI windows:
            main: tui_main_state::MainWndState::new(
                &tui_main_def::WND_MAIN_WGTS[..],
                Rc::clone(&cmdque),
            ),
            msgbox: tui_msgbox_state::MsgBoxState::new(
                &tui_msgbox_def::WND_MSGBOX_WGTS[..],
                Rc::clone(&cmdque),
            ),
        };

        ret.msgbox
            .center_on(ret.main.get_widgets().first().unwrap());
        ret
    }
}

/*
/// Iterator over manager windows
struct WndMngrIter<'a> {
    wm: &'a WndMngr,
    iter_idx: usize,
}

impl<'a> WndMngrIter<'a> {
    fn new(wm: &'a WndMngr) -> Self {
        WndMngrIter{
            wm,
            iter_idx: 0
        }
    }
}

impl<'a> Iterator for WndMngrIter<'a> {
    type Item = &'a dyn WindowState;

    fn next(&mut self) -> Option<Self::Item> {
        let result= match self.iter_idx {
            0 => Some(&self.wm.main as &dyn WindowState),
            1 => Some(&self.wm.msgbox as &dyn WindowState),
            _ => None,
        };

        self.iter_idx += 1;
        result
    }
}
*/

// impl<'a> WindowManager<WndMngrIter<'a>> for WndMngr {
impl WindowManager for WndMngr {
    fn get_ref(&self, wnd_idx: usize) -> Option<&dyn WindowState> {
        match wnd_idx {
            0 => Some(&self.main),
            1 => Some(&self.msgbox),
            _ => None,
        }
    }

    fn get_mut(&mut self, wnd_idx: usize) -> Option<&mut dyn WindowState> {
        match wnd_idx {
            0 => Some(&mut self.main),
            1 => Some(&mut self.msgbox),
            _ => None,
        }
    }

    #[inline]
    fn get_visible(&self) -> &[usize] {
        &self.visible[..]
    }

    #[inline]
    fn get_visible_mut(&mut self) -> &mut Vec<usize> {
        &mut self.visible
    }

    // fn iter(&self) -> WndMngrIter<'a> {
    //     WndMngrIter::new(self)
    // }
}

// ---------------------------------------------------------------------------------------------- //

fn main() {
    tui_colors::init();

    // window manager and all windows
    let mut wmngr = WndMngr::new();

    // replace default PAL with our own:
    TERM.try_write().unwrap().pal = Box::new(DemoFullPal::new());
    let mut mouse_on = true;

    // register function providing traces timestamp
    rtwins::tr_set_timestr_function!(|| {
        let local_time = chrono::Local::now();
        local_time.format("%H:%M:%S%.3f ").to_string()
    });

    // configure terminal
    {
        let mut term_guard = TERM.try_write().unwrap();
        term_guard.trace_row = {
            let coord = wmngr.main.get_window_coord();
            let sz = wmngr.main.get_window_size();
            coord.row as u16 + sz.height as u16 + 1
        };
        term_guard.write_str(rtwins::esc::TERM_RESET);
        term_guard.mouse_mode(rtwins::MouseMode::M2);
    }

    // first draw of the UI
    wmngr.show(0);
    wmngr.draw_all();

    rtwins::tr_info!("Press Ctrl-D to quit");
    // rtwins::tr_warn!("WARN MACRO 1");
    // rtwins::tr_err!("ERR MACRO 1");
    // rtwins::tr_debug!("DEBUG MACRO: X={} N={}", 42, "Warduna");
    rtwins::tr_flush!(&mut TERM.try_write().unwrap());

    let mut itty = rtwins::input_tty::InputTty::new(1000);
    let mut ique = rtwins::input_decoder::InputQue::new();
    let mut dec = rtwins::input_decoder::Decoder::default();
    let mut ii = rtwins::input::InputInfo::default();

    loop {
        let (inp_seq, q) = itty.read_input();
        // TODO: detect that application was sent to background and restore terminal config

        if q {
            rtwins::tr_warn!("Exit requested");
            break;
        }
        else if !inp_seq.is_empty() {
            ique.extend(inp_seq.iter());

            // print raw sequence
            /* if false {
                let mut s = String::with_capacity(10);
                for b in inp_seq {
                    if *b == 0 {
                        break;
                    }

                    if *b < b' ' {
                        s.push('�')
                    }
                    else {
                        s.push(*b as char)
                    };
                }
                rtwins::tr_debug!("seq={}", s);
            } */

            while dec.decode_input_seq(&mut ique, &mut ii) > 0 {
                use rtwins::input::InputEvent;
                use rtwins::input::Key;

                // pass the input event to the top-window
                let _key_handled = wgt::process_input(wmngr.get_top_mut().unwrap(), &ii);

                // input debug info
                match ii.evnt {
                    InputEvent::Char(ref ch) => {
                        rtwins::tr_debug!("char='{}'", ch.utf8str());
                    }
                    InputEvent::Key(ref _k) => {
                        rtwins::tr_debug!("key={}", ii.name);
                    }
                    InputEvent::Mouse(ref m) => {
                        let mut r = rtwins::Rect::cdeflt();
                        let wgt_opt =
                            wgt::find_at(wmngr.get_top_mut().unwrap(), m.col, m.row, &mut r);
                        if let Some(w) = wgt_opt {
                            rtwins::tr_debug!(
                                "mouse={:?} at {}:{} ({})",
                                m.evt,
                                m.col,
                                m.row,
                                w.prop
                            );
                        }
                        else {
                            rtwins::tr_debug!("mouse={:?} at {}:{}", m.evt, m.col, m.row);
                        }
                    }
                    InputEvent::None => {}
                }

                // input processing
                if let InputEvent::Key(ref key) = ii.evnt {
                    if *key == Key::F2 {
                        let top_ws = wmngr.get_top_mut().unwrap();
                        let en = !top_ws.is_enabled(&top_ws.get_widgets()[0]);

                        top_ws
                            .get_rstate()
                            .unwrap()
                            .set_enabled(tui_main_def::id::WND_MAIN, en);
                        top_ws.invalidate(wgt::WIDGET_ID_ALL);
                    }
                    else if *key == Key::F4 {
                        mouse_on = !mouse_on;
                        rtwins::tr_info!("Mouse {}", if mouse_on { "ON" } else { "OFF" });
                        let mut term_guard = TERM.try_write().unwrap();
                        term_guard.mouse_mode(tetrary!(
                            mouse_on,
                            rtwins::MouseMode::M2,
                            rtwins::MouseMode::Off
                        ));
                        term_guard.flush_buff();
                    }
                    else if *key == Key::F5 {
                        TERM.try_write().unwrap().screen_clr_all();
                        // draw windows from bottom to top
                        wmngr.draw_all();
                    }
                    else if *key == Key::F6 {
                        let mut term_guard = TERM.try_write().unwrap();
                        term_guard.trace_area_clear();
                    }
                    else if ii.kmod.has_ctrl() && (*key == Key::PgUp || *key == Key::PgDown) {
                        if wmngr.is_top(0) {
                            let main_ws = wmngr.get_top_mut().unwrap();

                            wgt::pagectrl_select_next_page(
                                main_ws,
                                tui_main_def::id::PG_CONTROL,
                                *key == Key::PgDown,
                            );
                            main_ws.invalidate(tui_main_def::id::PG_CONTROL);
                        }
                    }
                    else if wmngr.is_top(0) && (*key == Key::F9 || *key == Key::F10) {
                        let main_ws = wmngr.get_top_mut().unwrap();

                        wgt::pagectrl_select_next_page(
                            main_ws,
                            tui_main_def::id::PG_CONTROL,
                            *key == Key::F10,
                        );
                        main_ws.invalidate(tui_main_def::id::PG_CONTROL);
                    }
                }

                // process the command queue
                {
                    let cmdque = wmngr.cmdque.borrow_mut().take_commands();

                    if let Some(cmdque) = cmdque {
                        for cmd in cmdque.into_iter() {
                            match cmd {
                                Command::ShowPopup {
                                    title,
                                    message,
                                    buttons,
                                    on_button,
                                } => {
                                    tr_info!("Command: ShowPopup");
                                    wmngr.msgbox.show(title, message, buttons, on_button);
                                    wmngr.show(1);
                                }
                                Command::HidePopup => {
                                    tr_info!("Command: HidePopup");
                                    wmngr.hide(1);
                                }
                            }
                        }
                    }
                }

                wmngr.draw_top_invalidated();
            } // decode_input_seq
        }

        // flush the trace logs on every loop
        rtwins::tr_flush!(&mut TERM.try_write().unwrap());
    }

    // epilogue
    {
        let mut term_guard = TERM.try_write().unwrap();
        rtwins::tr_flush!(&mut term_guard);
        term_guard.mouse_mode(rtwins::MouseMode::Off);
        term_guard.trace_area_clear();
        // clear logs below the cursor
        let logs_row = term_guard.trace_row;
        term_guard.move_to(0, logs_row);
        term_guard.flush_buff();
    }
}

// ---------------------------------------------------------------------------------------------- //

#[test]
fn test_esc_codes() {
    use rtwins::esc;

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

#[test]
fn test_property_access() {
    for (idx, w) in tui_main_def::WND_MAIN_WGTS.iter().enumerate() {
        let w_par = wgt::get_parent(w);
        println!(
            "  {:2}. {:2}:{:10}, idx:{}, chidx:{}, parid {}:{}",
            idx, w.id, w.prop, w.link.own_idx, w.link.children_idx, w_par.id, w_par.prop
        );
    }

    println!("sizeof Widget: {}", std::mem::size_of::<wgt::Widget>());
    println!("sizeof Property: {}", std::mem::size_of::<wgt::Property>());
}
