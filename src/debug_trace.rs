//! Debug trace logs
//! This module exports `tr_` macros available anywhere, anytime

use crate::esc;
use std::collections::vec_deque::VecDeque;
use std::sync::Mutex;
use std::sync::Arc;

// ---------------------------------------------------------------------------------------------- //

pub struct TraceBuffer {
    queue: VecDeque<TraceItem>,
    pal: Arc<dyn crate::pal::Pal>,
    pub print_location: bool,
}

#[derive(Default)]
struct TraceItem {
    pub fg: String,
    pub time_str: String,
    pub prefix: String,
    pub msg: String,
}

thread_local! {
    pub static TR_BUFFER: Mutex<TraceBuffer> = Mutex::new(TraceBuffer::new());
}

// ---------------------------------------------------------------------------------------------- //

#[macro_export]
macro_rules! tr_debug {
    ($MSG:expr) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_d(file!(), line!(), $MSG);
            }
        });
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_d(file!(), line!(), format!($FMT, $($ARGS)+).as_str());
            }
        });
    };
}

#[macro_export]
macro_rules! tr_info {
    ($MSG:expr) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_i(file!(), line!(), $MSG);
            }
        });
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_i(file!(), line!(), format!($FMT, $($ARGS)+).as_str());
            }
        });
    };
}

#[macro_export]
macro_rules! tr_warn {
    ($MSG:expr) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_w(file!(), line!(), $MSG);
            }
        });
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_w(file!(), line!(), format!($FMT, $($ARGS)+).as_str());
            }
        });
    };
}

#[macro_export]
macro_rules! tr_err {
    ($MSG:expr) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_e(file!(), line!(), $MSG);
            }
        });
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        rtwins::TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.trace_e(file!(), line!(), format!($FMT, $($ARGS)+).as_str());
            }
        });
    };
}

// ---------------------------------------------------------------------------------------------- //

impl TraceBuffer {
    pub fn new() -> TraceBuffer {
        TraceBuffer{
            queue: Default::default(),
            pal: Arc::new(crate::pal::PalStub::default()),
            print_location: false
        }
    }

    pub fn set_pal(&mut self, p: Arc<dyn crate::pal::Pal>) {
        self.pal = p;
    }

    // TODO: msg should be a Cow<>
    fn push_log(&mut self, fg: &str, prfx: &str, msg: &str) {
        let time_str = self.pal.get_logs_timestr();

        // deferred log, as the Term is locked OR already contains some items on queue,
        // in order to preserve the messages ordering
        self.queue.push_back(TraceItem{
            fg: String::from(fg),
            time_str,
            prefix: String::from(prfx),
            msg: String::from(msg)
        });
    }

    pub fn flush(&mut self, term: &mut crate::Term) {
        self.queue.iter().for_each(|item| {
            term.log2(&item.fg, &item.time_str, &item.prefix, &item.msg);
        });

        self.queue.clear();
    }

    /// Print Debug message
    pub fn trace_d(&mut self, file: &str, line: u32, msg: &str) {
        if self.print_location {
            self.push_log(esc::FG_BLACK_INTENSE, "-D- ",
                format!("{}:{}: {}", file, line, msg).as_str()
            );
        }
        else {
            self.push_log(esc::FG_BLACK_INTENSE, "-D- ", msg);
        }
    }

    /// Print Info message
    pub fn trace_i(&mut self, file: &str, line: u32, msg: &str) {
        self.push_log(esc::FG_WHITE, "-I- ", msg);
    }

    /// Print Warning message
    pub fn trace_w(&mut self, msg: &str) {
        self.push_log(esc::FG_YELLOW, "-W- ", msg);
    }

    /// Print Error message
    pub fn trace_e(&mut self, file: &str, line: u32, msg: &str) {
        self.push_log(esc::FG_RED, "-E- ", msg);
    }
}
