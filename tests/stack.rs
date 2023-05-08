use std::time::Duration;

use tui::app::App;
use tui::callback::EventHook;
use tui::platform::event::{Event, KeyCode, KeyEvent, Modifiers};
use tui::style::{Color, Style};
use tui::widget::{Context, Handled};
use tui::widgets::*;

#[test]
fn foo() {
    let mut vstack = VStack::new();
    vstack.add_widget(
        Label::new("Hello, World!").with_style(Style {
            fg: Color::Cyan,
            ..Default::default()
        }),
        SizeConstraint::fixed(1),
    );
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_min(10));
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_max(20));
    vstack.set_focus(Some(1));

    let hooked = Hooked::new(vstack).event_hook(EventHook::new(
        |_ctx, widget: &mut VStack<(), ()>, event| match event {
            Event::Key(KeyEvent {
                key_code,
                modifiers: Modifiers::SHIFT,
            }) => match key_code {
                KeyCode::Up => {
                    widget.focus_prev();
                    Handled::Yes
                }
                KeyCode::Down => {
                    widget.focus_next();
                    Handled::Yes
                }
                _ => Handled::No,
            },
            _ => Handled::No,
        },
    ));

    let app: App<(), ()> = App::new((), hooked, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}
