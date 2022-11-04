use std::convert::TryInto;

use super::fld::FldMix;
use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct FastStableHasher {
    mixer: FldMix,
    count: u64,
}

#[cfg(test)]
impl FastStableHasher {
    pub(crate) fn rand() -> Self {
        use rand::thread_rng as rng;
        use rand::Rng as _;
        Self {
            mixer: FldMix::rand(),
            count: rng().gen(),
        }
    }
}

impl StableHasher for FastStableHasher {
    type Out = u128;
    type Addr = u128;
    type Bytes = [u8; 32];

    fn new() -> Self {
        Self {
            mixer: FldMix::new(),
            count: 0,
        }
    }

    fn mixin(&mut self, other: &Self) {
        self.mixer.mixin(&other.mixer);
        self.count = self.count.wrapping_add(other.count);
    }

    fn unmix(&mut self, other: &Self) {
        self.mixer.unmix(&other.mixer);
        self.count = self.count.wrapping_sub(other.count);
    }

    fn to_bytes(&self) -> Self::Bytes {
        let mixer = self.mixer.to_bytes();
        let count = self.count.to_le_bytes();

        let mut bytes = [0; 32];
        bytes[0..24].copy_from_slice(&mixer);
        bytes[24..32].copy_from_slice(&count);

        bytes
    }

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            mixer: FldMix::from_bytes(bytes[0..24].try_into().unwrap()),
            count: u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
        }
    }

    fn write(&mut self, field_address: Self::Addr, bytes: &[u8]) {
        profile_method!(write);

        // xxh3 128 has no weaknesses listed on SMHasher.
        // It also is built for checksumming, meaning all bytes are accounted for.
        // And it is the fastest, making it a clear choice.
        // Also considered: t1ha3, MetroHash, SipHasher24
        // For more information about XXH3, see this:
        // https://fastcompression.blogspot.com/2019/03/presenting-xxh3.html
        let hash = xxhash_rust::xxh3::xxh3_128_with_seed(bytes, field_address as u64);
        self.mixer.mix(hash, (field_address >> 64) as u64);
        self.count += 1;
    }

    fn finish(&self) -> u128 {
        profile_method!(finish);
        xxhash_rust::xxh3::xxh3_128_with_seed(&self.mixer.to_bytes(), self.count)
    }
}
