use std::io;

pub mod ansi;
pub mod linux;

pub trait Platform {
    type Term: crate::term::Terminal;
    type Events: crate::event::Events;

    fn get_term(&self) -> io::Result<Self::Term>;
    fn get_events(&self) -> io::Result<Self::Events>;
}

#[cfg(target_os = "linux")]
pub struct Linux;

#[cfg(target_os = "linux")]
impl Platform for Linux {
    type Events = ansi::AnsiEvents;
    type Term = linux::LinuxTerminal;

    fn get_term(&self) -> io::Result<Self::Term> {
        linux::LinuxTerminal::new()
    }

    fn get_events(&self) -> io::Result<Self::Events> {
        Ok(ansi::AnsiEvents::new())
    }
}
