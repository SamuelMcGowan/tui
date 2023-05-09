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
