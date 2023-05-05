use crate::buffer::Buffer;
use crate::event::Event;
use crate::widget::{BoxedWidget, Context, Handled, Widget};

pub struct VStack<State, Msg> {
    elements: Vec<StackElement<State, Msg>>,
    focused: Option<usize>,
}

impl<State, Msg> Default for VStack<State, Msg> {
    fn default() -> Self {
        Self {
            elements: vec![],
            focused: None,
        }
    }
}

impl<State, Msg> Widget<State, Msg> for VStack<State, Msg> {
    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: &Event) -> Handled {
        match self.focused {
            Some(focused) => self.elements[focused].widget.handle_event(ctx, event),
            None => Handled::No,
        }
    }

    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: &Msg) -> Handled {
        match self.focused {
            Some(focused) => self.elements[focused].widget.handle_msg(ctx, msg),
            None => Handled::No,
        }
    }

    fn update(&mut self, ctx: &mut Context<State, Msg>) {
        for element in &mut self.elements {
            element.widget.update(ctx);
        }
    }

    fn render(&mut self, buf: &mut Buffer) {
        let size = buf.size();
        self.allocate_space(size.height);

        let mut offset_y = 0;
        for (i, element) in self.elements.iter_mut().enumerate() {
            // This is dumb.
            let focused = self.focused == Some(i);

            element.buf.resize_and_clear([size.width, element.size]);
            element.widget.render(&mut element.buf);

            buf.blit([0, offset_y], &element.buf, focused, None);

            offset_y += element.size;
            if offset_y >= size.height {
                break;
            }
        }
    }
}

impl<State, Msg> VStack<State, Msg> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_widget(
        &mut self,
        widget: impl Widget<State, Msg> + 'static,
        constraint: SizeConstraint,
    ) {
        self.elements.push(StackElement {
            widget: Box::new(widget),
            constraint,
            buf: Buffer::default(),
            size: 0,
        });
    }

    pub fn set_focus(&mut self, focused: Option<usize>) {
        self.focused = match focused {
            Some(idx) if idx < self.elements.len() => Some(idx),
            _ => None,
        };
    }

    pub fn focus_up(&mut self) {
        self.focused = self.focused.map(|focus| focus.saturating_sub(1));
    }

    pub fn focus_down(&mut self) {
        // `focused` will only be `Some` when there is at least one element, so the
        // subtraction can't underflow.
        self.focused = self
            .focused
            .map(|focus| (focus + 1).min(self.elements.len() - 1));
    }

    fn allocate_space(&mut self, available: u16) {
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
}

pub struct StackElement<State, Msg> {
    pub widget: BoxedWidget<State, Msg>,
    pub constraint: SizeConstraint,

    buf: Buffer,
    size: u16,
}

#[derive(Default, Clone, Copy)]
pub struct SizeConstraint {
    pub min: Option<u16>,
    pub max: Option<u16>,
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

enum LayoutResult {
    Normal,
    Overflowed(u16),
    SpaceRemaining(u16),
}
