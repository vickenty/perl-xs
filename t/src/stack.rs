use perl_xs::{Context, IV, NV, UV};

package!("XSTest::Stack");

#[perlxs]
fn test_push(ctx: &mut Context) {
    ctx.st_push(-1 as IV);
    ctx.st_push(42 as UV);
    ctx.st_push((42.0 as NV).powf(0.5));
    ctx.st_push("Don't panic!");
    ctx.st_push(true);
    ctx.st_push(false);
}

#[perlxs]
fn test_push_unicode(ctx: &mut Context) {
    ctx.st_push("Don't panic!");
    ctx.st_push("Не паникуйте!");
    ctx.st_push("あわてるな");
    ctx.st_push("Nu intra i\u{0302}n panica\u{0306}");
    ctx.st_push("😱❌");
}
