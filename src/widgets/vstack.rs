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

    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: Msg) -> Handled {
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
        self.calculate_sizes(size.height);

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

    fn calculate_sizes(&mut self, available: u16) -> LayoutResult {
        // In case of divide-by-zero.
        if self.elements.is_empty() {
            if available == 0 {
                return LayoutResult::Normal;
            } else {
                return LayoutResult::SpaceRemaining(available);
            }
        }

        // Allocate each element its minimum requested space.
        let mut min_total = 0;
        for element in &mut self.elements {
            let min = element.constraint.min.unwrap_or_default();

            element.size = min;
            min_total += min;
        }

        // Allocate remaining space.
        if min_total < available {
            let mut remaining = available - min_total;

            // Give every element its fair share of the remaining space.
            let share = remaining / self.elements.len() as u16;
            let mut num_still_wanting = 0;

            for element in &mut self.elements {
                let share = match element.constraint.max {
                    Some(max) => share.max(max),
                    None => {
                        num_still_wanting += 1;
                        share
                    }
                };
                element.size += share;

                // Since `remaining` includes the remainder of the division anyway we subtract
                // from it instead of having to do modulus and then adding to that.
                remaining -= share;
            }

            // Distribute the remainder.
            let share = remaining / num_still_wanting;
            remaining %= num_still_wanting;

            for element in &mut self.elements {
                if element.constraint.max.is_some() {
                    element.size += share;
                }
            }

            // Done.
            if remaining > 0 {
                LayoutResult::SpaceRemaining(remaining)
            } else {
                LayoutResult::Normal
            }
        } else {
            LayoutResult::Overflowed(min_total - available)
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
