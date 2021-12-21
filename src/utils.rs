#[cfg(not(feature = "simd"))]
use crate::fld_mixer::FldMixScalar;
#[cfg(feature = "simd")]
use crate::fld_mixer_simd::FldMixSimd;
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
        profile_method!(stable_hash);

        if !self.0.is_empty() {
            state.write(sequence_number, self.0)
        }
    }
}

fn trim_zeros(bytes: &[u8]) -> &[u8] {
    profile_fn!(trim_zeros);

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
        profile_method!(stable_hash);

        self.is_negative
            .stable_hash(sequence_number.next_child(), state);
        let canon = trim_zeros(self.little_endian);
        if !canon.is_empty() {
            state.write(sequence_number, canon);
        }
    }
}

pub fn stable_hash<H: StableHasher + Default, T: StableHash>(value: &T) -> H::Out {
    profile_fn!(stable_hash);

    let mut hasher = H::default();
    value.stable_hash(H::Seq::root(), &mut hasher);
    hasher.finish()
}

pub fn stable_hash_with_hasher<T: std::hash::Hasher + Default, V: StableHash>(value: &V) -> u64 {
    profile_fn!(stable_hash_with_hasher);

    #[cfg(feature = "simd")]
    type Mixer = FldMixSimd;
    #[cfg(not(feature = "simd"))]
    type Mixer = FldMixScalar;

    stable_hash::<StableHasherWrapper<T, Mixer, SequenceNumberInt<u64>>, _>(value)
}

/// Wraps a Hasher to implement StableHasher. It must be known that the Hasher behaves in
/// a consistent manner regardless of platform or process.
#[derive(Default)]
pub struct StableHasherWrapper<H, M: FldMix + Default, Seq = u64> {
    mixer: M,
    _marker: PhantomData<*const (Seq, H)>,
}

impl<H: Hasher + Default, M: FldMix + Default, I: UInt> StableHasher
    for StableHasherWrapper<H, M, SequenceNumberInt<I>>
{
    type Out = u64;
    type Seq = SequenceNumberInt<I>;

    fn write(&mut self, sequence_number: Self::Seq, bytes: &[u8]) {
        profile_method!(write);

        let mut hasher = H::default();
        let seq_no = sequence_number.rollup().to_le_bytes();
        hasher.write(seq_no.borrow());
        hasher.write(bytes);

        self.mixer.mix(hasher.finish());
    }

    fn finish(&self) -> Self::Out {
        profile_method!(finish);

        let mut hasher = H::default();
        hasher.write(&self.mixer.finalize().to_le_bytes());
        hasher.finish()
    }
}
