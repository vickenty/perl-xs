use perl_xs::SV;

xs! {
    package XSTest::Hash;

    sub test_fetch(ctx, rv: SV, key: SV) {
        if let Some(hv) = rv.deref().and_then(|hv| hv.into_hv()) {
            if let Some(sv) = hv.fetch::<SV>(&key.str().unwrap()) {
                xs_return!(ctx, sv);
            }
        }

        xs_return!(ctx);
    }

    sub test_store(ctx, rv: SV, key: SV, val: SV) {
        if let Some(hv) = rv.deref_hv() {
            hv.store(&key.str().unwrap(), val);
        }

        xs_return!(ctx);
    }

    sub test_exists(ctx, rv: SV, sv: SV) {
        if let Some(hv) = rv.deref_hv() {
            xs_return!(ctx, hv.exists(&sv.str().unwrap()));
        }
        xs_return!(ctx);
    }

    sub test_clear(ctx, rv: SV) {
        if let Some(hv) = rv.deref_hv() {
            hv.clear();
        }
        xs_return!(ctx);
    }

    sub test_delete(ctx, rv: SV, sv: SV) {
        if let Some(hv) = rv.deref_hv() {
            if let Some(sv) = hv.delete::<SV>(&sv.str().unwrap()) {
                xs_return!(ctx, sv);
            }
        }

        xs_return!(ctx, sv);
    }
}
