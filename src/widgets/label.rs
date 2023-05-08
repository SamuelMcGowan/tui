use crate::buffer::{BufferView, Cell};
use crate::style::Style;
use crate::widget::Widget;

#[derive(Default)]
pub struct Label {
    pub text: String,
    pub style: Style,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<State, Msg> Widget<State, Msg> for Label {
    fn render(&mut self, buf: &mut BufferView) {
        let size = buf.size();

        if size.y == 0 {
            return;
        }

        for (x, c) in self.text.chars().enumerate().take(size.x as usize) {
            buf[[x as u16, 0]] = Some(Cell::new(c, self.style));
        }
    }
}
