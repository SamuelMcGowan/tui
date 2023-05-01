use std::io;
use std::time::Instant;

use bitflags::bitflags;

pub trait Events: Sized {
    fn read_with_deadline(&mut self, deadline: Instant) -> io::Result<Option<Event>>;
}

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    String(String),
    Unknown,
}

impl Event {
    pub(crate) fn just_key(key_code: KeyCode) -> Self {
        Self::Key(KeyEvent {
            key_code,
            modifiers: Modifiers::empty(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent {
    pub key_code: KeyCode,
    pub modifiers: Modifiers,
}

impl KeyEvent {
    pub fn key(key_code: KeyCode) -> Self {
        Self {
            key_code,
            modifiers: Modifiers::empty(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Char(char),
    Fn(u8),

    Tab,
    Newline,
    Return,

    Escape,

    Up,
    Down,
    Right,
    Left,

    End,
    Home,

    Insert,
    Delete,
    Backspace,

    PageUp,
    PageDown,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        const SHIFT = 0b0001;
        const ALT   = 0b0010;
        const CTRL  = 0b0100;
        const META  = 0b1000;
    }
}
