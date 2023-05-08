use std::io;
use std::time::{Duration, Instant};

use crate::buffer::Buffer;
use crate::draw_buffer::draw_diff;
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

            // TODO: permanent allocation for this
            let msgs = std::mem::take(&mut self.context.messages);
            for msg in msgs {
                let _ = self.root.handle_msg(&mut self.context.borrow(), &msg);
            }

            if self.context.should_quit {
                break;
            }

            self.render()?;
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        // Resize buffer.
        let term_size = self.term.size()?;
        self.root_buf.resize_and_clear(term_size);

        // Render widget to buffer.
        let mut root_buf_view = self.root_buf.view(true);
        self.root.render(&mut root_buf_view);

        // Draw changes to terminal.
        // TODO: make immutable view type.
        let root_buf_prev_view = self.root_buf_prev.view(false);
        draw_diff(&root_buf_prev_view, &root_buf_view, self.term.writer());

        // Swap buffers.
        self.root_buf_prev.clone_from(&self.root_buf);

        self.term.writer().flush()
    }
}
