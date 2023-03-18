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

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_where_clause_finish {
    ({
        [$($out_macro_and_bang:tt)+]
        [$($before:tt)*]
        [$($after:tt)*]
    } $where_clause:tt $rest:tt) => {
        $($out_macro_and_bang)+ {
            $($before)*
            where_clause! $where_clause
            rest! $rest
            $($after)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_where_predicate {
    ({ $($parsed:tt)* } {
        ,
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_predicate! { { $($parsed)*
        ,
    } { $($rest)* } $output } };
    // 'a: 'b + 'c
    ({ $($parsed:tt)* } {
        $lt:lifetime : $lt_bound:lifetime $(+ $lt_bounds:lifetime)+
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_predicate! { { $($parsed)*
        $lt : $lt_bound $(+ $lt_bounds)+
    } { $($rest)* } $output } };
    // 'a: 'b
    ({ $($parsed:tt)* } {
        $lt:lifetime : $lt_bound:lifetime
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_predicate! { { $($parsed)*
        $lt : $lt_bound
    } { $($rest)* } $output } };
    // for<'a>
    ({ $($parsed:tt)* } {
        for < $($lt:lifetime),* $(,)? >
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_predicate! { { $($parsed)*
        for < $($lt),* >
    } { $($rest)* } $output } };
    // __![$raw_where_clause]: __
    ({ $($parsed:tt)* } {
        __![$($raw_where_clause:tt)*]: __
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_predicate! { { $($parsed)*
        __![$($raw_where_clause)*]: __
    } { $($rest)* } $output } };
    // $ty : $bounds $EOF
    ({ $($parsed:tt)* } {
        $ty:ty : $($bound_lt:lifetime)? $(+ $bounds_lt:lifetime)* $( $( + $({$plus_ignore:tt })? )? $( ? $([$relax_ignore:tt])? )? $bounds:path )*
    } $output:tt) => { $crate::__impl_parse_where_predicate! { { $($parsed)*
        $ty : $($bound_lt)? $(+ $bounds_lt)* $( $( + $({$plus_ignore })? )? $( ? $([$relax_ignore])? )? $bounds )*
    } {} $output } };
    // $ty : $bounds ,
    ({ $($parsed:tt)* } {
        $ty:ty : $($bound_lt:lifetime)? $(+ $bounds_lt:lifetime)* $( $( + $({$plus_ignore:tt })? )? $( ? $([$relax_ignore:tt])? )? $bounds:path )*
        ,
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_predicate! { { $($parsed)*
        $ty : $($bound_lt)? $(+ $bounds_lt)* $( $( + $({$plus_ignore })? )? $( ? $([$relax_ignore])? )? $bounds )*
        ,
    } { $($rest)* } $output } };
    // $ty : $bounds {}
    ({ $($parsed:tt)* } {
        $ty:ty : $($bound_lt:lifetime)? $(+ $bounds_lt:lifetime)* $( $( + $({$plus_ignore:tt })? )? $( ? $([$relax_ignore:tt])? )? $bounds:path )*
        {$($brace:tt)*}
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output { $($parsed)*
        $ty : $($bound_lt)? $(+ $bounds_lt)* $( $( + $({$plus_ignore })? )? $( ? $([$relax_ignore])? )? $bounds )*
    } { {$($brace)*} $($rest)* } } };
    // $ty : $bounds ;
    ({ $($parsed:tt)* } {
        $ty:ty : $($bound_lt:lifetime)? $(+ $bounds_lt:lifetime)* $( $( + $({$plus_ignore:tt })? )? $( ? $([$relax_ignore:tt])? )? $bounds:path )*
        ;
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output { $($parsed)*
        $ty : $($bound_lt)? $(+ $bounds_lt)* $( $( + $({$plus_ignore })? )? $( ? $([$relax_ignore])? )? $bounds )*
    } { ; $($rest)* } } };
    // $ty : $bounds :
    ({ $($parsed:tt)* } {
        $ty:ty : $($bound_lt:lifetime)? $(+ $bounds_lt:lifetime)* $( $( + $({$plus_ignore:tt })? )? $( ? $([$relax_ignore:tt])? )? $bounds:path )*
        :
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output { $($parsed)*
        $ty : $($bound_lt)? $(+ $bounds_lt)* $( $( + $({$plus_ignore })? )? $( ? $([$relax_ignore])? )? $bounds )*
    } { : $($rest)* } } };
    // $ty : $bounds =
    ({ $($parsed:tt)* } {
        $ty:ty : $($bound_lt:lifetime)? $(+ $bounds_lt:lifetime)* $( $( + $({$plus_ignore:tt })? )? $( ? $([$relax_ignore:tt])? )? $bounds:path )*
        =
        $($rest:tt)*
    } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output { $($parsed)*
        $ty : $($bound_lt)? $(+ $bounds_lt)* $( $( + $({$plus_ignore })? )? $( ? $([$relax_ignore])? )? $bounds )*
    } { = $($rest)* } } };
    // ($parsed:tt { {$($brace:tt)*} $($rest:tt)* } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output $parsed { {$($brace)*} $($rest)* } } };
    // ($parsed:tt { ; $($rest:tt)* } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output $parsed { ; $($rest)* } } };
    // ($parsed:tt { : $($rest:tt)* } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output $parsed { : $($rest)* } } };
    // ($parsed:tt { = $($rest:tt)* } $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output $parsed { = $($rest)* } } };
    ($parsed:tt $rest:tt $output:tt) => { $crate::__impl_parse_where_clause_finish! { $output $parsed $rest } };
}

#[macro_export]
macro_rules! parse_where_clause {
    (
        $([ $($before:tt)* ])?
        { where $($parse:tt)* }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $crate::__impl_parse_where_predicate! {
            { where }
            {$($parse)*}
            { [$($out_macro_and_bang)+][$( $($before)* )?][$( $($after)* )?] }
        }
    };
    (
        $([ $($before:tt)* ])?
        { $($rest:tt)* }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $($out_macro_and_bang)+ {
            $($($before:tt)*)?
            where_clause! {}
            rest!         { $($rest)* }
            $($($after:tt)*)?
        }
    };
}
