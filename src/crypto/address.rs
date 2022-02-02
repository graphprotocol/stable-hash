use crate::prelude::*;
use blake3::{Hasher, OutputReader};
use leb128::write::unsigned as write_varint;

#[derive(Clone)]
pub struct CryptoAddress {
    hasher: Hasher,
}

impl FieldAddress for CryptoAddress {
    fn root() -> Self {
        profile_method!(root);

        Self {
            hasher: Hasher::new(),
        }
    }
    fn child(&self, number: u64) -> Self {
        profile_method!(child);

        let mut hasher = self.hasher.clone();
        // This has to be non-zero in order to be injective, since the payload marker writes 0
        // See also 91e48829-7bea-4426-971a-f092856269a5
        write_varint(&mut hasher, number + 1).unwrap();
        Self { hasher }
    }
}

impl CryptoAddress {
    pub(crate) fn finish(self, payload: &[u8]) -> OutputReader {
        profile_method!(finish);

        let Self { mut hasher, .. } = self;

        // To debug all the payloads in a hash to find a diff, this can be useful.
        /*
        dbg!(
            "Update:\n\tpayload: {}\n\tfield_address: {}",
            hex::encode(hasher.finalize().as_bytes()),
            hex::encode(payload)
        );
        */

        // See also 91e48829-7bea-4426-971a-f092856269a5
        hasher.update(&[0]);
        hasher.update(payload);
        hasher.finalize_xof()
    }
}
