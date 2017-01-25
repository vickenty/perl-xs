use perl_xs::{ IV, NV, SV };

xs! {
    package XSTest;

    sub test_sv_ok(ctx, sv: SV) {
        (
            sv.ok(),
            sv.iv_ok(),
            sv.uv_ok(),
            sv.nv_ok(),
            sv.pv_ok(),
            sv.rv_ok()
        )
    }

    sub test_sv_type(ctx, rv: SV) {
        let sv: SV = rv.deref().unwrap();
        (
           sv.is_scalar(),
           sv.is_array(),
           sv.is_hash(),
           sv.is_code(),
           sv.is_glob(),
        )
    }

    sub test_unicode(ctx, sv: SV) {
        let utf8: IV = if sv.utf8() { 1 } else { 0 };

        match sv.to_string() {
            Ok(s) => (Some(s), utf8, -1 as IV),
            Err(e) => (None, utf8, e.utf8_error().valid_up_to() as IV),
        }
    }

    sub test_new_sv_iv(ctx, iv: IV) {
        ctx.new_sv(iv)
    }

    sub test_new_sv_nv(ctx, nv: NV) {
        ctx.new_sv(nv)
    }

    sub test_new_sv_undef(ctx) {
        ctx.sv_undef()
    }
}
