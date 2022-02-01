use crate::prelude::*;
use std::collections::HashMap;

impl<K: StableHash, V: StableHash, S> StableHash for HashMap<K, V, S> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        super::unordered_unique_stable_hash(self.iter(), field_address, state)
    }
}
