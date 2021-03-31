use super::blake3_sequence::Blake3SeqNo;
use crate::prelude::*;
use crate::stable_hash::UnorderedAggregator;
use blake3::Hasher;
use ibig::UBig;
use lazy_static::lazy_static;
use num_traits::identities::One;
use std::default::Default;

lazy_static! {
    static ref P: UBig = "50763434429823703141085322590076158163032399096130816327134180611270739679038131809123861970975131471260684737408234060876742190838745219274061025048845231234136148410311444604554192918702297959809128216170781389312847013812749872750274650041183009144583521632294518996531883338553737214586176414455965584933129379474747808392433032576309945590584603359054260866543918929486383805924215982747035136255123252119828736134723149397165643360162699752374292974151421555939481822911026769138419707577501643119472226283015793622652706604535623136902831581637275314074553942039263472515423713366344495524733341031029964603383".parse().unwrap();
}

/// Based on https://crypto.stackexchange.com/a/54546
///
/// The idea here is to use the SequenceNumber to unambiguously identify each
/// field as within it's own database cell, and use an online order-independent
/// aggregator of the cells to produce a final result.
///
/// Within this framework a huge struct can be hashed incrementally or even in
/// parallel as long as sequence numbers are deterministically produced to
/// identify parts within the struct. Conveniently, the SequenceNumber::skip
/// method can be used to jump to parts of a vec or struct efficiently.
pub struct SetHasher {
    // TODO: (Performance). We want an int 2056 + 2048 = 4104 bit int (u4160 if using a word size of 64 at 65 words)
    // That's enough to handle any sequence of mixin operations without overflow.
    // https://github.com/paritytech/parity-common/issues/388
    // Not a bad idea to start here so that when we convert we know that the transformation is ok.
    value: UBig,
}

impl Default for SetHasher {
    fn default() -> Self {

        Self { value: UBig::one() }
    }
}

impl SetHasher {
    #[inline]
    pub fn new() -> Self {
        profile_method!(new);

        Default::default()
    }
    #[inline]
    fn mixin(&mut self, digits: &UBig) {
        profile_method!(mixin);

        self.value *= digits;
        self.value %= &*P;
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        profile_method!(to_bytes);

        self.value.to_le_bytes()
    }
    /// Panics if the bytes are not in a valid format.
    /// The only valid values are values returned from to_bytes()
    pub fn from_bytes(bytes: &[u8]) -> Self {
        profile_method!(from_bytes);

        assert!(bytes.len() <= 257);
        let value = UBig::from_le_bytes(bytes);
        Self { value }
    }
}

/// The SetHasher is already updated in an unordered fashion, so no special second struct
/// is needed. Starts at 1 and mixin when finished.
impl UnorderedAggregator<Blake3SeqNo> for SetHasher {
    #[inline]
    fn write(&mut self, value: impl StableHash, sequence_number: Blake3SeqNo) {
        profile_method!(write);

        // Add the hash of the value to the set.
        let hash = crate::utils::stable_hash::<Self, _>(&value);
        StableHasher::write(self, sequence_number, &hash);
    }
}

impl StableHasher for SetHasher {
    type Out = [u8; 32];
    type Seq = Blake3SeqNo;
    type Unordered = Self;

    fn write(&mut self, sequence_number: Self::Seq, bytes: &[u8]) {
        profile_method!(write);

        // Write the field into a database cell
        let mut output = sequence_number.finish(bytes);
        // Extend to the length necessary. This is a 2048 bit value, 1 bit
        // less than the prime the hash wraps around.
        let mut digits = [0u8; 256];
        output.fill(&mut digits);
        let digits = UBig::from_le_bytes(&digits);
        self.mixin(&digits)
    }

    #[inline]
    fn start_unordered(&mut self) -> Self::Unordered {
        profile_method!(start_unordered);

        Self::new()
    }

    #[inline]
    fn finish_unordered(&mut self, unordered: Self::Unordered, _sequence_number: Self::Seq) {
        profile_method!(finish_unordered);

        self.mixin(&unordered.value)
    }

    fn finish(&self) -> Self::Out {
        profile_method!(finish);

        // Re-mix the state with a Hasher.
        let mut hasher = Hasher::new();
        let le = self.value.to_le_bytes();
        hasher.update(&le);
        hasher.finalize().into()
    }
}
