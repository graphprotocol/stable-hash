use crate::prelude::*;
use std::collections::HashMap;

impl<K: StableHash, V: StableHash, S> StableHash for HashMap<K, V, S> {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        super::unordered_stable_hash(self.iter(), sequence_number, state)
    }
}
