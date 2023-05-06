mod raw_term;

use std::io::{self};

use raw_term::RawTerm;

use super::ansi::AnsiWriter;
use super::ansi_event::AnsiEvents;
use super::{Terminal, Writer};
use crate::platform::TermSize;

pub struct LinuxTerminal {
    raw_term: AnsiWriter<RawTerm>,
    events: AnsiEvents,
}

impl Terminal for LinuxTerminal {
    type Writer = AnsiWriter<RawTerm>;
    type Events = AnsiEvents;

    fn init() -> io::Result<Self> {
        let mut term = Self {
            raw_term: AnsiWriter::new(RawTerm::new()?),
            events: AnsiEvents::default(),
        };

        term.writer().clear_all();
        term.writer().flush()?;

        Ok(term)
    }

    fn size(&self) -> io::Result<TermSize> {
        self.raw_term.inner().get_size()
    }

    fn writer(&mut self) -> &mut Self::Writer {
        &mut self.raw_term
    }

    fn events(&mut self) -> &mut Self::Events {
        &mut self.events
    }
}
