use perl_xs::{ SV, AV, IV };

xs! {
    package XSTest::Array;

    sub test_store(ctx, rv: SV, sv: SV) {
        if let Some(av) = rv.deref_av() {
            av.store(0, sv);
        }
    }

    sub test_fetch(ctx, av: AV) {
        match av.fetch::<SV>(0) {
            Some(ref sv) if sv.ok() => 1 as IV,
            Some(_) => 2,
            None => 3,
        }
    }
}
