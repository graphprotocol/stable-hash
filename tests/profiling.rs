mod common;
use std::collections::HashMap;

use common::*;
use firestorm::profile_fn;
use stable_hash::*;

#[test]
fn compare() {
    let mut data = HashMap::new();

    data.insert("abc", 100u64);
    data.insert("abcdef", 100u64);
    data.insert("abcdefged", 0u64);
    data.insert("abcdefgedaekllw", 50000u64);
    data.insert("acfaek", 50000u64);
    data.insert("aek", 511110000u64);

    fn profile(value: &impl StableHash) {
        profile_fn!(profile);
        xxhash(value);
        crypto_hash(value);
    }

    if firestorm::enabled() {
        firestorm::bench("./firestorm", || profile(&data)).unwrap();
    }
}
