use perl_xs::IV;

xs! {
    package XSTest::Param;

    sub add(ctx, a: IV, b: IV) {
        xs_return!(ctx, a + b);
    }
}
