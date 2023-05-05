use std::io;
use std::time::{Duration, Instant};

use crate::buffer::Buffer;
use crate::event::Events as _;
use crate::platform::Events;
use crate::term2::linux::LinuxTerminal;
use crate::term2::Terminal;
use crate::widget::{BoxedWidget, ContextOwned, Widget};

pub struct App<State, Msg> {
    context: ContextOwned<State, Msg>,

    root: BoxedWidget<State, Msg>,
    root_buf: Buffer,

    term: LinuxTerminal,
    events: Events,

    refresh_rate: Duration,
}

impl<State, Msg> App<State, Msg> {
    pub fn new(
        state: State,
        root: impl Widget<State, Msg> + 'static,
        refresh_rate: Duration,
    ) -> io::Result<Self> {
        let mut term = LinuxTerminal::init()?;
        let term_size = term.get_size()?;

        Ok(Self {
            context: ContextOwned::new(state),

            root: Box::new(root),
            root_buf: Buffer::new(term_size),

            term,
            events: Events::new()?,

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

            while let Some(event) = self.events.read_with_deadline(deadline)? {
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
        let term_size = self.term.get_size()?;
        self.root_buf.resize_and_clear(term_size);

        self.root.render(&mut self.root_buf);

        self.root_buf.draw_to_terminal(&mut self.term);

        self.term.flush()
    }
}
