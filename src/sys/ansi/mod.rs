use std::io;
use std::time::Instant;

use self::input::PollingStdin;
use self::parse_event::{parse_event, Events};
use crate::event::Event;

mod input;
mod parse_event;

#[derive(Default)]
pub struct AnsiEvents {
    stdin: PollingStdin,
}

impl Events for AnsiEvents {
    fn new() -> io::Result<Self> {
        Ok(Self::default())
    }

    fn read_with_deadline(&mut self, deadline: Instant) -> io::Result<Option<Event>> {
        let Some(bytes) = self.stdin.read_with_deadline(deadline)? else {
            return Ok(None);
        };
        let event = parse_event(bytes.as_slice()).unwrap_or(Event::Unknown);
        Ok(Some(event))
    }
}
