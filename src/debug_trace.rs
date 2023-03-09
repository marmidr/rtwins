//! Debug trace logs
//! This module exports `tr_` macros available anywhere, anytime

use crate::esc;

use lazy_static::lazy_static;
use std::collections::vec_deque::VecDeque;
use std::sync::Mutex;

// ---------------------------------------------------------------------------------------------- //

pub struct Trace {
    queue: VecDeque<TraceItem>,
    pub print_location: bool,
    pub trace_timestr: Box<fn() -> String>,
}

#[derive(Default)]
struct TraceItem {
    pub fg_color: &'static str,
    pub time_str: String,
    pub prefix: &'static str,
    pub msg: String,
}

lazy_static! {
    static ref TRACE: Mutex<Trace> = Mutex::new(Trace::default());
}

// ---------------------------------------------------------------------------------------------- //

#[macro_export]
macro_rules! tr_debug {
    ($MSG:expr) => {
        $crate::Trace::trace_d(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        $crate::Trace::trace_d(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_info {
    ($MSG:expr) => {
        $crate::Trace::trace_i(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        $crate::Trace::trace_i(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_warn {
    ($MSG:expr) => {
        $crate::Trace::trace_w(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        $crate::Trace::trace_w(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_err {
    ($MSG:expr) => {
        $crate::Trace::trace_e(file!(), line!(), $MSG.into());
    };

    ($FMT:literal, $($ARGS:tt)+) => {
        $crate::Trace::trace_e(file!(), line!(), format!($FMT, $($ARGS)+).into());
    };
}

#[macro_export]
macro_rules! tr_flush {
    ($TRM:expr) => {
        $crate::Trace::trace_flush($TRM);
    };
}

#[macro_export]
macro_rules! tr_set_timestr_function {
    ($F:expr) => {
        $crate::Trace::set_timestr_fn(Box::new($F));
    };
}

// ---------------------------------------------------------------------------------------------- //

type Msg = std::borrow::Cow<'static, str>;

impl Trace {
    /// Creates a new trace entry on the internal queue
    fn push(
        &mut self,
        filepath: &str,
        line: u32,
        fg_color: &'static str,
        prefix: &'static str,
        msg: Msg,
    ) {
        let mut msg = msg.to_string();
        let time_str = self.trace_timestr.as_ref()();

        if self.print_location {
            let filename = filepath.split('/').last().unwrap_or_default();
            let longmsg = format!("{}:{}: {}", filename, line, msg);
            msg = longmsg;
        }

        // deferred log, as the Term is locked OR already contains some items on queue,
        // in order to preserve the messages ordering
        self.queue.push_back(TraceItem {
            fg_color,
            time_str,
            prefix,
            msg,
        });
    }

    /// Writes trace queue to the terminal
    fn flush(&mut self, term: &mut crate::Term) {
        if !self.queue.is_empty() {
            self.queue.iter().for_each(|item| {
                term.trace_message(item.fg_color, &item.time_str, item.prefix, &item.msg);
            });

            self.queue.clear();
        }
    }

    /// Print Debug message
    pub fn trace_d(filepath: &str, line: u32, msg: Msg) {
        if let Ok(ref mut guard) = TRACE.lock() {
            guard.push(filepath, line, esc::FG_BLACK_INTENSE, "-D- ", msg);
        }
    }

    /// Print Info message
    pub fn trace_i(filepath: &str, line: u32, msg: Msg) {
        if let Ok(ref mut guard) = TRACE.lock() {
            guard.push(filepath, line, esc::FG_WHITE, "-I- ", msg);
        }
    }

    /// Print Warning message
    pub fn trace_w(filepath: &str, line: u32, msg: Msg) {
        if let Ok(ref mut guard) = TRACE.lock() {
            guard.push(filepath, line, esc::FG_YELLOW, "-W- ", msg);
        }
    }

    /// Print Error message
    pub fn trace_e(filepath: &str, line: u32, msg: Msg) {
        if let Ok(ref mut guard) = TRACE.lock() {
            guard.push(filepath, line, esc::FG_RED, "-E- ", msg);
        }
    }

    /// Flush buffered messages
    pub fn trace_flush(term: &mut crate::Term) {
        if let Ok(ref mut guard) = TRACE.lock() {
            guard.flush(term);
        }
    }

    /// Set user provided pointer to function returning traces timestamp string
    pub fn set_timestr_fn(f: Box<fn() -> String>) {
        if let Ok(ref mut guard) = TRACE.lock() {
            guard.trace_timestr = f;
        }
    }

    /// Returns default timestamp string if system time or Pal is unavailable
    pub fn timestr_default() -> &'static str {
        " 0:00:00.000 "
    }
}

impl Default for Trace {
    fn default() -> Trace {
        Trace {
            queue: Default::default(),
            print_location: true,
            trace_timestr: Box::new(|| Trace::timestr_default().to_owned()),
        }
    }
}

// ---------------------------------------------------------------------------------------------- //
