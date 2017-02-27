use std::cell::RefCell;
use perl_xs::{ IV, DataRef };

xs! {
    package XSTest::Data;

    sub new(ctx, class: String, initial: IV) {
        ctx.new_sv_with_data(RefCell::new(initial)).bless(&class)
    }

    sub get(_ctx, this: DataRef<RefCell<IV>>) {
        return *this.borrow();
    }

    sub inc(_ctx, this: DataRef<RefCell<IV>>) {
        *this.borrow_mut() += 1;
    }
}
