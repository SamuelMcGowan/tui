mod ansi;
mod ansi_event;
mod input;

pub mod linux;

use std::io;

use crate::style::{Color, Weight};

pub trait Terminal: Sized {
    type Writer: TerminalWriter;
    type Events: crate::event::Events;

    fn init() -> io::Result<Self>;
    fn size(&self) -> io::Result<TermSize>;

    fn writer(&mut self) -> &mut Self::Writer;
    fn events(&mut self) -> &mut Self::Events;
}

pub trait TerminalWriter {
    fn flush(&mut self) -> io::Result<()>;

    fn clear_all(&mut self);

    fn set_cursor_home(&mut self);
    fn next_line(&mut self);

    fn set_cursor_pos(&mut self, pos: impl Into<TermPos>);
    fn set_cursor_vis(&mut self, vis: bool);

    fn set_fg_color(&mut self, c: Color);
    fn set_bg_color(&mut self, c: Color);

    fn set_weight(&mut self, weight: Weight);
    fn set_underline(&mut self, underline: bool);

    fn write_char(&mut self, c: char);
    fn write_str(&mut self, s: &str);
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
