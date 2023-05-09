use tui::platform::event::{Event, KeyCode, KeyEvent, Modifiers};
use tui::style::Style;
use tui::widget::Handled;
use tui::widget2::app::{App, Context};
use tui::widget2::widget::{View, Widget, WidgetWithView};
use tui::widgets2::container::{Container, LineStyle};
use tui::widgets2::label::Label;

fn main() {
    let mut root = MyApp { s: String::new() };

    let app = App::new(&mut root).unwrap();
    app.run().unwrap();
}

struct MyApp {
    s: String,
}

impl Widget for MyApp {
    type Msg = ();
    type View = Box<dyn View<()>>;

    fn on_event(&mut self, ctx: &mut Context<Self::Msg>, event: &Event) -> Handled {
        if matches!(
            event,
            Event::Key(KeyEvent {
                key_code: KeyCode::Char('Q'),
                modifiers: Modifiers::CTRL
            })
        ) {
            ctx.quit();
        }

        self.s = format!("{event:?}");
        ctx.rebuild_view();
        Handled::Yes
    }

    fn build(&mut self) -> WidgetWithView<Self> {
        let label = Label::new(self.s.to_owned());
        let view = Container::new(label).with_border(LineStyle::Line, Style::default());
        WidgetWithView::new(self, Box::new(view))
    }
}
