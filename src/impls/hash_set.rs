use crate::prelude::*;
use std::collections::HashSet;

impl<T: StableHash, S> StableHash for HashSet<T, S> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        super::unordered_unique_stable_hash(self.iter(), sequence_number, state)
    }
}
