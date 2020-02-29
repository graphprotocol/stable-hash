use crate::prelude::*;

macro_rules! impl_tuple {
    ($($T:ident),*) => {
        impl<$($T : StableHash,)*> StableHash for ($($T,)*) {
            #[allow(non_snake_case)]
            fn stable_hash(&self, mut sequence_number: impl SequenceNumber, state: &mut impl StableHasher) {
                let ($($T,)*) = self;

                $(
                    $T.stable_hash(sequence_number.next_child(), state);
                )*
            }
        }
    }
}

impl_tuple!(T0, T1);
impl_tuple!(T0, T1, T2);
impl_tuple!(T0, T1, T2, T3);
impl_tuple!(T0, T1, T2, T3, T4);
impl_tuple!(T0, T1, T2, T3, T4, T5);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
