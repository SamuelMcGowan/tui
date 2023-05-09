use crate::buffer::BufferView;
use crate::platform::event::Event;
use crate::widget::Handled;

pub trait View {
    fn on_event(&mut self, _event: &Event) -> Handled {
        Handled::No
    }

    fn render(&self, buf: &mut BufferView);
}

pub trait Widget: Sized {
    type View: View;

    fn on_event(&mut self, event: &Event) -> Handled;

    fn build(&self) -> WidgetWithView<Self>;
}

pub struct WidgetWithView<'a, W: Widget> {
    widget: &'a mut W,
    view: W::View,
}

impl<'a, W: Widget> View for WidgetWithView<'a, W> {
    fn on_event(&mut self, event: &Event) -> Handled {
        match self.view.on_event(event) {
            Handled::Yes => Handled::Yes,
            Handled::No => self.widget.on_event(event),
        }
    }

    fn render(&self, buf: &mut BufferView) {
        self.view.render(buf);
    }
}

pub struct App<W: Widget> {
    widget: W,
}

impl<W: Widget> App<W> {
    pub fn frame(&mut self) {
        let mut widget_with_view = self.widget.build();

        let event = next_event();
        let _ = widget_with_view.on_event(&event);
    }
}

fn next_event() -> Event {
    todo!()
}
