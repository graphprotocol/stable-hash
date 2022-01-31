use stable_hash::crypto::SetHasher;
use stable_hash::prelude::*;
use stable_hash::utils::*;
mod common;

struct One<T0> {
    one: T0,
}

impl<T0: StableHash> StableHash for One<T0> {
    fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Addr, state: &mut H) {
        self.one.stable_hash(sequence_number.next_child(), state);
    }
}

struct Two<T0, T1> {
    one: T0,
    two: T1,
}

impl<T0: StableHash, T1: StableHash> StableHash for Two<T0, T1> {
    fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Addr, state: &mut H) {
        self.one.stable_hash(sequence_number.next_child(), state);
        self.two.stable_hash(sequence_number.next_child(), state);
    }
}

#[test]
fn add_optional_field() {
    let one = One { one: 5u32 };
    let two = Two {
        one: 5u32,
        two: Option::<u32>::None,
    };
    equal!(93872313880446700352540600112003890100, "3428a4134bfdac56aa04614504705b0ffd1d48f27777b109a793e5a641324212"; one, two);
}

#[test]
fn add_default_field() {
    let one = One { one: "one" };
    let two = Two {
        one: "one",
        two: "",
    };
    equal!(299097693820868656192212373807570572314, "65bf96c193b5d365191b86da83097939ccd67ac226d9f3a3c991719e338de7ed"; one, two);
}

#[test]
fn add_non_default_field() {
    let one = One { one: "one" };
    let two = Two {
        one: "one",
        two: "two",
    };
    not_equal!(one, two);
}

#[test]
fn defaults_are_non_emitting() {
    let empty_2: String = hex::encode(SetHasher::default().finish());
    // TODO: Verify this number is non-emitting
    equal!(16283408782925993922971546446457817912, &empty_2; false, Option::<bool>::None, 0i32, Vec::<String>::new(), "");
}

#[test]
fn some_default_ne() {
    not_equal!(Some(0u32), Option::<u32>::None);
}

#[test]
fn path_to_some() {
    not_equal!(Some(0u32), true);
}

#[test]
fn empty_vec_is_default() {
    let one = One { one: true };
    let two = Two {
        one: true,
        two: Vec::<u32>::new(),
    };
    equal!(159331941191816660581068356539222878046, "db4657c873e33a60e581eb5458aba6c76f510e023872c76a3134608619342c59"; one, two);
}

#[test]
fn two_is_used() {
    let one = One { one: true };
    let two = Two {
        one: true,
        two: true,
    };
    not_equal!(one, two);
}

#[test]
fn omitted_defaults_dont_collide() {
    not_equal!(vec![1u32, 0u32, 2u32], vec![0u32, 1u32, 2u32]);
}

#[test]
fn as_bytes() {
    let v = vec![0u8];
    not_equal!(&v[..], AsBytes(&v[..]));

    let v = vec![1u8, 2u8];
    not_equal!(&v[..], AsBytes(&v[..]));
}

#[test]
fn numbers_through_vec() {
    equal!(
        314089837248698845900816223956694438045, "76f2baeae1278bdd771f9bbefd07ba570066cd710be014ac65785fb411d5c547";
        vec![1u32, 2u32],
        vec![1u16, 2u16]
    );
}
