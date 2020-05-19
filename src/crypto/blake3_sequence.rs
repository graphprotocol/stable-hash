use crate::prelude::*;
use blake3::{Hasher, OutputReader};
use leb128::write::unsigned as write_varint;
use std::convert::TryInto as _;
use std::num::NonZeroUsize;

#[derive(Clone)]
pub struct Blake3SeqNo {
    hasher: Hasher,
    // This has to be NonZero in order to be injective, since the payload marker writes 0
    // See also 91e48829-7bea-4426-971a-f092856269a5
    child: NonZeroUsize,
}

impl SequenceNumber for Blake3SeqNo {
    fn root() -> Self {
        Self {
            hasher: Hasher::new(),
            child: NonZeroUsize::new(1).unwrap(),
        }
    }
    fn next_child(&mut self) -> Self {
        let child = self.child;
        let mut hasher = self.hasher.clone();
        // Better to panic than overflow.
        self.child = NonZeroUsize::new(child.get() + 1).unwrap();
        // Include the child node
        write_varint(&mut hasher, child.get().try_into().unwrap()).unwrap();
        Self {
            hasher,
            child: NonZeroUsize::new(1).unwrap(),
        }
    }
    #[inline]
    fn skip(&mut self, count: usize) {
        self.child = NonZeroUsize::new(self.child.get() + count).unwrap();
    }
}

impl Blake3SeqNo {
    pub(crate) fn finish(self, payload: &[u8]) -> OutputReader {
        let Self { mut hasher, .. } = self;

        // To debug all the payloads in a hash to find a diff, this can be useful.
        /*
        #[derive(Debug)]
        struct Update {
            payload: String,
            seq_no: String,
        }
        let update = Update {
            seq_no: hex::encode(hasher.finalize().as_bytes()),
            payload: hex::encode(payload),
        };
        dbg!(update);
        */

        // See also 91e48829-7bea-4426-971a-f092856269a5
        hasher.update(&[0]);
        hasher.update(payload);
        hasher.finalize_xof()
    }
}
