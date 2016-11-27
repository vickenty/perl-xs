use perl_xs::{ IV, AV };

xs! {
    package XSTest::Param;

    sub add(ctx, a: IV, b: IV) {
        xs_return!(ctx, a + b);
    }

    sub len(ctx, a: AV) {
        xs_return!(ctx, a.top_index() + 1);
    }
}
