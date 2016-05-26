use perl_xs::SV;

xs! {
    package XSTest::Hash;

    sub test_fetch(ctx) {
        let rv: SV = ctx.st_fetch(0);
        let key: SV = ctx.st_fetch(1);
        
        if let Some(hv) = rv.deref().and_then(|hv| hv.into_hv()) {
            if let Some(sv) = hv.fetch::<SV>(&key.str().unwrap()) {
                xs_return!(ctx, sv);
            }
        }

        xs_return!(ctx);
    }

    sub test_store(ctx) {
        let rv: SV = ctx.st_fetch(0);
        let key: SV = ctx.st_fetch(1);
        let val: SV = ctx.st_fetch(2);
        
        if let Some(hv) = rv.deref().and_then(|hv| hv.into_hv()) {
            hv.store(&key.str().unwrap(), val);
        }

        xs_return!(ctx);
    }

    sub test_exists(ctx) {
        let rv: SV = ctx.st_fetch(0);
        let sv: SV = ctx.st_fetch(1);

        if let Some(hv) = rv.deref().and_then(|hv| hv.into_hv()) {
            xs_return!(ctx, hv.exists(&sv.str().unwrap()));
        }
        xs_return!(ctx);
    }

    sub test_clear(ctx) {
        let rv: SV = ctx.st_fetch(0);
        if let Some(hv) = rv.deref().and_then(|hv| hv.into_hv()) {
            hv.clear();
        }
        xs_return!(ctx);
    }

    sub test_delete(ctx) {
        let rv: SV = ctx.st_fetch(0);
        let sv: SV = ctx.st_fetch(1);

        if let Some(hv) = rv.deref().and_then(|hv| hv.into_hv()) {
            if let Some(sv) = hv.delete::<SV>(&sv.str().unwrap()) {
                xs_return!(ctx, sv);
            }
        }

        xs_return!(ctx, sv);
    }
}
