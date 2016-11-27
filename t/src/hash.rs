use perl_xs::{ SV, HV };

xs! {
    package XSTest::Hash;

    sub test_fetch(ctx, hv: HV, key: SV) {
        if let Some(sv) = hv.fetch::<SV>(&key.str().unwrap()) {
            xs_return!(ctx, sv);
        }
        xs_return!(ctx);
    }

    sub test_store(ctx, hv: HV, key: SV, val: SV) {
        hv.store(&key.str().unwrap(), val);
        xs_return!(ctx);
    }

    sub test_exists(ctx, hv: HV, sv: SV) {
        xs_return!(ctx, hv.exists(&sv.str().unwrap()));
    }

    sub test_clear(ctx, hv: HV) {
        hv.clear();
        xs_return!(ctx);
    }

    sub test_delete(ctx, hv: HV, sv: SV) {
        if let Some(sv) = hv.delete::<SV>(&sv.str().unwrap()) {
            xs_return!(ctx, sv);
        }
        xs_return!(ctx);
    }
}
