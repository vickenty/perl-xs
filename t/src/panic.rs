use perl_xs::IV;
use perl_xs::G_DISCARD;

static mut counter: IV = 0;

struct Cnt;

impl Cnt {
    fn new() -> Cnt {
        unsafe { counter = 0 };
        Cnt
    }
    fn get() -> IV {
        unsafe { counter }
    }
}

impl Drop for Cnt {
    fn drop(&mut self) {
        unsafe { counter += 1 };
    }
}

xs! {
    package XSTest::Panic;

    sub test_panic(_ctx) {
        let _cnt = Cnt::new();
        panic!("Panic!\n");
    }

    sub test_croak(ctx) {
        let _cnt = Cnt::new();
        ctx.call_pv(cstr!("XSTest::dies"), G_DISCARD);
        xs_return!(ctx, 42 as IV);
    }

    sub unwind_counter(ctx) {
        xs_return!(ctx, Cnt::get());
    }
}
