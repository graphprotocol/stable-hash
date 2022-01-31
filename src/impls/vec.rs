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

        for item in self.iter() {
            item.stable_hash(sequence_number.next_child(), state);
        }
    }
}
