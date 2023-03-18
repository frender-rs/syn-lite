macro_rules! assert_no_inner_attrs {
    (
        inner_attrs! {}
        rest! {}
    ) => {};
}

syn_lite::parse_inner_attrs! {
    {} => assert_no_inner_attrs!
}

#[test]
fn full() {
    let parsed;
    let rest;
    let before;
    let after;

    macro_rules! assert_no_inner_attrs {
        (
            before! { $($before:tt)* }
            inner_attrs! {#![inner = $inner:literal]}
            rest! { $($rest:tt)* }
            after! { $($after:tt)* }
        ) => {
            $($before)*
            parsed = $inner;
            $($rest)*
            $($after)*
        };
    }

    syn_lite::parse_inner_attrs! {
        [before! { before = true; }]
        {
            #![inner = true]
            rest = true;
        }
        [after! { after = true; }]
        => assert_no_inner_attrs!
    }

    assert!(parsed);
    assert!(rest);
    assert!(before);
    assert!(after);
}
