use {crate::prelude::*, std::borrow::Borrow as _, std::hash::Hasher, std::ops::BitXorAssign};

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

/// Wraps a Hasher to implement StableHasher. It must be known that the Hasher behaves in
/// a consistent manner regardless of platform or process.
#[derive(Default)]
pub struct StableHasherWrapper<T>(T);

impl<T: Hasher + Default> StableHasherWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl<T: Hasher + Default> StableHasher for StableHasherWrapper<T> {
    type Out = u64;
    fn write(&mut self, sequence_number: impl SequenceNumber, bytes: &[u8]) {
        // TODO: To nudge this closer to crypto strength, consider writing the
        // length (of bytes), followed by sequence depth (usize) and sequence
        // child (usize) all as prefix varint. This should make everything
        // injective.
        let seq_no = sequence_number.rollup();
        self.0.write(seq_no.borrow());
        self.0.write(bytes);
    }
    fn finish(&self) -> Self::Out {
        self.0.finish()
    }
}

pub(crate) fn trim_zeros(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len() - 1;
    while end != 0 && bytes[end] == 0 {
        end -= 1;
    }
    &bytes[0..=end]
}
