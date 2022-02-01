use crate::crypto::set_hasher::SetHasher;
use crate::mixers::fld::{FldMixA, FldMixB};
use crate::prelude::*;

/// Treat some &[u8] as a sequence of bytes, rather than a sequence of numbers.
/// Using this can result in a significant performance gain but does not support
/// the backward compatible change to different int types as numbers do by default
pub struct AsBytes<'a>(pub &'a [u8]);

impl StableHash for AsBytes<'_> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        if !self.0.is_empty() {
            state.write(field_address, self.0)
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
    fn stable_hash<H: StableHasher>(&self, mut field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        let canon = trim_zeros(self.little_endian);
        if !canon.is_empty() {
            self.is_negative.stable_hash(field_address.child(0), state);
            state.write(field_address, canon);
        }
    }
}

pub fn stable_hash<T: StableHash>(value: &T) -> u128 {
    profile_fn!(stable_hash);

    let mut hasher = StableHasherWrapper::new();
    value.stable_hash(FieldAddress::root(), &mut hasher);
    hasher.finish()
}

pub fn crypto_stable_hash<T: StableHash>(value: &T) -> [u8; 32] {
    profile_fn!(stable_hash);

    let mut hasher = SetHasher::new();
    value.stable_hash(FieldAddress::root(), &mut hasher);
    hasher.finish()
}

// TODO: Rename this
pub struct StableHasherWrapper {
    mixer1: FldMixA,
    mixer2: FldMixB,
    count: u64,
}

impl StableHasher for StableHasherWrapper {
    type Out = u128;
    type Addr = u64;

    fn new() -> Self {
        Self {
            mixer1: FldMixA::new(),
            mixer2: FldMixB::new(),
            count: 0,
        }
    }

    fn write(&mut self, field_address: Self::Addr, bytes: &[u8]) {
        profile_method!(write);

        // These are how much faster the current implementations are as compared to
        // the cryptographic one. Compiled with target-cpu=native on a Macbook Pro
        // xxh3 128: 150
        // t1ha3: 132
        // MetroHash: 120
        // SipHasher24: 86
        // Since this benchmark, we added a second fld.

        // Similarly, a MulFld was tested, which used a multiply within the largest
        // prime field that fit within a u128. Specialized code was used to do the mult
        // performantly, but the result was still much slower than desired.

        // xxh3 128 has no weaknesses listed on SMHasher (all the others do)
        // It also is built for checksumming, meaning all bytes are accounted for.
        // And it is the fastest, making it a clear choice.
        // In the future:
        //  * Would be good to use the u256 variant of xxh3 (which only differs in the
        //    finalization step) and write 127 bits of the value into each mixer.
        //    See also bdf7259b-12ee-4b95-b5d1-aefb60a935cf
        //    Instead, we are deriving a second hash here as recommended in this issue:
        //    https://github.com/Cyan4973/xxHash/issues/680
        //  * Verify that we are turning on the vectorizer. It is not clear if this is
        //    done automatically by the Rust compiler (and the SIMD story for Rust has
        //    been weak to date). Could be better performance.
        //  * Re-check the SIMD branch, but with target-cpu=native on (which may have been missed
        //    when testing the simple-simd branch
        // For more information about XXH3, see this:
        // https://fastcompression.blogspot.com/2019/03/presenting-xxh3.html
        // This hash is a beast.
        let hash = xxhash_rust::xxh3::xxh3_128_with_seed(bytes, field_address);
        self.mixer1.mix(hash);
        // Mixin the length and field address.
        // The rotate_left by 1 ensures that it's a different
        // bit that is cut off during mix (which uses 127 bits)
        // so that we use the whole 128 bits. Also we put the byte len
        // at the top since the top bit of that is definitely 0 and it
        // will get masked out so it should be unused.
        let hash2 = hash.rotate_left(1) ^ ((bytes.len() as u128) << 64 | (field_address as u128));
        self.mixer2.mix(hash2);
        self.count += 1;

        // For posterity, here are some of the unused variants

        // SipHasher
        /*
        use siphasher::sip128::{Hasher128 as _, SipHasher};
        use std::hash::Hasher;
        let mut hasher = SipHasher::new_with_keys(7, field_address.rollup());
        hasher.write(bytes);
        let hash = hasher.finish128();
        */

        // T1ha3
        /*
        let hash = t1ha::t1ha2_atonce128(bytes, field_address.rollup());
        */

        // MetroHash
        /*
        let mut hasher = metrohash::MetroHash128::with_seed(field_address.rollup());
        use std::hash::Hasher;
        hasher.write(bytes);
        let (h1, h2) = hasher.finish128();
        */
    }

    fn finish(&self) -> u128 {
        profile_method!(finish);

        let bytes: [u8; 32] = unsafe {
            std::mem::transmute((
                self.mixer1.raw().to_le_bytes(),
                self.mixer2.raw().to_le_bytes(),
            ))
        };
        xxhash_rust::xxh3::xxh3_128_with_seed(&bytes, self.count)
    }
}

// TODO: Write a sanity checker that exhaustively verifies that there
// are no collisions for field addresses
