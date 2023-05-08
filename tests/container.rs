use std::time::Duration;

use tui::app::App;
use tui::style::{Color, Style, Weight};
use tui::widgets::*;

mod logger;

#[test]
fn foo() {
    logger::init_logger();

    let label_style = Style::new().with_fg(Color::Red).with_weight(Weight::Bold);
    let border_style = Style::new().with_fg(Color::Yellow);

    let label = Label::new("Hello, World!").with_style(label_style);
    let root = Container::new(label).with_border(BorderKind::Line, border_style);

    let app: App<(), ()> = App::new((), root, Duration::from_millis(16)).unwrap();
    app.run().unwrap();
}
