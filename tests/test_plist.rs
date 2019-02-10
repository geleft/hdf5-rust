#[macro_use]
extern crate mashup;

use std::mem;

use libhdf5_sys::h5::hsize_t;

use h5::file::*;
use h5::plist::*;

macro_rules! test_plist {
    ($ty:ident, $builder:ident, $cls:expr => {$($field:ident($get:ident),)*}) => {
        test_plist!($ty, $builder, $cls => {$($field($get)),*});
    };

    ($ty:ident, $builder:ident, $cls:expr => {$($field:ident($get:ident)),*}) => {
        let pl_default = $ty::try_new()?;
        assert_eq!(pl_default.class()?, $cls);

        let mut builder = $ty::build();
        $(
        builder.$field($field.clone());
        )*

        let pl = builder.finish()?;
        assert_eq!(pl.class()?, $cls);
        $(
        assert_eq!(pl.$field(), $field);
        assert_eq!(pl.$get().unwrap(), $field);
        )*;

        assert_eq!(pl_default, pl_default);
        assert_eq!(pl, pl);
        assert_ne!(pl, pl_default);

        let pl_copy = $builder::from_plist(&pl)?.finish()?;
        assert_eq!(pl, pl_copy);
    }
}

macro_rules! test_pl {
    ($ty:ident, $field:ident ($($arg:expr),+): $($name:ident=$value:expr),+) => (
        test_pl!($ty, $field ($($arg,)+): $($name=$value,)+)
    );

    ($ty:ident, $field:ident ($($arg:expr,)+): $($name:ident=$value:expr,)+) => ({
        let mut b = $ty::build();
        mashup! { m["get" $field] = get_ $field; }
        b.$field($($arg,)+);
        let fapl = b.finish()?;
        $(assert_eq!(fapl.$field().$name, $value);)+
        m! { $(assert_eq!(fapl."get" $field()?.$name, $value);)+ }
    });

    ($ty:ident, $field:ident: $($name:ident=$value:expr),+) => (
        test_pl!($ty, $field: $($name=$value,)+)
    );

    ($ty:ident, $field:ident: $($name:ident=$value:expr,)+) => ({
        test_pl!($ty, $field ($($value,)+): $($name=$value,)+)
    });

    ($ty:ident, $field:ident ($arg:expr): $value:expr) => ({
        let mut b = $ty::build();
        mashup! { m["get" $field] = get_ $field; }
        b.$field($arg);
        let fapl = b.finish()?;
        assert_eq!(fapl.$field(), $value);
        m! { assert_eq!(fapl."get" $field()?, $value); }
    });

    ($ty:ident, $field:ident: $value:expr) => ({
        test_pl!($ty, $field ($value): $value)
    });
}

macro_rules! test_pl_common {
    ($cls:ident, $plc:expr, $func:expr) => {
        let pl_default = $cls::try_new()?;
        assert_eq!(pl_default.class()?, $plc);
        assert_eq!(pl_default, pl_default);

        let mut b = $cls::build();
        let pl = $func(&mut b)?;
        assert_eq!(pl.class()?, $plc);
        assert_eq!(pl, pl);
        assert_ne!(pl, pl_default);

        let pl2 = pl.copy();
        assert_eq!(pl.class()?, $plc);
        assert_eq!(pl2, pl);
        assert_ne!(pl2, pl_default);
    };
}

macro_rules! check_matches {
    ($e:expr, $o:expr, $($p:tt)+) => (
        match $e {
            $($p)+ => $o,
            ref e => panic!("assertion failed: `{:?}` does not match `{}`", e, stringify!($($p)+)),
        }
    )
}

#[test]
fn test_file_access_plist() -> h5::Result<()> {
    let fapl = FileAccess::try_new()?;
    println!("{:#?}", fapl);
    let mut b = FileAccess::build();

    b.split();
    let fapl3 = b.finish()?;
    println!("{:#?}", fapl3);
    println!("{:#?}", fapl3.get_driver());

    Ok(())
}
