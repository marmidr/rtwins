//! # RTWins reading terminal keys

use std::{os::unix::io::AsRawFd, io::Read};
use termios;
use libc;

const TTY_FILE_PATH: &str = "/dev/tty";
const KEY_BUF_LEN: usize = 8;

pub struct InputTty{
    tty_file: Option<std::fs::File>,
    c_lflag_bkp: termios::tcflag_t,
    eof_code: termios::cc_t,
    key_timeout_ms: u8,
    key_buff: [u8; KEY_BUF_LEN],
}

impl Drop for InputTty {
    fn drop(&mut self) {
        if let Some(ref f) = self.tty_file {
            let mut trm_ios = termios::Termios::from_fd(f.as_raw_fd()).unwrap();
            trm_ios.c_lflag = self.c_lflag_bkp;
            if let Err(_) = termios::tcsetattr(f.as_raw_fd(), termios::TCSAFLUSH, &trm_ios) {
                eprintln!("Cannot restore tty");
            }
        }
    }
}

impl InputTty {
    pub fn new(inp_timeout: u8) -> Self {
        let mut itty = InputTty{
            tty_file: None,
            c_lflag_bkp: 0, eof_code: 0,
            key_timeout_ms: inp_timeout,
            key_buff: [0u8; KEY_BUF_LEN]};

        // TODO: O_NONBLOCK
        itty.tty_file = match std::fs::File::open(TTY_FILE_PATH) {
            Ok(f) =>
                Some(f),
            Err(e) => {
                eprintln!("Cannot open tty : {:?}", e.kind());
                None
            }
        };

        if let Some(ref f) = itty.tty_file {
            let mut trm_ios = termios::Termios::from_fd(f.as_raw_fd()).unwrap();
            itty.c_lflag_bkp = trm_ios.c_lflag;
            itty.eof_code = trm_ios.c_cc[termios::VEOF];

            // disable canonical mode and echo
            trm_ios.c_lflag &= !(termios::ICANON | termios::ECHO);
            if let Err(_) = termios::tcsetattr(f.as_raw_fd(), termios::TCSAFLUSH, &trm_ios) {
                eprintln!("Cannot setup tty");
            }
        }

        itty
    }

    /// Returns tuple with NUL terminated ESC sequence and bool marker set to true,
    /// if application termination was requested (C-d)
    pub fn read_input(&mut self) -> (&[u8; 8], bool) {
        if self.wait_and_read_input_sequence() {
            let exit_requested = self.key_buff[1] == 0 && self.key_buff[0] == self.eof_code;
            return (&self.key_buff, exit_requested);
        }

        self.key_buff[0] = 0;
        return (&self.key_buff, false);
    }

    fn wait_and_read_input_sequence(&mut self) -> bool {
        if let Some(ref mut f) = self.tty_file {
            unsafe {
                let mut read_set = std::mem::MaybeUninit::<libc::fd_set>::uninit();
                let ptr_read_set = read_set.as_mut_ptr();
                libc::FD_ZERO(ptr_read_set);
                libc::FD_SET(f.as_raw_fd(), ptr_read_set);

                // wait for key
                let mut tv = libc::timeval{tv_sec: 0, tv_usec: self.key_timeout_ms as i64 * 1000};

                //https://docs.rs/libc/0.2.112/libc/fn.select.html
                let sel = libc::select(f.as_raw_fd() + 1,
                    ptr_read_set as *mut libc::fd_set,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut tv as *mut libc::timeval);

                if sel > 0 {
                    // read up to 8-1 bytes
                    let res = f.read(&mut self.key_buff[..KEY_BUF_LEN-1]);
                    if let Ok(nb) = res {
                        self.key_buff[nb] = 0;
                        return true;
                    }
                    else {
                        self.key_buff[0] = 0;
                    }
                }
            }
        }

        false
    }
}
