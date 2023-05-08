use std::time::Duration;

use tui::app::App;
use tui::platform::event::{Event, KeyCode, KeyEvent, Modifiers};
use tui::widget::{Context, Handled};
use tui::widgets::*;

#[test]
fn foo() {
    let mut vstack = VStack::new();
    vstack.add_widget(TextInput::new(), SizeConstraint::fixed(10));
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_min(10));
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_max(20));
    vstack.set_focus(Some(0));

    let hooked = Hooked::new(vstack).event_hook(vsplit_focus_hook);

    let app: App<(), ()> = App::new((), hooked, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}

fn vsplit_focus_hook<Flow>(
    _ctx: &mut Context<(), ()>,
    widget: &mut Stack<Flow, (), ()>,
    event: &Event,
) -> Handled {
    match event {
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
    }
}
