use perl_xs::{AV, IV, SV};

xs! {
    package XSTest::Array;

    sub test_clear(ctx, rv: SV) {
        rv.deref_av().map(|av| av.clear());
    }

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

    sub test_delete(ctx, av: AV) {
        av.delete::<SV>(0)
    }

    sub test_discard(ctx, av: AV) {
        av.discard(1);
    }

    sub test_exists(ctx, av: AV) {
        av.exists(0)
    }

    sub test_extend(ctx, av: AV) {
        av.extend(5);
    }

    sub test_fill(ctx, av: AV) {
        av.fill(4);
    }

    sub test_top_index(ctx, av: AV) {
        av.top_index()
    }

    sub test_pop(ctx, av: AV) {
        av.pop::<SV>()
    }

    sub test_push(ctx, av: AV, sv: SV) {
        av.push(sv);
    }

    sub test_shift(ctx, av: AV) {
        av.shift::<SV>()
    }

    sub test_unshift(ctx, av: AV) {
        av.unshift(2);
    }

    sub test_undef(ctx, av: AV) {
        av.undef();
    }

    sub test_iter(ctx, av: AV) {
        let n: IV = av.iter().filter_map(|sv| sv).map(|sv: SV| sv.iv()).sum();
        n
    }
}
