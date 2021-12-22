use super::fld_mixer::*;
use packed_simd::u128x4;
use std::num::Wrapping;

const BUFFER: usize = 4;

const P: u128x4 = u128x4::splat(3860031);
const Q: u128x4 = u128x4::splat(2779);
const R: u128x4 = u128x4::splat(2);
const IDENTITY: u128x4 = u128x4::splat(340282366920938463463374607431768210067);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FldMixSimd {
    x: u128x4,
    y: u128x4,
    index: usize,
}

impl Default for FldMixSimd {
    fn default() -> Self {
        Self {
            x: IDENTITY,
            y: u128x4::default(),
            index: 0,
        }
    }
}

impl FldMix for FldMixSimd {
    fn mix(&mut self, other: u64) {
        // TODO: Check if it's faster to use an aligned slice and conver the whole thing on mix rather than
        // "constructing" a new SIMD buffer each time.
        //self.y = unsafe { self.y.replace_unchecked(self.index, other as u128) };
        self.index += 1;
        if self.index == BUFFER {
            self.x = P + Q * (self.x + self.y) + R * self.x * self.y;
            self.index = 0;
        }
    }

    fn finalize(&self) -> u128 {
        // TODO: An actual finalize step
        unsafe { self.x.extract_unchecked(self.index) }
    }
}

/*
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
*/

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
