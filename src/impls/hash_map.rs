use crate::prelude::*;
use std::collections::HashMap;

// TODO: Search/Replace "sequence_number"
impl<K: StableHash, V: StableHash, S> StableHash for HashMap<K, V, S> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        super::unordered_unique_stable_hash(self.iter(), sequence_number, state)
    }
}
