use std::time::Duration;

use tui::app::App;
use tui::callback::{Callback, EventHook, MsgHook};
use tui::platform::event::{Event, KeyCode, KeyEvent, Modifiers};
use tui::style::Style;
use tui::widget::{Handled, Widget};
use tui::widgets::*;

const REFRESH_RATE: Duration = Duration::from_millis(17);
const BORDER_STYLE: Style = Style::new();

fn main() {
    let app = App::new((), root(), REFRESH_RATE).unwrap();
    app.run().unwrap();
}

enum Message {
    TextEntered(String),
}

fn root() -> impl Widget<(), Message> {
    let stack = stack();

    Hooked::new(stack).event_hook(EventHook::new(|ctx, _widget, event| match event {
        Event::Key(KeyEvent {
            key_code: KeyCode::Char('Q'),
            modifiers: Modifiers::CTRL,
        }) => {
            ctx.quit();
            Handled::Yes
        }
        _ => Handled::No,
    }))
}

fn stack() -> impl Widget<(), Message> {
    let text_field = bordered(text_input());
    let label = bordered(label());

    let mut stack = VStack::new();
    stack.add_widget(text_field, SizeConstraint::fixed(3));
    stack.add_widget(label, SizeConstraint::new());
    stack.set_focus(Some(0));

    stack
}

fn text_input() -> impl Widget<(), Message> {
    TextInput::new().on_enter(Callback::new(|ctx, widget: &mut TextInputState| {
        let s = widget.text.as_str().to_owned();

        if s.is_empty() {
            return;
        }

        widget.text.clear();
        ctx.write_msg(Message::TextEntered(s));
    }))
}

fn label() -> impl Widget<(), Message> {
    Hooked::new(Label::new("--")).msg_hook(MsgHook::new(
        |_ctx, widget: &mut Label, msg| match msg {
            Message::TextEntered(s) => {
                widget.text = s.to_uppercase();
                Handled::Yes
            }
        },
    ))
}

fn bordered(w: impl Widget<(), Message> + 'static) -> impl Widget<(), Message> {
    Container::new(w).with_border(BorderKind::Line, BORDER_STYLE)
}
