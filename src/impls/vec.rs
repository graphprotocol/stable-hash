use crate::prelude::*;

impl<T: StableHash> StableHash for Vec<T> {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        (&self[..]).stable_hash(sequence_number, state)
    }
}

impl<'a, T: StableHash> StableHash for &'a [T] {
    fn stable_hash(&self, mut sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        for item in self.iter() {
            item.stable_hash(sequence_number.next_child(), state);
        }
        self.len().stable_hash(sequence_number, state);
    }
}
