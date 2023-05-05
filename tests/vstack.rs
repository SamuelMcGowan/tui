use std::time::Duration;

use tui::app::App;
use tui::event::{Event, KeyCode, KeyEvent, Modifiers};
use tui::widget::{Context, Handled};
use tui::widgets::{Hooked, SizeConstraint, TextInput, VStack};

#[test]
fn foo() {
    let mut vstack = VStack::new();
    vstack.add_widget(TextInput::new(), SizeConstraint::fixed(5));
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_min(5));
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_max(10));
    vstack.set_focus(Some(0));

    let hooked = Hooked::new(vstack).event_hook(vsplit_focus_hook);

    let app: App<(), ()> = App::new((), hooked, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}

fn vsplit_focus_hook(
    _ctx: &mut Context<(), ()>,
    widget: &mut VStack<(), ()>,
    event: &Event,
) -> Handled {
    match event {
        Event::Key(KeyEvent {
            key_code,
            modifiers: Modifiers::SHIFT,
        }) => match key_code {
            KeyCode::Up => {
                widget.focus_up();
                Handled::Yes
            }
            KeyCode::Down => {
                widget.focus_down();
                Handled::Yes
            }
            _ => Handled::No,
        },
        _ => Handled::No,
    }
}
