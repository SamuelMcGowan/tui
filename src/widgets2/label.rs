use crate::buffer::Cell;
use crate::prelude::*;

#[derive(Default)]
pub struct Label {
    pub s: String,
    pub style: Style,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            s: text.into(),
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<Msg> View<Msg> for Label {
    fn render(&self, buf: &mut crate::buffer::BufferView) {
        let size = buf.size();

        if size.y == 0 {
            return;
        }

        for (x, c) in self.s.chars().enumerate().take(size.x as usize) {
            buf[[x as u16, 0]] = Some(Cell::new(c, self.style));
        }
    }
}
