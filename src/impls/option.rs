use crate::prelude::*;

impl<T: StableHash> StableHash for Option<T> {
    fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Seq, state: &mut H) {
        self.is_some()
            .stable_hash(sequence_number.next_child(), state);
        if let Some(value) = self {
            value.stable_hash(sequence_number, state);
        }
    }
}
