use crate::platform::event::Event;
use crate::widget::{Context, Handled};

macro_rules! callback_type {
    ($(#[$m:meta])* $name:ident $(<$($ty_param:ident),+>)? ($($arg:ident : $arg_ty:ty),*) -> $ret_ty:ty ) => {
        #[allow(clippy::unused_unit)]
        pub struct $name $(<$($ty_param),*>)? {
            callback: Box<dyn FnMut($($arg_ty),*) -> $ret_ty>
        }

        #[automatically_derived]
        #[allow(clippy::unused_unit)]
        impl<$($($ty_param),*)?> $name $(<$($ty_param),*>)? {
            pub fn new<F: FnMut($($arg_ty),*) -> $ret_ty + 'static>(f: F) -> Self {
                Self {
                    callback: Box::new(f),
                }
            }
        }

        #[automatically_derived]
        #[allow(unused_variables)]
        #[allow(clippy::unused_unit)]
        impl $(<$($ty_param),*>)? $name $(<$($ty_param),*>)? {
            pub fn dummy() -> Self {
                Self {
                    callback: Box::new(|$($arg),*| <$ret_ty>::default())
                }
            }

            pub fn call(&mut self, $($arg: $arg_ty),*) -> $ret_ty {
                (self.callback)($($arg),*)
            }
        }
    };
}

callback_type! {
    /// A generic callback.
    Callback<State, Msg, WidgetState>
    (ctx: &mut Context<State, Msg>, widget: &mut WidgetState)
    -> ()
}

callback_type! {
    /// A hook called before a widget processes an event.
    EventHook<State, Msg, WidgetState>
    (ctx: &mut Context<State, Msg>, widget: &mut WidgetState, event: &Event)
    -> Handled
}

callback_type! {
    /// A hook called before a widget processes a message.
    MsgHook<State, Msg, WidgetState>
    (ctx: &mut Context<State, Msg>, widget: &mut WidgetState, event: &Msg)
    -> Handled
}
