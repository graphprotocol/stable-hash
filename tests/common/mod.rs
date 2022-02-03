use firestorm::profile_fn;
use stable_hash::*;
pub use stable_hash::{fast_stable_hash, utils::check_for_child_errors};

#[allow(dead_code)]
pub fn crypto_stable_hash_str(value: &impl StableHash) -> String {
    profile_fn!(crypto_stable_hash_str);

    let raw = stable_hash::crypto_stable_hash(value);
    hex::encode(raw)
}

#[macro_export]
macro_rules! equal {
    ($value_xx:expr, $value_crypto:expr; $($data:expr),+) => {
        $(
            assert_eq!(common::fast_stable_hash(&$data), $value_xx);
            assert_eq!(&common::crypto_stable_hash_str(&$data), $value_crypto);
            assert_eq!(Ok(()), common::check_for_child_errors(&$data));
        )+
    }
}

#[macro_export]
macro_rules! not_equal {
    ($left:expr, $right:expr) => {{
        assert!(common::fast_stable_hash(&$left) != common::fast_stable_hash(&$right));
        assert!(common::crypto_stable_hash_str(&$left) != common::crypto_stable_hash_str(&$right));
        assert_eq!(Ok(()), common::check_for_child_errors(&$left));
        assert_eq!(Ok(()), common::check_for_child_errors(&$right));
    }};
}
