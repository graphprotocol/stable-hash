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
        2661109763294955775, "f83e01d65e6f3e9b4bc1d85f55001e185be78dbd64ea6dee53b3d027bdc66e62";
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
        18419013042853051292, "fdae4bd967f5196dc4a61a11e445c8fb4b4f5e9860788457d75a7d0f55a02ea1";
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
