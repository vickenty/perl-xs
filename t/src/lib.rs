#[macro_use]
extern crate perl_xs;

use perl_xs::{ IV, UV, NV };
use perl_xs::{ SV, AV };

XS! {
    package XSTest {
        sub test_push(ctx) {
            ctx.st_prepush();
            ctx.st_push(-1 as IV);
            ctx.st_push(42 as UV);
            ctx.st_push((42.0 as NV).powf(0.5));
            ctx.st_push("Don't panic!");
            ctx.st_putback();
        }

        sub test_push_unicode(ctx)  {
            ctx.st_prepush();
            ctx.st_push("Don't panic!");
            ctx.st_push("Не паникуйте!");
            ctx.st_push("慌てる必要がありません");
            ctx.st_push("☺");
            ctx.st_push("a\u{035c}\u{0361}");
            ctx.st_putback();
        }

        sub test_unicode(ctx) {
            let sv: SV = ctx.st_fetch(0);
            let utf8: IV = if sv.utf8() { 1 } else { 0 };

            match sv.str() {
                Ok(s) => xs_return!(ctx, s, utf8, -1 as IV),
                Err(e) => xs_return!(ctx, "", utf8, e.valid_up_to() as IV),
            }
        }
    }
    loader boot_XSTest;
}
