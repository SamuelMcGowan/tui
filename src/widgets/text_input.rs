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
    fn handle_event(&mut self, mut ctx: Context<State, Msg>, event: Event) -> Handled {
        match event {
            Event::Key(KeyEvent {
                key_code,
                modifiers,
            }) if modifiers.is_empty() => match key_code {
                KeyCode::Char(c) => {
                    self.state.text.insert_char(c);
                    Handled::Yes
                }

                KeyCode::Return => {
                    self.on_enter.callback(&mut ctx, &mut self.state);
                    Handled::Yes
                }

                KeyCode::Delete => {
                    self.state.text.delete_char();
                    Handled::Yes
                }
                KeyCode::Backspace => {
                    self.state.text.backspace();
                    Handled::Yes
                }

                KeyCode::Left => {
                    self.state.text.move_backwards();
                    Handled::Yes
                }
                KeyCode::Right => {
                    self.state.text.move_forwards();
                    Handled::Yes
                }
                KeyCode::Home => {
                    self.state.text.move_home();
                    Handled::Yes
                }
                KeyCode::End => {
                    self.state.text.move_end();
                    Handled::Yes
                }

                _ => Handled::No,
            },
            _ => Handled::No,
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
