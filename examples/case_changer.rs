use std::time::Duration;

use tui::app::App;
use tui::platform::event::{Event, KeyCode, KeyEvent, Modifiers};
use tui::style::Style;
use tui::widget::{Context, Handled, Widget};
use tui::widgets::*;

const REFRESH_RATE: Duration = Duration::from_millis(17);

fn main() {
    let border_style = Style::new();

    let text_field = text_input();
    let label = label();

    let mut stack = VStack::new();
    stack.add_widget(
        Container::new(text_field).with_border(BorderKind::Line, border_style),
        SizeConstraint::fixed(3),
    );
    stack.add_widget(
        Container::new(label).with_border(BorderKind::Line, border_style),
        SizeConstraint::new(),
    );
    stack.set_focus(Some(0));

    let root = root(stack);

    let app = App::new(State, root, REFRESH_RATE).unwrap();
    app.run().unwrap();
}

enum Message {
    TextEntered(String),
}

struct State;

fn root<W: Widget<State, Message>>(w: W) -> Hooked<State, Message, W> {
    Hooked::new(w).event_hook(
        |ctx: &mut Context<State, Message>, _widget: &mut W, event: &Event| match event {
            Event::Key(KeyEvent {
                key_code: KeyCode::Char('Q'),
                modifiers: Modifiers::CTRL,
            }) => {
                ctx.quit();
                Handled::Yes
            }
            _ => Handled::No,
        },
    )
}

fn text_input() -> impl Widget<State, Message> {
    TextInput::new().on_enter(
        |ctx: &mut Context<State, Message>, widget: &mut TextInputState| {
            let s = widget.text.as_str().to_owned();

            if s.is_empty() {
                return;
            }

            widget.text.clear();
            ctx.write_msg(Message::TextEntered(s));
        },
    )
}

fn label() -> impl Widget<State, Message> {
    Hooked::new(Label::new("--")).msg_hook(
        |_ctx: &mut Context<State, Message>, widget: &mut Label, msg: &Message| match msg {
            Message::TextEntered(s) => {
                widget.text = s.to_uppercase();
                Handled::Yes
            }
        },
    )
}
