use crate::prelude::*;

impl StableHash for bool {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        if *self {
            state.write(sequence_number, &[]);
        }
    }
}
