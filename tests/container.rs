use std::time::Duration;

use tui::app::App;
use tui::style::{Color, Style};
use tui::widgets::*;

mod logger;

#[test]
fn foo() {
    logger::init_logger();

    let blue = Style {
        fg: Color::Blue,
        ..Default::default()
    };

    let green = Style {
        fg: Color::Green,
        ..Default::default()
    };

    let root = Container::new(Label::new("Hello, World!").with_style(blue))
        .with_border(BorderKind::Line, green);

    let app: App<(), ()> = App::new((), root, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}
