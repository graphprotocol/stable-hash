use std::borrow::Borrow;
use std::num::Wrapping;

pub trait SequenceNumber {
    type Rollup: Borrow<[u8]>;
    fn rollup(&self) -> Self::Rollup;
    fn root() -> Self
    where
        Self: Sized;
    fn next_child(&mut self) -> Self
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct SequenceNumberInt<T> {
    rollup: Wrapping<T>,
    child: usize,
}

macro_rules! impl_sequence_no {
    ($T:ty, $size:expr, $prime_init:expr, $prime_mult:expr) => {
        impl SequenceNumber for SequenceNumberInt<$T> {
            type Rollup = [u8; $size];
            fn root() -> Self {
                Self {
                    rollup: Wrapping($prime_init),
                    child: 0,
                }
            }

            #[inline]
            fn next_child(&mut self) -> Self {
                let child = self.child;
                self.child += 1;

                let rollup = (self.rollup * Wrapping($prime_mult)) + Wrapping(child as $T);

                Self { rollup, child }
            }

            #[inline]
            fn rollup(&self) -> Self::Rollup {
                self.rollup.0.to_le_bytes()
            }
        }
    };
}

// These values are locked in!
// Don't change them. Ever.
impl_sequence_no!(u64, 8, 17, 486_187_739);
impl_sequence_no!(u32, 4, 17, 486_187_739);
