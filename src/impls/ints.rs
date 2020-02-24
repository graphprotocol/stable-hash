use {
    crate::prelude::*,
    std::mem::size_of,
};

struct Integer<T> {
    is_negative: bool,
    unsigned: T,
}


impl StableHash for usize {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
        // This could go up to any arbitrary size and be backward compatible. The assert is just there as a reminder
        // to put in a (*self as _platform_native_type_) if there is such a thing.
        assert!(size_of::<usize>() < size_of::<u64>());
        (*self as u64).stable_hash(sequence_number, state)
    }
}

macro_rules! impl_int {
    ($P:ty, $N:ty) => {
        impl StableHash for Integer<$P> {
            fn stable_hash(&self, mut sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
                self.is_negative.stable_hash(sequence_number.next_child(), state);
                let bytes = self.unsigned.to_le_bytes();
                state.write(sequence_number, trim_zeros(&bytes));
            }
        }

        impl StableHash for $P {
            fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
                Integer {
                    is_negative: false,
                    unsigned: *self,
                }.stable_hash(sequence_number, state)
            }
        }
        impl StableHash for $N {
            fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
                Integer {
                    is_negative: *self < 0,
                    unsigned: (*self).abs() as $P,
                }.stable_hash(sequence_number, state)
            }
        }
    }
}

impl_int!(u128, i128);
impl_int!(u64, i64);
impl_int!(u32, i32);
impl_int!(u16, i16);
impl_int!(u8, i8);