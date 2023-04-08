//! # RTWins reading terminal keys

use libc;

use std::io::Read;
use std::os::unix::io::AsRawFd;

const TTY_FILE_PATH: &str = "/dev/tty";

// ---------------------------------------------------------------------------------------------- //

pub struct InputTty {
    tty_file: Option<std::fs::File>,
    c_lflag_bkp: libc::tcflag_t,
    eof_code: libc::cc_t,
    input_timeout_ms: u16,
    input_buff: [u8; crate::esc::SEQ_MAX_LENGTH],
    input_len: u8,
}

impl Drop for InputTty {
    /// Restore original console settings, like ECHO
    fn drop(&mut self) {
        if let Some(ref f) = self.tty_file {
            unsafe {
                let mut tios = std::mem::MaybeUninit::<libc::termios>::uninit();
                if 0 == libc::tcgetattr(f.as_raw_fd(), tios.as_mut_ptr()) {
                    let tios_ref = tios.assume_init_mut();
                    tios_ref.c_lflag = self.c_lflag_bkp;
                    libc::tcsetattr(f.as_raw_fd(), libc::TCSAFLUSH, tios.as_ptr());
                }
                else {
                    let e = std::io::Error::last_os_error();
                    eprintln!("Cannot restore tty : {:?}", e.kind());
                }
            }
        }
    }
}

impl InputTty {
    /// Createas new TTY input reader with given timeout in [ms];
    /// the timeout applies when calling `read_input()`
    pub fn new(timeout_ms: u16) -> Self {
        let mut itty = InputTty {
            tty_file: None,
            c_lflag_bkp: 0,
            eof_code: 0,
            input_timeout_ms: timeout_ms,
            input_buff: [0u8; crate::esc::SEQ_MAX_LENGTH],
            input_len: 0,
        };

        itty.tty_file = match std::fs::File::open(TTY_FILE_PATH) {
            Ok(f) => Some(f),
            Err(e) => {
                eprintln!("Cannot open tty : {:?}", e);
                None
            }
        };

        if let Some(ref f) = itty.tty_file {
            unsafe {
                let mut tios = std::mem::MaybeUninit::<libc::termios>::uninit();
                if 0 == libc::tcgetattr(f.as_raw_fd(), tios.as_mut_ptr()) {
                    let tios_ref = tios.assume_init_mut();
                    itty.c_lflag_bkp = tios_ref.c_lflag;
                    itty.eof_code = tios_ref.c_cc[libc::VEOF];
                    tios_ref.c_lflag &= !(libc::ICANON | libc::ECHO);
                    libc::tcsetattr(f.as_raw_fd(), libc::TCSAFLUSH, tios.as_ptr());
                }
                else {
                    itty.tty_file = None;
                    let e = std::io::Error::last_os_error();
                    eprintln!("Cannot setup tty : {:?}", e.kind());
                }
            }
        }

        itty
    }

    /// Checks if the input is initialized properly
    pub fn opened(&self) -> bool {
        self.tty_file.is_some()
    }

    /// Returns tuple with ESC sequence slice and bool marker set to true,
    /// if application termination was requested (C-d)
    pub fn read_input(&mut self) -> (&[u8], bool) {
        for b in self.input_buff.iter_mut() {
            *b = 0;
        }

        if self.wait_and_read_input_sequence() {
            let exit_requested = self.input_buff[0] == self.eof_code && self.input_buff[1] == 0;
            return (&self.input_buff[..self.input_len as usize], exit_requested);
        }

        (&self.input_buff[..0], false)
    }

    fn wait_and_read_input_sequence(&mut self) -> bool {
        if let Some(ref mut f) = self.tty_file {
            if Self::wait_input(f.as_raw_fd(), self.input_timeout_ms) {
                // read up to 8-1 bytes
                let res = f.read(&mut self.input_buff[..crate::esc::SEQ_MAX_LENGTH - 1]);
                if let Ok(nb) = res {
                    // print!("nb={} ", nb);
                    self.input_buff[nb] = 0;
                    self.input_len = nb as u8;
                    return true;
                }
                else {
                    self.input_len = 0;
                    self.input_buff[0] = 0;
                }
            }
        }

        false
    }

    fn wait_input(fd: std::os::unix::io::RawFd, key_timeout_ms: u16) -> bool {
        unsafe {
            let mut read_set = std::mem::MaybeUninit::<libc::fd_set>::uninit();
            let ptr_read_set = read_set.as_mut_ptr();
            libc::FD_ZERO(ptr_read_set);
            libc::FD_SET(fd, ptr_read_set);

            // wait for key
            let mut tv = libc::timeval {
                tv_sec: 0,
                tv_usec: key_timeout_ms as i64 * 1000,
            };

            //https://docs.rs/libc/0.2.112/libc/fn.select.html
            let sel = libc::select(
                fd + 1,
                ptr_read_set as *mut libc::fd_set,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut tv as *mut libc::timeval,
            );

            sel > 0
        }
    }
}
