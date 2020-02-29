use crate::prelude::*;

impl StableHash for String {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        if self.len() != 0 {
            state.write(sequence_number, self.as_bytes());
        }
    }
}