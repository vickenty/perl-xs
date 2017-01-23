use perl_xs::{ IV, AV };

xs! {
    package XSTest::Param;

    sub add(ctx, a: IV, b: IV) {
        a + b
    }

    sub len(ctx, a: AV) {
        a.top_index() + 1
    }

    sub strlen(ctx, s: String) {
        s.chars().count() as IV
    }
}
