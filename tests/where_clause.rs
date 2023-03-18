use syn_lite::parse_where_clause;

macro_rules! assert_no_where {
    (
        where_clause! {}
        rest! {}
    ) => {};
}

parse_where_clause! {
    {} => assert_no_where!
}

macro_rules! assert_only_where {
    (
        where_clause! { where }
        rest! {}
    ) => {};
}

parse_where_clause! {
    {where} => assert_only_where!
}

#[test]
fn many() {
    let r_ty;
    let sized;
    let r_path;
    let t_ty;
    let t_path;
    macro_rules! assert_many {
        (
            where_clause! {
                where
                    'a: 'b + 'c,
                    'b: 'c,
                    $r_ty:ty : ? $sized:tt + $r_path:path,
                    for<'a> $t_ty:ty : $t_path:path
            }
            rest! { ; }
        ) => {
            r_ty = stringify!($r_ty);
            sized = stringify!($sized);
            r_path = stringify!($r_path);
            t_ty = stringify!($t_ty);
            t_path = stringify!($t_path);
        };
    }

    parse_where_clause! {
        {
            where
                'a: 'b + 'c,
                'b: 'c,
                R: ?Sized + AsRef<str>,
                for<'a> T: FnOnce(&'a str)
            ;
        } => assert_many!
    }

    assert_eq!(r_ty, "R");
    assert_eq!(sized, "Sized");
    assert_eq!(r_path, "AsRef<str>");
    assert_eq!(t_ty, "T");
    assert_eq!(t_path, "FnOnce(&'a str)");
}

#[test]
fn many_trailing() {
    let r_ty;
    let sized;
    let r_path;
    let t_ty;
    let t_path;
    macro_rules! assert_many_trailing {
        (
            where_clause! {
                where
                    'a: 'b + 'c,
                    'b: 'c,
                    $r_ty:ty : ? $sized:tt + $r_path:path,
                    for<'a> $t_ty:ty : $t_path:path,
            }
            rest! { {} }
        ) => {
            r_ty = stringify!($r_ty);
            sized = stringify!($sized);
            r_path = stringify!($r_path);
            t_ty = stringify!($t_ty);
            t_path = stringify!($t_path);
        };
    }

    parse_where_clause! {
        {
            where
                'a: 'b + 'c,
                'b: 'c,
                R: ?Sized + AsRef<str>,
                for<'a> T: FnOnce(&'a str),
            {}
        } => assert_many_trailing!
    }

    assert_eq!(r_ty, "R");
    assert_eq!(sized, "Sized");
    assert_eq!(r_path, "AsRef<str>");
    assert_eq!(t_ty, "T");
    assert_eq!(t_path, "FnOnce(&'a str)");
}
