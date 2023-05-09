use super::app::Context;
use crate::buffer::BufferView;
use crate::platform::event::Event;
use crate::widget::Handled;

pub trait View<Msg> {
    fn propagate_event(&mut self, _ctx: &mut Context<Msg>, _event: &Event) -> Handled {
        Handled::No
    }

    fn render(&self, buf: &mut BufferView);
}

pub trait Widget: Sized {
    type View: View<Self::Msg>;
    type Msg;

    fn on_event(&mut self, _ctx: &mut Context<Self::Msg>, _event: &Event) -> Handled {
        Handled::No
    }

    fn propagate_msg(&mut self, _ctx: &mut Context<Self::Msg>, msg: Self::Msg) -> Handled;

    fn update(&mut self) {}

    fn build(&self) -> WidgetWithView<Self>;
}

pub struct WidgetWithView<'a, W: Widget> {
    pub(super) widget: &'a mut W,
    view: W::View,
}

impl<'a, W: Widget> View<W::Msg> for WidgetWithView<'a, W> {
    fn propagate_event(&mut self, ctx: &mut Context<W::Msg>, event: &Event) -> Handled {
        match self.view.propagate_event(ctx, event) {
            Handled::Yes => Handled::Yes,
            Handled::No => self.widget.on_event(ctx, event),
        }
    }

    fn render(&self, buf: &mut BufferView) {
        self.view.render(buf);
    }
}
