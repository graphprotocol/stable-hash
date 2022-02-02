use crate::prelude::*;

impl<T: StableHash> StableHash for Option<T> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        if let Some(value) = self {
            value.stable_hash(field_address.child(0), state);
            state.write(field_address, &[]);
        }
    }
}
