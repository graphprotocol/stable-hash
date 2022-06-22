#[macro_export]
macro_rules! impl_stable_hash {
    ($T:ident$(<$lt:lifetime>)? {$($field:ident$(:$e:path)?),*}) => {

        impl $crate::StableHash for $T$(<$lt>)? {
            fn stable_hash<H: $crate::StableHasher>(&self, mut sequence_number: H::Seq, state: &mut H) {
                let $T { $($field,)* } = self;
                $(
                    $(let $field = $e($field);)*
                    $crate::StableHash::stable_hash(&$field, $crate::SequenceNumber::next_child(&mut sequence_number), state);
                )*
            }
        }
    };
    ($T:ident$(<$lt:lifetime>)? (transparent$(:$e:path)?)) => {
        impl $crate::StableHash for $T$(<$lt>)? {
            fn stable_hash<H: $crate::StableHasher>(&self, sequence_number: H::Seq, state: &mut H,
            ) {
                let Self(transparent) = self;
                $(let transparent = $e(transparent);)*
                $crate::StableHash::stable_hash(&transparent, sequence_number, state);
            }
        }
    };
}
