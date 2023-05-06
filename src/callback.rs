use paste::paste;

use crate::platform::event::Event;
use crate::widget::{Context, Handled};

macro_rules! callback_type {
    ($(#[$m:meta])* $name:ident $(<$($ty_param:ident),+>)? ($($arg:ident : $arg_ty:ty),*) -> $ret_ty:ty ) => {
        $(#[$m])*
        pub trait $name $(<$($ty_param),*>)? {
            /// Invoke the callback.
            fn call(&mut self, $($arg: $arg_ty),*) -> $ret_ty;

            paste!{
                /// Box this callback.
                fn boxed(self) -> [<Boxed $name>] $(<$($ty_param),*>)?
                    where Self: Sized + 'static
                {
                    Box::new(self)
                }
            }
        }

        #[automatically_derived]
        #[allow(clippy::unused_unit)]
        impl<F, $($($ty_param),*)?> $name $(<$($ty_param),*>)? for F where F: FnMut($($arg_ty),*) -> $ret_ty {
            fn call(&mut self, $($arg: $arg_ty),*) -> $ret_ty {
                self($($arg),*)
            }
        }

        paste! {
            /// Dummy implementation of this trait.
            pub struct [< Dummy $name >];

            #[automatically_derived]
            impl$(<$($ty_param),*>)? $name $(<$($ty_param),*>)? for [< Dummy $name >] {
                fn call(&mut self, $($arg: $arg_ty),*) -> $ret_ty {
                    <$ret_ty>::default()
                }
            }
        }

        paste! {
            /// A boxed trait object for the corresponding trait.
            pub type [< Boxed $name >] $(<$($ty_param),*>)? = Box<dyn $name $(<$($ty_param),*>)?>;
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
