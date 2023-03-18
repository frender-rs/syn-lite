//! define structs and impl Default

use syn_lite::{expand_if_else, parse_generics};

macro_rules! __impl_phantom_data_with_params_name {
    ($(
        $($lt:lifetime)?
        $($tp0:ident $($tp1:ident)? )?
    ),+) => {
        ($(
            $(& $lt ())?
            $(
                expand_if_else![[$($tp1)?] {()} {::core::marker::PhantomData<$tp0>}]
            )?
        ),+)
    };
}

macro_rules! __impl_phantom {
    (
        $vis:vis $name:ident
        generics! {
            params!        { $($params:tt)* }
            impl_generics! { $($impl_generics:tt)* }
            type_generics! { $($type_generics:tt)* }
            params_name!   $params_name:tt
        }
        rest! {;}
    ) => {
        $vis struct $name <$($params)*> {
            #[allow(unused_parens)]
            _phantom: ::core::marker::PhantomData<
                __impl_phantom_data_with_params_name! $params_name
            >
        }

        impl<$($impl_generics)*> ::core::default::Default for $name <$($type_generics)*> {
            fn default() -> Self {
                Self { _phantom: ::core::marker::PhantomData }
            }
        }
    };
}

macro_rules! phantom {
    (
        $vis:vis struct $name:ident
        $($generics_and_where_and_semi:tt)*
    ) => {
        parse_generics! {
            [$vis $name]
            {
                $($generics_and_where_and_semi)*
            } => __impl_phantom!
        }
    };
}

phantom!(
    pub struct MyPhantomData<T: ?Sized>;
);

phantom!(
    pub struct MyPhantomLifetimeAndData<'a, T: ?Sized>;
);

phantom!(
    pub struct MyPhantomAndConst<'a, T: ?Sized, const N: u8 = 1>;
);

fn main() {
    let _ = MyPhantomAndConst::<str>::default();
}
