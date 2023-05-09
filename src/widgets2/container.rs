use crate::buffer::Cell;
use crate::platform::event::Event;
use crate::style::Style;
use crate::vec2::Vec2;
use crate::widget::Handled;
use crate::widget2::app::Context;
use crate::widget2::widget::View;

#[derive(Default, Debug, Clone, Copy)]
pub enum LineStyle {
    #[default]
    Line,
}

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

pub struct Container<Msg> {
    pub view: Box<dyn View<Msg>>,
    pub border: Option<(LineStyle, Style)>,
}

impl<Msg> Container<Msg> {
    pub fn new(view: impl View<Msg> + 'static) -> Self {
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

impl<Msg> View<Msg> for Container<Msg> {
    fn propagate_event(&mut self, ctx: &mut Context<Msg>, event: &Event) -> Handled {
        self.view.propagate_event(ctx, event)
    }

    fn render(&self, buf: &mut crate::buffer::BufferView) {
        match self.border {
            None => self.view.render(buf),
            Some((kind, style)) => {
                let Vec2 { x: w, y: h } = buf.size();

                if w < 2 || h < 2 {
                    return;
                }

                let mut widget_buf_view = buf.view([1, 1], [w - 1, h - 1], true);
                self.view.render(&mut widget_buf_view);

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
