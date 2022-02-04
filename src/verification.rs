use {
    crate::prelude::*,
    std::{
        collections::HashSet,
        sync::{
            atomic::{AtomicU8, Ordering::SeqCst},
            Arc, Mutex,
        },
    },
};

pub struct ChildState {
    err: Arc<Mutex<Option<(ChildErr, Vec<PathItem>)>>>,
    children: Mutex<HashSet<u64>>,
    state: AtomicU8,
    path: Vec<PathItem>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ChildErr {
    InvalidStateTransition { from: u8, to: u8 },
    DuplicateChild(u64),
}

pub struct ChildChecker {
    err: Option<(ChildErr, Vec<PathItem>)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathItem {
    UnorderedRoot,
    UnorderedResult,
    Child(u64),
}

impl ChildState {
    const UNUSED: u8 = 1u8;
    const UNORDERED_ROOT: u8 = 2u8;
    const WRITTEN: u8 = 4u8;
    const PARENT: u8 = 8u8;
    const UNORDERED_RESULT: u8 = 16u8;
    const UNORDERED_PARENT: u8 = 32u8;

    fn new(parent: Option<(&ChildState, PathItem)>, state: u8) -> Self {
        let (err, path) = if let Some((parent, path)) = parent {
            (parent.err.clone(), {
                let mut v = parent.path.clone();
                v.push(path);
                v
            })
        } else {
            (Arc::new(Mutex::new(None)), Vec::new())
        };
        Self {
            path,
            err,
            children: Mutex::new(HashSet::new()),
            state: AtomicU8::new(state),
        }
    }

    fn set_err(&self, value: ChildErr) {
        *self.err.lock().unwrap() = Some((value, self.path.clone()));
    }
}

fn swap_state<F, T>(value: &ChildState, new_state: u8, ok_states: u8, f: F) -> T
where
    F: FnOnce(&ChildState) -> T,
{
    let prev_state = value.state.swap(new_state, SeqCst);
    if (prev_state & ok_states) == 0 {
        value.set_err(ChildErr::InvalidStateTransition {
            from: prev_state,
            to: new_state,
        })
    }
    f(value)
}

impl FieldAddress for ChildState {
    fn root() -> Self {
        ChildState::new(None, ChildState::UNUSED)
    }
    fn child(&self, number: u64) -> Self {
        swap_state(
            self,
            ChildState::PARENT,
            ChildState::PARENT | ChildState::UNUSED | ChildState::UNORDERED_ROOT,
            |s| {
                let mut children = s.children.lock().unwrap();
                if !children.insert(number) {
                    //panic!();
                    self.set_err(ChildErr::DuplicateChild(number))
                }
                ChildState::new(Some((s, PathItem::Child(number))), ChildState::UNUSED)
            },
        )
    }
    fn unordered(&self) -> (Self, Self) {
        swap_state(
            self,
            ChildState::UNORDERED_PARENT,
            ChildState::UNUSED | ChildState::UNORDERED_PARENT,
            |s| {
                (
                    ChildState::new(
                        Some((s, PathItem::UnorderedRoot)),
                        ChildState::UNORDERED_ROOT,
                    ),
                    ChildState::new(
                        Some((s, PathItem::UnorderedResult)),
                        ChildState::UNORDERED_RESULT,
                    ),
                )
            },
        )
    }
}

impl StableHasher for ChildChecker {
    type Out = Result<(), (ChildErr, Vec<PathItem>)>;
    type Addr = ChildState;

    fn new() -> Self {
        Self { err: None }
    }
    // TODO: We can check if all the field_address hit the same root
    fn write(&mut self, field_address: Self::Addr, _bytes: &[u8]) {
        swap_state(
            &field_address,
            ChildState::WRITTEN,
            ChildState::UNUSED
                | ChildState::PARENT
                | ChildState::UNORDERED_RESULT
                | ChildState::UNORDERED_ROOT,
            |s| {
                if self.err.is_none() {
                    self.err = s.err.lock().unwrap().clone();
                }
            },
        );
    }
    fn mixin(&mut self, _other: &Self) {
        todo!()
    }
    fn finish(&self) -> Self::Out {
        if let Some(err) = &self.err {
            Err(err.clone())
        } else {
            Ok(())
        }
    }

    type Bytes = [u8; 0];
    fn to_bytes(&self) -> Self::Bytes {
        []
    }
    fn from_bytes(_bytes: Self::Bytes) -> Self {
        todo!()
    }
}
