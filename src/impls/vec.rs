use crate::prelude::*;

impl<T: StableHash> StableHash for Vec<T> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        (&self[..]).stable_hash(field_address, state)
    }
}

impl<'a, T: StableHash> StableHash for &'a [T] {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        for (index, item) in self.iter().enumerate() {
            item.stable_hash(field_address.child(index as u64), state);
        }
        // This is needed to disambiguate when the last members are default
        // For example, vec![true, false] and vec![true, false, false] should
        // not collide.
        // See also 33a9b3bf-0d43-4fd0-a3ed-a77807505255
        self.len().stable_hash(field_address, state);
    }
}
