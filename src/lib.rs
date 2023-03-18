#![no_std]

#[macro_export]
macro_rules! parse_inner_attrs {
    (
        $([ $($before:tt)* ])?
        {
            $(#!$inner_attr:tt)+
            $($rest:tt)*
        }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $($out_macro_and_bang)+ {
            $($($before)*)?
            inner_attrs! { $(#!$inner_attr)+ }
            rest! { $($rest)* }
            $($($after)*)?
        }
    };
    (
        $([ $($before:tt)* ])?
        { $($rest:tt)* }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $($out_macro_and_bang)+ {
            $($($before)*)?
            inner_attrs! {}
            rest! { $($rest)* }
            $($($after)*)?
        }
    };
}
