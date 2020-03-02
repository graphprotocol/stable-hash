mod impls;
pub mod prelude;
mod sequence_number;
mod stable_hash;

pub use crate::sequence_number::{SequenceNumber, SequenceNumberInt};
pub use crate::stable_hash::{StableHash, StableHasher, StableHasherWrapper};

/// Treat some &[u8] as a sequence of bytes, rather than a sequence of numbers.
/// Using this can result in a significant performance gain but does not support
/// the backward compatible change to different int types as numbers do by default
pub struct AsBytes<'a>(pub &'a [u8]);

impl StableHash for AsBytes<'_> {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        if !self.0.is_empty() {
            state.write(sequence_number, self.0)
        }
    }
}
