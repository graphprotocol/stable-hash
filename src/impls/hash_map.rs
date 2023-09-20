use crate::prelude::*;
use std::collections::HashMap;

impl<K: StableHash, V: StableHash, S> StableHash for HashMap<K, V, S> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);

        crate::utils::AsUnorderedSet(self).stable_hash(sequence_number, state)
    }
}
