use perl_xs::{Context, IV, NV, SV};

package!("XSTest::Scalar");

#[perlxs]
fn test_sv_ok(sv: SV) -> (bool, bool, bool, bool, bool, bool) {
    (sv.ok(), sv.iv_ok(), sv.uv_ok(), sv.nv_ok(), sv.pv_ok(), sv.rv_ok())
}

#[perlxs]
fn test_sv_type(rv: SV) -> (bool, bool, bool, bool, bool) {
    let sv: SV = rv.deref().unwrap();
    (sv.is_scalar(), sv.is_array(), sv.is_hash(), sv.is_code(), sv.is_glob())
}

#[perlxs]
fn test_unicode(sv: SV) -> (Option<String>, i64, i64) {
    let utf8: IV = if sv.utf8() { 1 } else { 0 };

    match sv.to_string() {
        Ok(s) => (Some(s), utf8, -1 as IV),
        Err(e) => (None, utf8, e.utf8_error().valid_up_to() as IV),
    }
}

#[perlxs]
fn test_new_sv_iv(iv: IV, ctx: &mut Context) -> SV {
    ctx.new_sv(iv)
}

#[perlxs]
fn test_new_sv_nv(nv: NV, ctx: &mut Context) -> SV {
    ctx.new_sv(nv)
}

#[perlxs]
fn test_new_sv_undef(ctx: &mut Context) -> SV {
    ctx.sv_undef()
}
