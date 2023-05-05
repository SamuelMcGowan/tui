use super::string_editor::TextEdit;
use super::StringEditor;
use crate::buffer::{Buffer, Cell};
use crate::event::*;
use crate::style::Style;
use crate::widget::*;

#[derive(Default)]
pub struct TextInputState {
    pub text: StringEditor,
    pub style: Style,
}

pub struct TextInput<State, Msg> {
    state: TextInputState,
    on_enter: BoxedCallback<State, Msg, TextInputState>,
}

impl<State, Msg> Default for TextInput<State, Msg> {
    fn default() -> Self {
        Self {
            state: TextInputState::default(),
            on_enter: Box::new(EmptyCallback),
        }
    }
}

impl<State, Msg> TextInput<State, Msg> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.state.text.set_string(text);
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.state.style = style;
        self
    }

    pub fn on_enter(
        mut self,
        callback: impl Callback<State, Msg, TextInputState> + 'static,
    ) -> Self {
        self.on_enter = Box::new(callback);
        self
    }
}

impl<State, Msg> Widget<State, Msg> for TextInput<State, Msg> {
    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: Event) -> Handled {
        match event {
            Event::Key(KeyEvent {
                key_code: KeyCode::Return,
                modifiers,
            }) if modifiers.is_empty() => {
                self.on_enter.callback(ctx, &mut self.state);
                Handled::Yes
            }

            event => self.state.text.handle_event(event),
        }
    }

    fn render(&mut self, buf: &mut Buffer) {
        let size = buf.size();

        if size.area() == 0 {
            return;
        }

        for (x, c) in self
            .state
            .text
            .as_str()
            .chars()
            .enumerate()
            .take(size.width as usize - 1)
        {
            buf[[x as u16, 0]] = Some(Cell::new(c, self.state.style));
        }

        let cursor_x = self.state.text.cursor_pos_chars();
        if cursor_x < size.width as usize {
            buf.set_cursor(Some([cursor_x as u16, 0]));
        }
    }
}
