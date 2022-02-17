use super::u192::U192;
use std::mem::transmute;

// Useful reading: https://kevinventullo.com/2018/12/24/hashing-unordered-sets-how-far-will-cleverness-take-you/
// Followed by: https://jeremykun.com/2021/10/14/group-actions-and-hashing-unordered-multisets/
// Construction taken from this paper: https://www.preprints.org/manuscript/201710.0192/v1

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct FldMix(U192);

impl FldMix {
    const P: U192 = U192([2305843009213693959, 2305843009213693950, 0]);
    const Q: U192 = U192([18446744073709551609, 0, 0]);
    const R: U192 = U192([8, 0, 0]);
    const I: U192 = U192([
        16140901064495857665,
        18446744073709551615,
        18446744073709551615,
    ]);

    #[inline]
    pub const fn new() -> Self {
        Self(Self::I)
    }

    #[inline(always)]
    fn u(x: U192, y: U192) -> U192 {
        Self::P + Self::Q * (x + y) + Self::R * x * y
    }

    pub fn mix(&mut self, value: u128) {
        // Peformance: Somehow transmuting this is *way*
        // faster than (value, 0u64)
        let value = unsafe { transmute((0u64, value)) };
        self.0 = Self::u(self.0, value);
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
    pub fn to_bytes(&self) -> [u8; 24] {
        unsafe { transmute(self.0) }
    }

    #[inline]
    pub fn from_bytes(value: [u8; 24]) -> Self {
        unsafe { transmute(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let mut a = FldMix::new();
        a.combine(FldMix::new());
        assert_eq!(FldMix::new(), a);
    }

    #[test]
    fn mixme() {
        let mut a = FldMix::new();
        a.mix(100);
        a.mix(10);
        a.mix(999);

        let mut b = FldMix::new();
        b.mix(10);
        b.mix(999);
        b.mix(100);

        assert_eq!(a, b);

        let mut c = FldMix::new();
        let mut d = FldMix::new();
        c.mix(999);
        c.mix(10);
        d.mix(100);
        c.combine(d);
        assert_eq!(b, c);
    }
}
