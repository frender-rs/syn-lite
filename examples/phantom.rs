//! define structs and impl Default

use syn_lite::expand_if_else;

macro_rules! __impl_phantom {
    (
        $vis:vis $name:ident
        $(
            lt {$lt:tt}
            parsed_generics {
                generics      { $($params:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info {
                    $({
                        $(lifetime_attrs $lifetime_attrs:tt)?
                        lifetime {$lifetime:lifetime}
                        $(bounds {$($lifetime_bounds:tt)*})?
                    })*
                    $({
                        $(const_attrs $const_attrs:tt)?
                        $(type_attrs $type_attrs:tt)?
                        $(const $const:tt)?
                        name {$generic_name:ident}
                        $(bounds {$($generic_bounds:tt)*})?
                        $(default_ty $default_ty:tt)?
                        $(default_expr $default_expr:tt)?
                    })*
                }
            }
            gt {$gt:tt}
        )?
        rest {;}
    ) => {
        $vis struct $name $($lt $($params)* $gt)? {
            _phantom: ::core::marker::PhantomData<(
                $($(::core::marker::PhantomData<&$lifetime ()>,)*)?
                $($(
                    syn_lite::expand_if_else! {
                        [$($const)?]
                        {()} // for const generics
                        {::core::marker::PhantomData<$generic_name>} // for type params
                    }
                    ,
                )*)?
            )>
        }

        impl $($lt $($impl_generics)* $gt)?  ::core::default::Default for $name $($lt $($type_generics)* $gt)? {
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
        syn_lite::parse_optional_angle_bracketed_generics! {
            on_finish { __impl_phantom! }
            prepend { $vis $name }
            input { $($generics_and_where_and_semi)* }
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
