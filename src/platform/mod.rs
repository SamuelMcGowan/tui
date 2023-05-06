mod ansi;
mod ansi_event;
mod input;

pub mod linux;

pub mod writer;
use std::io;

pub use writer::Writer;

pub trait Terminal: Sized {
    type Writer: Writer;
    type Events: crate::event::Events;

    fn init() -> io::Result<Self>;
    fn size(&self) -> io::Result<TermSize>;

    fn writer(&mut self) -> &mut Self::Writer;
    fn events(&mut self) -> &mut Self::Events;
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermSize {
    pub width: u16,
    pub height: u16,
}

impl TermSize {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermPos {
    pub x: u16,
    pub y: u16,
}

impl TermPos {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn offset(&self, offset: TermPos) -> TermPos {
        Self {
            x: self.x.saturating_add(offset.x),
            y: self.y.saturating_add(offset.y),
        }
    }
}

impl From<[u16; 2]> for TermSize {
    fn from(value: [u16; 2]) -> Self {
        Self {
            width: value[0],
            height: value[1],
        }
    }
}

impl From<[u16; 2]> for TermPos {
    fn from(value: [u16; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}
