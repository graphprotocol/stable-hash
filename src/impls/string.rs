use crate::prelude::*;

impl StableHash for String {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        self.as_str().stable_hash(sequence_number, state);
    }
}

impl<'a> StableHash for &'a str {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        AsBytes(self.as_bytes()).stable_hash(sequence_number, state)
    }
}
