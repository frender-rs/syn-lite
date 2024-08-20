use syn_lite::parse_item_fn;

mod asserts {
    macro_rules! vis_and_ident {
        ($vis:vis $($ident:ident)+) => {};
    }

    vis_and_ident! {const}
    vis_and_ident! {pub const}
    vis_and_ident! {pub async unsafe const fn}
    vis_and_ident! {pub(in super) const}

    /*
    // local ambiguity
    macro_rules! vis_and_many_ident_and_rest {
        ($vis:vis $($ident:ident)+ $($rest:tt)*) => {};
    }

    vis_and_many_ident_and_rest! {fn name() {}}
    */
}

macro_rules! simple {
    (
        item_fn {
            vis { $vis:vis }
            sig {
                fn {fn}
                ident {simple}
                paren_inputs {()}
                output {}
            }
            block {{}}
        }
        rest {
            pub mod simple;
        }
    ) => {};
}

parse_item_fn! {
    on_finish {simple!}
    input {
        pub fn simple() {}
        pub mod simple;
    }
}

#[test]
fn simple_with_where() {
    let vis;
    let str_ty;
    let sized;

    macro_rules! simple_with_where {
        (
            item_fn {
                outer_attrs { #[doc = r" simple"] }
                vis { $vis:vis }
                sig {
                    fn {fn}
                    ident {simple}
                    paren_inputs {()}
                    output {}
                    where_clause { where $str_ty:ty : $sized:path }
                }
                block {{}}
            }
            rest {
                pub mod simple;
            }
        ) => {
            vis = stringify!($vis);
            str_ty = stringify!($str_ty);
            sized = stringify!($sized);
        };
    }

    parse_item_fn!(
        on_finish {simple_with_where!}
        input {
            /// simple
            pub fn simple() where str: Sized {}
            pub mod simple;
        }
    );

    assert_eq!(vis, "pub ");
    assert_eq!(str_ty, "str");
    assert_eq!(sized, "Sized");
}
