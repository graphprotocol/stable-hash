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
        5555886761104914033;
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
fn hash_set_eq() {
    equal!(
        17027054512634741606;
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
