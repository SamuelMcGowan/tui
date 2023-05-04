use std::time::Duration;

use tui::app::App;
use tui::widget::Context;
use tui::widgets::{TextInput, TextInputState};

#[test]
fn foo() {
    let root =
        TextInput::new().on_enter(|_ctx: &mut Context<_, _>, widget: &mut TextInputState| {
            widget.text.clear();
        });

    let app: App<(), ()> = App::new((), root, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}
