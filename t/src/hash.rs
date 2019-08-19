use perl_xs::{HV, IV, SV};

package!("XSTest::Hash");

#[perlxs]
fn test_fetch(hv: HV, key: SV) -> Option<SV> {
    hv.fetch::<SV>(&key.to_string().unwrap())
}

#[perlxs]
fn test_store(hv: HV, key: SV, val: SV) {
    hv.store(&key.to_string().unwrap(), val);
}

#[perlxs]
fn test_exists(hv: HV, sv: SV) -> bool {
    hv.exists(&sv.to_string().unwrap())
}

#[perlxs]
fn test_clear(hv: HV) {
    hv.clear();
}

#[perlxs]
fn test_delete(hv: HV, sv: SV) -> Option<SV> {
    hv.delete::<SV>(&sv.to_string().unwrap())
}

#[perlxs]
fn test_iter(hv: HV) -> IV {
    let n: IV = hv.iter().map(|(_, v): (&[u8], IV)| v).sum();
    n
}

#[perlxs]
fn test_values(hv: HV) -> IV {
    let n: IV = hv.values::<IV>().sum();
    n
}

#[perlxs]
fn test_keys(hv: HV) -> IV {
    let n: IV = hv.keys().map(|k| hv.fetch::<IV>(std::str::from_utf8(k).unwrap()).unwrap()).sum();
    n
}

#[perlxs]
fn test_for(hv: HV) -> IV {
    let mut n: IV = 0;
    for (_, v) in &hv {
        n += v.iv();
    }
    n
}
