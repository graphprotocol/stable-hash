use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};

pub trait UInt: TryFrom<usize> + Copy {
    type Bytes: Borrow<[u8]>;
    fn prime_init() -> Self;
    fn prime_mult() -> Self;
    fn to_le_bytes(self) -> Self::Bytes;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_mul(self, other: Self) -> Self;
}

pub trait SequenceNumber: Clone {
    fn root() -> Self;
    fn next_child(&mut self) -> Self;
    fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.next_child();
        }
    }
}

#[derive(Debug, Clone)]
pub struct SequenceNumberInt<T> {
    rollup: T,
    child: usize,
}

impl<T: UInt> Default for SequenceNumberInt<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::root()
    }
}

impl<T: UInt> SequenceNumberInt<T> {
    pub fn rollup(&self) -> T {
        self.rollup
    }
}

impl<T: UInt> SequenceNumber for SequenceNumberInt<T> {
    fn root() -> Self {
        Self {
            rollup: T::prime_init(),
            child: 0,
        }
    }
    #[inline]
    fn next_child(&mut self) -> Self {
        let child = self.child;
        self.child += 1;

        let rollup = self
            .rollup
            .wrapping_mul(T::prime_mult())
            .wrapping_add(child.try_into().unwrap_or_else(|_| panic!("Overflow")));

        Self { rollup, child: 0 }
    }
    #[inline]
    fn skip(&mut self, count: usize) {
        self.child += count;
    }
}

macro_rules! impl_sequence_no {
    ($T:ty, $size:expr) => {
        impl UInt for $T {
            type Bytes = [u8; $size];
            #[inline(always)]
            fn prime_init() -> Self {
                17
            }
            #[inline(always)]
            fn prime_mult() -> Self {
                486_187_739
            }
            #[inline(always)]
            fn wrapping_add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
            #[inline(always)]
            fn wrapping_mul(self, other: Self) -> Self {
                self.wrapping_mul(other)
            }
            #[inline(always)]
            fn to_le_bytes(self) -> Self::Bytes {
                self.to_le_bytes()
            }
        }
    };
}

impl_sequence_no!(u64, 8);
impl_sequence_no!(u32, 4);

#[cfg(test)]
mod test {
    use super::{SequenceNumber, SequenceNumberInt, UInt};

    use std::collections::HashSet;
    use std::hash::Hash;

    fn recurse<T: Hash + Eq + UInt>(
        mut sequence_number: SequenceNumberInt<T>,
        depth: usize,
        length: usize,
        collector: &mut HashSet<T>,
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
        let root = SequenceNumberInt::<u64>::root();
        collector.insert(root.rollup());
        recurse(root, 4, 50, &mut collector);
        assert_eq!(30831, collector.len());
    }

    #[test]
    fn no_collisions_for_common_prototypes_32() {
        let mut collector = HashSet::new();
        let root = SequenceNumberInt::<u32>::root();
        collector.insert(root.rollup());
        recurse(root, 4, 50, &mut collector);
        assert_eq!(30831, collector.len());
    }
}
