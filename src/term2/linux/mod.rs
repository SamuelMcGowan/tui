mod raw_term;

use std::io::{self};

use raw_term::{RawStdout, RawTerm};

use super::ansi::AnsiWriter;
use super::{Terminal, TerminalWriter};
use crate::term::TermSize;

pub struct LinuxTerminal {
    raw_term: RawTerm,
    writer: AnsiWriter<RawStdout>,
}

impl Terminal for LinuxTerminal {
    type Writer = AnsiWriter<RawStdout>;

    fn init() -> io::Result<Self> {
        let mut term = Self {
            raw_term: RawTerm::new()?,
            writer: AnsiWriter::new(RawStdout),
        };

        term.writer().clear_all();
        term.writer().flush()?;

        Ok(term)
    }

    fn size(&self) -> io::Result<TermSize> {
        self.raw_term.get_size()
    }

    fn writer(&mut self) -> &mut Self::Writer {
        &mut self.writer
    }
}
