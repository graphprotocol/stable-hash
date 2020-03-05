use {crate::prelude::*, std::ops::BitXorAssign};

/// Like Hasher, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
///
/// This is not a cryptographic strength digest.
pub trait StableHasher: Default {
    type Out: BitXorAssign + StableHash + Default;
    fn write(&mut self, sequence_number: impl SequenceNumber, bytes: &[u8]);
    fn finish(&self) -> Self::Out;
}

/// Like Hash, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
///
/// This is not a cryptographic strength digest.
pub trait StableHash {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher);
}
