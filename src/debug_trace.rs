//! Debug trace logs
//! This module exports `tr_` macros available anywhere, anytime

use crate::esc;

use std::collections::vec_deque::VecDeque;
use std::sync::Mutex;

// ---------------------------------------------------------------------------------------------- //

pub struct TraceBuffer {
    queue: VecDeque<TraceItem>,
    pub print_location: bool,
}

#[derive(Default)]
struct TraceItem {
    pub fg_color: &'static str,
    pub time_str: String,
    pub prefix: &'static str,
    pub msg: String,
}

thread_local! {
    static TR_BUFFER: Mutex<TraceBuffer> = Mutex::new(TraceBuffer::new());
}

// ---------------------------------------------------------------------------------------------- //

#[macro_export]
macro_rules! tr_debug {
    ($MSG:expr) => {
        TraceBuffer::trace_d(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        TraceBuffer::trace_d(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_info {
    ($MSG:expr) => {
        TraceBuffer::trace_i(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        TraceBuffer::trace_i(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_warn {
    ($MSG:expr) => {
        TraceBuffer::trace_w(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        TraceBuffer::trace_w(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_err {
    ($MSG:expr) => {
        TraceBuffer::trace_e(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        TraceBuffer::trace_e(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_flush {
    ($TRM:expr) => {
        TraceBuffer::trace_flush($TRM);
    };
}

// ---------------------------------------------------------------------------------------------- //

type Msg = std::borrow::Cow<'static, str>;

impl TraceBuffer {
    pub fn new() -> TraceBuffer {
        TraceBuffer{
            queue: Default::default(),
            print_location: true
        }
    }

    fn push_log(&mut self, filepath: &str, line: u32, fg_color: &'static str, prefix: &'static str, msg: Msg) {
        let time_str = crate::Term::lock_read().pal.get_logs_timestr();
        let mut msg = msg.to_string();

        if self.print_location {
            let filename = filepath.split('/').last().unwrap_or_default();
            let longmsg = format!("{}:{}: {}", filename, line, msg);
            msg = longmsg;
        }

        // deferred log, as the Term is locked OR already contains some items on queue,
        // in order to preserve the messages ordering
        self.queue.push_back(TraceItem{
            fg_color,
            time_str,
            prefix,
            msg,
        });
    }

    fn flush(&mut self, term: &mut crate::Term) {
        self.queue.iter().for_each(|item| {
            term.log2(&item.fg_color, &item.time_str, &item.prefix, &item.msg);
        });

        self.queue.clear();
    }

    /// Print Debug message
    pub fn trace_d(filepath: &str, line: u32, msg: Msg) {
        TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.push_log(filepath, line, esc::FG_BLACK_INTENSE, "-D- ", msg);
            }
        });
    }

    /// Print Info message
    pub fn trace_i(filepath: &str, line: u32, msg: Msg) {
        TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.push_log(filepath, line, esc::FG_WHITE, "-I- ", msg);
            }
        });
    }

    /// Print Warning message
    pub fn trace_w(filepath: &str, line: u32, msg: Msg) {
        TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.push_log(filepath, line, esc::FG_YELLOW, "-W- ", msg);
            }
        });
    }

    /// Print Error message
    pub fn trace_e(filepath: &str, line: u32, msg: Msg) {
        TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.push_log(filepath, line, esc::FG_RED, "-E- ", msg);
            }
        });
    }

    /// Flush buffered messages
    pub fn trace_flush(term: &mut crate::Term) {
        TR_BUFFER.with(|mtx|{
            if let Ok(ref mut guard) = mtx.lock() {
                guard.flush(term);
            }
        });
    }
}

// ---------------------------------------------------------------------------------------------- //
