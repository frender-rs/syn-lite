use syn_lite::consume_bounds;

macro_rules! expect_bounds {
    (
        consumed_bounds {MyTrait<MyType<'a>, MY_CONST>}
        rest {,}
    ) => {
        1
    };
}

const _: () = {
    assert!(
        consume_bounds! {
            on_finish {expect_bounds!}
            input {MyTrait<MyType<'a>, MY_CONST>,}
        } == 1
    )
};

macro_rules! expect_bounds_before_eq {
    (
        consumed_bounds {MyTrait<MyType<'a>, MY_CONST>}
        rest {= MyDefaultTy,}
    ) => {};
}
consume_bounds! {
    on_finish {expect_bounds_before_eq!}
    input {MyTrait<MyType<'a>, MY_CONST> = MyDefaultTy,}
}
consume_bounds! {
    on_finish {expect_bounds_before_eq!}
    input {MyTrait<MyType<'a>, MY_CONST>= MyDefaultTy,}
}

macro_rules! expect_fn_bounds {
    (
        consumed_bounds {for<'a> Fn(&'a ()) -> u8}
        rest {}
    ) => {};
}

consume_bounds! {
    on_finish {expect_fn_bounds!}
    input {for<'a> Fn(&'a ()) -> u8}
}

macro_rules! expect_fn_bounds_in_assoc_bounds {
    (
        consumed_bounds {Trait<Assoc: for<'a> Fn(&'a ()) -> u8>}
        rest {}
    ) => {};
}

consume_bounds! {
    on_finish {expect_fn_bounds_in_assoc_bounds!}
    input {Trait<Assoc: for<'a> Fn(&'a ()) -> u8>}
}
