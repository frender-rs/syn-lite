use syn_lite::consume_till_outer_gt;

macro_rules! path_gt {
    ($path:path >) => {};
}

macro_rules! full_path {
    (a::<a >>) => {};
}

path_gt! {a>}
path_gt! {a::<a>>} // this parses >> as two tokens
full_path! {a::<a>>} // this parses >> as one token

macro_rules! rematch_gt {
    ($t:tt) => {
        path_gt! $t
    };
}

rematch_gt! {{a::<a>>}}

macro_rules! rematch_gt_repetition {
    ($($t:tt)*) => {
        path_gt! {$($t)*}
    };
}
rematch_gt_repetition! {a::<a>>}

// macro_rules! expect_gt_and_eq {
//     (> >) => {};
// }

// macro_rules! rematch_gt_one {
//     ($($t:tt)*) => {
//         expect_gt_and_eq! {$($t)*}
//     };
// }

// rematch_gt_one! {>>}

macro_rules! check_gt_3 {
    (>> >) => {};
}

check_gt_3! {>>>}

macro_rules! check_lt_3 {
    (<< <) => {};
}

check_lt_3! {<<<}

macro_rules! expect_one_token {
    ($t:tt) => {};
}

expect_one_token! {<-}
expect_one_token! {<=}
expect_one_token! {<}
expect_one_token! {<<}
expect_one_token! {<<=}

expect_one_token! {->}
expect_one_token! {>=}
expect_one_token! {>}
expect_one_token! {>>}
expect_one_token! {>>=}
expect_one_token! {=>}

const _: () = {
    macro_rules! match_one_gt {
        (> $($rest:tt)*) => {
            true
        };
        ($($rest:tt)*) => {
            false
        };
    }
    assert!(match_one_gt!(>));
    assert!(match_one_gt!(> >));
    assert!(!match_one_gt!(>>));
};

macro_rules! expect_consume_till_gt {
    (
        before_gt {AsRef<u8>}
        gt_and_rest {>}
    ) => {};
}
consume_till_outer_gt!(
    on_finish {expect_consume_till_gt!}
    input {AsRef<u8>>}
);
consume_till_outer_gt!(
    on_finish {expect_consume_till_gt!}
    input {AsRef<u8> >}
);

macro_rules! expect_consume_till_gt_complex {
    (
        before_gt {AsRef<<u8 as Trait>::Type<u8>>}
        gt_and_rest {>}
    ) => {};
}

consume_till_outer_gt!(
    on_finish {expect_consume_till_gt_complex!}
    input {AsRef<<u8 as Trait>::Type<u8>>>}
);

consume_till_outer_gt!(
    on_finish {expect_consume_till_gt_complex!}
    input {AsRef<<u8 as Trait>::Type<u8>> >}
);

macro_rules! expect_consume_till_gt_complex_split {
    (
        before_gt {AsRef<<u8 as Trait>::Type<u8> >}
        gt_and_rest {>}
    ) => {};
}
consume_till_outer_gt!(
    on_finish {expect_consume_till_gt_complex_split!}
    input {AsRef<<u8 as Trait>::Type<u8> >>}
);

macro_rules! expect_gt_eq {
    (
        before_gt {Trait<Gat<'a>=()>}
        gt_and_rest {>}
    ) => {};
}

consume_till_outer_gt!(
    on_finish {expect_gt_eq!}
    input {Trait<Gat<'a>=()>>}
);

macro_rules! expect_eq_lt_lt {
    (
        before_gt {Trait<Assoc=<<T as Trait>::Assoc>>}
        gt_and_rest {>}
    ) => {};
}

consume_till_outer_gt!(
    on_finish {expect_eq_lt_lt!}
    input {Trait<Assoc=<<T as Trait>::Assoc>>>}
);

macro_rules! expect_gt_gt_eq {
    (
        before_gt {Trait<Gat<Self::Gat<T>>=()>}
        gt_and_rest {>}
    ) => {};
}

consume_till_outer_gt!(
    on_finish {expect_gt_gt_eq!}
    input {Trait<Gat<Self::Gat<T>>=()>>}
);

macro_rules! expect_gt_gt_eq_rest {
    (
        before_gt {Trait<Gat<Self::Gat<T>>=()>}
        gt_and_rest {>=}
    ) => {};
}

// note that the last >>= is splitted into two tokens > and >=
consume_till_outer_gt!(
    on_finish {expect_gt_gt_eq_rest!}
    input {Trait<Gat<Self::Gat<T>>=()>>=}
);

macro_rules! expect_gt_gt_eq_expr {
    (
        before_gt {None::<() >}
        gt_and_rest {>= None::<()>}
    ) => {};
}

consume_till_outer_gt!(
    on_finish {expect_gt_gt_eq_expr!}
    input {None::<() >>= None::<()>}
);

#[test]
fn valid_syntax() {
    assert! {None::<() >>= None::<()>};
}
