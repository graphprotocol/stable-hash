use firestorm::profile_fn;
use stable_hash::*;

pub fn xxhash(value: &impl StableHash) -> u128 {
    profile_fn!(xxhash);

    utils::stable_hash(value)
}

pub fn crypto_hash(value: &impl StableHash) -> String {
    profile_fn!(crypto_hash);

    let raw = utils::crypto_stable_hash(value);
    hex::encode(raw)
}

#[macro_export]
macro_rules! equal {
    ($value_xx:expr, $value_crypto:expr; $($data:expr),+) => {
        $(
            assert_eq!(common::xxhash(&$data), $value_xx);
            assert_eq!(&common::crypto_hash(&$data), $value_crypto);
        )+
    }
}

#[macro_export]
macro_rules! not_equal {
    ($left:expr, $right:expr) => {
        assert!(
            common::xxhash(&$left) != common::xxhash(&$right)
                && common::crypto_hash(&$left) != common::crypto_hash(&$right)
        )
    };
}
