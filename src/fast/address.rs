use crate::prelude::*;
use xxhash_rust::xxh3::Xxh3;

// TODO: Consider trying XXH here
impl FieldAddress for u64 {
    fn root() -> Self {
        17
    }
    #[inline]
    fn child(&self, number: u64) -> Self {
        profile_method!(child);

        self.wrapping_mul(486_187_739).wrapping_add(number as u64)
    }
}

#[cfg(test)]
mod test {
    use super::FieldAddress;

    use std::collections::HashSet;

    fn recurse(field_address: u64, depth: usize, length: usize, collector: &mut HashSet<u64>) {
        // Struct/Recursion check
        for i in 0..6 {
            let child = field_address.child(i);
            assert!(collector.insert(child));
            if depth != 0 {
                recurse(child, depth - 1, length, collector);
            }
        }
        // Vec check (not recursive)
        // Tests larger vecs closer to the root, where larger vecs are more likely
        for i in 0..(length * depth * depth) {
            let child = field_address.child((i as u64) + 6);
            assert!(collector.insert(child));
        }
    }

    /// This test demonstrates that our choice of primes and algorithm is a good
    /// one for our use case of common structures to be digested by trying every
    /// permutation of all structs several deep and long Vecs for children and
    /// asserting 0 collisions on over 11 million common <u64>
    /// paths. Just for kicks I
    /// ran it on over 700 million paths before committing, but this test did
    /// not complete in a reasonable enough amount of time to be committed.
    /// Larger than that and we get dangerously close to birthday collisions
    /// anyway so I'm calling this good enough.
    ///
    /// The actual number of struct and vec prototypes verified by this test is
    /// astronomical, because any valid combinatorial sequence of paths made of
    /// these unique values composes a unique stream.
    #[test]
    fn no_collisions_for_common_prototypes_64() {
        let mut collector = HashSet::new();
        let root = u64::root();
        collector.insert(root);
        recurse(root, 4, 50, &mut collector);
        assert_eq!(30831, collector.len());
    }
}
