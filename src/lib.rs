pub mod crypto;
pub mod fast;
mod impls;
pub mod prelude;
pub mod utils;
use prelude::*;

/// Like Hasher, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
pub trait StableHasher {
    type Out;
    type Addr: FieldAddress;

    fn new() -> Self;
    fn write(&mut self, field_address: Self::Addr, bytes: &[u8]);
    fn mixin(&mut self, other: &Self); // TODO: Also include removal
    fn finish(&self) -> Self::Out;

    type Bytes: AsRef<[u8]>;
    fn to_bytes(&self) -> Self::Bytes;
    fn from_bytes(bytes: Self::Bytes) -> Self;
}

/// Like Hash, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
pub trait StableHash {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H);
}

pub trait FieldAddress: Clone {
    fn root() -> Self;
    fn child(&self, number: u64) -> Self;
}

pub fn fast_stable_hash<T: StableHash>(value: &T) -> u128 {
    profile_fn!(fast_stable_hash);
    generic_stable_hash::<T, crate::fast::FastStableHasher>(value)
}

pub fn crypto_stable_hash<T: StableHash>(value: &T) -> [u8; 32] {
    profile_fn!(crypto_stable_hash);
    generic_stable_hash::<T, crate::crypto::CryptoStableHasher>(value)
}
