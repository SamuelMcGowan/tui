use crate::buffer::Buffer;
use crate::callback::*;
use crate::platform::event::Event;
use crate::widget::{Context, Handled, Widget};

pub struct Hooked<State, Msg, W: Widget<State, Msg>> {
    widget: W,

    event_hook: BoxedEventHook<State, Msg, W>,
    msg_hook: BoxedMsgHook<State, Msg, W>,

    update_hook: BoxedCallback<State, Msg, W>,
}

impl<State, Msg, W: Widget<State, Msg>> Hooked<State, Msg, W> {
    pub fn new(widget: W) -> Self {
        Self {
            widget,

            event_hook: DummyEventHook.boxed(),
            msg_hook: DummyMsgHook.boxed(),

            update_hook: DummyCallback.boxed(),
        }
    }

    pub fn event_hook(mut self, hook: impl EventHook<State, Msg, W> + 'static) -> Self {
        self.event_hook = hook.boxed();
        self
    }

    pub fn msg_hook(mut self, hook: impl MsgHook<State, Msg, W> + 'static) -> Self {
        self.msg_hook = hook.boxed();
        self
    }

    pub fn update_hook(mut self, hook: impl Callback<State, Msg, W> + 'static) -> Self {
        self.update_hook = hook.boxed();
        self
    }
}

impl<State, Msg, W: Widget<State, Msg>> Widget<State, Msg> for Hooked<State, Msg, W> {
    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: &Event) -> Handled {
        match self.event_hook.call(ctx, &mut self.widget, event) {
            Handled::Yes => Handled::Yes,
            _ => self.widget.handle_event(ctx, event),
        }
    }

    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: &Msg) -> Handled {
        match self.msg_hook.call(ctx, &mut self.widget, msg) {
            Handled::Yes => Handled::Yes,
            _ => self.widget.handle_msg(ctx, msg),
        }
    }

    fn update(&mut self, ctx: &mut Context<State, Msg>) {
        self.widget.update(ctx);
        self.update_hook.call(ctx, &mut self.widget);
    }

    fn render(&mut self, buf: &mut Buffer) {
        self.widget.render(buf);
    }
}
