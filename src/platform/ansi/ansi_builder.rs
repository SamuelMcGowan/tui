use std::fmt::Write;

use crate::style::*;
use crate::term::TermPos;

#[derive(Debug, Clone)]
pub struct AnsiBuilder {
    s: String,
    style: Style,

    cursor_visible: bool,
}

impl Default for AnsiBuilder {
    fn default() -> Self {
        let mut ansi_builder = AnsiBuilder {
            s: String::new(),
            style: Style::default(),

            // so that the following call works
            cursor_visible: true,
        };

        ansi_builder.set_cursor_position([0, 0]);
        ansi_builder.show_cursor(false);

        ansi_builder
    }
}

impl AnsiBuilder {
    pub fn write_str(&mut self, s: &str) {
        // remove the control characters without having to
        // push each character separately
        for part in s.split(|c: char| c.is_control()) {
            self.s.push_str(part);
        }
    }

    pub fn write_char(&mut self, c: char) {
        if !c.is_control() {
            self.s.push(c);
        }
    }

    pub fn write_newline(&mut self) {
        self.s.push_str("\r\n");
    }

    pub fn write_style(&mut self, style: Style) {
        macro_rules! sgr {
            ($($arg:tt)+) => {{
                write!(self.s, "\x1b[{}m", format_args!($($arg)*)).unwrap();
            }};
        }

        if style.fg != self.style.fg {
            sgr!("3{}", style.fg as u8);
        }

        if style.bg != self.style.bg {
            sgr!("4{}", style.bg as u8);
        }

        if style.weight != self.style.weight {
            match style.weight {
                Weight::Normal => sgr!("22"),
                Weight::Bold => sgr!("1"),
                Weight::Dim => sgr!("2"),
            }
        }

        if style.underline != self.style.underline {
            match style.underline {
                true => sgr!("4"),
                false => sgr!("24"),
            }
        }

        self.style = style;
    }

    pub fn clear_screen(&mut self) {
        self.s.push_str("\x1b[2J");
        self.s.push_str("\x1b[3J");
    }

    pub fn set_cursor_position(&mut self, pos: impl Into<TermPos>) {
        let pos = pos.into();
        let row = pos.y.saturating_add(1);
        let col = pos.x.saturating_add(1);
        write!(self.s, "\x1b[{row};{col}H").unwrap();
    }

    pub fn show_cursor(&mut self, show_cursor: bool) {
        if show_cursor == self.cursor_visible {
            return;
        }

        match show_cursor {
            true => write!(self.s, "\x1b[?25h").unwrap(),
            false => write!(self.s, "\x1b[?25l").unwrap(),
        }

        self.cursor_visible = show_cursor;
    }

    pub fn finish(mut self) -> String {
        self.write_style(Style::default());
        self.s
    }
}

// #[cfg(test)]
// mod tests {
//     use super::AnsiBuilder;
//     use crate::style::{Color, Style, Weight};

//     #[test]
//     fn my_test() {
//         let mut ansi = AnsiBuilder::default();
//         ansi.write_str("hello");
//         ansi.write_newline();

//         ansi.write_style(Style {
//             fg: Color::Magenta,
//             ..Default::default()
//         });
//         ansi.write_str("world\r\n");
//         ansi.write_newline();

//         ansi.write_style(Style {
//             fg: Color::Blue,
//             weight: Weight::Bold,
//             ..Default::default()
//         });
//         ansi.write_str("boo!\r\n");
//         ansi.write_newline();

//         print!("{}", ansi.finish());
//     }
// }
