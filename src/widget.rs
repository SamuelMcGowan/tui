use std::collections::VecDeque;

use crate::buffer::Buffer;
use crate::event::Event;

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl<State, Msg> ContextOwned<State, Msg> {
    pub fn new(state: State) -> Self {
        Self {
            state,
            messages: VecDeque::new(),
            should_quit: false,
        }
    }

    pub fn borrow(&mut self) -> Context<State, Msg> {
        Context(self)
    }
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

pub type BoxedWidget<State, Msg> = Box<dyn Widget<State, Msg>>;
