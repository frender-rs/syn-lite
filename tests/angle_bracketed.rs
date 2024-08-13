use syn_lite::consume_optional_angle_bracketed;

macro_rules! expect_empty {
    (rest {}) => {};
}

consume_optional_angle_bracketed! {
    on_finish {expect_empty!}
    input {}
}

macro_rules! expect_rest {
    (rest {there is no < >}) => {};
}

consume_optional_angle_bracketed! {
    on_finish {expect_rest!}
    input {there is no < >}
}

macro_rules! expect_empty_generics {
    (
        angle_bracketed {<>}
        rest {}
    ) => {};
}

consume_optional_angle_bracketed! {
    on_finish {expect_empty_generics!}
    input {<>}
}

macro_rules! expect_not_splitted_tokens {
    (
        angle_bracketed {<< >>}
        rest {}
    ) => {};
}

consume_optional_angle_bracketed! {
    on_finish {expect_not_splitted_tokens!}
    input {<<>>}
}

macro_rules! expect_generics {
    (
        angle_bracketed {
            <
                T: for<'a> Trait<'a, Gat<'a>= T::Gat<'b>>,
                T: 'a + Trait<'a> + 'b = DefaultTy,
                const T: usize = 1,
            >
        }
        rest {() {}}
    ) => {};
}

consume_optional_angle_bracketed! {
    on_finish {expect_generics!}
    input {
        <
            T: for<'a> Trait<'a, Gat<'a>= T::Gat<'b>>,
            T: 'a + Trait<'a> + 'b = DefaultTy,
            const T: usize = 1,
        > () {}
    }
}
