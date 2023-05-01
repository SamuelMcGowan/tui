use std::collections::VecDeque;

use crate::buffer::Buffer;
use crate::event::Event;

pub enum Handled {
    Yes,
    No,
}

pub trait Widget<State, Msg> {
    fn handle_event(&mut self, ctx: &mut Context<State, Msg>, event: Event) -> Handled;
    fn handle_msg(&mut self, ctx: &mut Context<State, Msg>, msg: Msg) -> Handled;

    fn update(&mut self, ctx: &mut Context<State, Msg>);
    fn render(&mut self, buf: &mut Buffer);
}

pub struct Context<State, Msg> {
    pub state: State,
    messages: VecDeque<Msg>,
    should_quit: bool,
}

impl<State, Msg> Context<State, Msg> {
    pub fn write_msg(&mut self, message: Msg) {
        self.messages.push_back(message);
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
