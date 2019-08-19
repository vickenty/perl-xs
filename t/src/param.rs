use perl_xs::{AV, IV};

package!("XSTest::Param");

#[perlxs]
fn add(a: IV, b: IV) -> IV {
    a + b
}

#[perlxs]
fn add_opt(a: IV, b: Option<IV>) -> IV {
    match b {
        Some(b) => a + b,
        None => a,
    }
}

#[perlxs]
fn len(a: AV) -> IV {
    a.top_index() + 1
}

#[perlxs]
fn strlen(s: String) -> IV {
    s.chars().count() as IV
}
