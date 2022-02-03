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
        256719470344896394094682169332321082604, "25130dd4633e3e9ff049594c26ca698f3a0513f9c14d98ff69744b8a2237ab9f";
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
        97193918653041483173484915455375010047, "4a87fcf3748ef16f7ebd64f1547d757a0b74c26d06a3368bcc03a8fce77734ef";
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
