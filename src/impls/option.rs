use crate::prelude::*;

impl<T: StableHash> StableHash for Option<T> {
    fn stable_hash(&self, mut sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        self.is_some().stable_hash(sequence_number.next_child(), state);
        if let Some(value) = self {
            value.stable_hash(sequence_number, state);
        }
    }
}