macro_rules! simple {
    (
        item_fn! {
            outer_attrs! {}
            vis! { $vis:vis }
            sig! {
                ident! {simple}
                generics! {
                    params! {}
                    impl_generics! {}
                    type_generics! {}
                    params_name! {}
                }
                paren_inputs! {()}
                output! {}
                where_clause! {}
            }
            inner_attrs! {}
            stmts! {}
        }
        rest! {
            pub mod simple;
        }
    ) => {};
}

syn_lite::parse_item_fn!({
    pub fn simple() {}
    pub mod simple;
} => simple!);

#[test]
fn simple_with_where() {
    let vis;
    let str_ty;
    let sized;

    macro_rules! simple_with_where {
        (
            item_fn! {
                outer_attrs! {}
                vis! { $vis:vis }
                sig! {
                    ident! {simple}
                    generics! {
                        params! {}
                        impl_generics! {}
                        type_generics! {}
                        params_name! {}
                    }
                    paren_inputs! {()}
                    output! {}
                    where_clause! { where $str_ty:ty : $sized:path }
                }
                inner_attrs! {}
                stmts! {}
            }
            rest! {
                pub mod simple;
            }
        ) => {
            vis = stringify!($vis);
            str_ty = stringify!($str_ty);
            sized = stringify!($sized);
        };
    }

    syn_lite::parse_item_fn!({
        pub fn simple() where str: Sized {}
        pub mod simple;
    } => simple_with_where!);

    assert_eq!(vis, "pub ");
    assert_eq!(str_ty, "str");
    assert_eq!(sized, "Sized");
}
