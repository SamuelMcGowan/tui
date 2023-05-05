mod raw_term;

use std::io::{self};

use raw_term::{RawStdout, RawTerm};

use super::ansi::AnsiWriter;
use super::ansi_event::AnsiEvents;
use super::{Terminal, TerminalWriter};
use crate::term2::TermSize;

pub struct LinuxTerminal {
    raw_term: RawTerm,
    writer: AnsiWriter<RawStdout>,
    events: AnsiEvents,
}

impl Terminal for LinuxTerminal {
    type Writer = AnsiWriter<RawStdout>;
    type Events = AnsiEvents;

    fn init() -> io::Result<Self> {
        let mut term = Self {
            raw_term: RawTerm::new()?,
            writer: AnsiWriter::new(RawStdout),
            events: AnsiEvents::default(),
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

    fn events(&mut self) -> &mut Self::Events {
        &mut self.events
    }
}
