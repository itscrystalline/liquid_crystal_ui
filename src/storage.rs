//! Storage adapter.

/// Trait abstracting ASCII text containers (strings).
pub trait StorableText: StackContainer {
    /// The length of text in ASCII characters.
    fn len(&self) -> usize {
        StackContainer::len(self)
    }
    /// If the string is empty.
    fn is_empty(&self) -> bool {
        StackContainer::is_empty(self)
    }
    /// Adds the ASCII byte `c` to the container.
    fn push_ascii(&mut self, c: u8) -> Result<(), Self::Error>;
    /// Adds the ASCII bytes in `iter` to the container.
    fn push_ascii_iter(&mut self, iter: impl IntoIterator<Item = u8>) -> Result<(), Self::Error> {
        for c in iter {
            self.push_ascii(c)?;
        }
        Ok(())
    }
    /// Iterator for the ASCII characters in the string.
    fn chars(&self) -> impl Iterator<Item = u8>;
}
/// Trait abstracting queues.
pub trait QueueContainer: IntoIterator<Item = Self::Content> {
    /// What this queue is storing.
    // duh, im lowk getting tired of doccing all ts
    type Content;
    /// The error type this container may emit.
    type Error;

    /// The length of the queue.
    fn len(&self) -> usize;
    /// If the queue is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Enqueues an element onto the end of this queue.
    fn enqueue(&mut self, elem: Self::Content) -> Result<(), Self::Error>;
    /// Dequeues an element off the front of this queue if there are things in the queue, returning it.
    fn dequeue(&mut self) -> Option<Self::Content>;
    /// Inspects the front of the queue, without dequeueing it.
    fn peek(&self) -> Option<&Self::Content>;

    /// Enqueues elements in an iterator onto the end of this queue.
    fn enqueue_iter(
        &mut self,
        elems: impl IntoIterator<Item = Self::Content>,
    ) -> Result<(), (Self::Error, usize)> {
        for (idx, elem) in elems.into_iter().enumerate() {
            self.enqueue(elem).map_err(|e| (e, idx))?
        }
        Ok(())
    }
}
/// Trait abstracting `Vec`. idk vro
pub trait StackContainer: IntoIterator<Item = Self::Content> {
    /// What this stack is storing.
    type Content;
    /// The error type this satck may emit.
    type Error;

    /// The length of the stack.
    fn len(&self) -> usize;
    /// If the stack is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Pushes an element onto this stack.
    fn push(&mut self, elem: Self::Content) -> Result<(), Self::Error>;
    /// Pops an element off this stack if there are things in the stack, returning it.
    fn pop(&mut self) -> Option<Self::Content>;
    /// Drains all the elements off the stack, starting from the start.
    fn drain_all(&mut self) -> impl Iterator<Item = Self::Content>;

    /// Enstacks elements in an iterator onto the end of this queue.
    fn push_iter(
        &mut self,
        elems: impl IntoIterator<Item = Self::Content>,
    ) -> Result<(), (Self::Error, usize)> {
        for (idx, elem) in elems.into_iter().enumerate() {
            self.push(elem).map_err(|e| (e, idx))?
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use crate::storage::{QueueContainer, StackContainer, StorableText};
    use core::convert::Infallible;

    impl StorableText for alloc::vec::Vec<u8> {
        fn len(&self) -> usize {
            self.len()
        }

        fn push_ascii(&mut self, c: u8) -> Result<(), Self::Error> {
            self.push(c);
            Ok(())
        }

        fn chars(&self) -> impl Iterator<Item = u8> {
            self.iter().copied()
        }
    }
    impl<T> StackContainer for alloc::vec::Vec<T> {
        type Content = T;
        type Error = Infallible;

        fn len(&self) -> usize {
            self.len()
        }

        fn push(&mut self, elem: Self::Content) -> Result<(), Self::Error> {
            self.push(elem);
            Ok(())
        }

        fn pop(&mut self) -> Option<Self::Content> {
            self.pop()
        }

        fn drain_all(&mut self) -> impl Iterator<Item = Self::Content> {
            self.drain(..)
        }
    }
    impl<T> QueueContainer for alloc::collections::vec_deque::VecDeque<T> {
        type Content = T;
        type Error = Infallible;

        fn len(&self) -> usize {
            self.len()
        }

        fn enqueue(&mut self, elem: Self::Content) -> Result<(), Self::Error> {
            self.push_back(elem);
            Ok(())
        }

        fn dequeue(&mut self) -> Option<Self::Content> {
            self.pop_front()
        }

        fn peek(&self) -> Option<&Self::Content> {
            self.front()
        }
    }
}
#[cfg(feature = "heapless")]
mod heapless_impl {
    use crate::{
        error::StorageError,
        storage::{QueueContainer, StackContainer, StorableText},
    };

    impl<const S: usize> StorableText for heapless::vec::Vec<u8, S> {
        fn push_ascii(&mut self, c: u8) -> Result<(), Self::Error> {
            if StackContainer::push(&mut self, c).is_err() {
                Err(StorageError::NotEnoughStorage)
            } else {
                Ok(())
            }
        }

        fn chars(&self) -> impl Iterator<Item = u8> {
            self.iter().copied()
        }
    }
    impl<const S: usize, T> StackContainer for heapless::vec::Vec<T, S> {
        type Error = StorageError;
        type Content = T;

        fn len(&self) -> usize {
            self.len()
        }

        fn push(&mut self, elem: Self::Content) -> Result<(), Self::Error> {
            if self.push(elem).is_err() {
                Err(StorageError::NotEnoughStorage)
            } else {
                Ok(())
            }
        }

        fn pop(&mut self) -> Option<Self::Content> {
            self.pop()
        }

        fn drain_all(&mut self) -> impl Iterator<Item = Self::Content> {
            self.drain(..)
        }
    }
    impl<const N: usize, T> QueueContainer for heapless::deque::Deque<T, N> {
        type Content = T;
        type Error = StorageError;

        fn len(&self) -> usize {
            self.len()
        }

        fn enqueue(&mut self, elem: Self::Content) -> Result<(), Self::Error> {
            if self.push_back(elem).is_err() {
                Err(StorageError::NotEnoughStorage)
            } else {
                Ok(())
            }
        }

        fn dequeue(&mut self) -> Option<Self::Content> {
            self.pop_front()
        }

        fn peek(&self) -> Option<&Self::Content> {
            self.front()
        }
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impl::*;
#[cfg(feature = "heapless")]
pub use heapless_impl::*;
