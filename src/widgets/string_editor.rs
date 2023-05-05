use crate::event::{Event, KeyCode, KeyEvent};
use crate::widget::Handled;

/// A string-based text buffer for small texts.
///
/// Use a rope instead for large texts.
#[derive(Default)]
pub struct StringEditor {
    s: String,
    len_chars: usize,

    cursor_pos: usize,
    cursor_pos_chars: usize,
}

impl StringEditor {
    pub fn as_str(&self) -> &str {
        &self.s
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    pub fn cursor_pos_chars(&self) -> usize {
        self.cursor_pos_chars
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub fn len_chars(&self) -> usize {
        self.len_chars
    }

    pub fn set_string(&mut self, s: impl Into<String>) {
        self.s = s.into();
        self.len_chars = self.s.chars().count();

        self.cursor_pos = self.s.len();
        self.cursor_pos_chars = self.len_chars;
    }

    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor_pos_chars = pos.min(self.len_chars);
        self.cursor_pos = self
            .s
            .char_indices()
            .nth(self.cursor_pos_chars)
            .map(|(idx, _)| idx)
            .unwrap_or(self.s.len());
    }

    pub fn clear(&mut self) {
        self.s.clear();
        self.len_chars = 0;

        self.cursor_pos = 0;
        self.cursor_pos_chars = 0;
    }

    fn before_cursor(&self) -> &str {
        &self.s[..self.cursor_pos]
    }

    fn after_cursor(&self) -> &str {
        &self.s[self.cursor_pos..]
    }
}

impl TextEdit for StringEditor {
    fn insert_char(&mut self, c: char) {
        self.s.insert(self.cursor_pos, c);
        self.len_chars += 1;

        self.cursor_pos += c.len_utf8();
        self.cursor_pos_chars += 1;
    }

    fn insert_str(&mut self, s: &str) {
        let len_chars = s.as_bytes().len();

        self.s.insert_str(self.cursor_pos, s);
        self.len_chars += len_chars;

        self.cursor_pos += len_chars;
        self.cursor_pos_chars += s.chars().count();
    }

    fn delete_char(&mut self) {
        if self.cursor_pos == self.s.len() {
            return;
        }

        self.s.remove(self.cursor_pos);
        self.len_chars -= 1;
    }

    fn backspace(&mut self) {
        if let Some(c) = self.before_cursor().chars().next_back() {
            self.cursor_pos -= c.len_utf8();
            self.cursor_pos_chars -= 1;

            self.s.remove(self.cursor_pos);
            self.len_chars -= 1;
        }
    }

    fn move_left(&mut self) {
        if let Some(c) = self.before_cursor().chars().next_back() {
            self.cursor_pos -= c.len_utf8();
            self.cursor_pos_chars -= 1;
        }
    }

    fn move_right(&mut self) {
        if let Some(c) = self.after_cursor().chars().next() {
            self.cursor_pos += c.len_utf8();
            self.cursor_pos_chars += 1;
        }
    }

    fn move_up(&mut self) {
        self.move_home()
    }

    fn move_down(&mut self) {
        self.move_end()
    }

    fn move_home(&mut self) {
        self.cursor_pos = 0;
        self.cursor_pos_chars = 0;
    }

    fn move_end(&mut self) {
        self.cursor_pos = self.s.len();
        self.cursor_pos_chars = self.len_chars;
    }
}

pub trait TextEdit {
    fn handle_event(&mut self, event: &Event) -> Handled {
        match event {
            Event::Key(KeyEvent {
                key_code,
                modifiers,
            }) if modifiers.is_empty() => match key_code {
                KeyCode::Char(c) => self.insert_char(*c),

                KeyCode::Delete => self.delete_char(),
                KeyCode::Backspace => self.backspace(),

                KeyCode::Left => self.move_left(),
                KeyCode::Right => self.move_right(),
                KeyCode::Up => self.move_up(),
                KeyCode::Down => self.move_down(),

                KeyCode::Home => self.move_home(),
                KeyCode::End => self.move_end(),

                _ => return Handled::No,
            },
            _ => return Handled::No,
        }

        Handled::Yes
    }

    fn insert_char(&mut self, c: char);
    fn insert_str(&mut self, s: &str);

    fn delete_char(&mut self);
    fn backspace(&mut self);

    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);

    fn move_home(&mut self);
    fn move_end(&mut self);
}
