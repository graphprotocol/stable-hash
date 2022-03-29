use super::u192::U192;

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

    pub fn mix(&mut self, value: u128, seed: u64) {
        // See also 0d123631-c654-4246-8d26-092c21d43037
        let v0 = seed & (u64::MAX >> 1);
        let v1 = value as u64;
        let v2 = (value >> 64) as u64;
        let value = U192([v0, v1, v2]);
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
        let mut bytes = [0; 24];
        bytes[0..8].copy_from_slice(&self.0 .0[0].to_le_bytes());
        bytes[8..16].copy_from_slice(&self.0 .0[1].to_le_bytes());
        bytes[16..24].copy_from_slice(&self.0 .0[2].to_le_bytes());
        bytes
    }

    #[inline]
    pub fn from_bytes(bytes: [u8; 24]) -> Self {
        let v0 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let v1 = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let v2 = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        Self(U192([v0, v1, v2]))
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
    fn seed_cannot_collide_with_identity() {
        // See also 0d123631-c654-4246-8d26-092c21d43037
        let first = FldMix::new().0 .0[0];
        assert!(first != first & (u64::MAX >> 1));
    }

    #[test]
    fn mixme() {
        let mut a = FldMix::new();
        a.mix(100, u64::MAX);
        a.mix(10, 10);
        a.mix(999, 100);

        let mut b = FldMix::new();
        b.mix(10, 10);
        b.mix(999, 100);
        b.mix(100, u64::MAX);

        assert_eq!(a, b);

        let mut c = FldMix::new();
        let mut d = FldMix::new();
        c.mix(999, 100);
        c.mix(10, 10);
        d.mix(100, u64::MAX);
        c.combine(d);
        assert_eq!(b, c);
    }
}
