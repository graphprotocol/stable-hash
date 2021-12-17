use crate::prelude::*;

pub trait UnorderedAggregator<T> {
    fn write(&mut self, value: impl StableHash, sequence_number: T);
}

/// Like Hasher, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
pub trait StableHasher: Default {
    type Out: StableHash;
    type Seq: SequenceNumber;
    fn write(&mut self, sequence_number: Self::Seq, bytes: &[u8]);
    fn finish(&self) -> Self::Out;
}

/// Like Hash, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
pub trait StableHash {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H);
}
