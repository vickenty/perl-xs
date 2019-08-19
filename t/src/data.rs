use perl_xs::{Context, DataRef, IV, SV};
use std::cell::RefCell;

package!("XSTest::Data");

#[perlxs]
fn new(class: String, initial: IV, ctx: &mut Context) -> SV {
    ctx.new_sv_with_data(RefCell::new(initial)).bless(&class)
}

#[perlxs]
fn get(this: DataRef<RefCell<IV>>) -> i64 {
    *this.borrow()
}

#[perlxs]
fn inc(this: DataRef<RefCell<IV>>) {
    *this.borrow_mut() += 1;
}
