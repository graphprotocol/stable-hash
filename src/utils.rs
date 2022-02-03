use crate::prelude::*;
use crate::verification::*;

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
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        // Having the negative sign be a child makes it possible to change the schema
        // from u32 to i64 in a backward compatible way.
        // This is also allowing for negative 0, like float, which is not used by
        // any standard impl but may be used by some types.
        if self.is_negative {
            state.write(field_address.child(0), &[]);
        }
        let canon = trim_zeros(self.little_endian);
        if !canon.is_empty() {
            state.write(field_address, canon);
        }
    }
}

pub(crate) fn generic_stable_hash<T: StableHash, H: StableHasher>(value: &T) -> H::Out {
    let mut hasher = H::new();
    value.stable_hash(FieldAddress::root(), &mut hasher);
    hasher.finish()
}

// TODO: Create unit tests where this should fail
pub fn check_for_child_errors<T: StableHash>(value: &T) -> Result<(), (ChildErr, Vec<PathItem>)> {
    profile_fn!(check_for_child_errors);
    generic_stable_hash::<T, crate::verification::ChildChecker>(value)
}
