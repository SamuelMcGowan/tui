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

pub trait View<Msg> {
    fn on_event(&mut self, _ctx: &mut Context<Msg>, _event: &Event) -> Handled {
        Handled::No
    }

    fn render(&mut self, buf: &mut BufferView);
}

impl<T: DerefMut<Target = V>, V: View<Msg> + ?Sized, Msg> View<Msg> for T {
    fn on_event(&mut self, ctx: &mut Context<Msg>, event: &Event) -> Handled {
        self.deref_mut().on_event(ctx, event)
    }

    fn render(&mut self, buf: &mut BufferView) {
        self.deref_mut().render(buf)
    }
}

pub trait Component: Sized {
    type View: View<Self::Msg>;
    type Msg;

    fn on_msg(&mut self, _ctx: &mut Context<Self::Msg>, _msg: &Self::Msg) -> Handled {
        Handled::No
    }

    fn update(&mut self) {}

    fn build(&self) -> Self::View;
}
