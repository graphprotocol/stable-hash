mod bool;
mod floats;
mod hash_map;
mod hash_set;
mod ints;
mod option;
mod string;
mod tuple;
mod vec;

use crate::prelude::*;

pub(self) fn unordered_stable_hash<S: SequenceNumber, H: StableHasher>(
    items: impl Iterator<Item = impl StableHash>,
    mut sequence_number: S,
    state: &mut H,
) {
    let mut rollup = H::Out::default();
    let mut count = 0usize;
    for member in items {
        let mut hasher = H::default();
        member.stable_hash(S::root(), &mut hasher);
        rollup ^= hasher.finish();
        count += 1;
    }
    rollup.stable_hash(sequence_number.next_child(), state);
    count.stable_hash(sequence_number, state);
}

impl<'a, T: StableHash> StableHash for &'a T {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        (*self).stable_hash(sequence_number, state)
    }
}
