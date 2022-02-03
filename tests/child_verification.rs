use {
    stable_hash::{prelude::*, utils::check_for_child_errors},
    std::collections::HashMap,
};

struct DoubleChild;

impl StableHash for DoubleChild {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        state.write(field_address.child(1), &[]);
        state.write(field_address.child(1), &[]);
    }
}

#[test]
fn double_child() {
    assert!(check_for_child_errors(&DoubleChild).is_err());
}

#[test]
fn double_child_through_unordered() {
    let mut map = HashMap::new();
    map.insert(1, DoubleChild);
    assert!(check_for_child_errors(&map).is_err());
}

struct UnorderedResultChild;

impl StableHash for UnorderedResultChild {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        let (_, a) = field_address.unordered();
        let b = a.child(0);
        state.write(b, &[])
    }
}

#[test]
fn unordered_result_child() {
    assert!(check_for_child_errors(&UnorderedResultChild).is_err());
}
