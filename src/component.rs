use std::ops::DerefMut;

use super::app::Context;
use crate::buffer::BufferView;
use crate::platform::event::Event;

#[must_use]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handled {
    Yes,
    #[default]
    No,
}

pub trait View<Message> {
    fn on_event(&mut self, _ctx: &mut Context<Message>, _event: &Event) -> Handled {
        Handled::No
    }

    fn render(&self, buf: &mut BufferView);
}

impl<T: DerefMut<Target = V>, V: View<Message> + ?Sized, Message> View<Message> for T {
    fn on_event(&mut self, ctx: &mut Context<Message>, event: &Event) -> Handled {
        self.deref_mut().on_event(ctx, event)
    }

    fn render(&self, buf: &mut BufferView) {
        self.deref().render(buf)
    }
}

pub trait Component: Sized {
    type Message;
    type View: View<Self::Message>;

    fn on_message(&mut self, _ctx: &mut Context<Self::Message>, _msg: &Self::Message) -> Handled {
        Handled::No
    }

    fn update(&mut self) {}

    fn build(&self) -> Self::View;
}
