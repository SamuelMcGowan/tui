mod raw_term;

use self::raw_term::{RawStdout, RawTerm};
pub use super::ansi::AnsiEvents as Events;

pub struct Terminal {
    raw_term: RawTerm,
    raw_stdout: RawStdout,
}

impl crate::term::Terminal for Terminal {
    fn new() -> std::io::Result<Self> {
        Ok(Self {
            raw_term: RawTerm::new()?,
            raw_stdout: RawStdout,
        })
    }

    fn size(&self) -> std::io::Result<crate::term::TermSize> {
        self.raw_term.size()
    }
}
