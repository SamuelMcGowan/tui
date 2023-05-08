use std::marker::PhantomData;

use crate::buffer2::BufferView;
use crate::platform::event::Event;
use crate::widget::{BoxedWidget, Context, Handled, Widget};

pub struct Horizontal;
pub struct Vertical;

pub type VStack<State, Msg> = Stack<Vertical, State, Msg>;
pub type HStack<State, Msg> = Stack<Horizontal, State, Msg>;

pub struct Stack<Flow, State, Msg> {
    elements: Vec<StackElement<State, Msg>>,
    focused: Option<usize>,
    _phantom: PhantomData<Flow>,
}

impl<Flow, State, Msg> Default for Stack<Flow, State, Msg> {
    fn default() -> Self {
        Self {
            elements: vec![],
            focused: None,
            _phantom: PhantomData,
        }
    }
}

impl<Flow, State, Msg> Stack<Flow, State, Msg> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Flow, State, Msg> Stack<Flow, State, Msg> {
    pub fn add_widget(
        &mut self,
        widget: impl Widget<State, Msg> + 'static,
        constraint: SizeConstraint,
    ) {
        self.elements.push(StackElement {
            widget: Box::new(widget),
            constraint,
            size: 0,
        });
    }

    pub fn set_focus(&mut self, focused: Option<usize>) {
        self.focused = match focused {
            Some(idx) if idx < self.elements.len() => Some(idx),
            _ => None,
        }
    }

    pub fn focus_next(&mut self) {
        // `focused` will only be `Some` when there is at least one element, so the
        // subtraction can't underFlow.
        self.focused = self
            .focused
            .map(|idx| idx.saturating_add(1).min(self.elements.len() - 1))
    }

    pub fn focus_prev(&mut self) {
        self.focused = self.focused.map(|idx| idx.saturating_sub(1))
    }

    pub fn focused(&self) -> Option<&dyn Widget<State, Msg>> {
        self.focused.map(|idx| self.elements[idx].widget.as_ref())
    }

    pub fn focused_mut(&mut self) -> Option<&mut StackElement<State, Msg>> {
        self.focused.map(|idx| &mut self.elements[idx])
    }

    pub fn as_slice(&self) -> &[StackElement<State, Msg>] {
        &self.elements
    }

    pub fn as_slice_mut(&mut self) -> &mut [StackElement<State, Msg>] {
        &mut self.elements
    }

    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: &Event) -> Handled {
        self.focused_mut()
            .map(|elem| elem.widget.handle_event(ctx, event))
            .unwrap_or(Handled::No)
    }

    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: &Msg) -> Handled {
        self.focused_mut()
            .map(|elem| elem.widget.handle_msg(ctx, msg))
            .unwrap_or(Handled::No)
    }

    fn update(&mut self, ctx: &mut Context<State, Msg>) {
        for widget in &mut self.elements {
            widget.widget.update(ctx);
        }
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

impl<State, Msg> Widget<State, Msg> for Stack<Vertical, State, Msg> {
    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: &Event) -> Handled {
        self.handle_event(ctx, event)
    }

    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: &Msg) -> Handled {
        self.handle_msg(ctx, msg)
    }

    fn update(&mut self, ctx: &mut Context<State, Msg>) {
        self.update(ctx)
    }

    fn render(&mut self, buf: &mut BufferView) {
        let size = buf.size();
        self.allocate_space(size.y);

        let mut offset_y = 0;
        for (i, element) in self.elements.iter_mut().enumerate() {
            // This is dumb.
            let focused = self.focused == Some(i);

            let mut buf_view = buf.view([0, offset_y], [size.x, offset_y + element.size], focused);
            element.widget.render(&mut buf_view);

            offset_y += element.size;
            if offset_y >= size.y {
                break;
            }
        }
    }
}

impl<State, Msg> Widget<State, Msg> for Stack<Horizontal, State, Msg> {
    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: &Event) -> Handled {
        self.handle_event(ctx, event)
    }

    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: &Msg) -> Handled {
        self.handle_msg(ctx, msg)
    }

    fn update(&mut self, ctx: &mut Context<State, Msg>) {
        self.update(ctx)
    }

    fn render(&mut self, buf: &mut BufferView) {
        let size = buf.size();
        self.allocate_space(size.x);

        let mut offset_x = 0;
        for (i, element) in self.elements.iter_mut().enumerate() {
            // Still dumb.
            let focused = self.focused == Some(i);

            let mut buf_view = buf.view([offset_x, 0], [offset_x + element.size, size.y], focused);
            element.widget.render(&mut buf_view);

            offset_x += element.size;
            if offset_x >= size.x {
                break;
            }
        }
    }
}

pub struct StackElement<State, Msg> {
    pub widget: BoxedWidget<State, Msg>,
    pub constraint: SizeConstraint,
    pub(crate) size: u16,
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
