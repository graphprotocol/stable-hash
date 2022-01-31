use crate::prelude::*;

macro_rules! impl_tuple {
    ($($T:ident),*) => {
        impl<$($T : StableHash,)*> StableHash for ($($T,)*) {
            #[allow(non_snake_case)]
            #[allow(unused_assignments)]
            fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Addr, state: &mut H) {
                profile_method!(stable_hash);

                let ($($T,)*) = self;

                let mut i = 0;

                $(
                    $T.stable_hash(sequence_number.child(i), state);
                    i += 1;
                )*
            }
        }
    }
}

macro_rules! impl_tuples {
    ($T:ident) => { };
    ($Head:ident, $($Tail:ident),+) => {
        impl_tuple!($Head, $($Tail),+);
        impl_tuples!($($Tail),+);
    }
}

impl_tuples!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
