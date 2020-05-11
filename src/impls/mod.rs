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
use crate::stable_hash::UnorderedAggregator;

pub(self) fn unordered_unique_stable_hash<H: StableHasher>(
    items: impl Iterator<Item = impl StableHash>,
    mut sequence_number: H::Seq,
    state: &mut H,
) {
    // First, create child nodes for each element.
    // Doing this here removes any opportunity for collisions
    let rollup_seq_no = sequence_number.next_child();
    let member_seq_no = sequence_number.next_child();
    let count_seq_no = sequence_number.next_child();

    let mut unordered = state.start_unordered();
    let mut count = 0usize;
    for member in items {
        unordered.write(member, member_seq_no.clone());
        count += 1;
    }
    state.finish_unordered(unordered, rollup_seq_no);
    count.stable_hash(count_seq_no, state);
}

impl<'a, T: StableHash> StableHash for &'a T {
    #[inline]
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        (*self).stable_hash(sequence_number, state)
    }
}
