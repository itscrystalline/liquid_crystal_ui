//! Storage adapter.

/// Trait abstracting ASCII text containers (strings).
pub trait TextContainer: StackContainer<u8> {
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

    /// Converts an `&str` to a TextContainer.
    fn from_str(s: &str) -> Result<Self, Self::Error> {
        let mut new = Self::new();
        new.push_ascii_iter(s.bytes())?;
        Ok(new)
    }
}
/// Trait abstracting queues.
pub trait QueueContainer<C>: IntoIterator<Item = C> + Sized {
    /// The error type this container may emit.
    type Error;

    /// Creates an empty container.
    fn new() -> Self;
    /// The length of the queue.
    fn len(&self) -> usize;
    /// If the queue is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Enqueues an element onto the end of this queue.
    fn enqueue(&mut self, elem: C) -> Result<(), Self::Error>;
    /// Dequeues an element off the front of this queue if there are things in the queue, returning it.
    fn dequeue(&mut self) -> Option<C>;
    /// Inspects the front of the queue, without dequeueing it.
    fn peek(&self) -> Option<&C>;
    /// Mutable ref to the front of the queue, without dequeueing it.
    fn peek_mut(&mut self) -> Option<&mut C>;

    /// Enqueues elements in an iterator onto the end of this queue.
    fn enqueue_iter(
        &mut self,
        elems: impl IntoIterator<Item = C>,
    ) -> Result<(), (Self::Error, usize)> {
        for (idx, elem) in elems.into_iter().enumerate() {
            self.enqueue(elem).map_err(|e| (e, idx))?
        }
        Ok(())
    }
}
/// Trait abstracting `Vec`. idk vro
pub trait StackContainer<C>: IntoIterator<Item = C> + Sized {
    /// The error type this satck may emit.
    type Error;

    /// Creates an empty container.
    fn new() -> Self;
    /// The length of the stack.
    fn len(&self) -> usize;
    /// If the stack is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Pushes an element onto this stack.
    fn push(&mut self, elem: C) -> Result<(), Self::Error>;
    /// Pops an element off this stack if there are things in the stack, returning it.
    fn pop(&mut self) -> Option<C>;
    /// Drains all the elements off the stack, starting from the start.
    fn drain_all(&mut self) -> impl Iterator<Item = C>;

    /// Enstacks elements in an iterator onto the end of this queue.
    fn push_iter(
        &mut self,
        elems: impl IntoIterator<Item = C>,
    ) -> Result<(), (Self::Error, usize)> {
        for (idx, elem) in elems.into_iter().enumerate() {
            self.push(elem).map_err(|e| (e, idx))?
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use crate::storage::{QueueContainer, StackContainer, TextContainer};
    use core::convert::Infallible;

    impl TextContainer for alloc::vec::Vec<u8> {
        fn push_ascii(&mut self, c: u8) -> Result<(), Self::Error> {
            self.push(c);
            Ok(())
        }

        fn chars(&self) -> impl Iterator<Item = u8> {
            self.iter().copied()
        }
    }
    impl<T> StackContainer<T> for alloc::vec::Vec<T> {
        type Error = Infallible;

        fn new() -> Self {
            alloc::vec::Vec::new()
        }

        fn len(&self) -> usize {
            self.len()
        }

        fn push(&mut self, elem: T) -> Result<(), Self::Error> {
            self.push(elem);
            Ok(())
        }

        fn pop(&mut self) -> Option<T> {
            self.pop()
        }

        fn drain_all(&mut self) -> impl Iterator<Item = T> {
            self.drain(..)
        }
    }
    impl<T> QueueContainer<T> for alloc::collections::vec_deque::VecDeque<T> {
        type Error = Infallible;

        fn new() -> Self {
            alloc::collections::vec_deque::VecDeque::new()
        }

        fn len(&self) -> usize {
            self.len()
        }

        fn enqueue(&mut self, elem: T) -> Result<(), Self::Error> {
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
}
#[cfg(feature = "heapless")]
mod heapless_impl {
    use crate::{
        error::StorageError,
        storage::{QueueContainer, StackContainer, TextContainer},
    };

    impl<const S: usize> TextContainer for heapless::vec::Vec<u8, S> {
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
    impl<const S: usize, T> StackContainer<T> for heapless::vec::Vec<T, S> {
        type Error = StorageError;

        fn len(&self) -> usize {
            self.len()
        }

        fn push(&mut self, elem: T) -> Result<(), Self::Error> {
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
    }
    impl<const N: usize, T> QueueContainer<T> for heapless::deque::Deque<T, N> {
        type Error = StorageError;

        fn len(&self) -> usize {
            self.len()
        }

        fn enqueue(&mut self, elem: T) -> Result<(), Self::Error> {
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
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impl::*;
#[cfg(feature = "heapless")]
pub use heapless_impl::*;
