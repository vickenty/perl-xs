use perl_xs::IV;
use perl_xs::G_DISCARD;

static mut COUNTER: IV = 0;

struct Cnt;

impl Cnt {
    fn new() -> Cnt {
        unsafe { COUNTER = 0 };
        Cnt
    }
    fn get() -> IV {
        unsafe { COUNTER }
    }
}

impl Drop for Cnt {
    fn drop(&mut self) {
        unsafe { COUNTER += 1 };
    }
}

xs! {
    package XSTest::Panic;

    sub test_panic(_ctx) {
        let _cnt = Cnt::new();
        croak!("Panic!\n");
    }

    sub test_croak(ctx) {
        let _cnt = Cnt::new();
        ctx.call_pv(cstr!("XSTest::dies"), G_DISCARD);
        42 as IV
    }

    sub unwind_counter(_ctx) {
        Cnt::get()
    }
}
