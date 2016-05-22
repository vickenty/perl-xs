use perl_xs::{ IV, NV, SV };

xs! {
    package XSTest;

    sub test_sv_ok(ctx) {
        let sv: SV = ctx.st_fetch(0);
        xs_return!(ctx,
            sv.ok(),
            sv.iv_ok(),
            sv.uv_ok(),
            sv.nv_ok(),
            sv.pv_ok(),
            sv.rv_ok());
    }

    sub test_sv_type(ctx) {
        let sv: SV = ctx.st_fetch::<SV>(0).deref().unwrap();
        xs_return!(ctx,
                   sv.is_scalar(),
                   sv.is_array(),
                   sv.is_hash(),
                   sv.is_code(),
                   sv.is_glob());
    }

    sub test_unicode(ctx) {
        let sv: SV = ctx.st_fetch(0);
        let utf8: IV = if sv.utf8() { 1 } else { 0 };

        match sv.str() {
            Ok(s) => xs_return!(ctx, &s[..], utf8, -1 as IV),
            Err(e) => xs_return!(ctx, "", utf8, e.utf8_error().valid_up_to() as IV),
        }
    }

    sub test_new_sv_iv(ctx) {
        let iv: IV = ctx.st_fetch(0);
        let sv = ctx.new_sv(iv);
        xs_return!(ctx, sv);
    }

    sub test_new_sv_nv(ctx) {
        let nv: NV = ctx.st_fetch(0);
        let sv = ctx.new_sv(nv);
        xs_return!(ctx, sv);
    }

    sub test_new_sv_undef(ctx) {
        let sv = ctx.new_sv(());
        xs_return!(ctx, sv);
    }
}
