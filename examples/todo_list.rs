use tui::prelude::*;

mod logger;

fn main() {
    logger::init_logger();

    let app = App::new(TodoList::default()).unwrap();
    app.run().unwrap();
}

enum Message {
    New(String),
}

#[derive(Default)]
struct TodoList {
    todos: Vec<Todo>,
}

struct Todo {
    text: String,
    done: bool,
}

impl Widget for TodoList {
    type View = Box<dyn View<Message>>;
    type Msg = Message;

    fn on_msg(&mut self, ctx: &mut Context<Self::Msg>, msg: &Self::Msg) -> Handled {
        match msg {
            Message::New(text) => {
                self.todos.push(Todo {
                    text: text.to_owned(),
                    done: false,
                });

                ctx.rebuild_view();

                Handled::Yes
            }
        }
    }

    fn build(&self) -> Self::View {
        let text_field = TextField::new().on_enter(Message::New);
        let text_field = bordered(text_field);

        let mut todos = Stack::new();
        for todo in &self.todos {
            let view = todo.build();
            let view = bordered(view);
            todos.push(view, SizeConstraint::fixed(3));
        }
        let todos = bordered(todos);

        let mut root_view = Stack::new();
        root_view.push(text_field, SizeConstraint::fixed(3));
        root_view.push(todos, SizeConstraint::new());
        root_view.set_focus(Some(0));

        Box::new(root_view)
    }
}

impl Widget for Todo {
    type View = Label;
    type Msg = Message;

    fn build(&self) -> Self::View {
        let done = if self.done { "[-]" } else { "[ ]" };

        Label::new(format!("{done} {}", &self.text))
    }
}

fn bordered<Msg>(view: impl View<Msg> + 'static) -> Container<Msg> {
    Container::new(view).with_border(LineStyle::Line, Style::new())
}
