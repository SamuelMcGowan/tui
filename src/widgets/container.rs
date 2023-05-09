use crate::buffer::{BufferView, Cell};
use crate::platform::event::Event;
use crate::style::Style;
use crate::vec2::Vec2;
use crate::widget::{BoxedWidget, Context, Handled, Widget};

#[derive(Default, Debug, Clone, Copy)]
pub enum BorderKind {
    #[default]
    Line,
}

impl BorderKind {
    fn h(&self) -> char {
        match self {
            Self::Line => '─',
        }
    }

    fn v(&self) -> char {
        match self {
            Self::Line => '│',
        }
    }

    fn bl(&self) -> char {
        match self {
            Self::Line => '└',
        }
    }

    fn tl(&self) -> char {
        match self {
            Self::Line => '┌',
        }
    }

    fn br(&self) -> char {
        match self {
            Self::Line => '┘',
        }
    }

    fn tr(&self) -> char {
        match self {
            Self::Line => '┐',
        }
    }
}

pub struct Container<State, Msg> {
    pub widget: BoxedWidget<State, Msg>,
    pub border: Option<(BorderKind, Style)>,
}

impl<State, Msg> Container<State, Msg> {
    pub fn new(widget: impl Widget<State, Msg> + 'static) -> Self {
        Self {
            widget: Box::new(widget),
            border: None,
        }
    }

    pub fn with_border(mut self, kind: BorderKind, style: Style) -> Self {
        self.border = Some((kind, style));
        self
    }
}

impl<State, Msg> Widget<State, Msg> for Container<State, Msg> {
    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: &Event) -> Handled {
        self.widget.handle_event(ctx, event)
    }

    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: &Msg) -> Handled {
        self.widget.handle_msg(ctx, msg)
    }

    fn update(&mut self, ctx: &mut Context<State, Msg>) {
        self.widget.update(ctx)
    }

    fn render(&mut self, buf: &mut BufferView) {
        match self.border {
            None => self.widget.render(buf),
            Some((kind, style)) => {
                let Vec2 { x: w, y: h } = buf.size();

                if w < 2 || h < 2 {
                    return;
                }

                let mut widget_buf_view = buf.view([1, 1], [w - 1, h - 1], true);
                self.widget.render(&mut widget_buf_view);

                for x in 1..(w - 1) {
                    buf[[x, 0]] = Some(Cell::new(kind.h(), style));
                    buf[[x, h - 1]] = Some(Cell::new(kind.h(), style));
                }

                for y in 1..(h - 1) {
                    buf[[0, y]] = Some(Cell::new(kind.v(), style));
                    buf[[w - 1, y]] = Some(Cell::new(kind.v(), style));
                }

                buf[[0, 0]] = Some(Cell::new(kind.tl(), style));
                buf[[w - 1, 0]] = Some(Cell::new(kind.tr(), style));
                buf[[0, h - 1]] = Some(Cell::new(kind.bl(), style));
                buf[[w - 1, h - 1]] = Some(Cell::new(kind.br(), style));
            }
        }
    }
}
