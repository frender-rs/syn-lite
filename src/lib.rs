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
        $($raw_where_clause)*
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

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_item_fn_finish {
    (
        [
            [
                output_macro_and_bang! { $($out_macro_and_bang:tt)+ }
                before! { $($before:tt)* }
                after! { $($after:tt)* }
                outer_attrs! $outer_attrs:tt
                vis! $vis:tt
                ident! $ident:tt
            ]
            generics! $generics:tt
            paren_inputs! $paren_inputs:tt
            output! $output:tt
        ]
        where_clause! $where_clause:tt
        rest! $rest:tt // rest tokens after ItemFn
        inner_attrs! $inner_attrs:tt
        rest! $stmts:tt // rest tokens after inner_attrs, that are stmts
    ) => {
        $($out_macro_and_bang)+ {
            $($before)*
            item_fn! {
                outer_attrs! $outer_attrs
                vis! $vis
                sig! {
                    ident! $ident
                    generics! $generics
                    paren_inputs! $paren_inputs
                    output! $output
                    where_clause! $where_clause
                }
                inner_attrs! $inner_attrs
                stmts! $stmts
            }
            rest! $rest
            $($after)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_item_fn_where_clause_parsed {
    (
        $other:tt
        where_clause! $where_clause:tt
        rest! {
            $fn_body:tt
            $($rest:tt)*
        }
    ) => {
        $crate::parse_inner_attrs! {
            [$other where_clause! $where_clause rest! { $($rest)* }]
            $fn_body
            => $crate::__impl_parse_item_fn_finish!
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_item_fn_generics_parsed {
    (
        $other:tt
        generics! $generics:tt
        rest! {
            $paren_inputs:tt
            $( -> $output_ty:ty )?
            where
            $($rest:tt)*
        }
    ) => {
        $crate::parse_where_clause! {
            [[
                $other
                generics! $generics
                paren_inputs! { $paren_inputs }
                output! { $( -> $output_ty )? }
            ]]
            { where $($rest)* }
            => $crate::__impl_parse_item_fn_where_clause_parsed!
        }
    };
    (
        $other:tt
        generics! $generics:tt
        rest! {
            $paren_inputs:tt
            $( -> $output_ty:ty )?
            { $($fn_body:tt)* }
            $($rest:tt)*
        }
    ) => {
        $crate::parse_inner_attrs! {
            [
                [
                    $other
                    generics! $generics
                    paren_inputs! { $paren_inputs }
                    output! { $( -> $output_ty )? }
                ]
                where_clause! {}
                rest! { $($rest)* } // after ItemFn
            ]
            { $($fn_body)* }
            => $crate::__impl_parse_item_fn_finish!
        }
    };
}

#[macro_export]
macro_rules! parse_item_fn {
    (
        $([ $($before:tt)* ])?
        {
            $(#$outer_attrs:tt)*
            $vis:vis fn $name:ident
            $($rest:tt)*
        }
        $([ $($after:tt)* ])?
        => $($out_macro_and_bang:tt)+
    ) => {
        $crate::parse_generics! {
            [[
                output_macro_and_bang! { $($out_macro_and_bang)+ }
                before! { $($($before)*)? }
                after! { $($($after)*)? }
                outer_attrs! { $(#$outer_attrs)* }
                vis! { $vis }
                ident! { $name }
            ]]
            { $($rest)* }
            []
            => $crate::__impl_parse_item_fn_generics_parsed!
        }
    };
}

// region: common utils
#[doc(hidden)]
#[macro_export]
macro_rules! __start_parsing_with {
    (
        parse_with {$($parse_with:tt)*}
        args {
            on_finish $on_finish:tt
            $(prepend $output_prepend:tt)?
            input $input:tt
            $(append $output_append:tt)?
        }
        $(after_input { $($after_input:tt)* })?
    ) => {
        $($parse_with)* {
            {} // initial state
            $input
            $input // clone input
            $($($after_input)*)?
            {
                on_finish $on_finish
                $(prepend $output_prepend)?
                $(append $output_append)?
            } // on finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __resolve_finish {
    (
        on_finish {
            on_finish { $($macro_and_bang:tt)* }
            $(prepend { $($output_prepend:tt)* })?
            $(append  { $($output_append:tt )* })?
        }
        output { $($output:tt)* }
    ) => {
        $($macro_and_bang)* {
            $($($output_prepend)*)?
            $($output)*
            $($($output_append)*)?
        }
    };
}

// endregion

// region: consume_till_outer_gt

/// Consume tokens till an outer `>`.
///
/// Note that this macro might split the following tokens
/// if the last `>` is an outer `>` which doesn't have a matched previous `>`:
/// - `->`
/// - `=>`
/// - `>=`
/// - `>>`
/// - `>>=`
#[macro_export]
macro_rules! consume_till_outer_gt {
    ($($args:tt)*) => {
        $crate::__start_parsing_with! {
            parse_with { $crate::__impl_consume_till_outer_gt! }
            args {
                $($args)*
            }
            after_input {
                [] // inner `<` list, start with an empty list
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_consume_till_outer_gt {
    // region: unmatched > or >>
    // >
    (
        $consumed:tt
        {> $($_rest:tt)*}
        $rest:tt
        [] // no inner `<` before this `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt $consumed
                gt_and_rest $rest
            }
        }
    };
    // >=
    (
        $consumed:tt
        {>= $($_rest:tt)*}
        $rest:tt
        [] // no inner `<` before this `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt $consumed
                gt_and_rest $rest
            }
        }
    };
    // ->
    (
        {$($consumed:tt)*}
        {-> $($rest:tt)*}
        $_rest:tt
        [] // no inner `<` before this `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt { $($consumed)* - } // split
                gt_and_rest { > $($rest)* } // split
            }
        }
    };
    // =>
    (
        {$($consumed:tt)*}
        {=> $($rest:tt)*}
        $_rest:tt
        [] // no inner `<` before this `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt { $($consumed)* = } // split
                gt_and_rest { > $($rest)* } // split
            }
        }
    };
    // >> only one matched
    (
        {$($consumed:tt)*}
        {>> $($_rest:tt)*}
        {>> $($rest:tt )*}
        [<] // no inner `<` before the second `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt {$($consumed)* >}
                gt_and_rest { > $($rest)* }
            }
        }
    };
    // >> neither matched
    (
        $consumed:tt
        {>>    $($_rest:tt)*}
        $rest:tt
        [] // no inner `<` before the first `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt $consumed
                gt_and_rest $rest
            }
        }
    };
    // >>= only one matched
    (
        {$($consumed:tt)*}
        {>>= $($_rest:tt)*}
        {>>= $($rest:tt )*}
        [<] // no inner `<` before the second `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt {$($consumed)* >}
                gt_and_rest { >= $($rest)* } // split >>= to > and >=
            }
        }
    };
    // >>= neither matched
    (
        $consumed:tt
        {>>=   $($_rest:tt)*}
        $rest:tt
        [] // no inner `<` before the first `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_gt $consumed
                gt_and_rest $rest
            }
        }
    };
    // endregion

    // region: < and <<
    // <
    (
        {$($consumed:tt)*}
        {<     $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [$($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)* $t]
            $finish
        }
    };
    // <-
    (
        {$($consumed:tt)*}
        {<-    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [$($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)* <]
            $finish
        }
    };
    // <=
    (
        {$($consumed:tt)*}
        {<=    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [$($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)* <]
            $finish
        }
    };
    // <<
    (
        {$($consumed:tt)*}
        {<<    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [$($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)* < <] // split `<<` into two `<`
            $finish
        }
    };
    // <<=
    (
        {$($consumed:tt)*}
        {<<=   $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [$($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)* < <] // split `<<` into two `<`
            $finish
        }
    };
    // endregion

    // region: `>` matched a previous `<` or `>>` matched previous `<<`
    // `>` matched a previous `<`
    (
        {$($consumed:tt)*}
        {>     $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [< $($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)*]
            $finish
        }
    };
    // `->` matched a previous `<`
    (
        {$($consumed:tt)*}
        {->    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [< $($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)*]
            $finish
        }
    };
    // `>=` matched a previous `<`
    (
        {$($consumed:tt)*}
        {>=    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [< $($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)*]
            $finish
        }
    };
    // `=>` matched a previous `<`
    (
        {$($consumed:tt)*}
        {=>    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [< $($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)*]
            $finish
        }
    };
    // `>>` matches two previous `<`
    (
        {$($consumed:tt)*}
        {>>    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [< < $($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)*]
            $finish
        }
    };
    // `>>=` matches two previous `<`
    (
        {$($consumed:tt)*}
        {>>=   $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        [< < $($got_lt:tt)*]
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            [$($got_lt)*]
            $finish
        }
    };
    // endregion

    // anything else
    (
        {$($consumed:tt)*}
        $_rest:tt
        {$t:tt $($rest:tt)*}
        $got_lt:tt
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            $got_lt
            $finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_split_gt_and_rest {
    (
        {>      $($_rest:tt)*}
        {$gt:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                gt {$gt}
                rest {$($rest)*}
            }
        }
    };
    (
        {>= $($rest:tt)*}
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                gt {>}
                rest { = $($rest)* } // >= is splitted into > and =
            }
        }
    };
    (
        {>> $($rest:tt)*}
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                gt {>}
                rest { > $($rest)* } // >> is splitted into > and >
            }
        }
    };
    (
        {>>= $($rest:tt)*}
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                gt {>}
                rest { >= $($rest)* } // >>= is splitted into > and >=
            }
        }
    };
}
// endregion

// region: consume_bounds

/// Consume tokens till one of the following tokens:
/// - `;`
/// - an outer `,` (not wrapped in `< >`)
/// - `where`
/// - an outer `>` (not matching a previous `<`)
/// - an outer `=` (not wrapped in `< >`)
/// - an outer `{..}` (not wrapped in `< >`)
/// - EOF
#[macro_export]
macro_rules! consume_bounds {
    ($($args:tt)*) => {
        $crate::__start_parsing_with! {
            parse_with { $crate::__impl_consume_bounds! }
            args {
                $($args)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_consume_bounds {
    // ,
    (
        $parsed_bounds:tt
        {, $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // ;
    (
        $parsed_bounds:tt
        {; $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // where
    (
        $parsed_bounds:tt
        {where $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // an outer =
    (
        $parsed_bounds:tt
        {= $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // {..}
    (
        $parsed_bounds:tt
        {{$($_t:tt)*} $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // EOF
    (
        $parsed_bounds:tt
        {} // EOF
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // an outer >
    (
        $parsed_bounds:tt
        {> $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // an outer >=
    (
        $parsed_bounds:tt
        {>= $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // an outer >>
    (
        $parsed_bounds:tt
        {>> $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // an outer >>=
    (
        $parsed_bounds:tt
        {>>= $($after:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds $parsed_bounds
                rest $rest
            }
        }
    };
    // an outer ->
    (
        {$($parsed_bounds:tt)*}
        {-> $($rest:tt)*}
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds {$($parsed_bounds)* -} // split -> into - and >
                rest {> $($rest)*}
            }
        }
    };
    // an outer =>
    (
        {$($parsed_bounds:tt)*}
        {=> $($rest:tt)*}
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed_bounds {$($parsed_bounds)* =} // split => into = and >
                rest {> $($rest)*}
            }
        }
    };
    // `<` , consume till a matched `>`
    (
        {$($parsed_bounds:tt)*}
        {<     $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($parsed_bounds)* $t}
            {$($rest)*}
            {$($rest)*}
            []
            {
                on_finish { $crate::__impl_consume_bounds_on_finish_consume_till_gt! }
                append { finish $finish }
            }
        }
    };
    // `<=` , consume till a matched `>`
    (
        {$($parsed_bounds:tt)*}
        {<=    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($parsed_bounds)* $t}
            {$($rest)*}
            {$($rest)*}
            []
            {
                on_finish { $crate::__impl_consume_bounds_on_finish_consume_till_gt! }
                append { finish $finish }
            }
        }
    };
    // `<-` , consume till a matched `>`
    (
        {$($parsed_bounds:tt)*}
        {<-    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($parsed_bounds)* $t}
            {$($rest)*}
            {$($rest)*}
            []
            {
                on_finish { $crate::__impl_consume_bounds_on_finish_consume_till_gt! }
                append { finish $finish }
            }
        }
    };
    // `<<` , consume till two matched `>` `>`
    (
        {$($parsed_bounds:tt)*}
        {<<    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($parsed_bounds)* $t}
            {$($rest)*}
            {$($rest)*}
            [<]
            {
                on_finish { $crate::__impl_consume_bounds_on_finish_consume_till_gt! }
                append { finish $finish }
            }
        }
    };
    // `<<=` , consume till two matched `>` `>`
    (
        {$($parsed_bounds:tt)*}
        {<<=   $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt! {
            {$($parsed_bounds)* $t}
            {$($rest)*}
            {$($rest)*}
            [<]
            {
                on_finish { $crate::__impl_consume_bounds_on_finish_consume_till_gt! }
                append { finish $finish }
            }
        }
    };
    // other cases, just consume
    (
        {$($parsed_bounds:tt)*}
        {$t:tt $($rest:tt)*}
        $t_and_rest:tt
        $finish:tt
    ) => {
        $crate::__impl_consume_bounds! {
            {$($parsed_bounds)* $t}
            {$($rest)*}
            {$($rest)*}
            $finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_consume_bounds_on_finish_consume_till_gt {
    (
        before_gt $before_gt:tt
        gt_and_rest $gt_and_rest:tt
        finish $finish:tt
    ) => {
        // continue parse bounds
        $crate::__impl_split_gt_and_rest! {
            $gt_and_rest
            $gt_and_rest
            {
                on_finish {$crate::__impl_consume_bounds_consume_first_gt_and_continue!}
                prepend {
                    finish $finish
                    before_gt $before_gt
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_consume_bounds_consume_first_gt_and_continue {
    (
        finish $finish:tt
        before_gt { $($before_gt:tt)* }
        gt {$gt:tt}
        rest $rest:tt
    ) => {
        $crate::__impl_consume_bounds! {
            {$($before_gt)* $gt}
            $rest
            $rest
            $finish
        }
    };
}

// endregion
