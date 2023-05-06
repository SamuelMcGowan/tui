use std::io;
use std::time::{Duration, Instant};

use crate::buffer::{draw_diff, Buffer};
use crate::platform::event::Events as _;
use crate::platform::linux::LinuxTerminal;
use crate::platform::{Terminal, Writer};
use crate::widget::{BoxedWidget, ContextOwned, Widget};

pub struct App<State, Msg> {
    context: ContextOwned<State, Msg>,

    root: BoxedWidget<State, Msg>,

    root_buf_prev: Buffer,
    root_buf: Buffer,

    term: LinuxTerminal,

    refresh_rate: Duration,
}

impl<State, Msg> App<State, Msg> {
    pub fn new(
        state: State,
        root: impl Widget<State, Msg> + 'static,
        refresh_rate: Duration,
    ) -> io::Result<Self> {
        let term = LinuxTerminal::init()?;
        let term_size = term.size()?;

        Ok(Self {
            context: ContextOwned::new(state),

            root: Box::new(root),

            root_buf_prev: Buffer::new(term_size),
            root_buf: Buffer::new(term_size),

            term,

            refresh_rate,
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        loop {
            let time = Instant::now();
            let deadline = time
                .checked_add(self.refresh_rate)
                .expect("deadline overflowed");

            self.root.update(&mut self.context.borrow());

            let events = self.term.events();
            while let Some(event) = events.read_with_deadline(deadline)? {
                let _ = self.root.handle_event(&mut self.context.borrow(), &event);
            }

            if self.context.should_quit {
                break;
            }

            self.render()?;
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        let term_size = self.term.size()?;
        self.root_buf.resize_and_clear(term_size);

        self.root.render(&mut self.root_buf);

        draw_diff(&self.root_buf_prev, &self.root_buf, self.term.writer());

        self.root_buf_prev.clone_from(&self.root_buf);

        self.term.writer().flush()
    }
}
