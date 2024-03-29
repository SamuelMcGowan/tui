use std::io;

use crate::style::{Color, Style, Weight};
use crate::vec2::Vec2;

pub trait Writer {
    fn flush(&mut self) -> io::Result<()>;

    fn clear_all(&mut self);

    fn set_cursor_home(&mut self);
    fn next_line(&mut self);

    fn set_cursor_pos(&mut self, pos: impl Into<Vec2>);
    fn set_cursor_vis(&mut self, vis: bool);

    fn set_fg_color(&mut self, c: Color);
    fn set_bg_color(&mut self, c: Color);

    fn set_weight(&mut self, weight: Weight);
    fn set_underline(&mut self, underline: bool);

    fn write_char(&mut self, c: char);
    fn write_str(&mut self, s: &str);

    fn write_style(&mut self, style: Style);
}
