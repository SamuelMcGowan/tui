use super::string_editor::{StringEditor, TextEdit};
use crate::buffer::Cell;
use crate::prelude::*;

pub struct TextField<Message> {
    pub editor: StringEditor,
    pub style: Style,

    on_enter: Option<Box<dyn FnMut(String) -> Message>>,
}

impl<Message> Default for TextField<Message> {
    fn default() -> Self {
        Self {
            editor: StringEditor::default(),
            style: Style::default(),

            on_enter: None,
        }
    }
}

impl<Message> TextField<Message> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.editor.set_string(text);
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn on_enter(mut self, f: impl FnMut(String) -> Message + 'static) -> Self {
        self.on_enter = Some(Box::new(f));
        self
    }
}

impl<Message> View<Message> for TextField<Message> {
    fn on_event(&mut self, ctx: &mut Context<Message>, event: &Event) -> Handled {
        let handled = self.editor.handle_event(event);
        if let Some(s) = self.editor.entered() {
            if let Some(f) = &mut self.on_enter {
                let msg = f(s);
                ctx.send(msg);
            }
        }
        handled
    }

    fn render(&mut self, buf: &mut crate::buffer::BufferView) {
        let size = buf.size();

        if size.area() == 0 {
            return;
        }

        for (x, c) in self
            .editor
            .as_str()
            .chars()
            .enumerate()
            .take(size.x as usize)
        {
            buf[[x as u16, 0]] = Some(Cell::new(c, self.style));
        }

        let cursor_x = self.editor.cursor_pos_chars();
        if cursor_x < size.x as usize {
            buf.set_cursor(Some([cursor_x as u16, 0]));
        }
    }
}
