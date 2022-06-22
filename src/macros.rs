/// Implements StableHash. This macro supports two forms:
/// Struct { field1, field2, ... } and Tuple(transparent). Each field supports
/// an optional modifier. For example: Tuple(transparent: AsBytes)
///
/// This API is unstable and will likely be modified for a 1.0 release.
/// It's just a stub to cover some common cases.
#[macro_export]
macro_rules! impl_stable_hash {
    ($T:ident$(<$lt:lifetime>)? {$($field:ident$(:$e:path)?),*}) => {
        impl $crate::StableHash for $T$(<$lt>)? {
            // This suppressed warning is for the final index + 1, which is unused
            // in the next "iteration of the loop"
            #[allow(unused_assignments)]
            fn stable_hash<H: $crate::StableHasher>(&self, field_address: H::Addr, state: &mut H) {
                // Destructuring ensures we have all of the fields. If a field is added to the struct,
                // it must be added to the macro or it will fail to compile.
                let $T { $($field,)* } = self;
                let mut index = 0;
                $(
                    // We might need to "massage" the value, for example, to wrap
                    // it in AsBytes. So we provide a way to inject those.
                    $(let $field = $e($field);)?
                    $crate::StableHash::stable_hash(&$field, $crate::FieldAddress::child(&field_address, index), state);
                    index += 1;
                )*
            }
        }
    };
    ($T:ident$(<$lt:lifetime>)? (transparent$(:$e:path)?)) => {
        impl $crate::StableHash for $T$(<$lt>)? {
            #[allow(unused_assignments)]
            fn stable_hash<H: $crate::StableHasher>(&self, field_address: H::Addr, state: &mut H) {
                let Self(transparent) = self;
                $(let transparent = $e(transparent);)?
                $crate::StableHash::stable_hash(&transparent, field_address, state);
            }
        }
    };
}
