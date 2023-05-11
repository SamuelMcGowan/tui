use std::io;
use std::time::{Duration, Instant};

use super::widget::{View, Widget, WidgetWithView};
use crate::buffer::Buffer;
use crate::draw_buffer::draw_diff;
use crate::platform::event::Events;
use crate::platform::linux::LinuxTerminal;
use crate::platform::{Terminal, Writer};

pub struct App<'a, W: Widget> {
    root: Option<WidgetWithView<'a, W>>,

    buf_old: Buffer,
    buf_new: Buffer,

    term: LinuxTerminal,

    context: Context<W::Msg>,
    messages_current: Vec<W::Msg>, // A buffer for messages currently being processed.

    refresh_rate: Duration,
}

impl<'a, W: Widget> App<'a, W> {
    pub fn new(widget: &'a mut W) -> io::Result<Self> {
        Ok(Self {
            root: Some(widget.build()),

            buf_old: Buffer::default(),
            buf_new: Buffer::default(),

            term: LinuxTerminal::init()?,

            context: Context {
                messages: vec![],
                should_rebuild_view: false,
                should_quit: false,
            },
            messages_current: vec![],

            refresh_rate: Duration::from_millis(16),
        })
    }

    pub fn with_refresh_rate(mut self, refresh_rate: Duration) -> Self {
        self.refresh_rate = refresh_rate;
        self
    }

    pub fn run(mut self) -> io::Result<()> {
        loop {
            self.frame()?;

            if self.context.should_quit {
                break;
            }

            if self.context.should_rebuild_view {
                self.rebuild_view();
            }

            self.render()?;
        }

        Ok(())
    }

    fn rebuild_view(&mut self) {
        let root_widget = self.root.take().unwrap().widget;
        self.root = Some(root_widget.build());
    }

    fn frame(&mut self) -> io::Result<()> {
        // Work out how long we have.
        let time = Instant::now();
        let deadline = time
            .checked_add(self.refresh_rate)
            .expect("deadline overflowed");

        let root = self.root.as_mut().unwrap();

        // Update the widget tree.
        root.widget.update();

        // Handle events.
        let events = self.term.events();
        while let Some(event) = events.read_with_deadline(deadline)? {
            let _ = root.propagate_event(&mut self.context, &event);
        }

        // Handle messages.
        std::mem::swap(&mut self.messages_current, &mut self.context.messages);
        for message in self.messages_current.drain(..) {
            let _ = root.widget.propagate_msg(&mut self.context, message);
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        let root = self.root.as_mut().unwrap();

        // Resize buffer.
        let term_size = self.term.size()?;
        self.buf_new.resize_and_clear(term_size);

        // Render widget to buffer.
        let mut buf_view = self.buf_new.view(true);
        root.render(&mut buf_view);

        // Draw changes to terminal.
        // TODO: make immutable view type.
        let buf_old_view = self.buf_old.view(false);
        draw_diff(&buf_old_view, &buf_view, self.term.writer());

        // Swap buffers.
        self.buf_old.clone_from(&self.buf_new);

        self.term.writer().flush()
    }
}

pub struct Context<Msg> {
    messages: Vec<Msg>,
    should_rebuild_view: bool,
    should_quit: bool,
}

impl<Msg> Context<Msg> {
    pub fn send(&mut self, message: Msg) {
        self.messages.push(message);
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn rebuild_view(&mut self) {
        self.should_rebuild_view = true;
    }
}
