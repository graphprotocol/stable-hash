use crate::prelude::*;
use std::collections::HashSet;

impl<T: StableHash, S> StableHash for HashSet<T, S> {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        super::unordered_stable_hash(self.iter(), sequence_number, state)
    }
}
