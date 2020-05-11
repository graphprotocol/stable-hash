#[macro_use]
mod common;
use std::mem::size_of;

macro_rules! nums_equal {
    ($value:expr, $result_xx:expr, $result_crypto:expr; $min:ty, $($t:ty),+) => {
        equal!($result_xx, $result_crypto; ($value as $min));
        $(
            equal!($result_xx, $result_crypto; ($value as $t));
        )+

        if size_of::<$min>() <= size_of::<usize>() && $value >= 0 {
            equal!($result_xx, $result_crypto; ($value as usize));
        }
        if size_of::<$min>() <= size_of::<isize>() {
            equal!($result_xx, $result_crypto; ($value as isize));
        }
    }
}

#[test]
fn up_to_u8() {
    nums_equal!(9, 15695816615077189814, "173097115007a0965e818effe3bc946da648604343807e529b1999b39a3a1e0b"; u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
}

#[test]
fn up_to_u16() {
    nums_equal!(22768, 14145019129129205421, "83b02b2e018f5e08a6d20f8e0f9fec918f1a091483673436c4f2a9cada41cb22"; u16, i16, u32, i32, u64, i64, u128, i128);
}

#[test]
fn up_to_u32() {
    nums_equal!(1147483648, 11536801980706475161, "c33297c44fb3f4a51d7b8e4bb619414e46a67419f3e8bc4e34b8b80f27db033a"; u32, i32, u64, i64, u128, i128);
}

#[test]
fn up_to_u64() {
    nums_equal!(8223372036854775808u64, 16219818938521503862, "a7e31391c320def7aa0e5732034c0942ca6950adca6624fbaf2caee1147ceae2"; u64, i64, u128, i128);
}

#[test]
fn up_to_u128() {
    nums_equal!(160141183460469231731687303715884105728u128, 13892203652687889343, "1c22c3e5312542ca82d3b63e21f2608864ac5a377d55f7c24d2c12eae2cd743e"; u128, i128);
}

#[test]
fn down_to_i64() {
    nums_equal!(-9223372036854775808i64, 6256200190077353066, "8aaff1a84ed29f58b02ee2d09605bfddfb8c3003bf7bb2d0c71265ceaa3457c2"; i64, i128);
}

#[test]
fn down_to_i32() {
    nums_equal!(-2147483647i32, 16152790417736434501, "473b346a411b0dd9066dbd81959418d22127c467670abc734450db67851ff004"; i32, i64, i128);
}

#[test]
fn down_to_i16() {
    nums_equal!(-12768i16, 16986113607939961363, "f7d1323c9e76079022c5a120e4abd30044c2755cc96e467221a96320920daaea"; i16, i32, i64, i128);
}

#[test]
fn down_to_i8() {
    nums_equal!(-12i8, 3386756839162099456, "867b0b908a1ee3f4b1473febd9a76e8950692e631b1c4e39b4c18d26606cba40"; i8, i16, i32, i64, i128);
}
