mod raw_term;

use std::io::{self, Write};

use raw_term::{RawStdout, RawTerm};

use super::Terminal;
use crate::style::{Color, Weight};
use crate::term::{TermPos, TermSize};

const CSI: &str = "\x1b[";

pub struct LinuxTerminal {
    raw_term: RawTerm,
    raw_stdout: RawStdout,

    buf: Vec<u8>,
}

impl Terminal for LinuxTerminal {
    fn init() -> io::Result<Self> {
        let mut term = Self {
            raw_term: RawTerm::new()?,
            raw_stdout: RawStdout,

            buf: vec![],
        };
        term.clear_all();
        Ok(term)
    }

    fn get_size(&mut self) -> io::Result<TermSize> {
        self.raw_term.get_size()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.raw_stdout.write_all(&self.buf)?;

        // let mut stdout = std::io::stdout();
        // stdout.write_all(&self.buf)?;
        // stdout.flush()?;

        self.buf.clear();
        Ok(())
    }

    fn clear_all(&mut self) {
        write!(self.buf, "{CSI}2J{CSI}3J").unwrap();
    }

    fn set_cursor_home(&mut self) {
        write!(self.buf, "{CSI}H").unwrap();
    }

    fn set_cursor_pos(&mut self, pos: impl Into<TermPos>) {
        let pos = pos.into();

        let row = pos.y.saturating_add(1);
        let col = pos.x.saturating_add(1);

        write!(self.buf, "{CSI}{row};{col}H").unwrap();
    }

    fn set_cursor_vis(&mut self, vis: bool) {
        match vis {
            true => write!(self.buf, "{CSI}?25h").unwrap(),
            false => write!(self.buf, "{CSI}?25l").unwrap(),
        }
    }

    fn next_line(&mut self) {
        self.buf.push(b'\n');
    }

    fn set_fg_color(&mut self, c: Color) {
        write!(self.buf, "{CSI}3{}m", c as u8).unwrap();
    }

    fn set_bg_color(&mut self, c: Color) {
        write!(self.buf, "{CSI}4{}m", c as u8).unwrap();
    }

    fn set_weight(&mut self, weight: Weight) {
        match weight {
            Weight::Normal => write!(self.buf, "{CSI}22m").unwrap(),
            Weight::Bold => write!(self.buf, "{CSI}1").unwrap(),
            Weight::Dim => write!(self.buf, "{CSI}2").unwrap(),
        }
    }

    fn set_underline(&mut self, underline: bool) {
        match underline {
            true => write!(self.buf, "{CSI}4m").unwrap(),
            false => write!(self.buf, "{CSI}24m").unwrap(),
        }
    }

    fn write_char(&mut self, c: char) {
        if c.is_control() {
            return;
        }
        write!(self.buf, "{c}").unwrap();
    }

    fn write_str(&mut self, s: &str) {
        for part in s.split(|c: char| c.is_control()) {
            write!(self.buf, "{part}").unwrap();
        }
    }
}
