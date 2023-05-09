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
    let mut stack = VStack::new();
    stack.add_widget(bordered(text_input()), SizeConstraint::fixed(3));
    stack.add_widget(bordered(todo_list()), SizeConstraint::new());
    stack.set_focus(Some(0));

    stack
}

fn todo_list() -> impl Widget<(), Message> {
    let stack = VStack::new();

    Hooked::new(stack).msg_hook(MsgHook::new(
        |_ctx, widget: &mut VStack<(), Message>, msg| match msg {
            Message::TextEntered(s) => {
                widget.add_widget(Label::new(format!("TODO: {s}")), SizeConstraint::fixed(1));
                Handled::Yes
            }
        },
    ))
}

fn text_input() -> impl Widget<(), Message> {
    TextInput::new().on_enter(Callback::new(|ctx, widget: &mut TextInputState| {
        let s = widget.text.as_str().trim().to_owned();

        if s.is_empty() {
            return;
        }

        widget.text.clear();
        ctx.write_msg(Message::TextEntered(s));
    }))
}

fn bordered(w: impl Widget<(), Message> + 'static) -> impl Widget<(), Message> {
    Container::new(w).with_border(BorderKind::Line, BORDER_STYLE)
}
