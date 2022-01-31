use crate::prelude::*;

impl<T: StableHash> StableHash for Vec<T> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        (&self[..]).stable_hash(sequence_number, state)
    }
}

impl<'a, T: StableHash> StableHash for &'a [T] {
    fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        for (index, item) in self.iter().enumerate() {
            item.stable_hash(sequence_number.child(index as u64), state);
        }
        // This is needed to disambiguate when the last members are default
        // See also 33a9b3bf-0d43-4fd0-a3ed-a77807505255
        self.len().stable_hash(sequence_number, state);
    }
}
