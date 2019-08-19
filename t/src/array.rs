use perl_xs::{AV, IV, SV};

package!("XSTest::Array");

#[perlxs]
fn test_clear(rv: SV) {
    rv.deref_av().map(|av| av.clear());
}

#[perlxs]
fn test_store(rv: SV, sv: SV) {
    if let Some(av) = rv.deref_av() {
        av.store(0, sv);
    }
}

#[perlxs]
fn test_fetch(av: AV) -> i64 {
    match av.fetch::<SV>(0) {
        Some(ref sv) if sv.ok() => 1 as IV,
        Some(_) => 2,
        None => 3,
    }
}

#[perlxs]
fn test_delete(av: AV) -> Option<SV> {
    av.delete::<SV>(0)
}

#[perlxs]
fn test_discard(av: AV) {
    av.discard(1);
}

#[perlxs]
fn test_exists(av: AV) -> bool {
    av.exists(0)
}

#[perlxs]
fn test_extend(av: AV) {
    av.extend(5);
}

#[perlxs]
fn test_fill(av: AV) {
    av.fill(4);
}

#[perlxs]
fn test_top_index(av: AV) -> i64 {
    av.top_index()
}

#[perlxs]
fn test_pop(av: AV) -> Option<SV> {
    av.pop::<SV>()
}

#[perlxs]
fn test_push(av: AV, sv: SV) {
    av.push(sv);
}

#[perlxs]
fn test_shift(av: AV) -> Option<SV> {
    av.shift::<SV>()
}

#[perlxs]
fn test_unshift(av: AV) {
    av.unshift(2);
}

#[perlxs]
fn test_undef(av: AV) {
    av.undef();
}

#[perlxs]
fn test_iter(av: AV) -> i64 {
    let n: IV = av.iter().filter_map(|sv| sv).map(|sv: SV| sv.iv()).sum();
    n
}
