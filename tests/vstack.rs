use std::time::Duration;

use tui::app::App;
use tui::widgets::{SizeConstraint, TextInput, VStack};

#[test]
fn foo() {
    let mut vstack = VStack::new();
    vstack.add_widget(TextInput::new(), SizeConstraint::fixed(5));
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_min(5));
    vstack.add_widget(TextInput::new(), SizeConstraint::new().with_max(10));
    vstack.set_focus(Some(0));

    let app: App<(), ()> = App::new((), vstack, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}
