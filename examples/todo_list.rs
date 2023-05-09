use std::time::Duration;

use tui::app::App;
use tui::callback::{Callback, EventHook, MsgHook};
use tui::platform::event::{Event, KeyCode, KeyEvent, Modifiers};
use tui::style::{Color, Style, Weight};
use tui::widget::{Handled, Widget};
use tui::widgets::*;

const REFRESH_RATE: Duration = Duration::from_millis(17);

const ACCENT: Color = Color::Red;
const BORDER_STYLE: Style = Style::new().with_weight(Weight::Dim);
const BORDER_STYLE_FOCUSED: Style = Style::new().with_fg(ACCENT).with_weight(Weight::Bold);

fn main() {
    let app = App::new((), root(), REFRESH_RATE).unwrap();
    app.run().unwrap();
}

enum Message {
    TextEntered(String),

    FocusUp,
    FocusDown,

    OnFocus,
    OnDefocus,
}

fn root() -> impl Widget<(), Message> {
    let stack = stack();

    Hooked::new(stack).event_hook(EventHook::new(|ctx, _widget, event| match event {
        Event::Key(KeyEvent {
            key_code,
            modifiers,
        }) => match (key_code, modifiers) {
            (KeyCode::Char('Q'), &Modifiers::CTRL) => {
                ctx.quit();
                Handled::Yes
            }
            (KeyCode::Up, modifiers) if modifiers.is_empty() => {
                ctx.write_msg(Message::FocusUp);
                Handled::Yes
            }
            (KeyCode::Down, modifiers) if modifiers.is_empty() => {
                ctx.write_msg(Message::FocusDown);
                Handled::Yes
            }
            _ => Handled::No,
        },
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
        |ctx, widget: &mut VStack<(), Message>, msg| match msg {
            Message::TextEntered(s) => {
                let label = Label::new(format!("TODO: {s}"));

                let border_style = if widget.as_slice().is_empty() {
                    BORDER_STYLE_FOCUSED
                } else {
                    BORDER_STYLE
                };
                let label = Container::new(label).with_border(BorderKind::Line, border_style);

                let label = Hooked::new(label).msg_hook(MsgHook::new(
                    |_ctx, widget: &mut Container<(), Message>, msg| match msg {
                        Message::OnFocus => {
                            widget.border = Some((BorderKind::Line, BORDER_STYLE_FOCUSED));
                            Handled::Yes
                        }
                        Message::OnDefocus => {
                            widget.border = Some((BorderKind::Line, BORDER_STYLE));
                            Handled::Yes
                        }
                        _ => Handled::No,
                    },
                ));

                widget.add_widget(label, SizeConstraint::fixed(3));
                if widget.as_slice().len() == 1 {
                    widget.set_focus(Some(0));
                }

                Handled::Yes
            }

            Message::FocusUp => {
                if let Some(focused) = widget.focused_mut() {
                    let _ = focused.widget.handle_msg(ctx, &Message::OnDefocus);
                }

                widget.focus_prev();

                if let Some(focused) = widget.focused_mut() {
                    let _ = focused.widget.handle_msg(ctx, &Message::OnFocus);
                }

                Handled::Yes
            }

            Message::FocusDown => {
                if let Some(focused) = widget.focused_mut() {
                    let _ = focused.widget.handle_msg(ctx, &Message::OnDefocus);
                }

                widget.focus_next();

                if let Some(focused) = widget.focused_mut() {
                    let _ = focused.widget.handle_msg(ctx, &Message::OnFocus);
                }

                Handled::Yes
            }

            _ => Handled::No,
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
