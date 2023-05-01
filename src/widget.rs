use std::collections::VecDeque;

use crate::buffer::Buffer;
use crate::event::Event;

pub enum Handled {
    Yes,
    No,
}

pub trait Widget<State, Msg> {
    fn handle_event(&mut self, ctx: Context<State, Msg>, event: Event) -> Handled;
    fn handle_msg(&mut self, ctx: Context<State, Msg>, msg: Msg) -> Handled;

    fn update(&mut self, ctx: Context<State, Msg>);
    fn render(&mut self, buf: &mut Buffer);
}

pub struct ContextOwned<State, Msg> {
    pub state: State,
    pub messages: VecDeque<Msg>,
    pub should_quit: bool,
}

pub struct Context<'a, State, Msg>(&'a mut ContextOwned<State, Msg>);

impl<State, Msg> Context<'_, State, Msg> {
    pub fn write_msg(&mut self, message: Msg) {
        self.0.messages.push_back(message);
    }

    pub fn quit(&mut self) {
        self.0.should_quit = true;
    }
}
