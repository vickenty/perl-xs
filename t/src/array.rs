use perl_xs::SV;

xs! {
    package XSTest::Array;

    sub test_store(ctx) {
        let rv: SV = ctx.st_fetch(0);
        let sv: SV = ctx.st_fetch(1);
        if let Some(av) = rv.deref().and_then(|av| av.as_av()) {
            av.store(0, sv);
        }
    }
}
