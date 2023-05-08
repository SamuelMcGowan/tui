use std::time::Duration;

use tui::app::App;
use tui::callback::Callback;
use tui::widgets::{TextInput, TextInputState};

#[test]
fn foo() {
    let root = TextInput::new().on_enter(Callback::new(|_ctx, widget: &mut TextInputState| {
        widget.text.clear();
    }));

    let app: App<(), ()> = App::new((), root, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}
