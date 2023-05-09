use crate::buffer::BufferView;
use crate::prelude::*;

pub enum Direction {
    Right,
    Down,
}

pub struct Stack<Msg> {
    elements: Vec<StackElement<Msg>>,
    focused: Option<usize>,
    direction: Direction,
}

struct StackElement<Msg> {
    view: Box<dyn View<Msg>>,
    constraint: SizeConstraint,
    size: u16,
}

#[derive(Default, Clone, Copy)]
pub struct SizeConstraint {
    pub min: Option<u16>,
    pub max: Option<u16>,
}

impl<Msg> Default for Stack<Msg> {
    fn default() -> Self {
        Self {
            elements: vec![],
            focused: None,
            direction: Direction::Down,
        }
    }
}

impl<Msg> View<Msg> for Stack<Msg> {
    fn propagate_event(&mut self, ctx: &mut Context<Msg>, event: &Event) -> Handled {
        if let Some(focused) = self.focused() {
            focused.view.propagate_event(ctx, event)
        } else {
            Handled::No
        }
    }

    fn render(&mut self, buf: &mut BufferView) {
        match self.direction {
            Direction::Down => self.render_down(buf),
            Direction::Right => self.render_right(buf),
        }
    }
}

impl<Msg> Stack<Msg> {
    pub fn set_focus(&mut self, focused: Option<usize>) {
        self.focused = match focused {
            Some(idx) if idx < self.elements.len() => Some(idx),
            _ => None,
        }
    }

    fn focused(&mut self) -> Option<&mut StackElement<Msg>> {
        self.focused.map(|idx| &mut self.elements[idx])
    }

    fn allocate_space(&mut self, available: u16) {
        if self.elements.is_empty() {
            return;
        }

        let min_required = self.allocate_min();
        if min_required < available {
            let available = self.allocate_max(available, min_required);
            self.allocate_remainder(available);
        }
    }

    fn allocate_min(&mut self) -> u16 {
        let mut total = 0;
        for element in &mut self.elements {
            let min = element.constraint.min.unwrap_or(1);

            element.size = min;
            total += min;
        }
        total
    }

    fn allocate_max(&mut self, available: u16, min_required: u16) -> u16 {
        let remainder = available - min_required;
        let share = remainder / self.elements.len() as u16;

        let mut total = 0;
        for element in &mut self.elements {
            let min = element.constraint.min.unwrap_or(1);

            let incr = match element.constraint.max {
                Some(max) => max.saturating_sub(min).min(share),
                None => share,
            };

            element.size += incr;
            total += incr;
        }

        remainder.saturating_sub(total)
    }

    fn allocate_remainder(&mut self, remainder: u16) {
        let num_without_max = self
            .elements
            .iter()
            .filter(|elem| elem.constraint.max.is_none())
            .count();

        // Watch for division-by-zero.
        if num_without_max == 0 {
            return;
        }

        let share = remainder / num_without_max as u16;
        let mut rem = remainder as usize % num_without_max;

        for element in &mut self.elements {
            if element.constraint.max.is_none() {
                element.size += share;

                // This uses up the final remaining space such that the entire space is used if
                // possible.
                if rem > 0 {
                    element.size += 1;
                    rem -= 1;
                }
            }
        }
    }

    fn render_down(&mut self, buf: &mut BufferView) {
        let size = buf.size();
        self.allocate_space(size.y);

        let mut offset_y = 0;
        for (i, element) in self.elements.iter_mut().enumerate() {
            // This is dumb.
            let focused = self.focused == Some(i);

            let mut buf_view = buf.view([0, offset_y], [size.x, offset_y + element.size], focused);
            element.view.render(&mut buf_view);

            offset_y += element.size;
            if offset_y >= size.y {
                break;
            }
        }
    }

    fn render_right(&mut self, buf: &mut BufferView) {
        let size = buf.size();
        self.allocate_space(size.x);

        let mut offset_x = 0;
        for (i, element) in self.elements.iter_mut().enumerate() {
            // Still dumb.
            let focused = self.focused == Some(i);

            let mut buf_view = buf.view([offset_x, 0], [offset_x + element.size, size.y], focused);
            element.view.render(&mut buf_view);

            offset_x += element.size;
            if offset_x >= size.x {
                break;
            }
        }
    }
}

impl SizeConstraint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fixed(size: u16) -> Self {
        Self::new().with_min(size).with_max(size)
    }

    pub fn with_min(mut self, min: u16) -> Self {
        self.min = Some(min);
        self
    }

    pub fn with_max(mut self, max: u16) -> Self {
        self.max = Some(max);
        self
    }
}
