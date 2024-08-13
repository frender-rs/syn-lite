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
