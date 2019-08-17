use perl_xs::{Context, DeriveTryFromContext, TryFromContext};

package!("XSTest::Derive");

#[perlxs]
fn test_from_kv(test: TestStruct) -> String {
    // Offset should be made automatic after arg unpacking
    format!("{:?}", test)
}

#[perlxs]
fn test_from_kv_dual_arg_unpack(test: TestStruct, ctx: &mut Context) -> (String, String) {
    // Known limitation: Context has to be in the last arg position due to borrowing.
    // TODO: determine if we can clone the Context for this, or maybe wrap it in an Rc?

    // We should be able to unpack this twice.
    // Generally you wouldn't do this, but it's useful to test whether we're handling Context fn args properly

    let auto = format!("{:?}", test);

    let mut index: isize = 0;
    let manual = format!("{:?}", TestStruct::try_from_context(ctx, "thingy", &mut index).unwrap());

    // Should be the same. Also test tuple of Strings
    (auto, manual)
}

#[perlxs]
fn test_from_kv_bool(_test: TestStruct) -> bool {
    true
}

mod error {
    use super::*;
    package!("XSTest::Derive::Error");

    #[perlxs]
    fn test_from_kv_error(ctx: &mut Context) -> String {
        let mut index: isize = 0;
        let err = TestStruct::try_from_context(ctx, "thingy", &mut index).unwrap_err();
        format!("{:?}", err)
    }

    #[perlxs]
    fn test_from_kv_error_display(ctx: &mut Context) -> String {
        let mut index: isize = 0;
        let err = TestStruct::try_from_context(ctx, "thingy", &mut index).unwrap_err();
        format!("{}", err)
    }
}

#[perlxs]
fn test_from_kv_debug(ctx: &mut Context) -> String {
    let mut index: isize = 0;
    // Offset should be made automatic after arg unpacking
    match <TestStruct as TryFromContext>::try_from_context(ctx, "thingy", &mut index) {
        Ok(s) => format!("{:?}", s),
        Err(e) => {
            croak!(format!("{}", e));
        }
    }
}

//pub const PERL_XS:
//&'static [(&'static str, ::perl_xs::raw::XSUBADDR_t)] =
//    &[("XSTest::Derive::test_from_kv", _xs_test_from_kv as ::perl_xs::raw::XSUBADDR_t),
//    ("XSTest::Derive::test_from_kv_dual_arg_unpack", _xs_test_from_kv_dual_arg_unpack as ::perl_xs::raw::XSUBADDR_t),
//    ("XSTest::Derive::test_from_kv_bool", _xs_test_from_kv_bool as ::perl_xs::raw::XSUBADDR_t),
//    ("XSTest::Derive::test_from_kv_error", _xs_test_from_kv_error as ::perl_xs::raw::XSUBADDR_t),
//    ("XSTest::Derive::test_from_kv_error_display", _xs_test_from_kv_error_display as ::perl_xs::raw::XSUBADDR_t),
//    ("XSTest::Derive::test_from_kv_debug", _xs_test_from_kv_debug as ::perl_xs::raw::XSUBADDR_t),
//    ];

#[derive(DeriveTryFromContext, Debug)]
struct TestStruct {
    alpha: bool,
    beta: String,
    #[perlxs(key = "-charlie", key = "-charles", key = "-chuck")]
    charlie: String,
    delta: Option<bool>,
    #[perlxs(key = "_echo")]
    echo: Option<String>,
}
