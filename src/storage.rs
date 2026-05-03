//! Storage adapters.
//!
//! ### Usage with custom containers
//! If the [`AllocStorage`] or [`HeaplessStorage`] adapters don't suit your usecase e.g. you have
//! custom buffers that you want to use as backing storage, you will have to create an adapter
//! struct, then implement the [`Storage`] trait on it. For the associated types in the [`Storage`]
//! trait, you must also implement [`TextContainer`], [`QueueContainer`], [`StackContainer`] and
//! [`SetContainer`] on those types as well. See impls for [`AllocStorage`] or [`HeaplessStorage`]
//! for examples.

use crate::error::StorageError;
use core::fmt::Debug;
use core::hash::Hash;

/// Trait describing a family of storage containers to use.
pub trait Storage {
    /// ASCII String container. See [`TextContainer`].
    type Text: TextContainer;
    /// Queue container. See [`QueueContainer`].
    type Queue<T>: QueueContainer<T>;
    /// Stack (Vec-like) container. See [`StackContainer`].
    type Vec<T>: StackContainer<T>;
    /// Set container. See [`SetContainer`].
    type Set<T: Ord + Hash>: SetContainer<T>;
}

/// Trait abstracting ASCII text containers (strings).
pub trait TextContainer: StackContainer<u8> + Debug {
    /// If the string is empty.
    fn is_empty(&self) -> bool {
        StackContainer::is_empty(self)
    }
    /// Adds the ASCII byte `c` to the container.
    fn push_ascii(&mut self, c: u8) -> Result<(), StorageError>;
    /// Adds the ASCII bytes in `iter` to the container.
    fn push_ascii_iter(&mut self, iter: impl IntoIterator<Item = u8>) -> Result<(), StorageError> {
        for c in iter {
            self.push_ascii(c)?;
        }
        Ok(())
    }
    /// Iterator for the ASCII characters in the string.
    fn chars(&self) -> impl Iterator<Item = u8>;

    /// Converts an `&str` to a TextContainer.
    fn from_str(s: &str) -> Result<Self, StorageError> {
        let mut new = Self::new();
        new.push_ascii_iter(s.bytes())?;
        Ok(new)
    }
}
/// Trait abstracting queues.
pub trait QueueContainer<C>: IntoIterator<Item = C> + Sized + Default + Extend<C> {
    /// Creates an empty container.
    fn new() -> Self;
    /// The length of the queue.
    fn len(&self) -> usize;
    /// If the queue is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Enqueues an element onto the end of this queue.
    fn enqueue(&mut self, elem: C) -> Result<(), StorageError>;
    /// Dequeues an element off the front of this queue if there are things in the queue, returning it.
    fn dequeue(&mut self) -> Option<C>;
    /// Inspects the front of the queue, without dequeueing it.
    fn peek(&self) -> Option<&C>;
    /// Mutable ref to the front of the queue, without dequeueing it.
    fn peek_mut(&mut self) -> Option<&mut C>;
}
/// Trait abstracting `Vec`. idk vro
pub trait StackContainer<C>:
    FromIterator<C> + IntoIterator<Item = C> + Sized + Default + Extend<C>
{
    /// Creates an empty container.
    fn new() -> Self;
    /// The length of the stack.
    fn len(&self) -> usize;
    /// If the stack is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Pushes an element onto this stack.
    fn push(&mut self, elem: C) -> Result<(), StorageError>;
    /// Pops an element off this stack if there are things in the stack, returning it.
    fn pop(&mut self) -> Option<C>;
    /// Drains all the elements off the stack, starting from the bottom.
    fn drain_all(&mut self) -> impl Iterator<Item = C>;
    /// Produces a ref to all the elements on the stack, starting from the bottom.
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a C>
    where
        C: 'a;
    /// Produces a mutable ref to all the elements on the stack, starting from the bottom.
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut C>
    where
        C: 'a;
    /// If the container contains this element.
    fn contains(&self, elem: &C) -> bool
    where
        C: PartialEq;
}

/// Trait abstracting sets.
pub trait SetContainer<C>: Sized + Default + Extend<C> {
    /// Creates an empty set.
    fn new() -> Self;
    /// Iterator over the set.
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a C>
    where
        C: 'a;
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use alloc::collections::btree_set::BTreeSet;

    use crate::error::StorageError;
    use crate::storage::{QueueContainer, SetContainer, StackContainer, Storage, TextContainer};
    use core::hash::Hash;

    /// Tells [`crate::ui::LcdScreen`] and [`crate::ui::AsyncLcdScreen`] to use the `alloc` crate's
    /// containers.
    pub struct AllocStorage;
    impl Storage for AllocStorage {
        type Text = alloc::vec::Vec<u8>;
        type Queue<T> = alloc::collections::VecDeque<T>;
        type Vec<T> = alloc::vec::Vec<T>;
        type Set<T: Ord + Hash> = alloc::collections::BTreeSet<T>;
    }

    impl TextContainer for alloc::vec::Vec<u8> {
        fn push_ascii(&mut self, c: u8) -> Result<(), StorageError> {
            self.push(c);
            Ok(())
        }

        fn chars(&self) -> impl Iterator<Item = u8> {
            self.iter().copied()
        }
    }
    impl<T> StackContainer<T> for alloc::vec::Vec<T> {
        fn new() -> Self {
            alloc::vec::Vec::new()
        }

        fn len(&self) -> usize {
            self.len()
        }

        fn push(&mut self, elem: T) -> Result<(), StorageError> {
            self.push(elem);
            Ok(())
        }

        fn pop(&mut self) -> Option<T> {
            self.pop()
        }

        fn drain_all(&mut self) -> impl Iterator<Item = T> {
            self.drain(..)
        }

        fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
        where
            T: 'a,
        {
            self.as_mut_slice().iter_mut()
        }

        fn contains(&self, elem: &T) -> bool
        where
            T: PartialEq,
        {
            self.as_slice().contains(elem)
        }

        fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
        where
            T: 'a,
        {
            self.as_slice().iter()
        }
    }
    impl<T> QueueContainer<T> for alloc::collections::vec_deque::VecDeque<T> {
        fn new() -> Self {
            alloc::collections::vec_deque::VecDeque::new()
        }

        fn len(&self) -> usize {
            self.len()
        }

        fn enqueue(&mut self, elem: T) -> Result<(), StorageError> {
            self.push_back(elem);
            Ok(())
        }

        fn dequeue(&mut self) -> Option<T> {
            self.pop_front()
        }

        fn peek(&self) -> Option<&T> {
            self.front()
        }

        fn peek_mut(&mut self) -> Option<&mut T> {
            self.front_mut()
        }
    }
    impl<T: Ord + Hash> SetContainer<T> for alloc::collections::BTreeSet<T> {
        fn new() -> Self {
            BTreeSet::new()
        }

        fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
        where
            T: 'a,
        {
            self.iter()
        }
    }
}
#[cfg(feature = "heapless")]
mod heapless_impl {
    use core::hash::Hash;
    use heapless::{Deque, Vec, index_set::FnvIndexSet};

    use crate::{
        error::StorageError,
        storage::{QueueContainer, SetContainer, StackContainer, Storage, TextContainer},
    };

    /// Tells [`crate::ui::LcdScreen`] and [`crate::ui::AsyncLcdScreen`] to use the `heapless` crate's
    /// containers. the `MAX_CAPACITY` generic specifies the size for *all* containers, so it must
    /// be a power of 2 via [`heapless::index_set::FnvIndexSet`]'s constraints on its capacity.
    pub struct HeaplessStorage<const MAX_CAPACITY: usize>;
    impl<const MAX_CAPACITY: usize> Storage for HeaplessStorage<MAX_CAPACITY> {
        type Text = heapless::vec::Vec<u8, MAX_CAPACITY>;
        type Queue<T> = heapless::deque::Deque<T, MAX_CAPACITY>;
        type Vec<T> = heapless::vec::Vec<T, MAX_CAPACITY>;
        type Set<T: Ord + Hash> = heapless::index_set::FnvIndexSet<T, MAX_CAPACITY>;
    }

    impl<const S: usize> TextContainer for heapless::vec::Vec<u8, S> {
        fn push_ascii(&mut self, c: u8) -> Result<(), StorageError> {
            if self.push(c).is_err() {
                Err(StorageError::NotEnoughStorage)
            } else {
                Ok(())
            }
        }

        fn chars(&self) -> impl Iterator<Item = u8> {
            self.iter().copied()
        }
    }
    impl<const S: usize, T> StackContainer<T> for heapless::vec::Vec<T, S> {
        fn len(&self) -> usize {
            self.as_slice().len()
        }

        fn push(&mut self, elem: T) -> Result<(), StorageError> {
            if self.push(elem).is_err() {
                Err(StorageError::NotEnoughStorage)
            } else {
                Ok(())
            }
        }

        fn pop(&mut self) -> Option<T> {
            self.pop()
        }

        fn drain_all(&mut self) -> impl Iterator<Item = T> {
            self.drain(..)
        }

        fn new() -> Self {
            Vec::new()
        }

        fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
        where
            T: 'a,
        {
            self.as_slice().iter()
        }

        fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
        where
            T: 'a,
        {
            self.as_mut_slice().iter_mut()
        }

        fn contains(&self, elem: &T) -> bool
        where
            T: PartialEq,
        {
            self.as_slice().contains(elem)
        }
    }
    impl<const N: usize, T> QueueContainer<T> for heapless::deque::Deque<T, N> {
        fn len(&self) -> usize {
            self.len()
        }

        fn enqueue(&mut self, elem: T) -> Result<(), StorageError> {
            if self.push_back(elem).is_err() {
                Err(StorageError::NotEnoughStorage)
            } else {
                Ok(())
            }
        }

        fn dequeue(&mut self) -> Option<T> {
            self.pop_front()
        }

        fn peek(&self) -> Option<&T> {
            self.front()
        }

        fn new() -> Self {
            Deque::new()
        }

        fn peek_mut(&mut self) -> Option<&mut T> {
            self.front_mut()
        }
    }
    impl<const N: usize, T: Ord + Hash> SetContainer<T> for heapless::index_set::FnvIndexSet<T, N> {
        fn new() -> Self {
            FnvIndexSet::new()
        }

        fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
        where
            T: 'a,
        {
            self.iter()
        }
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impl::*;
#[cfg(feature = "heapless")]
pub use heapless_impl::*;
