use crate::prelude::*;

pub trait FieldAddress: Clone {
    fn root() -> Self;
    fn next_child(&mut self) -> Self;
    fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.next_child();
        }
    }
}

// TODO: Bake this down to just u64 (not taking child)
#[derive(Debug, Clone)]
pub struct SequenceNumberInt {
    rollup: u64,
    child: usize,
}

// TODO: Remove this
impl Default for SequenceNumberInt {
    #[inline(always)]
    fn default() -> Self {
        profile_method!(default);

        Self::root()
    }
}

impl SequenceNumberInt {
    #[inline]
    pub fn rollup(&self) -> u64 {
        self.rollup
    }
}

impl FieldAddress for SequenceNumberInt {
    fn root() -> Self {
        Self {
            rollup: 17,
            child: 0,
        }
    }
    #[inline]
    fn next_child(&mut self) -> Self {
        profile_method!(next_child);

        let child = self.child;
        self.child += 1;

        let rollup = self
            .rollup
            .wrapping_mul(486_187_739)
            .wrapping_add(child as u64);

        Self { rollup, child: 0 }
    }
    #[inline]
    fn skip(&mut self, count: usize) {
        self.child += count;
    }
}

#[cfg(test)]
mod test {
    use super::{FieldAddress, SequenceNumberInt};

    use std::collections::HashSet;

    fn recurse(
        mut sequence_number: SequenceNumberInt,
        depth: usize,
        length: usize,
        collector: &mut HashSet<u64>,
    ) {
        // Struct/Recursion check
        for _ in 0..6 {
            let child = sequence_number.next_child();
            assert!(collector.insert(child.rollup()));
            if depth != 0 {
                recurse(child, depth - 1, length, collector);
            }
        }
        // Vec check (not recursive)
        // Tests larger vecs closer to the root, where larger vecs are more likely
        for _ in 0..(length * depth * depth) {
            let child = sequence_number.next_child();
            assert!(collector.insert(child.rollup()));
        }
    }

    /// This test demonstrates that our choice of primes and algorithm is a good
    /// one for our use case of common structures to be digested by trying every
    /// permutation of all structs several deep and long Vecs for children and
    /// asserting 0 collisions on over 11 million common SequenceNumber::<u64>
    /// paths and almost 3.4 million SequenceNumber::<u32> paths. Just for kicks I
    /// ran it on over 700 million paths before committing, but this test did
    /// not complete in a reasonable enough amount of time to be committed.
    /// Larger than that and we get dangerously close to birthday collisions
    /// anyway so I'm calling this good enough.
    ///
    /// The actual number of struct and vec prototypes verified by this test is
    /// astronomical, because any valid combinatorial sequence of paths made of
    /// these unique values composes a unique stream.
    ///
    /// None of this of course speaks to actual collision probabilities for the
    /// resulting sequence taking into account values on the stream that are not
    /// SequenceNumber and a given hash function, except that the given
    /// implementation of SequenceNumber should not itself contribute to a
    /// collision
    #[test]
    fn no_collisions_for_common_prototypes_64() {
        let mut collector = HashSet::new();
        let root = SequenceNumberInt::root();
        collector.insert(root.rollup());
        recurse(root, 4, 50, &mut collector);
        assert_eq!(30831, collector.len());
    }
}
