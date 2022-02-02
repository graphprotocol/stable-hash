use std::{num::Wrapping, u128};

// Useful reading: https://kevinventullo.com/2018/12/24/hashing-unordered-sets-how-far-will-cleverness-take-you/
// Followed by: https://jeremykun.com/2021/10/14/group-actions-and-hashing-unordered-multisets/
// Construction taken from this paper: https://www.preprints.org/manuscript/201710.0192/v1
#[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord)]
pub struct FldMix<const P: u128, const Q: u128, const R: u128>(Wrapping<u128>);

// // See also 0a3c85e1-117e-4322-8b8c-0adbe22ce8eb
pub type FldMixA = FldMix<3860031, 2779, 2>;
pub type FldMixB = FldMix<42535295865117307898334180790765617159, 18446744073709551609, 8>;

// TODO: Add a function to solve for the inverse.
impl<const P: u128, const Q: u128, const R: u128> FldMix<P, Q, R> {
    #[inline]
    pub const fn new() -> Self {
        Self(Wrapping(0u128.wrapping_sub(P / Q)))
    }

    #[inline(always)]
    fn u(x: Wrapping<u128>, y: Wrapping<u128>) -> Wrapping<u128> {
        Wrapping(P) + Wrapping(Q) * (x + y) + Wrapping(R) * x * y
    }

    // See also bdf7259b-12ee-4b95-b5d1-aefb60a935cf
    pub fn mix(&mut self, value: u128) {
        // The hash space needs to be a smaller space than the accumulator
        // space, also should have no collision with identity.
        // Note that the value 0 is not a problem.
        const MASK: u128 = u128::MAX >> 1;
        let y = Wrapping(value & MASK);
        self.0 = Self::u(self.0, y);
    }

    pub fn mixin(&mut self, value: &Self) {
        self.0 = Self::u(self.0, value.0);
    }

    #[cfg(test)]
    fn combine(&mut self, other: Self) {
        let x = self.0;
        let y = other.0;
        self.0 = Self::u(x, y);
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; 16] {
        self.0 .0.to_le_bytes()
    }

    #[inline]
    pub fn from_bytes(value: [u8; 16]) -> Self {
        Self(Wrapping(u128::from_le_bytes(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_one<const P: u128, const Q: u128, const R: u128>() {
        let mut a = FldMix::<P, Q, R>::new();
        a.mix(100);
        a.mix(10);
        a.mix(999);

        let mut b = FldMix::<P, Q, R>::new();
        b.mix(10);
        b.mix(999);
        b.mix(100);

        assert_eq!(a, b);

        let mut c = FldMix::<P, Q, R>::new();
        let mut d = FldMix::<P, Q, R>::new();
        c.mix(999);
        c.mix(10);
        d.mix(100);
        c.combine(d);
        assert_eq!(b, c);
    }

    #[test]
    fn mixme() {
        // See also 0a3c85e1-117e-4322-8b8c-0adbe22ce8eb
        test_one::<3860031, 2779, 2>();
        test_one::<42535295865117307898334180790765617159, 18446744073709551609, 8>();
    }
}
