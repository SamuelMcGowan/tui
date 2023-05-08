use std::ops::{Index, IndexMut};

use crate::style::Style;
use crate::vec2::Vec2;

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
    buf: Vec<Option<Cell>>,
    size: Vec2,
    cursor: Option<Vec2>,
}

impl Buffer {
    pub fn new(size: impl Into<Vec2>) -> Self {
        let size: Vec2 = size.into();
        Self {
            buf: vec![None; size.area()],
            size,
            cursor: None,
        }
    }

    pub fn resize_and_clear(&mut self, size: impl Into<Vec2>) {
        let size: Vec2 = size.into();

        if size != self.size {
            self.buf.clear();
            self.buf.extend(std::iter::repeat(None).take(size.area()));
            self.size = size;
        } else {
            self.buf.fill(None);
        }

        self.cursor = None;
    }

    pub fn view(&mut self, set_cursor: bool) -> BufferView {
        let start = [0, 0].into();
        let end = self.size;

        BufferView {
            buf: self,

            start,
            end,

            set_cursor,
        }
    }

    pub fn cursor(&self) -> Option<Vec2> {
        self.cursor
    }
}

#[derive(Debug)]
pub struct BufferView<'a> {
    buf: &'a mut Buffer,

    start: Vec2,
    end: Vec2,

    set_cursor: bool,
}

impl<'a> BufferView<'a> {
    pub fn view(
        &mut self,
        start: impl Into<Vec2>,
        end: impl Into<Vec2>,
        set_cursor: bool,
    ) -> BufferView {
        let start = start.into();
        let end = end.into();

        BufferView {
            buf: self.buf,
            start: (self.start + start).min(self.end),
            end: (self.start + end).min(self.end),
            set_cursor: set_cursor && self.set_cursor,
        }
    }

    pub fn size(&self) -> Vec2 {
        self.end - self.start
    }

    pub fn get(&self, index: impl Into<Vec2>) -> Option<&Option<Cell>> {
        self.buf.buf.get(self.index(index)?)
    }

    pub fn get_mut(&mut self, index: impl Into<Vec2>) -> Option<&mut Option<Cell>> {
        let index = self.index(index)?;
        self.buf.buf.get_mut(index)
    }

    fn index(&self, index: impl Into<Vec2>) -> Option<usize> {
        let pos: Vec2 = self.start + index.into();

        if pos.x >= self.end.x || pos.y >= self.end.y {
            return None;
        }

        let index = pos.y as usize * self.buf.size.x as usize + pos.x as usize;

        Some(index)
    }

    pub fn set_cursor(&mut self, cursor: Option<impl Into<Vec2>>) {
        if !self.set_cursor {
            return;
        }

        self.buf.cursor = cursor.and_then(|pos| {
            let pos: Vec2 = self.start + pos.into();

            if pos.x < self.end.x && pos.y < self.end.y {
                Some(pos)
            } else {
                None
            }
        })
    }
}

impl<I: Into<Vec2>> Index<I> for BufferView<'_> {
    type Output = Option<Cell>;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).expect("out of bounds")
    }
}

impl<I: Into<Vec2>> IndexMut<I> for BufferView<'_> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index).expect("out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::{Buffer, Cell};
    use crate::style::Style;

    macro_rules! assert_matches {
        ($e:expr, $p:pat $( if $guard:expr )?) => {{
            match $e {
                $p $( if $guard )? => {}
                value => {
                    panic!(
                        "assertion failed: expected {}, found {:?}",
                        stringify!($p $( if $guard )?),
                        value
                    );
                }
            }
        }};
    }

    #[test]
    fn set_cells() {
        let mut buffer = Buffer::new([10, 10]);
        let mut view = buffer.view(true);

        assert_eq!(view.size(), [10, 10].into());

        view[[0, 0]] = Some(Cell::new('a', Style::default()));
        view[[0, 9]] = Some(Cell::new('b', Style::default()));
        view[[1, 0]] = Some(Cell::new('c', Style::default()));
        view[[9, 9]] = Some(Cell::new('d', Style::default()));

        view.set_cursor(Some([9, 9]));
        assert_eq!(view.buf.cursor(), Some([9, 9].into()));

        view.set_cursor(Some([10, 10]));
        assert_eq!(view.buf.cursor(), None);

        assert_matches!(view[[0, 0]], Some(cell) if cell.c == 'a');
        assert_matches!(view[[0, 9]], Some(cell) if cell.c == 'b');
        assert_matches!(view[[1, 0]], Some(cell) if cell.c == 'c');
        assert_matches!(view[[9, 9]], Some(cell) if cell.c == 'd');

        assert!(view.get([10, 10]).is_none());
    }

    #[test]
    fn view() {
        let mut buffer = Buffer::new([10, 10]);
        let mut view = buffer.view(true);
        let mut view2 = view.view([1, 1], [9, 9], true);

        assert_eq!(view2.size(), [8, 8].into());

        view2[[0, 0]] = Some(Cell::new('a', Style::default()));
        view2[[0, 7]] = Some(Cell::new('b', Style::default()));
        view2[[1, 0]] = Some(Cell::new('c', Style::default()));
        view2[[7, 7]] = Some(Cell::new('d', Style::default()));

        view2.set_cursor(Some([8, 8]));
        assert_eq!(view2.buf.cursor(), None);

        view2.set_cursor(Some([7, 7]));
        assert_eq!(view2.buf.cursor(), Some([8, 8].into()));

        assert_matches!(view2[[0, 0]], Some(cell) if cell.c == 'a');
        assert_matches!(view2[[0, 7]], Some(cell) if cell.c == 'b');
        assert_matches!(view2[[1, 0]], Some(cell) if cell.c == 'c');
        assert_matches!(view2[[7, 7]], Some(cell) if cell.c == 'd');

        assert!(view2.get([8, 8]).is_none());

        assert_matches!(view[[1, 1]], Some(cell) if cell.c == 'a');
        assert_matches!(view[[1, 8]], Some(cell) if cell.c == 'b');
        assert_matches!(view[[2, 1]], Some(cell) if cell.c == 'c');
        assert_matches!(view[[8, 8]], Some(cell) if cell.c == 'd');
    }
}
