use perl_xs::{ SV, AV, IV };

xs! {
    package XSTest::Array;

    sub test_store(ctx) {
        let rv: SV = ctx.st_fetch(0);
        let sv: SV = ctx.st_fetch(1);

        if let Some(av) = rv.deref_av() {
            av.store(0, sv);
        }
    }

    sub test_fetch(ctx) {
        let rv: SV = ctx.st_fetch(0);
        let av: AV = rv.deref_av().expect("an array reference");
        let rc: IV = match av.fetch::<SV>(0) {
            Some(ref sv) if sv.ok() => 1,
            Some(_) => 2,
            None => 3,
        };
        xs_return!(ctx, rc);
    }
}
