use crate::prelude::*;

impl StableHash for bool {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        if *self {
            state.write(sequence_number, &[]);
        }
    }
}