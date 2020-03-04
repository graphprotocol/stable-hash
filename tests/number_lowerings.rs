#[macro_use]
mod common;

use std::mem::size_of;

macro_rules! nums_equal {
    ($value:expr, $result:expr; $min:ty, $($t:ty),+) => {
        assert_eq!(common::xxhash(&($value as $min)), $result);
        $(
            assert_eq!(common::xxhash(&($value as $t)), $result);
        )+

        if size_of::<$min>() <= size_of::<usize>() && $value >= 0 {
            assert_eq!(common::xxhash(&($value as usize)), $result);
        }
        if size_of::<$min>() <= size_of::<isize>() {
            assert_eq!(common::xxhash(&($value as isize)), $result);
        }
    }
}

#[test]
fn up_to_u8() {
    nums_equal!(9, 15695816615077189814; u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
}

#[test]
fn up_to_u16() {
    nums_equal!(22768, 14145019129129205421; u16, i16, u32, i32, u64, i64, u128, i128);
}

#[test]
fn up_to_u32() {
    nums_equal!(1147483648, 11536801980706475161; u32, i32, u64, i64, u128, i128);
}

#[test]
fn up_to_u64() {
    nums_equal!(8223372036854775808u64, 16219818938521503862; u64, i64, u128, i128);
}

#[test]
fn up_to_u128() {
    nums_equal!(160141183460469231731687303715884105728u128, 13892203652687889343; u128, i128);
}

#[test]
fn down_to_i64() {
    nums_equal!(-9223372036854775808i64, 6256200190077353066; i64, i128);
}

#[test]
fn down_to_i32() {
    nums_equal!(-2147483647i32, 16152790417736434501; i32, i64, i128);
}

#[test]
fn down_to_i16() {
    nums_equal!(-12768i16, 16986113607939961363; i16, i32, i64, i128);
}

#[test]
fn down_to_i8() {
    nums_equal!(-12i8, 3386756839162099456; i8, i16, i32, i64, i128);
}
