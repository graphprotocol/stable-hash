use std::ops::{Add, Mul};

// This was started by the output of the uint crate,
// then heavily reduced to only the parts we need
// (which was a significant optimization, especially
// in the mul which carried out the full multiplication
// and discarded the top-half)
//
/// Little-endian large integer type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct U192(pub [u64; 3]);

impl Mul for U192 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        // The generated implementation of this method was 360 lines long!
        let me = &self.0;
        let you = &other.0;

        let mult = |m: usize, y: usize| {
            let v = me[m] as u128 * you[y] as u128;
            (v as u64, (v >> 64) as u64)
        };

        let (r0, r1) = mult(0, 0);
        let (low, hi0) = mult(1, 0);
        let (r1, overflow0) = low.overflowing_add(r1);
        let (low, hi1) = mult(0, 1);
        let (r1, overflow1) = low.overflowing_add(r1);

        let r2 = (hi0 + overflow0 as u64)
            .wrapping_add(hi1 + overflow1 as u64)
            .wrapping_add(me[2].wrapping_mul(you[0]))
            .wrapping_add(me[1].wrapping_mul(you[1]))
            .wrapping_add(me[0].wrapping_mul(you[2]));

        U192([r0, r1, r2])
    }
}

impl Add for U192 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let me = &self.0;
        let you = &other.0;

        let (r0, overflow0) = me[0].overflowing_add(you[0]);
        let (res, overflow1a) = me[1].overflowing_add(you[1]);
        let (r1, overflow1b) = res.overflowing_add(overflow0 as u64);

        let r2 = me[2]
            .wrapping_add(you[2])
            .wrapping_add(overflow1a as u64 + overflow1b as u64);

        U192([r0, r1, r2])
    }
}
