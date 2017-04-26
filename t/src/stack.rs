use perl_xs::{ IV, UV, NV };

xs! {
    package XSTest;

    sub test_push(ctx) {
        ctx.st_push(-1 as IV);
        ctx.st_push(42 as UV);
        ctx.st_push((42.0 as NV).powf(0.5));
        ctx.st_push("Don't panic!");
        ctx.st_push(true);
        ctx.st_push(false);
    }

    sub test_push_unicode(ctx)  {
        ctx.st_push("Don't panic!");
        ctx.st_push("Не паникуйте!");
        ctx.st_push("あわてるな");
        ctx.st_push("Nu intra i\u{0302}n panica\u{0306}");
        ctx.st_push("😱❌");
    }
}
