use stable_hash::*;
use twox_hash::XxHash64;

pub fn xxhash(value: &impl StableHash) -> u64 {
    let mut hasher = StableHasherWrapper::<XxHash64>::default();
    value.stable_hash(SequenceNumberInt::<u64>::root(), &mut hasher);
    hasher.finish()
}

#[macro_export]
macro_rules! equal {
    ($value:expr; $($data:expr),+) => {
        $(
            assert_eq!(common::xxhash(&$data), $value);
        )+
    }
}

#[macro_export]
macro_rules! not_equal {
    ($left:expr, $right:expr) => {
        assert_ne!(common::xxhash(&$left), common::xxhash(&$right));
    };
}
