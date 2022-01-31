mod common;

use std::collections::{HashMap, HashSet};

macro_rules! map(
    { $($key:expr => $value:expr),* } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
     };
);

macro_rules! set(
    { $($item:expr),* } => {
        {
            let mut m = HashSet::new();
            $(
                m.insert($item);
            )+
            m
        }
     };
);

#[test]
fn hash_map_eq() {
    equal!(
        293704917662630748122356071058265608388, "902c39ceef787565061cfeb44f679281e72a0e2edaa7cc68f480eaf7830359df";
        map!{ 1 => "one", 2 => "two", 3 => "three" },
        map!{ 3 => "three", 1 => "one", 2 => "two" }
    );
}

#[test]
fn hash_map_ne_count() {
    not_equal!(
        map! { 1 => "one", 2 => "two", 3 => "three", 0 => "" },
        map! { 1 => "one", 2 => "two", 3 => "three" }
    );
}

#[test]
fn hash_map_ne_key() {
    not_equal!(
        map! { 9 => "one", 2 => "two", 3 => "three" },
        map! { 1 => "one", 2 => "two", 3 => "three" }
    );
}

#[test]
fn hash_map_ne_value() {
    not_equal!(
        map! { 1 => "X", 2 => "two", 3 => "three" },
        map! { 1 => "one", 2 => "two", 3 => "three" }
    );
}

#[test]
fn hash_map_ne_swap() {
    not_equal!(
        map! { 1 => "one", 2 => "two" },
        map! { 1 => "two", 2 => "one" }
    )
}

#[test]
fn hash_set_eq() {
    equal!(
        192268494789251188407238827158023402710, "f38933334514c644985d53be4361285bdf3aae54a521eed564382a3ddc564402";
        set!{1, 2, 3},
        set!{3, 2, 1}
    );
}

#[test]
fn hash_set_ne_count() {
    not_equal!(set! {0, 1, 2}, set! {1, 2})
}

#[test]
fn hash_set_ne_item() {
    not_equal!(set! {1, 2}, set! {3, 2})
}
