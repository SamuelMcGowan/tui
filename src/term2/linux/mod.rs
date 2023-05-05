mod raw_term;

use std::io::{self, Write};

use raw_term::{RawStdout, RawTerm};

use super::ansi::AnsiBuilder;
use super::Terminal;
use crate::style::{Color, Weight};
use crate::term::{TermPos, TermSize};

pub struct LinuxTerminal {
    raw_term: RawTerm,
    raw_stdout: RawStdout,

    ansi: AnsiBuilder,
}

impl Terminal for LinuxTerminal {
    fn init() -> io::Result<Self> {
        let mut term = Self {
            raw_term: RawTerm::new()?,
            raw_stdout: RawStdout,

            ansi: AnsiBuilder::default(),
        };

        term.clear_all();
        term.flush()?;

        Ok(term)
    }

    fn get_size(&mut self) -> io::Result<TermSize> {
        self.raw_term.get_size()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.ansi.write(&mut self.raw_stdout)?;
        self.raw_stdout.flush()
    }

    fn clear_all(&mut self) {
        self.ansi.clear_all();
    }

    fn set_cursor_home(&mut self) {
        self.ansi.set_cursor_home();
    }

    fn next_line(&mut self) {
        self.ansi.next_line();
    }

    fn set_cursor_pos(&mut self, pos: impl Into<TermPos>) {
        self.ansi.set_cursor_pos(pos);
    }

    fn set_cursor_vis(&mut self, vis: bool) {
        self.ansi.set_cursor_vis(vis);
    }

    fn set_fg_color(&mut self, c: Color) {
        self.ansi.set_fg_color(c);
    }

    fn set_bg_color(&mut self, c: Color) {
        self.ansi.set_bg_color(c);
    }

    fn set_weight(&mut self, weight: Weight) {
        self.ansi.set_weight(weight);
    }

    fn set_underline(&mut self, underline: bool) {
        self.ansi.set_underline(underline);
    }

    fn write_char(&mut self, c: char) {
        self.ansi.write_char(c);
    }

    fn write_str(&mut self, s: &str) {
        self.ansi.write_str(s);
    }
}
