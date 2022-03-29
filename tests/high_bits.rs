mod common;

#[test]
fn uses_high_bits_in_seed() {
    // This simply has enough depth to trigger the seed to be nonzero when mixing with the fast impl
    // It's here so that changes to the seed don't cause this to break.
    let value = vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![10u8]]]]]]]]]];
    equal!(43065473437393246775826330103672755677, "1c245fc1dba448e20a6b93beb5a48d233837d611b354ef894c274e3c43553e82"; value);
}
