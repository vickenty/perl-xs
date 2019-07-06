use perl_xs::{ SV, HV, IV };

xs! {
    package XSTest::Hash;

    sub test_fetch(ctx, hv: HV, key: SV) {
        hv.fetch::<SV>(&key.to_string().unwrap())
    }

    sub test_store(ctx, hv: HV, key: SV, val: SV) {
        hv.store(&key.to_string().unwrap(), val);
    }

    sub test_exists(ctx, hv: HV, sv: SV) {
        hv.exists(&sv.to_string().unwrap())
    }

    sub test_clear(ctx, hv: HV) {
        hv.clear();
    }

    sub test_delete(ctx, hv: HV, sv: SV) {
        hv.delete::<SV>(&sv.to_string().unwrap())
    }

    sub test_iter(ctx, hv: HV) {
        let n: IV = hv.iter().map(|(_, v): (&[u8], IV)| v).sum();
        n
    }

    sub test_values(ctx, hv: HV) {
        let n: IV = hv.values::<IV>().sum();
        n
    }

    sub test_keys(ctx, hv: HV) {
        let n: IV = hv.keys().map(|k| {
            hv.fetch::<IV>(std::str::from_utf8(k).unwrap()).unwrap()
        }).sum();
        n
    }

    sub test_for(ctx, hv: HV) {
        let mut n: IV = 0;
        for (_, v) in &hv {
            n += v.iv();
        }
        n
    }
}
