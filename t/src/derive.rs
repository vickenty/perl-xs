use perl_xs::FromPerlKV;

xs! {
    package XSTest::Derive;

    sub from_kv(ctx) {
        let teststruct = TestStruct::from_perl_kv(&mut ctx, 0).unwrap(); // Offset should be made automatic after arg unpacking
        format!("{:?}",teststruct)
    }
}

#[derive(FromPerlKV,Debug)]
struct TestStruct {
    alpha:          bool,
    beta:           Option<String>,
    #[perlxs(key = "-charlie")]
    charlie:        Option<String>,
    delta:          Option<bool>,
    #[perlxs(key = "_echo")]
    echo:          Option<String>,
}