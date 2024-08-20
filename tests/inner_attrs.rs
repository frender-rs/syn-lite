mod asserts {
    macro_rules! one_token {
        ($t:tt) => {
            1
        };
    }
    macro_rules! token_count {
        ($($t:tt)*) => {

            0 $(+ one_token! {$t})*
        };
    }

    const _: () = {
        assert!(token_count! {#![cfg(..)]} == 3);
        assert!(
            token_count! {
                //! hello
            } == 3
        );
    };
}

macro_rules! assert_no_inner_attrs {
    (
        inner_attrs {}
        rest {}
    ) => {};
}

syn_lite::consume_inner_attrs! {
    on_finish {assert_no_inner_attrs!}
    input {}
}

#[test]
fn one() {
    let parsed;
    let rest;
    let before;
    let after;

    macro_rules! assert_no_inner_attrs {
        (
            before { $($before:tt)* }
            inner_attrs {#![inner = $inner:literal]}
            rest { $($rest:tt)* }
            after { $($after:tt)* }
        ) => {
            $($before)*
            parsed = $inner;
            $($rest)*
            $($after)*
        };
    }

    syn_lite::consume_inner_attrs! {
        on_finish {assert_no_inner_attrs!}
        prepend { before { before = true; } }
        input {
            #![inner = true]
            rest = true;
        }
        append { after { after = true; } }
    }

    assert!(parsed);
    assert!(rest);
    assert!(before);
    assert!(after);
}

#[test]
fn two() {
    let parsed;
    let rest;
    let before;
    let after;

    macro_rules! assert_no_inner_attrs {
        (
            before { $($before:tt)* }
            inner_attrs {
                #![inner = $inner:literal]
                #![cfg(..)]
            }
            rest { $($rest:tt)* }
            after { $($after:tt)* }
        ) => {
            $($before)*
            parsed = $inner;
            $($rest)*
            $($after)*
        };
    }

    syn_lite::consume_inner_attrs! {
        on_finish {assert_no_inner_attrs!}
        prepend { before { before = true; } }
        input {
            #![inner = true]
            #![cfg(..)]
            rest = true;
        }
        append { after { after = true; } }
    }

    assert!(parsed);
    assert!(rest);
    assert!(before);
    assert!(after);
}
