use super::Mixer;

pub struct MulMix(u128);

// This is the highest prime in u128, making this the largest
// of this kind of field in that space.
const PRIME: u128 = 340282366920938463463374607431768211297;
const FIELD_MAX: u128 = PRIME - 1;

// TODO: Unit test this
// Original taken from https://stackoverflow.com/a/54531811/11837266
fn field_mult(a: u128, b: u128) -> u128 {
    let add = |x: u128, y: u128| x.checked_sub(PRIME - y).unwrap_or_else(|| x + y);
    let split = |x: u128| (x >> 64, x & !(!0 << 64));
    let (a_hi, a_lo) = split(a);
    let (b_hi, b_lo) = split(b);
    let mut c = a_hi * b_hi % PRIME;
    let (d_hi, d_lo) = split(a_lo * b_hi);
    c = add(c, d_hi);
    let (e_hi, e_lo) = split(a_hi * b_lo);
    c = add(c, e_hi);
    for _ in 0..64 {
        c = add(c, c);
    }
    c = add(c, d_lo);
    c = add(c, e_lo);
    let (f_hi, f_lo) = split(a_lo * b_lo);
    c = add(c, f_hi);
    for _ in 0..64 {
        c = add(c, c);
    }
    add(c, f_lo)
}

impl Mixer for MulMix {
    const IDENTITY: Self = Self(1);
    fn mix(&mut self, value: u128) {
        // We need to make sure the input is not identity (1) because
        // that would be a collision with being missing from the set.
        // We also need to make sure that the value 0 is never stored.
        // So, what we want to see is that for any Non-Zero self.0,
        // if you multiply it by value the result is not 0. This can
        // be shown for this range because only if value * self.0
        // is a multiple of PRIME would self.0 be stored, but the
        // result cannot be a multiple of PRIME due to factorization.
        let value = FIELD_MAX - (value >> 1);
        self.0 = field_mult(self.0, value);
    }

    fn combine(self, other: Self) -> Self {
        MulMix(field_mult(self.0, other.0))
    }

    fn raw(&self) -> u128 {
        self.0
    }
}

impl Default for MulMix {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}
