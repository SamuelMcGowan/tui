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
        self.draw_frame(|ansi_buffer| {
            for y in 0..buf.size().height {
                for x in 0..buf.size().width {
                    let cell = buf[[x, y]].unwrap_or_default();

                    ansi_buffer.write_style(cell.style);
                    ansi_buffer.write_char(cell.c);
                }

                if buf.size().height == 0 || y < buf.size().height - 1 {
                    ansi_buffer.write_newline();
                }
            }

            if let Some(pos) = buf.cursor() {
                ansi_buffer.set_cursor_position(pos);
                ansi_buffer.show_cursor(true);
            }
        })
    }
}

impl Terminal {
    fn draw_frame(&mut self, f: impl Fn(&mut AnsiBuilder)) -> io::Result<()> {
        let mut ansi_buffer = AnsiBuilder::default();
        ansi_buffer.clear_screen();

        f(&mut ansi_buffer);

        let ansi = ansi_buffer.finish();

        // Perform one write to stdout each frame.
        // No buffering is performed, so no flushing is required.
        write!(self.raw_stdout, "{ansi}")
    }
}
