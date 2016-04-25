use perl_xs::{ IV, SV };

xs! {
    package XSTest;

    sub test_unicode(ctx) {
        let sv: SV = ctx.st_fetch(0);
        let utf8: IV = if sv.utf8() { 1 } else { 0 };

        match sv.str() {
            Ok(s) => xs_return!(ctx, &s[..], utf8, -1 as IV),
            Err(e) => xs_return!(ctx, "", utf8, e.utf8_error().valid_up_to() as IV),
        }
    }
}
