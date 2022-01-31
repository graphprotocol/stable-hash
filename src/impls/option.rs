use crate::prelude::*;

impl<T: StableHash> StableHash for Option<T> {
    fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        if let Some(value) = self {
            value.stable_hash(sequence_number.child(0), state);
            state.write(sequence_number, &[]);
        }
    }
}
