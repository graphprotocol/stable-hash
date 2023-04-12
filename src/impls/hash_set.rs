use crate::prelude::*;
use std::collections::HashSet;

impl<T: StableHash, S> StableHash for HashSet<T, S> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        crate::utils::AsUnorderedSet(self).stable_hash(field_address, state)
    }
}
