use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub trait AnyId:
    Debug + PartialEq + Eq + Hash + Clone + Copy + Serialize + for<'a> Deserialize<'a>
{
    const FIRST: Self;
    fn from_u64(id: u64) -> Self;
    fn as_u64(&self) -> u64;
    fn as_mut_u64(&mut self) -> &mut u64;
}

macro_rules! impl_id {
    ($name:ty) => {
        impl AnyId for $name {
            const FIRST: Self = Self(0);

            fn from_u64(id: u64) -> Self {
                Self(id)
            }

            fn as_u64(&self) -> u64 {
                self.0
            }

            fn as_mut_u64(&mut self) -> &mut u64 {
                &mut self.0
            }
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ReplicaId(u64);
impl_id!(ReplicaId);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ClientId(u64);
impl_id!(ClientId);

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct RequestId(u64);
impl_id!(RequestId);

pub struct IdIter<I: AnyId> {
    next: I,
}

impl<I: AnyId> Default for IdIter<I> {
    fn default() -> Self {
        Self { next: I::FIRST }
    }
}

impl<I: AnyId> Iterator for IdIter<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next;

        let id = self.next.as_mut_u64();

        if let Some(next) = id.checked_add(1) {
            *id = next;
            Some(ret)
        } else {
            None
        }
    }
}
