use syn_lite::parse_generics;

macro_rules! assert_no_generics {
    (
        generics! {
            params! {}
            impl_generics! {}
            type_generics! {}
            params_name! {}
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {} => assert_no_generics!
}
parse_generics! {
    {<>} => assert_no_generics!
}

// FIXME: this should not be allowed
macro_rules! assert_comma {
    (
        generics! {
            params! { , }
            impl_generics! { , }
            type_generics! { , }
            params_name! { , }
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {<,>} => assert_comma!
}

macro_rules! assert_lifetime {
    (
        generics! {
            params! { 'a }
            impl_generics! { 'a }
            type_generics! { 'a }
            params_name! { 'a }
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {<'a>} => assert_lifetime!
}

macro_rules! assert_lifetime_comma {
    (
        generics! {
            params! { 'a, }
            impl_generics! { 'a, }
            type_generics! { 'a, }
            params_name! { 'a, }
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {<'a,>} => assert_lifetime_comma!
}

macro_rules! assert_lifetimes {
    (
        generics! {
            params! { 'a, 'b: 'a }
            impl_generics! { 'a, 'b: 'a }
            type_generics! { 'a, 'b }
            params_name! { 'a, 'b }
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {<'a, 'b: 'a>} => assert_lifetimes!
}

macro_rules! assert_lifetimes_comma {
    (
        generics! {
            params! { 'a, 'b: 'a, }
            impl_generics! { 'a, 'b: 'a, }
            type_generics! { 'a, 'b, }
            params_name! { 'a, 'b, }
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {<'a, 'b: 'a,>} => assert_lifetimes_comma!
}

macro_rules! assert_tp {
    (
        generics! {
            params! { T }
            impl_generics! { T }
            type_generics! { $crate::expand_or![[] T] }
            params_name! { T }
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {<T>} => assert_tp!
}

macro_rules! assert_tp_comma {
    (
        generics! {
            params! { T, }
            impl_generics! { T, }
            type_generics! { $crate::expand_or![[] T], }
            params_name! { T, }
        }
        rest! {}
    ) => {};
}

parse_generics! {
    {<T,>} => assert_tp_comma!
}

#[test]
fn full() {
    let r_paths;
    let sized;
    let s_paths;
    let s_default_ty;
    let n_path;
    let m_path;
    let m_default_lit;

    macro_rules! assert_full {
        (
            generics! {
                params! {
                    'a,
                    'b: 'a,
                    R: 'a +'b $(+ $r_paths:path)+,
                    S: 'b + ?$sized:tt $(+ $s_paths:path)+ = $s_default_ty:ty,
                    T,
                    const N: $n_path:path,
                    const M: $m_path:path = $m_default_lit:literal,
                }
                impl_generics! {
                    'a,
                    'b: 'a,
                    R: 'a +'b $(+ $r_paths2:path)+,
                    S: 'b + ?$sized2:tt $(+ $s_paths2:path)+,
                    T,
                    const N: $n_path2:path,
                    const M: $m_path2:path,
                }
                type_generics! {
                    'a,
                    'b,
                    $crate::expand_or![[] R],
                    $crate::expand_or![[] S],
                    $crate::expand_or![[] T],
                    $crate::expand_or![[N] const],
                    $crate::expand_or![[M] const],
                }
                params_name! { 'a, 'b, R, S, T, const N, const M, }
            }
            rest! {}
        ) => {
            r_paths = stringify!($([$r_paths])+);
            sized = stringify!($sized);
            s_paths = stringify!($([$s_paths])+);
            s_default_ty = stringify!($s_default_ty);
            n_path = <$n_path>::MAX;
            m_path = <$m_path>::MAX;
            m_default_lit = $m_default_lit;
        };
    }

    parse_generics! {
        {<
            'a,
            'b: 'a,
            R: 'a +'b + Default + FnOnce() -> T,
            S: 'b + ?Sized + Default + AsRef<T> = &'b T,
            T,
            const N: usize,
            const M: u8 = 1,
        >} => assert_full!
    }

    assert_eq!(r_paths, "[Default] [FnOnce() -> T]");
    assert_eq!(sized, "Sized");
    assert_eq!(s_paths, "[Default] [AsRef<T>]");
    assert_eq!(s_default_ty, "&'b T");
    assert_eq!(n_path, usize::MAX);
    assert_eq!(m_path, u8::MAX);
    assert_eq!(m_default_lit, 1);
}
