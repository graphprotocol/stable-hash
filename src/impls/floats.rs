// TODO: Implement stable_hash for f32 and f64.
// For backward compatible migrations for arbitrary float precision to be satisfied,
// it needs to be implemented using the following structs, which are similar to Integer<T>
// TODO: +-0.0 compare the same, so have is_negative be false in that case.

/*
enum Float<T> {
    // Must be discriminant 0 for this to be the default
    Number(Finite<T>),
    PosInfinity,
    NegInfinity,
    Nan,
}

// https://floating-point-gui.de/formats/fp/
// https://evanw.github.io/float-toy/
struct Finite<T> {
    is_negative: bool,
    exponent: i16, // This could be generic, but this fits all values required for f32 and f64
    mantissa: T,   // Must be Borrow<[u8]>, should trim_zeroes when writing.
}
*/

// TODO: Test which exhaustively verifies all f32 bit patterns hash to the same values as (f32 as f64)
