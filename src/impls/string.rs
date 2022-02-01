use crate::prelude::*;

impl StableHash for String {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        self.as_str().stable_hash(field_address, state);
    }
}

impl<'a> StableHash for &'a str {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        AsBytes(self.as_bytes()).stable_hash(field_address, state)
    }
}
