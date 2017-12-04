use perl_xs::FromPerlKV;

xs! {
    package XSTest::Derive;

    sub test_from_kv_debug(ctx) {
        // Offset should be made automatic after arg unpacking
        match TestStruct::from_perl_kv(&mut ctx, 0) {
            Ok(s) => {
                format!("{:?}",s)
            },
            Err(e) => {
                croak!(format!("{}",e));
            }
        }
    }
    sub test_from_kv_error(ctx) {
        let err = TestStruct::from_perl_kv(&mut ctx, 0).unwrap_err();
        format!("{:?}",err)
    }
    sub test_from_kv_error_display(ctx) {
        let err = TestStruct::from_perl_kv(&mut ctx, 0).unwrap_err();
        format!("{}",err)
    }
}

#[derive(FromPerlKV,Debug)]
struct TestStruct {
    alpha:          bool,
    beta:           String,
    #[perlxs(key="-charlie", key="-charles", key="-chuck")]
    charlie:        String,
    delta:          Option<bool>,
    #[perlxs(key = "_echo")]
    echo:          Option<String>,
}