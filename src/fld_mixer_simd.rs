use super::fld_mixer::*;
use packed_simd::u128x4;
use std::num::Wrapping;

const BUFFER: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FldMixSimd {
    buffer: [(Accumulator, Accumulator); BUFFER],
    index: usize,
}

impl Default for FldMixSimd {
    fn default() -> Self {
        Self {
            buffer: [(IDENTITY, IDENTITY); BUFFER],
            index: 0,
        }
    }
}

impl FldMix for FldMixSimd {
    fn mix(&mut self, other: u64) {
        self.buffer[self.index].1 = Wrapping(other as u128);
        self.index += 1;
        if self.index == BUFFER {
            let p = u128x4::splat(P.0);
            let q = u128x4::splat(Q.0);
            let r = u128x4::splat(R.0);
            let y = u128x4::new(
                self.buffer[0].1 .0,
                self.buffer[1].1 .0,
                self.buffer[2].1 .0,
                self.buffer[3].1 .0,
            );
            let a = r * y + q;
            let b = q * y + p;
            for i in 0..BUFFER {
                let a_i = Wrapping(unsafe { a.extract_unchecked(i) });
                let b_i = Wrapping(unsafe { b.extract_unchecked(i) });
                self.buffer[i].0 = self.buffer[i].0 * a_i + b_i;
            }
            self.index = 0;
        }
    }

    fn finalize(&self) -> u128 {
        let mut x = IDENTITY;
        let p = u128x4::splat(P.0);
        let q = u128x4::splat(Q.0);
        let r = u128x4::splat(R.0);
        let y = u128x4::new(
            self.buffer[0].0 .0,
            self.buffer[1].0 .0,
            self.buffer[2].0 .0,
            self.buffer[3].0 .0,
        );
        let a = r * y + q;
        let b = q * y + p;
        for i in 0..BUFFER {
            let a_i = Wrapping(unsafe { a.extract_unchecked(i) });
            let b_i = Wrapping(unsafe { b.extract_unchecked(i) });
            x = x * a_i + b_i;
        }
        for i in 0..self.index {
            x = Self::uv(x, self.buffer[i].1);
        }
        x.0
    }
}

impl FldMixSimd {
    #[inline(always)]
    fn u(x: Accumulator, y: Accumulator) -> Accumulator {
        P + Q * (x + y) + R * x * y
    }

    #[inline(always)]
    fn uv(x: Accumulator, y: Accumulator) -> Accumulator {
        let a = R * y + Q;
        let b = Q * y + P;
        x * a + b
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
        for _ in 0..(BUFFER * 3) {
            inputs.push(rng.next_u64());
        }
        for i in 0..inputs.len() {
            test_mixer(&inputs[0..i]);
        }
    }
}
