use perl_xs::{ SV, HV };

xs! {
    package XSTest::Hash;

    sub test_fetch(ctx, hv: HV, key: SV) {
        hv.fetch::<SV>(&key.str().unwrap())
    }

    sub test_store(ctx, hv: HV, key: SV, val: SV) {
        hv.store(&key.str().unwrap(), val);
    }

    sub test_exists(ctx, hv: HV, sv: SV) {
        hv.exists(&sv.str().unwrap())
    }

    sub test_clear(ctx, hv: HV) {
        hv.clear();
    }

    sub test_delete(ctx, hv: HV, sv: SV) {
        hv.delete::<SV>(&sv.str().unwrap())
    }
}
