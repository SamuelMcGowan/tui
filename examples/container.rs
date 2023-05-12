use tui::prelude::*;

fn main() {
    let app = App::new(MyApp).unwrap();
    app.run().unwrap();
}

struct MyApp;

impl Widget for MyApp {
    type View = Container<()>;
    type Msg = ();

    fn build(&self) -> Self::View {
        let label_style = Style::new().with_fg(Color::Red).with_weight(Weight::Bold);
        let border_style = Style::new().with_fg(Color::Yellow);

        let label = Label::new("Hello, World!").with_style(label_style);

        Container::new(label).with_border(LineStyle::Line, border_style)
    }
}
