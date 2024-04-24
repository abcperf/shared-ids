use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash, marker::PhantomData};

#[doc(hidden)]
pub use paste::paste;

#[macro_export]
macro_rules! id_type {
    ($vis:vis $name:ident) => {
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[doc(hidden)]
        $vis struct $name;
        impl $crate::IdType for $name {
            const NAME: &'static str = ::std::stringify!($name);
        }
        $crate::paste! {
            $vis type [< $name Id >] = $crate::Id<$name>;
        }
    };
}

id_type!(pub Client);
id_type!(pub Request);
id_type!(pub Replica);

pub trait IdType: Debug + Clone + Copy + Hash + PartialEq + Eq + PartialOrd + Ord {
    const NAME: &'static str;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent, bound = "")]
pub struct Id<T: IdType> {
    id: u64,
    phantom_data: PhantomData<T>,
}

impl<T: IdType> Id<T> {
    pub const fn first() -> Self {
        Self::from_u64(0)
    }

    pub const fn from_u64(id: u64) -> Self {
        Self {
            id,
            phantom_data: PhantomData,
        }
    }

    pub const fn as_u64(&self) -> u64 {
        self.id
    }

    pub fn as_mut_u64(&mut self) -> &mut u64 {
        &mut self.id
    }
}

impl<T: IdType> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Id({})", T::NAME, self.id)
    }
}

impl<T: IdType> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Id({})", T::NAME, self.id)
    }
}

pub struct IdIter<T: IdType> {
    next: Id<T>,
}

impl<T: IdType> Default for IdIter<T> {
    fn default() -> Self {
        Self { next: Id::first() }
    }
}

impl<T: IdType> Iterator for IdIter<T> {
    type Item = Id<T>;

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
