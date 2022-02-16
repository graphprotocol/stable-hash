mod common;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use common::*;
use firestorm::profile_fn;
use legacy;
use stable_hash::utils::AsBytes;
use stable_hash::*;

#[test]
#[ignore = "benchmark"]
fn compare() {
    let mut data = HashMap::new();

    data.insert("abc", 100u64);
    data.insert("abcdef", 100u64);
    data.insert("abcdefged", 0u64);
    data.insert("abcdefgedaekllw", 50000u64);
    data.insert("acfaek", 50000u64);
    data.insert("aek", 511110000u64);

    fn profile(value: &impl StableHash) {
        profile_fn!(profile);
        fast_stable_hash(value);
        crypto_stable_hash(value);
    }

    if firestorm::enabled() {
        firestorm::bench("./firestorm", || profile(&data)).unwrap();
    }
}

use rand::{thread_rng, Rng, RngCore};
trait R {
    fn rand() -> Self;
}

impl R for i32 {
    fn rand() -> Self {
        thread_rng().gen()
    }
}

impl R for usize {
    fn rand() -> Self {
        let num: u32 = thread_rng().gen();
        (num % 45) as usize
    }
}

impl<T> R for Vec<T>
where
    T: R,
{
    fn rand() -> Self {
        let count = R::rand();
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push(R::rand());
        }
        v
    }
}

impl<K, V> R for HashMap<K, V>
where
    K: R + Hash + Eq,
    V: R,
{
    fn rand() -> Self {
        let count = R::rand();
        let mut h = HashMap::with_capacity(count);
        for _ in 0..count {
            let k = R::rand();
            let v = R::rand();
            h.insert(k, v);
        }
        h
    }
}

impl<T> R for HashSet<T>
where
    T: R + Hash + Eq,
{
    fn rand() -> Self {
        let count = R::rand();
        let mut h = HashSet::with_capacity(count);
        for _ in 0..count {
            let t = R::rand();
            h.insert(t);
        }
        h
    }
}

impl R for u8 {
    fn rand() -> Self {
        thread_rng().gen()
    }
}

impl R for String {
    fn rand() -> Self {
        loop {
            let bytes = R::rand();
            if let Ok(s) = String::from_utf8(bytes) {
                return s;
            }
        }
    }
}

impl R for bool {
    fn rand() -> Self {
        thread_rng().gen()
    }
}

impl R for [u8; 32] {
    fn rand() -> Self {
        let mut value = Self::default();
        thread_rng().fill_bytes(&mut value);
        value
    }
}

impl R for Value {
    fn rand() -> Self {
        let d: u32 = thread_rng().gen();
        match d % 5 {
            0 => Value::Null,
            1 => Value::Number(R::rand()),
            2 => Value::String(R::rand()),
            3 => Value::Bool(R::rand()),
            4 => Value::Array(R::rand()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Value {
    Null,
    Number(i32),
    String(String),
    Bool(bool),
    Array([u8; 32]),
}

impl StableHash for Value {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        let variant = match self {
            Self::Null => return,
            Self::Number(n) => {
                n.stable_hash(field_address.child(0), state);
                1
            }
            Self::String(n) => {
                n.stable_hash(field_address.child(0), state);
                2
            }
            Self::Bool(n) => {
                n.stable_hash(field_address.child(0), state);
                3
            }
            Self::Array(n) => {
                AsBytes(n).stable_hash(field_address.child(0), state);
                4
            }
        };
        state.write(field_address, &[variant]);
    }
}

#[derive(Debug)]
struct C {
    s: HashMap<String, Value>,
    n: i32,
}

impl StableHash for C {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        self.s.stable_hash(field_address.child(0), state);
        self.n.stable_hash(field_address.child(1), state);
    }
}

impl R for C {
    fn rand() -> Self {
        Self {
            s: R::rand(),
            n: R::rand(),
        }
    }
}

#[derive(Debug)]
struct A {
    v1: Vec<B>,
    v2: Vec<B>,
    v3: Vec<B>,
}

impl StableHash for A {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        self.v1.stable_hash(field_address.child(0), state);
        self.v2.stable_hash(field_address.child(1), state);
        self.v3.stable_hash(field_address.child(2), state);
    }
}

impl R for A {
    fn rand() -> Self {
        Self {
            v1: R::rand(),
            v2: R::rand(),
            v3: R::rand(),
        }
    }
}

#[derive(Debug)]
struct B {
    a: u8,
    c: HashMap<String, C>,
}

impl StableHash for B {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        self.a.stable_hash(field_address.child(0), state);
        self.c.stable_hash(field_address.child(1), state);
    }
}

impl R for B {
    fn rand() -> Self {
        Self {
            a: R::rand(),
            c: R::rand(),
        }
    }
}

#[test]
//#[ignore = "benchmark"]
fn bench() {
    mod legacy_impl {
        use super::{Value, A, B, C};
        use legacy::prelude::*;
        use legacy::utils::AsBytes;

        impl StableHash for A {
            fn stable_hash<H: StableHasher>(&self, mut field_address: H::Seq, state: &mut H) {
                self.v1.stable_hash(field_address.next_child(), state);
                self.v2.stable_hash(field_address.next_child(), state);
                self.v3.stable_hash(field_address.next_child(), state);
            }
        }

        impl StableHash for C {
            fn stable_hash<H: StableHasher>(&self, mut field_address: H::Seq, state: &mut H) {
                self.s.stable_hash(field_address.next_child(), state);
                self.n.stable_hash(field_address.next_child(), state);
            }
        }

        impl StableHash for B {
            fn stable_hash<H: StableHasher>(&self, mut field_address: H::Seq, state: &mut H) {
                self.a.stable_hash(field_address.next_child(), state);
                self.c.stable_hash(field_address.next_child(), state);
            }
        }

        impl StableHash for Value {
            fn stable_hash<H: legacy::StableHasher>(
                &self,
                mut field_address: H::Seq,
                state: &mut H,
            ) {
                let child = field_address.next_child();
                let variant = match self {
                    Self::Null => return,
                    Self::Number(n) => {
                        n.stable_hash(field_address.next_child(), state);
                        "Number"
                    }
                    Self::String(n) => {
                        n.stable_hash(field_address.next_child(), state);
                        "String"
                    }
                    Self::Bool(n) => {
                        n.stable_hash(field_address.next_child(), state);
                        "Bool"
                    }
                    Self::Array(n) => {
                        AsBytes(n).stable_hash(field_address.next_child(), state);
                        "Array"
                    }
                };
                variant.stable_hash(child, state);
            }
        }
    }

    let mut factor = 0.0;

    fn legacy_crypto<T: legacy::StableHash>(value: &T) -> [u8; 32] {
        legacy::utils::stable_hash::<legacy::crypto::SetHasher, T>(value)
    }

    let count = 80;
    for _ in 0..count {
        use std::time::Instant;
        let value: A = R::rand();

        let s = Instant::now();
        let x = fast_stable_hash(&value);
        let duration_x = Instant::now() - s;

        let s = Instant::now();
        //let c = legacy_crypto(&value);
        let c = crypto_stable_hash(&value);
        let duration_c = Instant::now() - s;

        assert_eq!(Ok(()), check_for_child_errors(&value));

        factor += duration_c.as_secs_f64() / duration_x.as_secs_f64();

        drop((x, c, value));
    }
    factor /= count as f64;
    println!("Factor: {}", factor);
}
