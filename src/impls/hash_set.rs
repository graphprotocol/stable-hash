use crate::prelude::*;
use std::collections::HashSet;

impl<T: StableHash, S> StableHash for HashSet<T, S> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        super::unordered_unique_stable_hash(self.iter(), field_address, state)
    }
}
