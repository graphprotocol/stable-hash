use crate::prelude::*;
use crate::sequence_number::UInt;
use crate::SequenceNumberInt;
use std::borrow::Borrow as _;
use std::hash::Hasher;
use std::marker::PhantomData;

/// Treat some &[u8] as a sequence of bytes, rather than a sequence of numbers.
/// Using this can result in a significant performance gain but does not support
/// the backward compatible change to different int types as numbers do by default
pub struct AsBytes<'a>(pub &'a [u8]);

impl StableHash for AsBytes<'_> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
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
    fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Seq, state: &mut H) {
        self.is_negative
            .stable_hash(sequence_number.next_child(), state);
        let canon = trim_zeros(self.little_endian);
        if !canon.is_empty() {
            state.write(sequence_number, canon);
        }
    }
}

pub fn stable_hash<H: StableHasher + Default, T: StableHash>(value: &T) -> H::Out {
    let mut hasher = H::default();
    value.stable_hash(H::Seq::root(), &mut hasher);
    hasher.finish()
}

pub fn stable_hash_with_hasher<T: std::hash::Hasher + Default, V: StableHash>(value: &V) -> u64 {
    stable_hash::<StableHasherWrapper<T, SequenceNumberInt<u64>>, _>(value)
}

/// Wraps a Hasher to implement StableHasher. It must be known that the Hasher behaves in
/// a consistent manner regardless of platform or process.
#[derive(Default)]
pub struct StableHasherWrapper<H, Seq> {
    hasher: H,
    _marker: PhantomData<*const Seq>,
}

pub struct XorAggregator<T> {
    value: u64,
    _marker: PhantomData<*const T>,
}

impl<H: Hasher + Default, I: UInt> crate::stable_hash::UnorderedAggregator<SequenceNumberInt<I>>
    for XorAggregator<StableHasherWrapper<H, SequenceNumberInt<I>>>
{
    fn write(&mut self, value: impl StableHash, sequence_number: SequenceNumberInt<I>) {
        let mut hasher: StableHasherWrapper<H, SequenceNumberInt<I>> = Default::default();
        value.stable_hash(sequence_number, &mut hasher);
        self.value ^= hasher.finish();
    }
}

impl<H: Hasher + Default, I: UInt> StableHasher for StableHasherWrapper<H, SequenceNumberInt<I>> {
    type Out = u64;
    type Seq = SequenceNumberInt<I>;
    type Unordered = XorAggregator<Self>;
    fn start_unordered(&mut self) -> Self::Unordered {
        XorAggregator {
            value: 0,
            _marker: PhantomData,
        }
    }
    fn finish_unordered(
        &mut self,
        unordered: Self::Unordered,
        sequence_number: SequenceNumberInt<I>,
    ) {
        unordered.value.stable_hash(sequence_number, self);
    }
    fn write(&mut self, sequence_number: Self::Seq, bytes: &[u8]) {
        let seq_no = sequence_number.rollup().to_le_bytes();
        self.hasher.write(seq_no.borrow());
        self.hasher.write(bytes);
    }
    fn finish(&self) -> Self::Out {
        self.hasher.finish()
    }
}
