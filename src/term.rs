use std::io;

#[cfg(target_os = "linux")]
pub fn get_term() -> io::Result<impl Terminal> {
    crate::sys::linux::UnixTerm::get_term()
}

pub trait Terminal: io::Write + Sized {
    fn get_term() -> io::Result<Self>;
    fn size(&self) -> io::Result<TermSize>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermSize {
    pub width: u16,
    pub height: u16,
}
