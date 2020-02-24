use std::num::Wrapping;
use std::borrow::Borrow;

pub trait SequenceNumber {
    type Rollup : Borrow<[u8]>;
    fn rollup(&self) -> Self::Rollup;
    fn root() -> Self where Self : Sized;
    fn next_child(&mut self) -> Self where Self : Sized;
}

pub struct SequenceNumberInt<T> {
    rollup: Wrapping<T>,
    child: usize,
}

macro_rules! impl_sequence_no {
    ($T:ty, $size:expr, $prime_init:expr, $prime_mult:expr) => {
        impl SequenceNumber for SequenceNumberInt<$T> {
            type Rollup=[u8; $size];
            fn root() -> Self {
                Self {
                    rollup: Wrapping($prime_init),
                    child: 0,
                }
            }

            #[inline]
            fn next_child(&mut self) -> Self {
                let child = self.child;
                self.child += 1;

                let rollup = (self.rollup * Wrapping($prime_mult)) + Wrapping(child as $T);

                Self {
                    rollup,
                    child,
                }
            }

            #[inline]
            fn rollup(&self) -> Self::Rollup {
                self.rollup.0.to_le_bytes()
            }
        }
    }
}

// These values are locked in!
// Don't change them. Ever.
impl_sequence_no!(u64, 8, 17, 486_187_739);
impl_sequence_no!(u32, 4, 17, 486_187_739);

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::borrow::Borrow;
    use super::*;

    /// This test demonstrates that our choice of primes and algorithm is a good
    /// one for our use case of common structures to be digested by trying every
    /// permutation of all structs several deep and long Vecs for children and
    /// asserting 0 collisions on over 11 million common SequenceNumber paths.
    /// Just for kicks I ran it on over 700 million paths before committing, but
    /// this test did not complete in a reasonable enough amount of time to be
    /// committed. Larger than that and we get dangerously close to birthday
    /// collisions anyway so I'm calling this good enough.
    ///
    /// The actual number of struct and vec prototypes verified by this test is
    /// astronomical, because any valid combinatorial sequence of paths made of these
    /// unique values composes a unique stream.
    ///
    /// None of this of course speaks to actual collision probabilities for the
    /// resulting sequence taking into account values on the stream that are not
    /// SequenceNumber and a given hash function, except that the given
    /// implementation of SequenceNumber should not itself contribute to a collision
    #[test]
    fn no_collisions_for_common_prototypes() {
        let mut collector = HashSet::new();
        let root = SequenceNumberInt::<u64>::root();

        fn recurse<T: Hash + Eq + Borrow<[u8]>>(mut sequence_number: impl SequenceNumber<Rollup=T>, depth: usize, collector: &mut HashSet<T>) {
            // Struct/Recursion check
            for _ in 0..6 {
                let child = sequence_number.next_child();
                assert!(collector.insert(child.rollup()));
                if depth != 0 {
                    recurse(child, depth - 1, collector);
                }
            }
            // Vec check (not recursive)
            // Tests larger vecs closer to the root, where larger vecs are more likely
            for _ in 0..(100 * depth * depth) {
                let child = sequence_number.next_child();
                assert!(collector.insert(child.rollup()));
            }
        }

        collector.insert(root.rollup());
        recurse(root, 7, &mut collector);

        assert_eq!(11420039, collector.len());
    }
}