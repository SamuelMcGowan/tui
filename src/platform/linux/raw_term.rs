use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, mem};

use libc::{termios as Termios, winsize as WinSize, STDIN_FILENO, STDOUT_FILENO};

use crate::vec2::Vec2;

static RAW_TERM: AtomicBool = AtomicBool::new(false);

macro_rules! c_result {
    ($res:expr) => {{
        match $res {
            -1 => Err(io::Error::last_os_error()),
            res => Ok(res),
        }
    }};
}

unsafe fn get_termios(fd: RawFd) -> io::Result<Termios> {
    unsafe {
        let mut termios: Termios = mem::zeroed();
        c_result!(libc::tcgetattr(fd, &mut termios))?;
        Ok(termios)
    }
}

unsafe fn set_termios(fd: RawFd, termios: &Termios) -> io::Result<()> {
    c_result!(unsafe { libc::tcsetattr(fd, libc::TCSANOW, termios) })?;
    Ok(())
}

unsafe fn get_size(fd: RawFd) -> io::Result<Vec2> {
    let mut size: WinSize = unsafe { mem::zeroed() };
    c_result!(unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut size) })?;
    Ok([size.ws_col, size.ws_row].into())
}

pub struct RawTerm {
    termios_prev: Termios,
}

impl RawTerm {
    pub fn new() -> io::Result<Self> {
        if RAW_TERM.swap(true, Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "a raw terminal already exists",
            ));
        };

        let mut termios = unsafe { get_termios(STDIN_FILENO)? };
        let termios_prev = termios;

        unsafe { libc::cfmakeraw(&mut termios as *mut Termios) };
        unsafe { set_termios(STDIN_FILENO, &termios)? };

        Ok(Self { termios_prev })
    }

    pub fn get_size(&self) -> io::Result<Vec2> {
        unsafe { get_size(STDIN_FILENO) }
    }
}

impl Drop for RawTerm {
    fn drop(&mut self) {
        let _ = unsafe { set_termios(STDIN_FILENO, &self.termios_prev) };
        RAW_TERM.store(false, Ordering::Relaxed);
    }
}

impl io::Write for RawTerm {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe { use_stdout(|stdout| stdout.write(buf)) }
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        unsafe { use_stdout(|stdout| stdout.write_vectored(bufs)) }
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe { use_stdout(|stdout| stdout.flush()) }
    }
}

/// Safety: don't close stdout by dropping the file.
unsafe fn use_stdout<T>(f: impl Fn(&mut ManuallyDrop<File>) -> T) -> T {
    let mut stdout = ManuallyDrop::new(File::from_raw_fd(STDOUT_FILENO));
    f(&mut stdout)
}
