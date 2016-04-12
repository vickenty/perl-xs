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
            ctx.st_putback();
        }
    }
    loader boot_XSTest;
}
