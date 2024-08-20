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

// this doesn't change the order of tokens
#[doc(hidden)]
#[macro_export]
macro_rules! __start_parsing_with_v2 {
    (
        parse_with {$($parse_with:tt)*}
        $(before_input { $($before_input:tt)* })?
        args {
            on_finish $on_finish:tt
            $(prepend $output_prepend:tt)?
            input $input:tt
            $(append $output_append:tt)?
        }
        $(after_input { $($after_input:tt)* })?
    ) => {
        $($parse_with)* {
            {
                on_finish $on_finish
                $(prepend $output_prepend)?
            }
            $($($before_input)*)?
            $input
            $input // clone input
            $($($after_input)*)?
            {
                $(append $output_append)?
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __resolve_finish_v2 {
    (
        {
            on_finish { $($macro_and_bang:tt)* }
            $(prepend { $($output_prepend:tt)* })?
        }
        { $($output:tt)* }
        { $(append  { $($output_append:tt )* })? }
    ) => {
        $($macro_and_bang)* {
            $($($output_prepend)*)?
            $($output)*
            $($($output_append)*)?
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

#[doc(hidden)]
#[macro_export]
macro_rules! __resolve_finish_flat {
    (
        $finish:tt
        $($output:tt)*
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output { $($output)* }
        }
    };
}

// endregion

// region: consume_till_outer_gt

/// Consume tokens till an outer `>`.
///
/// `>` is considered an outer `>` if there is not a previous `<` matching it.
///
/// Note that this macro might split the following tokens
/// if the last `>` is an outer `>` which doesn't have a matched previous `>`:
/// - `=>`
/// - `>=`
/// - `>>`
/// - `>>=`
///
/// The `>` in `->` is not considered as a splitted `>`.
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

// region: consume_till_outer_gt_inclusive

/// Consume tokens till an outer `>`, and consume this `>` as well.
///
/// See [consume_till_outer_gt!] for what is an outer `>`.
#[macro_export]
macro_rules! consume_till_outer_gt_inclusive {
    ($($args:tt)*) => {
        $crate::__start_parsing_with! {
            parse_with { $crate::__impl_consume_till_outer_gt_inclusive! }
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
macro_rules! __impl_consume_till_outer_gt_inclusive {
    // region: unmatched > or >>
    // >
    (
        {$($consumed:tt)*}
        {>      $($_rest:tt)*}
        {$gt:tt $($rest:tt )*}
        [] // no inner `<` before this `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_and_gt {$($consumed)* $gt}
                after_gt {$($rest)*}
            }
        }
    };
    // >=
    (
        {$($consumed:tt)*}
        {>=     $($rest:tt)*}
        $_rest:tt
        [] // no inner `<` before this `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_and_gt { $($consumed)* > } // >= is splitted
                after_gt { = $($rest)* }
            }
        }
    };
    // =>
    (
        {$($consumed:tt)*}
        {=>     $($_rest:tt)*}
        {$gt:tt $($rest:tt )*}
        [] // no inner `<` before this `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_and_gt {$($consumed)* $gt}
                after_gt {$($rest)*}
            }
        }
    };
    // >> only one matched
    (
        {$($consumed:tt)*}
        {>>        $($_rest:tt)*}
        {$gt_gt:tt $($rest:tt )*}
        [<] // no inner `<` before the second `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_and_gt {$($consumed)* $gt_gt}
                after_gt { $($rest)* }
            }
        }
    };
    // >> neither matched
    (
        {$($consumed:tt)*}
        {>>    $($rest:tt)*}
        $_rest:tt
        [] // no inner `<` before the first `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_and_gt {$($consumed)* >} // >> is splitted
                after_gt { > $($rest)*}
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
                before_and_gt {$($consumed)* >>}
                after_gt { = $($rest)* } // >>= is splitted
            }
        }
    };
    // >>= neither matched
    (
        {$($consumed:tt)*}
        {>>= $($rest:tt)*}
        $_rest:tt
        [] // no inner `<` before the first `>`
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                before_and_gt {$($consumed)* >}
                after_gt { >= $($rest)* } // >>= is splitted
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
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
        $crate::__impl_consume_till_outer_gt_inclusive! {
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
            $got_lt
            $finish
        }
    };
}

// endregion

// region: consume_bounds

/// Consume tokens till one of the following tokens:
/// - `;`
/// - an outer `,` (not wrapped in `< >`)
/// - `where`
/// - an outer `>` (See [consume_till_outer_gt!] for what an outer `>` is.)
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

// region: consume_optional_angle_bracketed

#[macro_export]
macro_rules! consume_optional_angle_bracketed {
    ($($args:tt)*) => {
        $crate::__start_parsing_with! {
            parse_with { $crate::__impl_consume_optional_angle_bracketed_start! }
            args {
                $($args)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_consume_optional_angle_bracketed_start {
    (
        {}
        {<      $($_rest:tt)*}
        {$lt:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt_inclusive! {
            {$lt}
            {$($rest)*}
            {$($rest)*}
            []
            {
                on_finish {$crate::__impl_consume_optional_angle_bracketed_finish!}
                prepend { on_finish $finish }
            }
        }
    };
    (
        {}
        {<-    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt_inclusive! {
            {$t}
            {$($rest)*}
            {$($rest)*}
            []
            {
                on_finish {$crate::__impl_consume_optional_angle_bracketed_finish!}
                prepend { on_finish $finish }
            }
        }
    };
    (
        {}
        {<=    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt_inclusive! {
            {$t}
            {$($rest)*}
            {$($rest)*}
            []
            {
                on_finish {$crate::__impl_consume_optional_angle_bracketed_finish!}
                prepend { on_finish $finish }
            }
        }
    };
    (
        {}
        {<<    $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt_inclusive! {
            {$t}
            {$($rest)*}
            {$($rest)*}
            [<] // one inner <
            {
                on_finish {$crate::__impl_consume_optional_angle_bracketed_finish!}
                prepend { on_finish $finish }
            }
        }
    };
    (
        {}
        {<<=   $($_rest:tt)*}
        {$t:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_consume_till_outer_gt_inclusive! {
            {$t}
            {$($rest)*}
            {$($rest)*}
            [<] // one inner <
            {
                on_finish {$crate::__impl_consume_optional_angle_bracketed_finish!}
                prepend { on_finish $finish }
            }
        }
    };
    // no leading <
    (
        {}
        $rest:tt
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                // angle_bracketed $before_and_gt
                rest $rest
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_consume_optional_angle_bracketed_finish {
    (
        on_finish $finish:tt
        before_and_gt $before_and_gt:tt
        after_gt $after_gt:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                angle_bracketed $before_and_gt
                rest $after_gt
            }
        }
    };
}

// endregion

// region: parse_generics

/// Parse generics
///
/// Input should be zero or many generics separated by `,` with an optional trailing `,` and
/// end with `EOF` or `>`.
///
/// ```
/// macro_rules! expect_output {
///     (
///         parsed_generics {
///             generics {$($generics:tt)*} // the original generics with a trailing comma
///             impl_generics {$($impl_generics:tt)*} // generics with a trailing comma but without default types and default exprs
///             type_generics {$($type_generics:tt)*} // generics with a trailing comma that can be used as type parameters of a generic path
///             generics_info {$($generics_info:tt)*} // info of all generics
///         }
///         rest { > }
///     ) => {
///         // ..
///     };
/// }
///
/// syn_lite::parse_generics! {
///     on_finish { expect_output! }
///     input { 'a, 'b: 'c, T: ?Sized + FnOnce(), > }
/// }
/// ```
///
/// Here is a full example:
///
/// ```
/// # let res = syn_lite::parse_generics! { on_finish { full_output! }
/// input {
///     /// lifetime
///     'a: 'b + 'c,
///     /// type param
///     T: FnOnce() -> u8 + 'static + ?Sized = dyn FnOnce() -> u8,
///     /// const
///     const N: &'static str = "default expr"
/// }
/// # };
/// # assert_eq!(res, ("dyn FnOnce() -> u8", "&'static str", "default expr"));
/// /// output
/// # #[macro_export] macro_rules! full_output {(
/// parsed_generics {
///     // the original generics with a trailing comma
///     generics      { #[doc = r" lifetime"] 'a: 'b + 'c, #[doc = r" type param"] T: FnOnce() -> u8 + 'static + ?Sized = $DefaultType:ty, #[doc = r" const"] const N: $ConstType:ty = $const_default_expr:expr, }
///     // generics with a trailing comma but without default types and default exprs
///     impl_generics { #[doc = r" lifetime"] 'a: 'b + 'c, #[doc = r" type param"] T: FnOnce() -> u8 + 'static + ?Sized                  , #[doc = r" const"] const N: $ConstTyp_:ty                           , }
///     // generics with a trailing comma that can be used as type parameters of a generic path
///     type_generics { #[doc = r" lifetime"] 'a         , #[doc = r" type param"] T                                                     , #[doc = r" const"]       N                                          , }
///     // info of all generics
///     generics_info {
///         {
///             lifetime_attrs {#[doc = r" lifetime"]} // present if there are attributes
///             lifetime { 'a }
///             bounds { 'b + 'c } // present if there is a colon. This might be empty
///         }
///         {
///             type_attrs {#[doc = r" type param"]} // present if there are attributes
///             name { T }
///             bounds { FnOnce() -> u8 + 'static + ?Sized } // present if there is a colon. This might be empty
///             default_ty { $DefaultTypeOfT:ty } // present if there is a `= $default_ty:ty`
///         }
///         {
///             const_attrs {#[doc = r" const"]} // present if there are attributes
///             const { const }
///             name { N }
///             bounds { $TypeOfN:ty }
///             default_expr { $DefaultExprOfN:expr } // present if there is a `= $default_expr:expr`
///         }
///     }
/// }
/// rest {}
/// # )=>{{
/// # assert_eq!(stringify!($ConstType), stringify!($ConstTyp_));
/// # assert_eq!(stringify!($DefaultType), stringify!($DefaultTypeOfT));
/// # assert_eq!(stringify!($DefaultExprOfN), stringify!($DefaultExprOfN));
/// # (stringify!($DefaultType), stringify!($ConstType), $const_default_expr)
/// # }}}
/// ```
#[macro_export]
macro_rules! parse_generics {
    ($($args:tt)*) => {
        $crate::__start_parsing_with! {
            parse_with { $crate::__parse_generics_start! }
            args {
                $($args)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_start {
    (
        {}
        $($rest:tt)*
    ) => {
        $crate::__parse_generics! {
            {
                generics {}
                impl_generics {}
                type_generics {}
                generics_info {}
            }
            $($rest)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics {
    // EOF
    (
        $parsed_generics:tt
        {}
        {}
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics $parsed_generics
                rest {}
            }
        }
    };
    // >
    (
        $parsed_generics:tt
        { >      $($_rest:tt)* }
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics $parsed_generics
                rest $rest
            }
        }
    };
    // >>
    (
        $parsed_generics:tt
        { >>    $($_rest:tt)* }
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics $parsed_generics
                rest $rest
            }
        }
    };
    // >=
    (
        $parsed_generics:tt
        { >=    $($_rest:tt)* }
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics $parsed_generics
                rest $rest
            }
        }
    };
    // >>=
    (
        $parsed_generics:tt
        { >>=    $($_rest:tt)* }
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics $parsed_generics
                rest $rest
            }
        }
    };
    // 'a:
    (
        $parsed:tt
        { $(#[$($_attr:tt)*])* $_lt:lifetime :         $($_bounds_and_rest:tt)* }
        { $(#$attr:tt       )* $lt:lifetime  $colon:tt $($bounds_and_rest:tt )* }
        $finish:tt
    ) => {
        $crate::__impl_consume_bounds! {
            {}
            {$($bounds_and_rest)*}
            {$($bounds_and_rest)*}
            {
                on_finish { $crate::__parse_generics_lifetime_process_consumed_bounds! }
                prepend {
                    parsed_generics $parsed
                    generic_and_colon {
                        attrs { $(#$attr)* }
                        $lt $colon
                    }
                }
                append {
                    finish $finish
                }
            }
        }
    };
    // 'a
    (
        {
            generics { $($generics:tt)* }
            impl_generics { $($impl_generics:tt)* }
            type_generics { $($type_generics:tt)* }
            generics_info { $($generics_info:tt)* }
        }
        { $($(#[$($_attr:tt)*])+)? $_lt:lifetime $($_rest:tt)* }
        { $($(#$attr:tt       )+)? $lt:lifetime  $($rest:tt )* }
        $finish:tt
    ) => {
        $crate::__parse_generics_match_one_of_comma_gt_eof! {
            // trailing comma is added later
            {
                generics      { $($generics)*      $($(#$attr)+)? $lt }
                impl_generics { $($impl_generics)* $($(#$attr)+)? $lt }
                type_generics { $($type_generics)* $($(#$attr)+)? $lt }
                generics_info {
                    $($generics_info)*
                    {
                        $( lifetime_attrs {$(#$attr)+} )?
                        lifetime {$lt}
                    }
                }
            }
            {$($rest)*}
            {$($rest)*}
            $finish
        }
    };
    // `const N: usize =`
    (
        $parsed_generics:tt
        { $(#[$($_attr:tt)*])* const        $_name:ident :         $_bounds:ty = $($_default_expr_and_rest:tt)* }
        { $(#$attr:tt       )* $const:ident $name:ident  $colon:tt $bounds:ty  = $($default_expr_and_rest:tt )* }
        $finish:tt
    ) => {
        $crate::__parse_generics_match_default_expr! {
            {
                parsed_generics $parsed_generics
                generic {
                    attrs {$(#$attr)*}
                    $const $name
                }
                colon {$colon}
                ty {$bounds}
                eq {=}
            }
            {$($default_expr_and_rest)*}
            {$($default_expr_and_rest)*}
            $finish
        }
    };
    // `const N: usize,` or `const N: usize EOF`
    (
        {
            generics { $($generics:tt)* }
            impl_generics { $($impl_generics:tt)* }
            type_generics { $($type_generics:tt)* }
            generics_info { $($generics_info:tt)* }
        }
        { $($(#[$($_attr:tt)*])+)? const        $_name:ident :         $_bounds:ty $(, $($_rest:tt)*)? }
        { $($(#$attr:tt       )+)? $const:ident $name:ident  $colon:tt $bounds:ty  $(, $($rest:tt )*)? }
        $finish:tt
    ) => {
        $crate::__parse_generics! {
            {
                generics      { $($generics)*      $($(#$attr)+)? $const $name $colon $bounds , }
                impl_generics { $($impl_generics)* $($(#$attr)+)? $const $name $colon $bounds , }
                type_generics { $($type_generics)* $($(#$attr)+)?        $name                , }
                generics_info {
                    $($generics_info)*
                    {
                        $(const_attrs {$(#$attr)+})?
                        const {$const}
                        name {$name}
                        bounds {$bounds}
                    }
                }
            }
            {$($($rest)*)?}
            {$($($rest)*)?}
            $finish
        }
    };
    // `const N: usize >`
    (
        {
            generics { $($generics:tt)* }
            impl_generics { $($impl_generics:tt)* }
            type_generics { $($type_generics:tt)* }
            generics_info { $($generics_info:tt)* }
        }
        { $($(#[$($_attr:tt)*])+)? const        $_name:ident :         $_bounds:ty > $($_rest:tt)* }
        { $($(#$attr:tt       )+)? $const:ident $name:ident  $colon:tt $bounds:ty  > $($rest:tt )* }
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics {
                    generics      { $($generics)*      $($(#$attr)+)? $const $name $colon $bounds , }
                    impl_generics { $($impl_generics)* $($(#$attr)+)? $const $name $colon $bounds , }
                    type_generics { $($type_generics)* $($(#$attr)+)?        $name                  }
                    generics_info {
                        $($generics_info)*
                        {
                            $(const_attrs {$(#$attr)+})?
                            const {$const}
                            name {$name}
                            bounds {$bounds}
                        }
                    }
                }
                rest { > $($rest)* }
            }
        }
    };
    // `const N: usize >>`
    (
        {
            generics { $($generics:tt)* }
            impl_generics { $($impl_generics:tt)* }
            type_generics { $($type_generics:tt)* }
            generics_info { $($generics_info:tt)* }
        }
        { $($(#[$($_attr:tt)*])+)? const        $_name:ident :         $_bounds:ty >> $($_rest:tt)* }
        { $($(#$attr:tt       )+)? $const:ident $name:ident  $colon:tt $bounds:ty  >> $($rest:tt )* }
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics {
                    generics      { $($generics)*      $($(#$attr)+)? $const $name $colon $bounds , }
                    impl_generics { $($impl_generics)* $($(#$attr)+)? $const $name $colon $bounds , }
                    type_generics { $($type_generics)* $($(#$attr)+)?        $name                  }
                    generics_info {
                        $($generics_info)*
                        {
                            $(const_attrs {$(#$attr)+})?
                            const {$const}
                            name {$name}
                            bounds {$bounds}
                        }
                    }
                }
                rest { >> $($rest)* }
            }
        }
    };
    // T:
    (
        $parsed:tt
        { $(#[$($_attr:tt)*])* $_name:ident :         $($_bounds_and_rest:tt)* }
        { $(#$attr:tt       )* $name:ident  $colon:tt $($bounds_and_rest:tt )* }
        $finish:tt
    ) => {
        $crate::__impl_consume_bounds! {
            {}
            {$($bounds_and_rest)*}
            {$($bounds_and_rest)*}
            {
                on_finish { $crate::__parse_generics_type_process_consumed_bounds! }
                prepend {
                    parsed_generics $parsed
                    generic { attrs {$(#$attr)*} $name }
                    colon { $colon }
                }
                append {
                    finish $finish
                }
            }
        }
    };
    // T
    (
        $parsed_generics:tt
        { $(#[$($_attr:tt)*])* $_name:ident $($_rest:tt)* }
        { $(#$attr:tt       )* $name:ident  $($rest:tt )* }
        $finish:tt
    ) => {
        $crate::__parse_generics_after_type_parse_bounds! {
            parsed_generics $parsed_generics
            generic { attrs {$(#$attr)*} $name }
            rest {$($rest)*} {$($rest)*}
            finish $finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_lifetime_process_consumed_bounds {
    (
        parsed_generics {
            generics { $($generics:tt)* }
            impl_generics { $($impl_generics:tt)* }
            type_generics { $($type_generics:tt)* }
            generics_info { $($generics_info:tt)* }
        }
        generic_and_colon {
            attrs {$($($attrs:tt)+)?}
            $lt:lifetime $colon:tt
        }
        consumed_bounds {$($parsed_bounds:tt)*}
        rest $rest:tt
        finish $finish:tt
    ) => {
        $crate::__parse_generics_match_one_of_comma_gt_eof! {
            // trailing comma is added later
            {
                generics      { $($generics)*      $($($attrs)+)? $lt $colon $($parsed_bounds)* }
                impl_generics { $($impl_generics)* $($($attrs)+)? $lt $colon $($parsed_bounds)* }
                type_generics { $($type_generics)* $($($attrs)+)? $lt                           }
                generics_info {
                    $($generics_info)*
                    {
                        $( lifetime_attrs {$($attrs)+} )?
                        lifetime {$lt}
                        bounds {$($parsed_bounds)*}
                    }
                }
            }
            $rest $rest
            $finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_type_process_consumed_bounds {
    (
        parsed_generics $parsed_generics:tt
        generic $generic:tt
        colon $colon:tt
        consumed_bounds $parsed_bounds:tt
        rest $rest:tt
        finish $finish:tt
    ) => {
        $crate::__parse_generics_after_type_parse_bounds! {
            parsed_generics $parsed_generics
            generic $generic
            colon $colon
            parsed_bounds $parsed_bounds
            rest $rest $rest
            finish $finish
        }
    };
}

// without trailing comma
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_match_one_of_comma_gt_eof {
    // eof
    (
        $parsed_generics:tt
        {} $rest:tt
        $finish:tt
    ) => {
        $crate::__parse_generics_append_trailing_comma! {
            $parsed_generics
            ,
            {
                on_finish { $crate::__parse_generics_end! }
                append {
                    $rest
                    $finish
                }
            }
        }
    };
    // ,
    (
        $parsed_generics:tt
        {,         $($_rest:tt)*}
        {$comma:tt $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_generics_append_trailing_comma! {
            $parsed_generics
            $comma
            {
                on_finish { $crate::__parse_generics! }
                append {
                    {$($rest)*}
                    {$($rest)*}
                    $finish
                }
            }
        }
    };
    // >
    (
        $parsed_generics:tt
        {> $($_rest:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__parse_generics_append_trailing_comma! {
            $parsed_generics
            ,
            {
                on_finish { $crate::__parse_generics_end! }
                append {
                    $rest
                    $finish
                }
            }
        }
    };
    // >=
    (
        $parsed_generics:tt
        {>= $($_rest:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__parse_generics_append_trailing_comma! {
            $parsed_generics
            ,
            {
                on_finish { $crate::__parse_generics_end! }
                append {
                    $rest
                    $finish
                }
            }
        }
    };
    // >>
    (
        $parsed_generics:tt
        {>> $($_rest:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__parse_generics_append_trailing_comma! {
            $parsed_generics
            ,
            {
                on_finish { $crate::__parse_generics_end! }
                append {
                    $rest
                    $finish
                }
            }
        }
    };
    // >>=
    (
        parsed_generics $parsed_generics:tt
        {>>= $($_rest:tt)*}
        $rest:tt
        $finish:tt
    ) => {
        $crate::__parse_generics_append_trailing_comma! {
            $parsed_generics
            ,
            {
                on_finish { $crate::__parse_generics_end! }
                append {
                    $rest
                    $finish
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_append_trailing_comma {
    (
        {
            generics { $($generics:tt)* }
            impl_generics { $($impl_generics:tt)* }
            type_generics { $($type_generics:tt)* }
            generics_info $generics_info:tt
        }
        $comma:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                {
                    generics { $($generics)* $comma }
                    impl_generics { $($impl_generics)* $comma }
                    type_generics { $($type_generics)* $comma }
                    generics_info $generics_info
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_end {
    (
        $parsed_generics:tt
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics $parsed_generics
                rest $rest
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_after_type_parse_bounds {
    (
        parsed_generics $parsed_generics:tt
        generic $generic:tt
        $(
            colon $colon:tt
            parsed_bounds $parsed_bounds:tt
        )?
        rest
        {=      $($_rest:tt)*}
        {$eq:tt $($rest:tt )*}
        finish $finish:tt
    ) => {
        $crate::__parse_generics_match_default_type! {
            {
                parsed_generics $parsed_generics
                generic $generic
                $(
                    colon $colon
                    parsed_bounds $parsed_bounds
                )?
                eq {$eq}
            }
            {$($rest)*}
            {$($rest)*}
            $finish
        }
    };
    (
        parsed_generics {
            generics { $($generics:tt)* }
            impl_generics { $($impl_generics:tt)* }
            type_generics { $($type_generics:tt)* }
            generics_info { $($generics_info:tt)* }
        }
        generic {
            attrs {$($($attrs:tt)+)?}
            $name:ident
        }
        $(
            colon {$colon:tt}
            parsed_bounds {$($parsed_bounds:tt)*}
        )?
        rest $rest:tt $_rest:tt
        finish $finish:tt
    ) => {
        $crate::__parse_generics_match_one_of_comma_gt_eof! {
            {
                generics      { $($generics)*      $($($attrs)+)? $name $($colon $($parsed_bounds)*)? }
                impl_generics { $($impl_generics)* $($($attrs)+)? $name $($colon $($parsed_bounds)*)? }
                type_generics { $($type_generics)* $($($attrs)+)? $name                               }
                generics_info {
                    $($generics_info)*
                    {
                        $(type_attrs { $($attrs)+ })?
                        name { $name }
                        $(bounds { $($parsed_bounds)* })?
                    }
                }
            }
            $rest
            $_rest
            $finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_match_default_type {
    // ,
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $name:ident
            }
            $(
                colon {$colon:tt}
                parsed_bounds { $($parsed_bounds:tt)* }
            )?
            eq {$eq:tt}
        }
        $_rest:tt
        {$ty:ty $(, $($rest:tt)* )?}
        $finish:tt
    ) => {
        $crate::__parse_generics! {
            {
                generics      { $($generics)*      $($($attrs)+)? $name $($colon $($parsed_bounds)*)? $eq $ty , }
                impl_generics { $($impl_generics)* $($($attrs)+)? $name $($colon $($parsed_bounds)*)?         , }
                type_generics { $($type_generics)* $($($attrs)+)? $name                                       , }
                generics_info {
                    $($generics_info)*
                    {
                        $(type_attrs { $($attrs)+ })?
                        name { $name }
                        $(bounds { $($parsed_bounds)* })?
                        default_ty { $ty }
                    }
                }
            }
            {$($($rest)*)?}
            {$($($rest)*)?}
            $finish
        }
    };
    // >
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $name:ident
            }
            $(
                colon {$colon:tt}
                parsed_bounds { $($parsed_bounds:tt)* }
            )?
            eq {$eq:tt}
        }
        {$ty:ty > $($rest:tt)*}
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics {
                    generics      { $($generics)*      $($($attrs)+)? $name $($colon $($parsed_bounds)*)? $eq $ty , }
                    impl_generics { $($impl_generics)* $($($attrs)+)? $name $($colon $($parsed_bounds)*)?         , }
                    type_generics { $($type_generics)* $($($attrs)+)? $name                                       , }
                    generics_info {
                        $($generics_info)*
                        {
                            $(type_attrs { $($attrs)+ })?
                            name { $name }
                            $(bounds { $($parsed_bounds)* })?
                            default_ty { $ty }
                        }
                    }
                }
                rest {> $($rest)*}
            }
        }
    };
    // >>
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $name:ident
            }
            $(
                colon {$colon:tt}
                parsed_bounds { $($parsed_bounds:tt)* }
            )?
            eq {$eq:tt}
        }
        {$ty:ty >> $($rest:tt)*}
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics {
                    generics      { $($generics)*      $($($attrs)+)? $name $($colon $($parsed_bounds)*)? $eq $ty , }
                    impl_generics { $($impl_generics)* $($($attrs)+)? $name $($colon $($parsed_bounds)*)?         , }
                    type_generics { $($type_generics)* $($($attrs)+)? $name                                       , }
                    generics_info {
                        $($generics_info)*
                        {
                            $(type_attrs { $($attrs)+ })?
                            name { $name }
                            $(bounds { $($parsed_bounds)* })?
                            default_ty { $ty }
                        }
                    }
                }
                rest {>> $($rest)*}
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_generics_match_default_expr {
    // we cannot match $expr or `const N: usize = 1>` will be wrongly parsed
    // $path ,
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $const:ident $name:ident
            }
            colon {$colon:tt}
            ty {$ty:ty}
            eq {$eq:tt}
        }
        $_rest:tt
        {$expr:path $(, $($rest:tt)* )?}
        $finish:tt
    ) => {
        $crate::__parse_generics! {
            {
                generics      { $($generics)*      $($($attrs)+)? $const $name $colon $ty $eq $expr , }
                impl_generics { $($impl_generics)* $($($attrs)+)? $const $name $colon $ty           , }
                type_generics { $($type_generics)* $($($attrs)+)?        $name                      , }
                generics_info {
                    $($generics_info)*
                    {
                        $(const_attrs { $($attrs)+ })?
                        const {$const}
                        name { $name }
                        bounds { $ty }
                        default_expr { $expr }
                    }
                }
            }
            {$($($rest)*)?}
            {$($($rest)*)?}
            $finish
        }
    };
    // $path >
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $const:ident $name:ident
            }
            colon {$colon:tt}
            ty {$ty:ty}
            eq {$eq:tt}
        }
        $_rest:tt
        {$expr:path > $($rest:tt)*}
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics {
                    generics      { $($generics)*      $($($attrs)+)? $const $name $colon $ty $eq $expr , }
                    impl_generics { $($impl_generics)* $($($attrs)+)? $const $name $colon $ty           , }
                    type_generics { $($type_generics)* $($($attrs)+)?        $name                      , }
                    generics_info {
                        $($generics_info)*
                        {
                            $(const_attrs { $($attrs)+ })?
                            const {$const}
                            name { $name }
                            bounds { $ty }
                            default_expr { $expr }
                        }
                    }
                }
                rest { > $($rest)* }
            }
        }
    };
    // $path >>
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $const:ident $name:ident
            }
            colon {$colon:tt}
            ty {$ty:ty}
            eq {$eq:tt}
        }
        $_rest:tt
        {$expr:path >> $($rest:tt)*}
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                parsed_generics {
                    generics      { $($generics)*      $($($attrs)+)? $const $name $colon $ty $eq $expr , }
                    impl_generics { $($impl_generics)* $($($attrs)+)? $const $name $colon $ty           , }
                    type_generics { $($type_generics)* $($($attrs)+)?        $name                      , }
                    generics_info {
                        $($generics_info)*
                        {
                            $(const_attrs { $($attrs)+ })?
                            const {$const}
                            name { $name }
                            bounds { $ty }
                            default_expr { $expr }
                        }
                    }
                }
                rest { >> $($rest)* }
            }
        }
    };
    // {..}
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $const:ident $name:ident
            }
            colon {$colon:tt}
            ty {$ty:ty}
            eq {$eq:tt}
        }
        {{$($_expr:tt)*} $($_rest:tt)*}
        {$expr:tt        $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_generics_match_one_of_comma_gt_eof! {
            {
                generics      { $($generics)*      $($($attrs)+)? $const $name $colon $ty $eq $expr }
                impl_generics { $($impl_generics)* $($($attrs)+)? $const $name $colon $ty           }
                type_generics { $($type_generics)* $($($attrs)+)?        $name                      }
                generics_info {
                    $($generics_info)*
                    {
                        $(const_attrs { $($attrs)+ })?
                        const {$const}
                        name { $name }
                        bounds { $ty }
                        default_expr { $expr }
                    }
                }
            }
            { $($rest)* }
            { $($rest)* }
            $finish
        }
    };
    // $literal
    (
        {
            parsed_generics {
                generics { $($generics:tt)* }
                impl_generics { $($impl_generics:tt)* }
                type_generics { $($type_generics:tt)* }
                generics_info { $($generics_info:tt)* }
            }
            generic {
                attrs {$($($attrs:tt)+)?}
                $const:ident $name:ident
            }
            colon {$colon:tt}
            ty {$ty:ty}
            eq {$eq:tt}
        }
        {$_expr:literal $($_rest:tt)*}
        {$expr:literal  $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_generics_match_one_of_comma_gt_eof! {
            {
                generics      { $($generics)*      $($($attrs)+)? $const $name $colon $ty $eq $expr }
                impl_generics { $($impl_generics)* $($($attrs)+)? $const $name $colon $ty           }
                type_generics { $($type_generics)* $($($attrs)+)?        $name                      }
                generics_info {
                    $($generics_info)*
                    {
                        $(const_attrs { $($attrs)+ })?
                        const {$const}
                        name { $name }
                        bounds { $ty }
                        default_expr { $expr }
                    }
                }
            }
            { $($rest)* }
            { $($rest)* }
            $finish
        }
    };
}

// endregion

// region: parse_optional_angle_bracketed_generics

/// Parse an optional angle bracketed generics `<'a, T, const N: usize>`
///
/// See also [`parse_generics!`].
///
/// ### Examples:
///
/// #### no angle bracketed generics
///
/// ```
/// macro_rules! expect_no_generics {
///     (rest { () -> u8 {} }) => {};
/// }
/// syn_lite::parse_optional_angle_bracketed_generics! {
///     on_finish {expect_no_generics!}
///     input { () -> u8 {} }
/// }
/// ```
///
/// #### empty generics
///
/// ```
/// macro_rules! expect {
///     (
///         lt {<}
///         parsed_generics {
///             generics {}
///             impl_generics {}
///             type_generics {}
///             generics_info {}
///         }
///         gt {>}
///         rest { () -> u8 {} }
///     ) => {};
/// }
/// syn_lite::parse_optional_angle_bracketed_generics! {
///     on_finish {expect!}
///     input { <>() -> u8 {} }
/// }
/// ```
///
/// #### lifetime generics
///
/// ```
/// macro_rules! expect {
///     (
///         lt {<}
///         parsed_generics {
///             generics {'a,}
///             impl_generics {'a,}
///             type_generics {'a,}
///             generics_info {
///                 { lifetime {'a} }
///             }
///         }
///         gt {>}
///         rest { () ->&'a str {} }
///     ) => {};
/// }
/// syn_lite::parse_optional_angle_bracketed_generics! {
///     on_finish {expect!}
///     input { <'a>() -> &'a str {} }
/// }
/// ```
#[macro_export]
macro_rules! parse_optional_angle_bracketed_generics {
    ($($args:tt)*) => {
        $crate::__start_parsing_with! {
            parse_with { $crate::__impl_parse_optional_angle_bracketed_generics_start! }
            args {
                $($args)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_optional_angle_bracketed_generics_start {
    // <>
    (
        {}
        {<      >      $($_rest:tt)*}
        {$lt:tt $gt:tt $($rest:tt)*}
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                lt {$lt}
                parsed_generics {
                    generics {}
                    impl_generics {}
                    type_generics {}
                    generics_info {}
                }
                gt {$gt}
                rest { $($rest)* }
            }
        }
    };
    // <
    (
        {}
        {<      $($_rest:tt)*}
        {$lt:tt $($rest:tt)*}
        $finish:tt
    ) => {
        $crate::__parse_generics! {
            {
                generics {}
                impl_generics {}
                type_generics {}
                generics_info {}
            }
            {$($rest)*}
            {$($rest)*}
            {
                on_finish { $crate::__impl_parse_optional_angle_bracketed_generics_after_parse_generics! }
                prepend {
                    on_finish $finish
                    lt {$lt}
                }
            }
        }
    };
    // anything else
    (
        {}
        $_rest:tt
        $rest:tt
        $finish:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                rest $rest
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_parse_optional_angle_bracketed_generics_after_parse_generics {
    (
        on_finish $finish:tt
        lt $lt:tt
        parsed_generics $parsed_generics:tt
        rest $rest:tt
    ) => {
        $crate::__impl_split_gt_and_rest! {
            $rest
            $rest
            {
                on_finish {$crate::__resolve_finish_flat!}
                prepend {
                    $finish
                    lt $lt
                    parsed_generics $parsed_generics
                }
            }
        }
    };
}

// endregion

// region: parse_item_fn

/// Parse an item fn
///
/// Example:
///
/// ```
/// macro_rules! expect_item_fn {
///     (
///         item_fn {
///             $(outer_attrs { #[cfg(..)] })? // present if there are outer attributes
///             vis {$vis:tt}
///             sig {
///                                       // the following keywords are present if specified
///                 $(default {default})?
///                 $(const   {const}  )?
///                 $(async   {async}  )?
///                 $(unsafe  {unsafe} )?
///                 $(extern  {extern $($extern_name:literal)?})?
///
///                 fn { fn }
///                 ident { get }
///
///                 $(                                   // present if there is `<...>`
///                     lt {<}
///                     parsed_generics $parsed_generics:tt
///                     gt {>}
///                 )?
///
///                 paren_inputs { (self) }
///                 output { $(-> $output_ty:ty)? }
///
///                 $(                                      // present if there is where clause
///                     where_clause { where $($where_clause:tt)* }
///                 )?
///             }
///                                  // either block or semicolon will be present
///             $(block { {..} })?   // present if there is a block as fn body
///             $(semicolon { ; })?  // present if there is a semicolon
///         }
///         rest {}
///     ) => {};
/// }
///
/// syn_lite::parse_item_fn! {
///     on_finish { expect_item_fn! }
///     input {
///         #[cfg(..)]
///         fn get<T>(self) -> Option<T>
///         where
///             Self: Sized,
///         {
///             ..
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! parse_item_fn {
    ($($args:tt)*) => {
        $crate::__start_parsing_with! {
            parse_with { $crate::__parse_item_fn_start! }
            args {
                $($args)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_start {
    (
        {}
        {
            $($(#$outer_attrs:tt)+)?
            $vis:vis
            $keyword:ident
            $($rest:tt)*
        }
        $_rest:tt
        $finish:tt
    ) => {
        $crate::__parse_item_fn_signature! {
            {}
            { $keyword $($rest)* }
            { $keyword $($rest)* }
            {
                on_finish {$crate::__parse_item_fn_after_sig!}
                prepend {
                    on_finish $finish
                    before_sig {
                        $(outer_attrs {$(#$outer_attrs)+})?
                        vis {$vis}
                    }
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_after_sig {
    (
        on_finish $finish:tt
        before_sig {$($before_sig:tt)*}
        sig $sig:tt
        rest $rest:tt
    ) => {
        $crate::__parse_item_fn_block_or_semi! {
            $finish
            {
                $($before_sig)*
                sig $sig
            }
            $rest
            $rest
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_block_or_semi {
    (
        $finish:tt
        {$($parsed_fn:tt)*}
        {;             $($_rest:tt)*}
        {$semicolon:tt $( $rest:tt)*}
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                item_fn {
                    $($parsed_fn)*
                    semicolon {$semicolon}
                }
                rest {$($rest)*}
            }
        }
    };
    (
        $finish:tt
        {$($parsed_fn:tt)*}
        {{$($_block:tt)*} $($_rest:tt)*}
        {$block:tt        $( $rest:tt)*}
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                item_fn {
                    $($parsed_fn)*
                    block {$block}
                }
                rest {$($rest)*}
            }
        }
    };
}

// keyword order for functions declaration is `pub`, `default`, `const`, `async`, `unsafe`, `extern`
//
// this macro doesn't check the order
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_signature {
    (
        {$($parsed_sig:tt)*}
        { fn     $_ident:ident $($_rest:tt)*}
        { $fn:tt $ident:ident  $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__impl_parse_optional_angle_bracketed_generics_start! {
            {}
            {$($rest)*}
            {$($rest)*}
            {
                on_finish {$crate::__parse_item_fn_signature_process_abg!}
                prepend {
                    finish $finish
                    parsed_sig {
                        $($parsed_sig)*
                        fn {$fn}
                        ident {$ident}
                    }
                }
            }
        }
    };
    (
        {$($parsed_sig:tt)*}
        { default        $($_rest:tt)*}
        { $default:ident $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_item_fn_signature! {
            { $($parsed_sig)* default {$default} }
            {$($rest)*}
            {$($rest)*}
            $finish
        }
    };
    (
        {$($parsed_sig:tt)*}
        { const        $($_rest:tt)*}
        { $const:ident $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_item_fn_signature! {
            { $($parsed_sig)* const {$const} }
            {$($rest)*}
            {$($rest)*}
            $finish
        }
    };
    (
        {$($parsed_sig:tt)*}
        { async        $($_rest:tt)*}
        { $async:ident $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_item_fn_signature! {
            { $($parsed_sig)* async {$async} }
            {$($rest)*}
            {$($rest)*}
            $finish
        }
    };
    (
        {$($parsed_sig:tt)*}
        { unsafe        $($_rest:tt)*}
        { $unsafe:ident $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_item_fn_signature! {
            { $($parsed_sig)* unsafe {$unsafe} }
            {$($rest)*}
            {$($rest)*}
            $finish
        }
    };
    (
        {$($parsed_sig:tt)*}
        { extern        $($_name:literal)? $_other:ident $($_rest:tt)*}
        { $extern:ident $($name:literal )? $other:ident  $($rest:tt )*}
        $finish:tt
    ) => {
        $crate::__parse_item_fn_signature! {
            { $($parsed_sig)* extern {$extern $($name)?} }
            {$other $($rest)*}
            {$other $($rest)*}
            $finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_signature_process_abg {
    (
        finish $finish:tt
        parsed_sig { $($parsed_sig:tt)* }
        $(
            lt $lt:tt
            parsed_generics $parsed_generics:tt
            gt $gt:tt
        )?
        rest $rest:tt
    ) => {
        $crate::__parse_item_fn_signature_after_generics! {
            $finish
            {
                $($parsed_sig)*
                $(
                    lt $lt
                    parsed_generics $parsed_generics
                    gt $gt
                )?
            }
            $rest $rest
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_signature_after_generics {
    (
        $finish:tt
        { $($parsed_sig:tt)* }
        { ($($inputs:tt)*) ->        $($_output_ty_and_rest:tt)* }
        { $paren_inputs:tt $arrow:tt $($output_ty_and_rest:tt )* }
    ) => {
        $crate::__impl_consume_bounds! {
            { $arrow }
            { $($output_ty_and_rest)* }
            { $($output_ty_and_rest)* }
            {
                on_finish {$crate::__parse_item_fn_signature_process_consumed_bounds_as_type!}
                prepend {
                    on_finish $finish
                    parsed_sig {
                        $($parsed_sig)*
                        paren_inputs {$paren_inputs}
                    }
                }
            }
        }
    };
    (
        $finish:tt
        { $($parsed_sig:tt)* }
        { ($($inputs:tt)*) where     $($_where_predicates_and_rest:tt)*}
        { $paren_inputs:tt $where:tt $( $where_predicates_and_rest:tt)*}
    ) => {
        $crate::__consume_where_predicates! {
            {
                on_finish {$crate::__parse_item_fn_signature_process_consumed_where_predicates!}
                prepend {
                    finish $finish
                    parsed_sig {
                        $($parsed_sig)*
                        paren_inputs {$paren_inputs}
                        output {}
                    }
                }
            }
            {$where}
            {$($where_predicates_and_rest)*}
            {$($where_predicates_and_rest)*}
        }
    };
    (
        $finish:tt
        { $($parsed_sig:tt)* }
        { ($($inputs:tt)*)  $($_rest:tt)*}
        { $paren_inputs:tt  $( $rest:tt)*}
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                sig {
                    $($parsed_sig)*
                    paren_inputs {$paren_inputs}
                    output {}
                }
                rest {$($rest)*}
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_signature_process_consumed_bounds_as_type {
    (
        on_finish $finish:tt
        parsed_sig {$($parsed_sig:tt)*}
        consumed_bounds $output:tt
        rest $rest:tt
    ) => {
        $crate::__parse_item_fn_signature_consume_optional_where_clause! {
            $finish
            { $($parsed_sig)* output $output }
            $rest
            $rest
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_signature_consume_optional_where_clause {
    (
        $finish:tt
        $parsed_sig:tt
        { where     $($_where_predicates_and_rest:tt)* }
        { $where:tt $( $where_predicates_and_rest:tt)* }
    ) => {
        $crate::__consume_where_predicates! {
            {
                on_finish {$crate::__parse_item_fn_signature_process_consumed_where_predicates!}
                prepend {
                    finish $finish
                    parsed_sig $parsed_sig
                }
            }
            {$where}
            {$($where_predicates_and_rest)*}
            {$($where_predicates_and_rest)*}
        }
    };
    (
        $finish:tt
        $sig:tt
        $_rest:tt
        $rest:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                sig $sig
                rest $rest
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_item_fn_signature_process_consumed_where_predicates {
    (
        finish $finish:tt
        parsed_sig { $($parsed_sig:tt)* }
        consumed $where_clause:tt
        rest $rest:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                sig {
                    $($parsed_sig)*
                    where_clause $where_clause
                }
                rest $rest
            }
        }
    };
}

// endregion

// region: consume_where_clause

/// consume till one of the following tokens:
/// - EOF
/// - `;`
/// - `{..}`
#[doc(hidden)]
#[macro_export]
macro_rules! __consume_where_predicates {
    // EOF
    (
        $finish:tt
        $consumed:tt
        {}
        $rest:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed $consumed
                rest $rest
            }
        }
    };
    // ;
    (
        $finish:tt
        $consumed:tt
        {; $($_rest:tt)*}
        $rest:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed $consumed
                rest $rest
            }
        }
    };
    // {..}
    (
        $finish:tt
        $consumed:tt
        {{$($block:tt)*} $($_rest:tt)*}
        $rest:tt
    ) => {
        $crate::__resolve_finish! {
            on_finish $finish
            output {
                consumed $consumed
                rest $rest
            }
        }
    };
    // anything else
    (
        $finish:tt
        {$($consumed:tt)*}
        $_rest:tt
        {$t:tt $($rest:tt)*}
    ) => {
        $crate::__consume_where_predicates! {
            $finish
            {$($consumed)* $t}
            {$($rest)*}
            {$($rest)*}
        }
    };
}

#[macro_export]
macro_rules! consume_optional_where_clause {
    ($($args:tt)*) => {
        $crate::__start_parsing_with_v2! {
            parse_with { $crate::__consume_optional_where_clause! }
            args {
                $($args)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __consume_optional_where_clause {
    (
        $on_finish_and_prepend:tt
        {where     $($_rest:tt)*}
        {$where:tt $( $rest:tt)*}
        $on_finish_append:tt
    ) => {
        $crate::__consume_where_predicates! {
            {
                on_finish {$crate::__consume_optional_where_clause_after_consume_where_predicates!}
                prepend { $on_finish_and_prepend }
                append { $on_finish_append }
            }
            {$where}
            {$($rest)*}
            {$($rest)*}
        }
    };
    (
        $on_finish_and_prepend:tt
        $_rest:tt
        $rest:tt
        $on_finish_append:tt
    ) => {
        $crate::__resolve_finish_v2! {
            $on_finish_and_prepend
            {
                rest $rest
            }
            $on_finish_append
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __consume_optional_where_clause_after_consume_where_predicates {
    (
        $on_finish_and_prepend:tt
        consumed $where_clause:tt
        rest $rest:tt
        $on_finish_append:tt
    ) => {
        $crate::__resolve_finish_v2! {
            $on_finish_and_prepend
            {
                where_clause $where_clause
                rest $rest
            }
            $on_finish_append
        }
    };
}

// endregion

// region: consume_inner_attrs

#[macro_export]
macro_rules! consume_inner_attrs {
    ($($args:tt)*) => {
        $crate::__start_parsing_with_v2! {
            parse_with { $crate::__consume_inner_attrs! }
            before_input {{}} // consumed
            args {
                $($args)*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __consume_inner_attrs {
    (
        $on_finish_and_prepend:tt
        {$($parsed:tt)*}
        {#         !        [$($_attr1:tt)*] #         !        [$($_attr2:tt)*] $($_rest:tt)*}
        {$pound1:tt $bang1:tt $attr1:tt      $pound2:tt $bang2:tt $attr2:tt      $( $rest:tt)*}
        $on_finish_append:tt
    ) => {
        $crate::__consume_inner_attrs! {
            $on_finish_and_prepend
            {$($parsed)* $pound1 $bang1 $attr1 $pound2 $bang2 $attr2 }
            {$($rest)*}
            {$($rest)*}
            $on_finish_append
        }
    };
    (
        $on_finish_and_prepend:tt
        {$($parsed:tt)*}
        {#         !        [$($_attr:tt)*] $($_rest:tt)*}
        {$pound:tt $bang:tt $attr:tt        $( $rest:tt)*}
        $on_finish_append:tt
    ) => {
        $crate::__consume_inner_attrs! {
            $on_finish_and_prepend
            {$($parsed)* $pound $bang $attr}
            {$($rest)*}
            {$($rest)*}
            $on_finish_append
        }
    };
    (
        $on_finish_and_prepend:tt
        $consumed:tt
        $_rest:tt
        $rest:tt
        $on_finish_append:tt
    ) => {
        $crate::__resolve_finish_v2! {
            $on_finish_and_prepend
            {
                inner_attrs $consumed
                rest $rest
            }
            $on_finish_append
        }
    };
}

// endregion
