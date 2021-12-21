use std::num::Wrapping;

pub(crate) type Accumulator = Wrapping<u128>;

// Useful reading: https://kevinventullo.com/2018/12/24/hashing-unordered-sets-how-far-will-cleverness-take-you/
// Rebuttal:
//   We use a larger accumulator state.

// Followed by: https://jeremykun.com/2021/10/14/group-actions-and-hashing-unordered-multisets/
// TODO: Consider mixing u128 values in a u256 space
//
// From this paper: https://www.preprints.org/manuscript/201710.0192/v1
// Choose odd q, even r, and prefer large values with gcd(p, r) = 1
// and pr = q(q-1).

pub trait FldMix {
    fn mix(&mut self, value: u64);
    fn finalize(&self) -> u128;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FldMixScalar(Accumulator);

// TODO: Search for other constants,
// since the paper was limited to u32

pub(crate) const P: Accumulator = Wrapping(3860031);
pub(crate) const Q: Accumulator = Wrapping(2779);
pub(crate) const R: Accumulator = Wrapping(2);

/// To find the identity:
/// u(x, y) = p + q(x + y) + rxy
/// We want u(x, *) = x;
///
/// Using these consts:
/// P = 3860031;
/// Q = 2779;
/// R = 2;
///
/// Which plugs to:
/// 0 = 3860031 + 2779(x + y) + 2xy - x;
///
/// If you put that in WolframAlpha it solves at:
/// y = -1389
///
/// u128::MAX -1389 + 1 is the identity. (Same as 0.wrapping_sub(1389))
pub(crate) const IDENTITY: Accumulator = Wrapping(340282366920938463463374607431768210067);

impl Default for FldMixScalar {
    fn default() -> Self {
        Self(IDENTITY)
    }
}

impl FldMix for FldMixScalar {
    fn mix(&mut self, other: u64) {
        let x = self.0;
        let y = Wrapping(other as u128);
        self.0 = Self::u(x, y);
    }

    fn finalize(&self) -> u128 {
        self.0 .0
    }
}

impl FldMixScalar {
    #[inline(always)]
    fn u(x: Accumulator, y: Accumulator) -> Accumulator {
        P + Q * (x + y) + R * x * y
    }

    #[cfg(test)]
    fn combine(&mut self, other: Self) {
        let x = self.0;
        let y = other.0;
        self.0 = Self::u(x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixme() {
        let mut a = FldMixScalar::default();
        a.mix(100);
        a.mix(10);
        a.mix(999);

        let mut b = FldMixScalar::default();
        b.mix(10);
        b.mix(999);
        b.mix(100);

        assert_eq!(a, b);

        let mut c = FldMixScalar::default();
        let mut d = FldMixScalar::default();
        c.mix(999);
        c.mix(10);
        d.mix(100);
        c.combine(d);
        assert_eq!(b, c);
    }
}
