use perl_xs::{Context, G_DISCARD, IV};

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

package!("XSTest::Panic");

#[perlxs]
fn test_panic() {
    let _cnt = Cnt::new();
    croak!("Panic!\n");

    #[allow(unreachable_code)]
    ()
}

#[perlxs]
fn test_croak(ctx: &mut Context) -> IV {
    let _cnt = Cnt::new();
    ctx.call_pv(cstr!("XSTest::dies"), G_DISCARD);
    42 as IV
}

#[perlxs]
fn unwind_counter() -> IV {
    Cnt::get()
}
