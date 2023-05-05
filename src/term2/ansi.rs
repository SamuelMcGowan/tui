use std::fmt::Write as _;
use std::io;

use crate::style::{Color, Weight};
use crate::term::TermPos;

const CSI: &str = "\x1b[";

#[derive(Default)]
pub(crate) struct AnsiBuilder {
    buf: String,
}

impl AnsiBuilder {
    pub fn write(&mut self, mut w: impl io::Write) -> io::Result<()> {
        w.write_all(self.buf.as_bytes())?;
        self.buf.clear();
        Ok(())
    }

    pub fn clear_all(&mut self) {
        write!(self.buf, "{CSI}2J{CSI}3J").unwrap();
    }

    pub fn set_cursor_home(&mut self) {
        write!(self.buf, "{CSI}H").unwrap();
    }

    pub fn set_cursor_pos(&mut self, pos: impl Into<TermPos>) {
        let pos = pos.into();

        let row = pos.y.saturating_add(1);
        let col = pos.x.saturating_add(1);

        write!(self.buf, "{CSI}{row};{col}H").unwrap();
    }

    pub fn set_cursor_vis(&mut self, vis: bool) {
        match vis {
            true => write!(self.buf, "{CSI}?25h").unwrap(),
            false => write!(self.buf, "{CSI}?25l").unwrap(),
        }
    }

    pub fn next_line(&mut self) {
        self.buf.push('\n');
    }

    pub fn set_fg_color(&mut self, c: Color) {
        write!(self.buf, "{CSI}3{}m", c as u8).unwrap();
    }

    pub fn set_bg_color(&mut self, c: Color) {
        write!(self.buf, "{CSI}4{}m", c as u8).unwrap();
    }

    pub fn set_weight(&mut self, weight: Weight) {
        match weight {
            Weight::Normal => write!(self.buf, "{CSI}22m").unwrap(),
            Weight::Bold => write!(self.buf, "{CSI}1").unwrap(),
            Weight::Dim => write!(self.buf, "{CSI}2").unwrap(),
        }
    }

    pub fn set_underline(&mut self, underline: bool) {
        match underline {
            true => write!(self.buf, "{CSI}4m").unwrap(),
            false => write!(self.buf, "{CSI}24m").unwrap(),
        }
    }

    pub fn write_char(&mut self, c: char) {
        if c.is_control() {
            return;
        }
        write!(self.buf, "{c}").unwrap();
    }

    pub fn write_str(&mut self, s: &str) {
        for part in s.split(|c: char| c.is_control()) {
            write!(self.buf, "{part}").unwrap();
        }
    }
}
