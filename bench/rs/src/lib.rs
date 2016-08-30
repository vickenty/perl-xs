#[macro_use]
extern crate perl_xs;
#[macro_use]
extern crate perl_sys;

use perl_xs::{ SV, AV, IV, NV };

xs! {
    package XSBench::RS;
    
    sub rs_sum_loop(ctx) {
        let param: SV = ctx.st_fetch(0);
        let array: AV = param.deref_av().expect("not an array ref");
        let mut sum = 0.0;
        for i in 0..array.top_index() + 1 {
            if let Some(val) = array.fetch(i) {
                sum += val;
            }
        }
        xs_return!(ctx, sum);
    }

    sub rs_sum_iter(ctx) {
        let param: SV = ctx.st_fetch(0);
        let array: AV = param.deref_av().expect("not an array ref");
        let sum: NV = array.iter().map(|v| v.unwrap_or(0.0)).sum();
        xs_return!(ctx, sum);
    }

    sub rs_gcd(ctx) {
        let mut a = ctx.st_fetch::<IV>(0);
        let mut b = ctx.st_fetch::<IV>(1);
        while b > 0 {
            let t = a;
            a = b;
            b = t % b;
        }

        let res = if a < 0 {
            -a
        } else {
            a
        };

        xs_return!(ctx, res);
    }
}

xs! {
    bootstrap boot_XSBench__RS;
    use self;
}
