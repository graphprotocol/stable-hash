mod bool;
mod floats;
mod hash_map;
mod hash_set;
mod ints;
mod option;
mod string;
mod tuple;
mod vec;

use crate::prelude::*;

pub(self) fn unordered_unique_stable_hash<H: StableHasher>(
    items: impl Iterator<Item = impl StableHash>,
    field_address: H::Addr,
    state: &mut H,
) {
    profile_fn!(unordered_unique_stable_hash);

    for member in items {
        // Must create an independent hasher to "break" relationship between
        // independent field addresses.
        let mut new_hasher = H::new();
        member.stable_hash(H::Addr::root(), &mut new_hasher);
        state.write(field_address.clone(), new_hasher.to_bytes().as_ref());
    }
}

impl<'a, T: StableHash> StableHash for &'a T {
    #[inline]
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        (*self).stable_hash(field_address, state)
    }
}
