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

pub(crate) fn stable_hash_generic<T: StableHash, H: StableHasher>(value: &T) -> H::Out {
    let mut hasher = H::new();
    value.stable_hash(FieldAddress::root(), &mut hasher);
    hasher.finish()
}

// TODO: Write a sanity checker that exhaustively verifies that there
// are no collisions for field addresses
