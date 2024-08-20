use syn_lite::parse_generics;

pub mod asserts {
    macro_rules! lifetimes {
        ($($lt:lifetime $(+ $lts:lifetime)* $(+)? )?) => {};
    }

    lifetimes! {}
    lifetimes! {'a}
    lifetimes! {'a + 'b}
    lifetimes! {'a + 'b +}
    lifetimes! {'a + 'b + 'c}
    lifetimes! {'a + 'b + 'c +}

    macro_rules! many_plus_and_path {
        ( $(+ $lts:path)* ) => {};
    }

    many_plus_and_path! {+ Sized}
    many_plus_and_path! {+ mod_path::Sized}
    many_plus_and_path! {+ mod_path::Sized + __}
    many_plus_and_path! {+ Trait::<_>}
    many_plus_and_path! {+ Trait<_>}
    many_plus_and_path! {+ Trait<_>}

    macro_rules! match_many_generics_info {
        (
            $({
                attrs {$($lifetime_attr:tt)*}
                lifetime {}
            })*
            $({
                attrs {$($name_attr:tt)*}
                name {}
            })*
        ) => {};
    }

    /*
    // this would leads to local ambiguity
    macro_rules! match_many_generics_info {
        (
            $({
                attrs $attr:tt
                lifetime {}
            })*
            $({
                attrs $type_attr:tt
                name {}
            })*
        ) => {};
    }
    */

    match_many_generics_info! {
        {attrs {} lifetime {}}
        {attrs {} name {}}
    }

    /*
    // this would leads to local ambiguity
    match_many_generics_info! {
        {attrs {#[__]} lifetime {}}
        {attrs {} name {}}
    }
    */

    macro_rules! expect_expr {
        ($e:expr) => {};
    }
    macro_rules! rematch_path_as_expr {
        ($p:path) => {
            expect_expr! {$p}
        };
    }

    rematch_path_as_expr! {a}
    rematch_path_as_expr! {a::a}
    rematch_path_as_expr! {A<A>::A}
}

macro_rules! assert_no_generics {
    (
        parsed_generics {
            generics {}
            impl_generics {}
            type_generics {}
            generics_info {}
        }
        rest {}
    ) => {};
}

parse_generics! {
    on_finish {assert_no_generics!}
    input {}
}

macro_rules! assert_no_generics_with_rest {
    (
        parsed_generics {
            generics {}
            impl_generics {}
            type_generics {}
            generics_info {}
        }
        rest {> for _ {}}
    ) => {};
}

parse_generics! {
    on_finish {assert_no_generics_with_rest!}
    input {> for _ {}}
}

macro_rules! assert_no_generics_with_gt {
    (
        parsed_generics {
            generics {}
            impl_generics {}
            type_generics {}
            generics_info {}
        }
        rest {>}
    ) => {};
}

parse_generics! {
    on_finish {assert_no_generics_with_gt!}
    input {>}
}

macro_rules! assert_lifetime {
    (
        parsed_generics {
            generics { 'a , }
            impl_generics { 'a , }
            type_generics { 'a , }
            generics_info { { lifetime {'a} } }
        }
        rest {>}
    ) => {};
}

parse_generics! {
    on_finish {assert_lifetime!}
    input {'a>}
}

parse_generics! {
    on_finish {assert_lifetime!}
    input {'a,>}
}

macro_rules! assert_lifetimes {
    (
        parsed_generics {
            generics { 'a, 'b: 'a, }
            impl_generics { 'a, 'b: 'a, }
            type_generics { 'a, 'b, }
            generics_info {
                {lifetime {'a}}
                {lifetime {'b} bounds {'a}}
            }
        }
        rest {>}
    ) => {};
}

parse_generics! {
    on_finish {assert_lifetimes!}
    input {'a, 'b: 'a>}
}

parse_generics! {
    on_finish {assert_lifetimes!}
    input {'a, 'b: 'a,>}
}

macro_rules! assert_tp {
    (
        parsed_generics {
            generics { T, }
            impl_generics { T, }
            type_generics { T, }
            generics_info {
                { name {T} }
            }
        }
        rest {}
    ) => {};
}

parse_generics! {
    on_finish{assert_tp!}
    input{T}
}

parse_generics! {
    on_finish{assert_tp!}
    input{T,}
}

#[test]
fn full() {
    let s_default_ty;
    let n_ty;
    let n_ty_1;
    let m_ty;
    let m_ty_1;
    let m_default_expr;
    let s_default_ty_2;
    let n_ty_2;
    let m_ty_2;
    let m_default_expr_2;
    macro_rules! assert_full {
        (
            parsed_generics {
                generics {
                    'a,
                    'b: 'a,
                    #[doc = r" hello"]
                    'c:,
                    R: 'a +'b + Default + FnOnce() -> T,
                    S: 'b + ?Sized + Default + AsRef<T> = $s_default_ty:ty,
                    T,
                    #[doc = "world"]
                    U,
                    const N: $n_ty:ty,
                    #[cfg(..)]
                    const M: $m_ty:ty = $m_default_expr:expr,
                }
                impl_generics {
                    'a,
                    'b: 'a,
                    #[doc = r" hello"]
                    'c:,
                    R: 'a +'b + Default + FnOnce() -> T,
                    S: 'b + ?Sized + Default + AsRef<T>,
                    T,
                    #[doc = "world"]
                    U,
                    const N: $n_ty_1:ty,
                    #[cfg(..)]
                    const M: $m_ty_1:ty,
                }
                type_generics {
                    'a,
                    'b,
                    #[doc = r" hello"]
                    'c,
                    R,
                    S,
                    T,
                    #[doc = "world"]
                    U,
                    N,
                    #[cfg(..)]
                    M,
                }
                generics_info {
                    { lifetime {'a} }
                    { lifetime {'b} bounds {'a} }
                    { lifetime_attrs {#[doc = r" hello"]} lifetime {'c} bounds {} }
                    { name {R} bounds {'a +'b + Default + FnOnce() -> T} }
                    { name {S} bounds {'b + ?Sized + Default + AsRef<T>} default_ty {$s_default_ty_2:ty} }
                    { name {T} }
                    { type_attrs {#[doc = "world"]} name {U} }
                    { const {const} name {N} bounds {$n_ty_2:ty} }
                    { const_attrs {#[cfg(..)]} const {const} name {M} bounds {$m_ty_2:ty} default_expr {$m_default_expr_2:expr} }
                }
            }
            rest {}
        ) => {
            s_default_ty = stringify!($s_default_ty);
            n_ty = stringify!($n_ty);
            m_ty = stringify!($m_ty);
            m_default_expr = stringify!($m_default_expr);

            n_ty_1 = stringify!($n_ty_1);
            m_ty_1 = stringify!($m_ty_1);

            s_default_ty_2 = stringify!($s_default_ty_2);
            n_ty_2 = stringify!($n_ty_2);
            m_ty_2 = stringify!($m_ty_2);
            m_default_expr_2 = stringify!($m_default_expr_2);
        };
    }

    parse_generics! {
        on_finish {assert_full!}
        input {
            'a,
            'b: 'a,
            /// hello
            'c:,
            R: 'a +'b + Default + FnOnce() -> T,
            S: 'b + ?Sized + Default + AsRef<T> = &'b T,
            T,
            #[doc = "world"]
            U,
            const N: usize,
            #[cfg(..)]
            const M: u8 = 1,
        }
    }

    assert_eq!(s_default_ty, "&'b T");
    assert_eq!(n_ty, "usize");
    assert_eq!(m_ty, "u8");
    assert_eq!(m_default_expr, "1");

    assert_eq!(n_ty_1, "usize");
    assert_eq!(m_ty_1, "u8");

    assert_eq!(s_default_ty_2, "&'b T");
    assert_eq!(n_ty_2, "usize");
    assert_eq!(m_ty_2, "u8");
    assert_eq!(m_default_expr_2, "1");
}
