use crate::prelude::*;

impl StableHash for bool {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        if *self {
            state.write(sequence_number, &[]);
        }
    }
}
