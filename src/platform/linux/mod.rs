mod raw_term;

use std::io::{self, Write};

use self::raw_term::{RawStdout, RawTerm};
pub use super::ansi::AnsiEvents as Events;
use crate::buffer::Buffer;
use crate::platform::ansi::AnsiBuilder;
use crate::term::TermSize;

pub struct Terminal {
    raw_term: RawTerm,
    raw_stdout: RawStdout,
}

impl crate::term::Terminal for Terminal {
    fn new() -> io::Result<Self> {
        Ok(Self {
            raw_term: RawTerm::new()?,
            raw_stdout: RawStdout,
        })
    }

    fn size(&self) -> io::Result<TermSize> {
        self.raw_term.size()
    }

    fn draw(&mut self, buf: &Buffer) -> io::Result<()> {
        let ansi = AnsiBuilder::frame_of_buffer(buf);
        write!(self.raw_stdout, "{ansi}")
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let ansi = AnsiBuilder::frame(|ansi_builder| {
            ansi_builder.clear_screen();
            ansi_builder.show_cursor(true);
        });
        let _ = write!(self.raw_stdout, "{ansi}");
    }
}
