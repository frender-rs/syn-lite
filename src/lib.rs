#![no_std]

#[macro_export]
macro_rules! expand_or {
    ([] $($or:tt)*  ) => { $($or)* };
    ([$($expand:tt)+] $($or:tt)* ) => { $($expand)+ };
}

#[macro_export]
macro_rules! expand_if_else {
    ([] $then:tt {$($else:tt)*}  ) => { $($else)* };
    ([$($if:tt)+] {$($then:tt)*} $else:tt ) => { $($then)* };
}

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

/// Generics inside `<...>` but without `<` `>` and without `where` clause.
#[macro_export]
macro_rules! parse_generics {
    (
        $([ $($before:tt)* ])?
        {
            <>
            $($rest:tt)*
        }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $($out_macro_and_bang)+ {
            $( $($before)* )?
            generics! {
                params! {}
                impl_generics! {}
                type_generics! {}
                params_name! {}
            }
            rest! { $($rest)* }
            $( $($after)* )?
        }
    };
    (
        $([ $($before:tt)* ])?
        {
            <$(
                $($lt:lifetime)?
                $($tp1:ident $($tp2:ident)?)?
                $(
                    :
                    $($bound_lt:lifetime)?
                    $(+ $bounds_lt:lifetime)*
                    $(
                        $( + $({$plus_ignore:tt })? )?
                        $( ? $([$relax_ignore:tt])? )?
                        $bounds:path
                    )*
                )?
                $(
                    =
                    $($default_lit:literal)?
                    $({ $($default_const_block:tt)* })?
                    $($default_ty:ty)?
                )?
            ),+ >
            $($rest:tt)*
        }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $($out_macro_and_bang)+ {
            $( $($before)* )?
            generics! {
                params! {$(
                    $($lt)?
                    $($tp1 $($tp2)?)?
                    $(
                        :
                        $($bound_lt)?
                        $(+ $bounds_lt)*
                        $(
                            $( + $({$plus_ignore })? )?
                            $( ? $([$relax_ignore])? )?
                            $bounds
                        )*
                    )?
                    $(
                        =
                        $($default_lit)?
                        $({ $($default_const_block)* })?
                        $($default_ty)?
                    )?
                ),+}
                impl_generics! {$(
                    $($lt)?
                    $($tp1 $($tp2)?)?
                    $(
                        :
                        $($bound_lt)?
                        $(+ $bounds_lt)*
                        $(
                            $( + $({$plus_ignore })? )?
                            $( ? $([$relax_ignore])? )?
                            $bounds
                        )*
                    )?
                ),+}
                type_generics! { $( $($lt)? $($crate::expand_or![[$($tp2)?] $tp1 ])? ),+ }
                params_name! { $( $($lt)? $($tp1 $($tp2)?)? ),+ }
            }
            rest! { $($rest)* }
            $( $($after)* )?
        }
    };
    (
        $([ $($before:tt)* ])?
        { $($rest:tt)* }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $($out_macro_and_bang)+ {
            $( $($before)* )?
            generics! {
                params! {}
                impl_generics! {}
                type_generics! {}
                params_name! {}
            }
            rest! { $($rest)* }
            $( $($after)* )?
        }
    };
}
