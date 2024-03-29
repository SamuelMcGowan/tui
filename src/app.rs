use std::io;
use std::time::{Duration, Instant};

use super::component::{Component, View};
use crate::buffer::Buffer;
use crate::draw_buffer::draw_diff;
use crate::platform::event::Events;
use crate::platform::linux::LinuxTerminal;
use crate::platform::{Terminal, Writer};

pub struct App<C: Component> {
    root: C,
    root_view: C::View,

    buf_old: Buffer,
    buf_new: Buffer,

    term: LinuxTerminal,

    context: Context<C::Message>,
    messages_current: Vec<C::Message>, // A buffer for messages currently being processed.

    refresh_rate: Duration,
}

impl<C: Component> App<C> {
    pub fn new(root: C) -> io::Result<Self> {
        Ok(Self {
            root_view: root.build(),
            root,

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
                self.context.should_rebuild_view = false;
            }

            self.render()?;
        }

        Ok(())
    }

    fn rebuild_view(&mut self) {
        self.root_view = self.root.build();
    }

    fn frame(&mut self) -> io::Result<()> {
        // Work out how long we have.
        let time = Instant::now();
        let deadline = time
            .checked_add(self.refresh_rate)
            .expect("deadline overflowed");

        // Update the component tree.
        self.root.update();

        // Handle events.
        let events = self.term.events();
        while let Some(event) = events.read_with_deadline(deadline)? {
            let _ = self.root_view.on_event(&mut self.context, &event);
        }

        // Handle messages.
        std::mem::swap(&mut self.messages_current, &mut self.context.messages);
        for message in self.messages_current.drain(..) {
            let _ = self.root.on_message(&mut self.context, &message);
        }

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        // Resize buffer.
        let term_size = self.term.size()?;
        self.buf_new.resize_and_clear(term_size);

        // Render component to buffer.
        let mut buf_view = self.buf_new.view(true);
        self.root_view.render(&mut buf_view);

        // Draw changes to terminal.
        // TODO: make immutable view type.
        let buf_old_view = self.buf_old.view(false);
        draw_diff(&buf_old_view, &buf_view, self.term.writer());

        // Swap buffers.
        self.buf_old.clone_from(&self.buf_new);

        self.term.writer().flush()
    }
}

pub struct Context<Message> {
    messages: Vec<Message>,
    should_rebuild_view: bool,
    should_quit: bool,
}

impl<Message> Context<Message> {
    pub fn send(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn rebuild_view(&mut self) {
        self.should_rebuild_view = true;
    }
}
