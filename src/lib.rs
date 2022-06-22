//! This crate provides a stable, structured hash with backward compatibility features.
//! What does that mean?
//!  * Stable: The value of the hash will not change across minor versions of this library,
//!    even when the compiler, process, architecture, or std lib does change.
//!  * Structured: Hashes structs, rather than streams
//!  * Backward compatibility: It is possible to make limited changes to a struct's schema
//!    without changing the value of the hash. One change is that you can add new fields.
//!    This is accomplished by skipping default values when hashing. For example, the values
//!    None, 0, false, and vec![] do not contribute to the hash. Therefore structs Old { a: 1 }
//!    and New { a: 1, b: None } hash to the same value. Another feature enabling backward compatibility
//!    is that the size of an integer can be increased without changing the value. For example,
//!    Old { a: 1u16 } and New { a: 1u32 } hash to the same value. Note that even though two structs
//!    with different schemas are allowed to collide, two structs with the same schema never collide
//!    (where collide is defined as contribution to the hash is injective in respect to the encoding. It is
//!    still possible to find collisions in the final output, especially for the non-cryptographic version)

pub mod crypto;
pub mod fast;
mod impls;
mod macros;
pub mod prelude;
pub mod utils;
mod verification;
use prelude::*;

/// Like Hasher, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
pub trait StableHasher {
    type Out;
    type Addr: FieldAddress;

    fn new() -> Self;
    fn write(&mut self, field_address: Self::Addr, bytes: &[u8]);
    fn mixin(&mut self, other: &Self);
    fn finish(&self) -> Self::Out;

    type Bytes: AsRef<[u8]>;
    fn to_bytes(&self) -> Self::Bytes;
    fn from_bytes(bytes: Self::Bytes) -> Self;
}

/// Like Hash, but consistent across:
/// * builds (independent of rustc version or std implementation details)
/// * platforms (eg: 32 bit & 64 bit, x68 and ARM)
/// * processes (multiple runs of the same program)
///
/// For examples of best practices when implementing:
/// See also d3ba3adc-6e9b-4586-a7e7-6b542df39462
pub trait StableHash {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H);
}

/// Tracks the path from the root of a struct to a member value. For example,
/// within the value vec![ { num: 0, string: "Alice" }, { num: 1, string: "Bob" } ],
/// the value Alice exists at the path:
///
///    FieldAddress::root()  // Vec
///       .child(0)          // [0]
///       .child(1)          // .string
///
/// Because default values do not contribute to the final hash in order to support
/// backward compatibility, this concept is necessary to disambiguate in cases where default
/// values exist in a struct with multiple fields. For example given the struct:
///
/// Struct {
///    a: 0,
///    b: 1,
/// }
///
/// and
///
/// Struct {
///    a: 1,
///    b: 0,
/// }
///
/// if we naively write out the struct as a series of fields without addresses each would produce
/// the following encoded stream (default values omitted):
///
/// {1}
/// {1}
///
/// But, with field addresses you get this encoding instead:
///
/// {(1, 0)}
/// {(0, 1)}
///
/// which fixes the collision.
pub trait FieldAddress: Sized {
    /// The starting value
    fn root() -> Self;
    /// A nesting along the path. When calling this function,
    /// a consistent number should be supplied to identify a given field.
    /// To maintain backward compatibility, this number should remain consistent
    /// even as fields are added, removed, or re-ordered.
    fn child(&self, number: u64) -> Self;
    /// This one is tricky, as it involves hashing a set online.
    /// In this case, each member of the set has the same FieldAddress, but work must
    /// be done to relate multiple field addresses within the same set. The first return
    /// value is for a member of the set, and the second for relating members within the set.
    /// This is confusing. Consider the following example _without_ having two values returned
    /// here:
    ///
    /// { ("a", 1), ("b", 2) }
    /// { ("b", 1), ("a", 2) }
    ///
    /// See that these collide unless we do something to "relate" children within different
    /// members of the same set even while the members must have the same address to keep the
    /// set unordered.
    ///
    /// Understand ye, and despair. The other option was to sort the set. But, this has two
    /// drawbacks. First, it would require `StableHash: Ord`. (Ironically, unordered sets like
    /// HashMap and HashSet where this is relevant do not implement Ord... so this is a no-go but
    /// even if they did we still would prefer to not have this constraint). The second problem
    /// with sorting is that it is less online. Within the graph-node Proof of Indexing (the flagship
    /// use-case which this crate was implemented for) we need to be able to add and remove items
    /// from very large unordered sets without re-sorting and re-calculating everything up to the root.
    /// This ability becomes necessary when indexing off-chain data sources with unreliable availability.
    ///
    /// Please avoid implementing or calling this function unless you know what you are doing.
    /// See also a817fb02-7c77-41d6-98e4-dee123884287
    fn unordered(&self) -> (Self, Self);
}

pub fn fast_stable_hash<T: StableHash>(value: &T) -> u128 {
    profile_fn!(fast_stable_hash);
    generic_stable_hash::<T, crate::fast::FastStableHasher>(value)
}

pub fn crypto_stable_hash<T: StableHash>(value: &T) -> [u8; 32] {
    profile_fn!(crypto_stable_hash);
    generic_stable_hash::<T, crate::crypto::CryptoStableHasher>(value)
}
