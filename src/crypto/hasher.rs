use super::address::CryptoAddress;
use crate::prelude::*;
use blake3::Hasher;
use ibig::UBig;
use lazy_static::lazy_static;
use num_traits::{identities::One, Zero};
use std::default::Default;

// TODO: Consider using a Solinas prime
/*
From Jackson:
    In particular we could change the prime to be a Solinas prime.
    If we implement the algorithm for fast modular multiplication around a solinas prime then we get a big speed up.
    So the changes would be just change the public parameter prime in the codebase, and don’t just naively multiply and reduce, but write the algorithm to take advantage of the structure inherent in Solinas primes
    (they are prime numbers that have really low hamming weights, a sort of generalization of Mersenne primes — and so computers love these numbers)
*/
lazy_static! {
    static ref P: UBig = "50763434429823703141085322590076158163032399096130816327134180611270739679038131809123861970975131471260684737408234060876742190838745219274061025048845231234136148410311444604554192918702297959809128216170781389312847013812749872750274650041183009144583521632294518996531883338553737214586176414455965584933129379474747808392433032576309945590584603359054260866543918929486383805924215982747035136255123252119828736134723149397165643360162699752374292974151421555939481822911026769138419707577501643119472226283015793622652706604535623136902831581637275314074553942039263472515423713366344495524733341031029964603383".parse().unwrap();
}

/// Based on https://crypto.stackexchange.com/a/54546
///
/// The idea here is to use the FieldAddress to unambiguously identify each
/// field as within it's own database cell, and use an online order-independent
/// aggregator of the cells to produce a final result.
///
/// Within this framework a huge struct can be hashed incrementally or even in
/// parallel as long as field addresses are deterministically produced to
/// uniquely identify parts within the struct. Conveniently, the FieldAddress::skip
/// method can be used to jump to parts of a vec or struct efficiently.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CryptoStableHasher {
    // TODO: (Performance). We want an int 2056 + 2048 = 4104 bit int (u4160 if using a word size of 64 at 65 words)
    // That's enough to handle any sequence of mixin operations without overflow.
    // https://github.com/paritytech/parity-common/issues/388
    // Not a bad idea to start here so that when we convert we know that the transformation is ok.
    value: UBig,
}

impl Default for CryptoStableHasher {
    fn default() -> Self {
        Self { value: UBig::one() }
    }
}

#[inline]
fn mul_mod_p(into: &mut UBig, value: &UBig) {
    profile_method!(mul_mod_p);
    *into *= value;
    *into %= &*P;
}

impl StableHasher for CryptoStableHasher {
    type Out = [u8; 32];
    type Addr = CryptoAddress;
    type Bytes = Vec<u8>;

    #[inline]
    fn new() -> Self {
        profile_method!(new);

        Default::default()
    }

    fn write(&mut self, field_address: Self::Addr, bytes: &[u8]) {
        profile_method!(write);

        // Write the field into a database cell
        let mut output = field_address.finish(bytes);
        // Extend to the length necessary. This is a 2048 bit value, 1 bit
        // less than the prime the hash wraps around.
        let mut digits = [0u8; 256];
        output.fill(&mut digits);
        let digits = UBig::from_le_bytes(&digits);
        mul_mod_p(&mut self.value, &digits);
    }

    #[inline]
    fn mixin(&mut self, other: &Self) {
        mul_mod_p(&mut self.value, &other.value);
    }

    fn unmix(&mut self, other: &Self) {
        // The capacity 2049 is because that's the maximum number of divisions there will be.
        // With high probability, it will all be used.
        let mut todo = Vec::with_capacity(2049);

        // Find the multiplicative inverse under the field.
        let mut y = &*P - UBig::from(2u32);
        while !y.is_zero() {
            todo.push(y.clone());
            y = y / 2;
        }
        let mut p = UBig::one();
        while let Some(next) = todo.pop() {
            let clone = p.clone();
            mul_mod_p(&mut p, &clone);
            if next % 2 != 0 {
                mul_mod_p(&mut p, &other.value);
            }
        }

        // If it's the multiplicative inverse, and we multiply it, then we've inversed it.
        mul_mod_p(&mut self.value, &p);
    }

    fn finish(&self) -> Self::Out {
        profile_method!(finish);

        // Re-mix the state with a Hasher.
        let mut hasher = Hasher::new();
        let le = self.value.to_le_bytes();
        hasher.update(&le);
        hasher.finalize().into()
    }

    fn to_bytes(&self) -> Self::Bytes {
        profile_method!(to_bytes);
        self.value.to_le_bytes()
    }
    /// Panics if the bytes are not in a valid format.
    /// The only valid values are values returned from to_bytes()
    fn from_bytes(bytes: Vec<u8>) -> Self {
        profile_method!(from_bytes);

        let value = UBig::from_le_bytes(&bytes);
        assert!(&value <= &*P);
        Self { value }
    }
}

#[cfg(test)]
impl CryptoStableHasher {
    pub(crate) fn rand() -> Self {
        use rand::Rng;
        loop {
            let mut bytes = Vec::new();
            for _ in 0..257 {
                let rand_byte: u8 = rand::thread_rng().gen();
                if bytes.len() == 0 && rand_byte == 0 {
                    continue;
                }
                bytes.push(rand_byte);
            }
            let big = UBig::from_be_bytes(&bytes);
            if &big >= &*P {
                continue;
            }
            return CryptoStableHasher { value: big };
        }
    }
}
