use crate::prelude::*;

// TODO: Remove this
pub trait UnorderedAggregator<T> {
    fn write(&mut self, value: impl StableHash, field_address: T);
}

/// Like Hasher, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
pub trait StableHasher {
    type Out: StableHash;
    type Addr: FieldAddress;
    fn write(&mut self, field_address: Self::Addr, bytes: &[u8]);
    fn finish(&self) -> Self::Out;
    fn new() -> Self;
}

/// Like Hash, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
pub trait StableHash {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H);
}
