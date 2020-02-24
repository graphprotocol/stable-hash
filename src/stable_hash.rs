use {
    std::hash::Hasher,
    std::borrow::Borrow as _,
    crate::prelude::*,
};

pub trait StableHasher : Default {
    fn write(&mut self, sequence_number: impl SequenceNumber, bytes: &[u8]);
}

pub trait StableHash {
    fn stable_hash(&self, sequence_number: impl SequenceNumber, state: &mut impl StableHasher);
}

#[derive(Default)]
pub struct StableHasherWrapper<T>(T);

impl<T: Hasher + Default> StableHasherWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl<T: Hasher + Default> StableHasher for StableHasherWrapper<T> {
    fn write(&mut self, sequence_number: impl SequenceNumber, bytes: &[u8]) {
        let seq_no = sequence_number.rollup();
        self.0.write(seq_no.borrow());
        self.0.write(bytes);
    }
}


pub(crate) fn trim_zeros(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len() - 1;
    while end != 0 && bytes[end] == 0 {
        end -= 1;
    }
    &bytes[0..=end]
}