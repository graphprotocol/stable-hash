use crate::prelude::*;
use crate::SequenceNumberInt;
use std::borrow::Borrow as _;
use std::hash::Hasher;

/// Treat some &[u8] as a sequence of bytes, rather than a sequence of numbers.
/// Using this can result in a significant performance gain but does not support
/// the backward compatible change to different int types as numbers do by default
pub struct AsBytes<'a>(pub &'a [u8]);

impl StableHash for AsBytes<'_> {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        if !self.0.is_empty() {
            state.write(sequence_number, self.0)
        }
    }
}

fn trim_zeros(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len();
    while end != 0 && bytes[end - 1] == 0 {
        end -= 1;
    }
    &bytes[0..end]
}

/// Canonical way to write an integer of any size.
///
/// Backward compatibility:
/// * The value +0 never writes bytes to the stream.
/// * Integers of any size (u8..u24..u128...uN) are written in a canonical form, and can be written in any order.
pub struct AsInt<'a> {
    pub is_negative: bool,
    pub little_endian: &'a [u8],
}

impl StableHash for AsInt<'_> {
    fn stable_hash(&self, mut sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        self.is_negative
            .stable_hash(sequence_number.next_child(), state);
        let canon = trim_zeros(self.little_endian);
        if !canon.is_empty() {
            state.write(sequence_number, canon);
        }
    }
}

pub fn stable_hash<T: StableHasher, S: SequenceNumber, V: StableHash>(value: &V) -> T::Out {
    let mut hasher = T::default();
    value.stable_hash(S::root(), &mut hasher);
    hasher.finish()
}

pub fn stable_hash_with_hasher<T: std::hash::Hasher + Default, V: StableHash>(value: &V) -> u64 {
    stable_hash::<StableHasherWrapper<T>, SequenceNumberInt<u64>, _>(value)
}

/// Wraps a Hasher to implement StableHasher. It must be known that the Hasher behaves in
/// a consistent manner regardless of platform or process.
#[derive(Default)]
pub struct StableHasherWrapper<T>(T);

impl<T: Hasher + Default> StableHasher for StableHasherWrapper<T> {
    type Out = u64;
    fn write(&mut self, sequence_number: impl SequenceNumber, bytes: &[u8]) {
        let seq_no = sequence_number.rollup();
        self.0.write(seq_no.borrow());
        self.0.write(bytes);
    }
    fn finish(&self) -> Self::Out {
        self.0.finish()
    }
}
