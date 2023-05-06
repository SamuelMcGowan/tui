use std::ops::{Index, IndexMut};

use crate::platform::{TermPos, TermSize, Writer};
use crate::style::Style;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub c: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            style: Style::default(),
        }
    }
}

impl Cell {
    pub fn new(c: char, style: Style) -> Self {
        Self { c, style }
    }
}

#[derive(Default, Debug)]
pub struct Buffer {
    data: Vec<Option<Cell>>,

    size: TermSize,
    cursor: Option<TermPos>,
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),

            size: self.size,
            cursor: self.cursor,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);

        self.size = source.size;
        self.cursor = source.cursor;
    }
}

impl Buffer {
    pub fn new(size: impl Into<TermSize>) -> Self {
        Self::new_inner(size.into(), vec![])
    }

    pub fn resize_and_clear(&mut self, size: impl Into<TermSize>) {
        let size = size.into();

        if size != self.size {
            let data = std::mem::take(&mut self.data);
            *self = Self::new_inner(size, data);
        } else {
            self.data.fill(None);
            self.cursor = None;
        }
    }

    fn new_inner(size: TermSize, mut data: Vec<Option<Cell>>) -> Self {
        data.clear();
        data.extend(std::iter::repeat(None).take(size.area()));

        Self {
            data,

            size,
            cursor: None,
        }
    }

    pub fn size(&self) -> TermSize {
        self.size
    }

    pub fn as_slice(&self) -> &[Option<Cell>] {
        &self.data
    }

    pub fn get(&self, index: impl Into<TermPos>) -> Option<&Option<Cell>> {
        self.data.get(self.index(index)?)
    }

    pub fn get_mut(&mut self, index: impl Into<TermPos>) -> Option<&mut Option<Cell>> {
        let index = self.index(index)?;
        self.data.get_mut(index)
    }

    fn index(&self, pos: impl Into<TermPos>) -> Option<usize> {
        let pos = pos.into();

        if pos.x >= self.size.width || pos.y > self.size.height {
            return None;
        }

        let index = pos.y as usize * self.size.width as usize + pos.x as usize;

        Some(index)
    }

    pub fn set_cursor(&mut self, cursor: Option<impl Into<TermPos>>) {
        self.cursor = cursor.map(Into::into);
    }

    pub fn cursor(&self) -> Option<TermPos> {
        self.cursor
    }

    pub fn blit(
        &mut self,
        offset: impl Into<TermPos>,
        buf: &Buffer,
        override_cursor: bool,
        clear: Option<Cell>,
    ) {
        let offset = offset.into();

        for (x, buf_x) in (offset.x..self.size.width).zip(0..buf.size.width) {
            for (y, buf_y) in (offset.y..self.size.height).zip(0..buf.size.height) {
                self[[x, y]] = buf[[buf_x, buf_y]].or(clear);
            }
        }

        if override_cursor {
            self.cursor = buf.cursor.map(|pos| pos.offset(offset));
        }
    }
}

impl<I: Into<TermPos>> Index<I> for Buffer {
    type Output = Option<Cell>;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).expect("out of bounds")
    }
}

impl<I: Into<TermPos>> IndexMut<I> for Buffer {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index).expect("out of bounds")
    }
}

pub fn draw_diff(old: &Buffer, new: &Buffer, w: &mut impl Writer) {
    if old.size() != new.size() {
        draw_no_diff(new, w);
        return;
    }

    w.set_cursor_home();

    let mut cursor_pos = TermPos::from([0, 0]);
    let mut style = Style::default();

    for y in 0..new.size().height {
        for x in 0..new.size().width {
            let old_cell = old[[x, y]];
            let new_cell = new[[x, y]];

            if old_cell == new_cell {
                continue;
            }

            let cell = new_cell.unwrap_or_default();

            draw_style_diff(style, cell.style, w);
            style = cell.style;

            let cell_pos = TermPos::from([x, y]);
            if cell_pos != cursor_pos {
                w.set_cursor_pos(cell_pos);
                cursor_pos = cell_pos;
            }

            cursor_pos.x = cursor_pos.x.saturating_add(1);

            w.write_char(cell.c);
        }
    }

    match (old.cursor(), new.cursor()) {
        (None, None) => {}
        (None, Some(pos)) => {
            w.set_cursor_pos(pos);
            w.set_cursor_vis(true);
        }
        (Some(_), None) => {
            w.set_cursor_vis(false);
        }
        (Some(_), Some(new)) => {
            w.set_cursor_pos(new);
        }
    }
}

fn draw_no_diff(buf: &Buffer, w: &mut impl Writer) {
    w.clear_all();

    w.set_cursor_home();
    w.set_cursor_vis(false);

    let mut style = Style::default();

    for y in 0..buf.size.height {
        w.set_cursor_pos([0, y]);
        for x in 0..buf.size.width {
            let Some(cell) = buf[[x, y]] else {
                continue;
            };

            draw_style_diff(style, cell.style, w);
            style = cell.style;

            w.write_char(cell.c);
        }
    }

    match buf.cursor {
        Some(pos) => {
            w.set_cursor_pos(pos);
            w.set_cursor_vis(true);
        }
        None => {
            w.set_cursor_vis(false);
        }
    }
}

fn draw_style_diff(old: Style, new: Style, w: &mut impl Writer) {
    if new.fg != old.fg {
        w.set_fg_color(new.fg);
    }
    if new.bg != old.bg {
        w.set_bg_color(new.bg);
    }
    if new.weight != old.weight {
        w.set_weight(new.weight);
    }
    if new.underline != old.underline {
        w.set_underline(new.underline);
    }
}

#[cfg(test)]
mod tests {
    use super::{Buffer, Cell};
    use crate::style::Style;

    #[test]
    fn set_cells() {
        let mut buffer = Buffer::new([10, 10]);

        buffer[[0, 0]] = Some(Cell::new('a', Style::default()));
        buffer[[0, 9]] = Some(Cell::new('b', Style::default()));
        buffer[[1, 0]] = Some(Cell::new('c', Style::default()));
        buffer[[9, 9]] = Some(Cell::new('d', Style::default()));

        assert!(matches!(buffer[[0, 0]], Some(cell) if cell.c == 'a'));
        assert!(matches!(buffer[[0, 9]], Some(cell) if cell.c == 'b'));
        assert!(matches!(buffer[[1, 0]], Some(cell) if cell.c == 'c'));
        assert!(matches!(buffer[[9, 9]], Some(cell) if cell.c == 'd'));

        assert!(buffer.get([10, 10]).is_none());
    }
}
