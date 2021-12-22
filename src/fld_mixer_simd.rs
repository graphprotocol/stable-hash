use super::fld_mixer::*;
use std::num::Wrapping;

const BUFFER: usize = 8;

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
            for row in &mut self.buffer {
                row.0 = Self::uv(row.0, row.1);
            }
            self.index = 0;
        }
    }

    fn finalize(&self) -> u128 {
        let mut x = IDENTITY;
        for row in &self.buffer {
            x = Self::uv(x, row.0);
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
