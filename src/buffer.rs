use std::ops::{Index, IndexMut};

use crate::platform::{TermPos, TermSize, TerminalWriter};
use crate::style::Style;

#[derive(Debug, Clone, Copy)]
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

#[derive(Default, Debug, Clone)]
pub struct Buffer {
    data: Box<[Option<Cell>]>,

    size: TermSize,
    cursor: Option<TermPos>,
}

impl Buffer {
    pub fn new(size: impl Into<TermSize>) -> Self {
        Self::new_inner(size.into(), vec![])
    }

    pub fn resize_and_clear(&mut self, size: impl Into<TermSize>) {
        let size = size.into();

        if size != self.size {
            let data = std::mem::take(&mut self.data).into_vec();
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
            data: data.into_boxed_slice(),

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
        self.data.get_mut(self.index(index)?)
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

    pub fn draw_to_terminal<W: TerminalWriter>(&self, term: &mut W) {
        term.clear_all();
        term.set_cursor_vis(false);

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                term.set_cursor_pos([x, y]);
                let cell = self[[x, y]].unwrap_or_default();

                term.set_fg_color(cell.style.fg);
                term.set_bg_color(cell.style.bg);
                term.set_weight(cell.style.weight);
                term.set_underline(cell.style.underline);

                term.write_char(cell.c);
            }
        }

        match self.cursor {
            Some(pos) => {
                term.set_cursor_pos(pos);
                term.set_cursor_vis(true);
            }
            None => {
                term.set_cursor_vis(false);
            }
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
