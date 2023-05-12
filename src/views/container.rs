use crate::buffer::Cell;
use crate::prelude::*;

impl LineStyle {
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

pub struct Container<Message> {
    pub view: Box<dyn View<Message>>,
    pub border: Option<(LineStyle, Style)>,
}

impl<Message> Container<Message> {
    pub fn new(view: impl View<Message> + 'static) -> Self {
        Self {
            view: Box::new(view),
            border: None,
        }
    }

    pub fn with_border(mut self, line: LineStyle, style: Style) -> Self {
        self.border = Some((line, style));
        self
    }
}

impl<Message> View<Message> for Container<Message> {
    fn on_event(&mut self, ctx: &mut Context<Message>, event: &Event) -> Handled {
        self.view.on_event(ctx, event)
    }

    fn render(&mut self, buf: &mut crate::buffer::BufferView) {
        match self.border {
            None => self.view.render(buf),
            Some((kind, style)) => {
                let Vec2 { x: w, y: h } = buf.size();

                if w < 2 || h < 2 {
                    return;
                }

                let mut inner_buf_view = buf.view([1, 1], [w - 1, h - 1], true);
                self.view.render(&mut inner_buf_view);

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
