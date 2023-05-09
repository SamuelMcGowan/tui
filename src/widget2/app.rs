use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

use super::widget::{View, Widget};
use crate::buffer::Buffer;
use crate::draw_buffer::draw_diff;
use crate::platform::event::Events;
use crate::platform::linux::LinuxTerminal;
use crate::platform::{Terminal, Writer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShouldQuit {
    Yes,
    No,
}

pub struct App<W: Widget> {
    root: W,

    root_buf_prev: Buffer,
    root_buf: Buffer,

    term: LinuxTerminal,

    context: Context<W::Msg>,

    refresh_rate: Duration,
}

impl<W: Widget> App<W> {
    pub fn run(mut self) -> io::Result<()> {
        while let ShouldQuit::No = self.frame()? {}
        Ok(())
    }

    pub fn frame(&mut self) -> io::Result<ShouldQuit> {
        // Work out how long we have.
        let time = Instant::now();
        let deadline = time
            .checked_add(self.refresh_rate)
            .expect("deadline overflowed");

        self.root.update();

        // Build widget view.
        let mut widget_with_view = self.root.build();

        // Process events and propagate them through the view tree.
        let events = self.term.events();
        while let Some(event) = events.read_with_deadline(deadline)? {
            let _ = widget_with_view.propagate_event(&mut self.context, &event);
        }

        // Propagate messages.
        let messages = std::mem::take(&mut self.context.messages);
        for message in messages {
            let _ = widget_with_view.widget.propagate_msg(&mut self.context, message);
        }

        render(
            &mut self.root_buf_prev,
            &mut self.root_buf,
            widget_with_view,
            &mut self.term,
        )?;

        Ok(match self.context.quit {
            true => ShouldQuit::Yes,
            false => ShouldQuit::No,
        })
    }
}

fn render<Msg>(
    buf_old: &mut Buffer,
    buf_new: &mut Buffer,
    view: impl View<Msg>,
    term: &mut LinuxTerminal,
) -> io::Result<()> {
    // Resize buffer.
    let term_size = term.size()?;
    buf_new.resize_and_clear(term_size);

    // Render widget to buffer.
    let mut buf_view = buf_new.view(true);
    view.render(&mut buf_view);

    // Draw changes to terminal.
    // TODO: make immutable view type.
    let buf_old_view = buf_old.view(false);
    draw_diff(&buf_old_view, &buf_view, term.writer());

    // Swap buffers.
    buf_old.clone_from(buf_new);

    term.writer().flush()
}

pub struct Context<Msg> {
    messages: VecDeque<Msg>,
    quit: bool,
}
