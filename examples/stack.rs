use tui::prelude::*;

fn main() {
    let app = App::new(MyApp).unwrap();
    app.run().unwrap();
}

struct MyApp;

impl Component for MyApp {
    type View = Stack<()>;
    type Msg = ();

    fn build(&self) -> Self::View {
        let mut stack = Stack::new();

        stack.push(
            Label::new("Hello, World!").with_style(Style::new().with_fg(Color::Cyan)),
            SizeConstraint::fixed(1),
        );
        stack.push(TextField::new(), SizeConstraint::new().with_min(10));
        stack.push(TextField::new(), SizeConstraint::new().with_max(20));
        stack.set_focus(Some(1));

        stack
    }
}
