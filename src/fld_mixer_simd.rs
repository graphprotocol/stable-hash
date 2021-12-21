use super::fld_mixer::*;
use packed_simd::u128x4;
use std::num::Wrapping;

const LANES: usize = 4;
type Buffer = u128x4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FldMixSimd {
    accumulator: Accumulator,
    buffer: Buffer,
    index: usize,
}

impl Default for FldMixSimd {
    fn default() -> Self {
        Self {
            accumulator: IDENTITY,
            buffer: Buffer::splat(0),
            index: 0,
        }
    }
}

impl FldMix for FldMixSimd {
    fn mix(&mut self, other: u64) {
        self.buffer = unsafe { self.buffer.replace_unchecked(self.index, other as u128) };
        self.index += 1;
        if self.index == LANES {
            self.mix_buffer();
        }
    }

    fn finalize(&self) -> u128 {
        if self.index == 0 {
            self.accumulator.0
        } else {
            self.mixed_buffer().0
        }
    }
}

impl FldMixSimd {
    #[inline(always)]
    fn mix_buffer(&mut self) {
        self.accumulator = self.mixed_buffer();
        self.index = 0;
    }

    /// The original mixing operation is `x' = P + Q(x + y) + Rxy`. To adapt this function for SIMD
    /// with N lanes we extract the operations that have no data dependencies on the value of x. The
    /// following is the vectorized form, where constants P, Q, and R are splatted vectors of those
    /// constants, y is a vector containing N buffered values to mix, and x remains the scalar
    /// accumulator.
    /// ```
    /// let a = y * R + Q;
    /// let b = y * Q + P;
    /// for i in 0..LANES { x = x * a[i] + b[i]; }
    /// ```
    #[inline(always)]
    fn mixed_buffer(&self) -> Accumulator {
        let p = u128x4::splat(P.0);
        let q = u128x4::splat(Q.0);
        let r = u128x4::splat(R.0);
        let y = self.buffer;
        let a = r * y + q;
        let b = q * y + p;
        let mut x = self.accumulator;
        for i in 0..self.index {
            let a_i = Wrapping(unsafe { a.extract_unchecked(i) });
            let b_i = Wrapping(unsafe { b.extract_unchecked(i) });
            x = x * a_i + b_i;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::{super::fld_mixer::FldMix, *};
    use rand::{rngs::SmallRng, thread_rng, RngCore as _, SeedableRng as _};

    fn test_mixer(inputs: &[u64]) {
        let mut mixer = FldMixSimd::default();
        let mut model = FldMixScalar::default();
        for input in inputs {
            mixer.mix(*input);
            model.mix(*input);
        }
        assert_eq!(mixer.finalize(), model.finalize());
    }

    #[test]
    fn consistent_with_scalar_impl() {
        let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
        let mut inputs = Vec::new();
        for _ in 0..(LANES * 3) {
            inputs.push(rng.next_u64());
        }
        for i in 0..inputs.len() {
            test_mixer(&inputs[0..i]);
        }
    }
}
