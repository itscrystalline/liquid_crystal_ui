//! Storage adapters.
//!
//! ### Usage with custom containers
//! If the [`AllocStorage`] or [`HeaplessStorage`] adapters don't suit your usecase e.g. you have
//! custom buffers that you want to use as backing storage, you will have to create an adapter
//! struct, then implement the [`Storage`] trait on it. For the associated types in the [`Storage`]
//! trait, you must also implement [`TextContainer`], [`QueueContainer`] and [`StackContainer`]
//! on those types as well. See impls for [`AllocStorage`] or [`HeaplessStorage`]
//! for examples.

use crate::error::StorageError;
use core::fmt::Debug;

/// Trait describing a family of storage containers to use.
pub trait Storage {
    /// ASCII String container. See [`TextContainer`].
    type Text: TextContainer;
    /// Queue container. See [`QueueContainer`].
    type Queue<T>: QueueContainer<T>;
    /// Stack (Vec-like) container. See [`StackContainer`].
    type Vec<T>: StackContainer<T>;
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
    /// Shrinks the stack's allocation to only fit its contents. Use a blank impl if your storage
    /// does not allocate.
    fn minimize(&mut self);
    /// Clears the stack.
    fn clear(&mut self);
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
    /// Clears the stack.
    fn clear(&mut self);
    /// Shrinks the stack's allocation to only fit its contents. Use a blank impl if your storage
    /// does not allocate.
    fn minimize(&mut self);
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
    /// Removes the item at `idx`, returning it while shifting later elements down. Returns `None`
    /// if `idx` is out of range.
    fn remove(&mut self, idx: usize) -> Option<C>;
    /// Removes the item at `idx`, returning it and putting the last element in the vacent spot. Returns `None`
    /// if `idx` is out of range.
    fn swap_remove(&mut self, idx: usize) -> Option<C>;
    /// Sorts and removes consecutive duplicate elements.
    fn sort_dedup(&mut self)
    where
        C: PartialEq + Ord;

    /// Removes the first occurence of the item `elem` if it is in the stack, while shifting later elements down.
    fn remove_elem(&mut self, elem: &C) -> Option<C>
    where
        C: PartialEq,
    {
        let idx = self
            .iter()
            .enumerate()
            .find(|&(_, e)| *e == *elem)
            .map(|(idx, _)| idx)?;
        self.remove(idx)
    }
    /// Removes the first occurence of the item `elem` if it is in the stack, while moving the last element to take it's place.
    fn swap_remove_elem(&mut self, elem: &C) -> Option<C>
    where
        C: PartialEq,
    {
        let idx = self
            .iter()
            .enumerate()
            .find(|&(_, e)| *e == *elem)
            .map(|(idx, _)| idx)?;
        self.swap_remove(idx)
    }
}

#[cfg(feature = "alloc")]
mod alloc_impl {
    use crate::error::StorageError;
    use crate::storage::{QueueContainer, StackContainer, Storage, TextContainer};

    /// Tells [`LcdScreen`](`crate::ui::LcdScreen`) and [`AsyncLcdScreen`](`crate::ui::AsyncLcdScreen`) to use the [`alloc`] crate's
    /// containers.
    pub struct AllocStorage;
    impl Storage for AllocStorage {
        type Text = alloc::vec::Vec<u8>;
        type Queue<T> = alloc::collections::VecDeque<T>;
        type Vec<T> = alloc::vec::Vec<T>;
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

        fn remove(&mut self, idx: usize) -> Option<T> {
            (idx < self.len()).then_some(self.remove(idx))
        }
        fn swap_remove(&mut self, idx: usize) -> Option<T> {
            (idx < self.len()).then_some(self.swap_remove(idx))
        }

        fn sort_dedup(&mut self)
        where
            T: PartialEq + Ord,
        {
            self.sort();
            self.dedup();
        }

        fn clear(&mut self) {
            self.clear();
        }

        fn minimize(&mut self) {
            self.shrink_to_fit();
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

        fn clear(&mut self) {
            self.clear();
        }

        fn minimize(&mut self) {
            self.shrink_to_fit();
        }
    }
}
#[cfg(feature = "heapless")]
mod heapless_impl {
    use heapless::{Deque, Vec};

    use crate::{
        error::StorageError,
        storage::{QueueContainer, StackContainer, Storage, TextContainer},
    };

    /// Tells [`LcdScreen`](`crate::ui::LcdScreen`) and [`AsyncLcdScreen`](`crate::ui::AsyncLcdScreen`) to use the [`heapless`] crate's
    /// containers. the `MAX_CAPACITY` generic specifies the size for *all* containers.  
    pub struct HeaplessStorage<const MAX_CAPACITY: usize>;
    impl<const MAX_CAPACITY: usize> Storage for HeaplessStorage<MAX_CAPACITY> {
        type Text = heapless::vec::Vec<u8, MAX_CAPACITY>;
        type Queue<T> = heapless::deque::Deque<T, MAX_CAPACITY>;
        type Vec<T> = heapless::vec::Vec<T, MAX_CAPACITY>;
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
        fn new() -> Self {
            Vec::new()
        }

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

        fn remove(&mut self, idx: usize) -> Option<T> {
            (idx < self.len()).then_some(self.remove(idx))
        }
        fn swap_remove(&mut self, idx: usize) -> Option<T> {
            (idx < self.len()).then_some(self.swap_remove(idx))
        }
        fn sort_dedup(&mut self)
        where
            T: PartialEq + Ord,
        {
            quicksort_ord(self.as_mut_slice());
            let len = {
                let (deduped, _) = {
                    let this = &mut *self;
                    partition_dedup_by(this, |a, b| a == b)
                };
                deduped.len()
            };
            self.truncate(len);
        }

        fn clear(&mut self) {
            self.clear();
        }

        fn minimize(&mut self) {}
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

        fn clear(&mut self) {
            self.clear();
        }

        fn minimize(&mut self) {}
    }

    fn quicksort_ord<T>(arr: &mut [T])
    where
        T: Ord,
    {
        if arr.len() <= 1 {
            return;
        }

        fn i<T>(arr: &mut [T]) -> usize
        where
            T: Ord,
        {
            let len = arr.len();
            let mut i = 0;
            for j in 0..len - 1 {
                if arr[j] <= arr[len - 1] {
                    arr.swap(i, j);
                    i += 1;
                }
            }

            arr.swap(i, len - 1);
            i
        }

        let i = i(arr);
        quicksort_ord(&mut arr[0..i]);
        quicksort_ord(&mut arr[i + 1..]);
    }

    // yoinked from stdlib https://github.com/rust-lang/rust/issues/54279
    fn partition_dedup_by<F, T>(this: &mut [T], mut same_bucket: F) -> (&mut [T], &mut [T])
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        // Although we have a mutable reference to `self`, we cannot make
        // *arbitrary* changes. The `same_bucket` calls could panic, so we
        // must ensure that the slice is in a valid state at all times.
        //
        // The way that we handle this is by using swaps; we iterate
        // over all the elements, swapping as we go so that at the end
        // the elements we wish to keep are in the front, and those we
        // wish to reject are at the back. We can then split the slice.
        // This operation is still `O(n)`.
        //
        // Example: We start in this state, where `r` represents "next
        // read" and `w` represents "next_write".
        //
        //           r
        //     +---+---+---+---+---+---+
        //     | 0 | 1 | 1 | 2 | 3 | 3 |
        //     +---+---+---+---+---+---+
        //           w
        //
        // Comparing self[r] against self[w-1], this is not a duplicate, so
        // we swap self[r] and self[w] (no effect as r==w) and then increment both
        // r and w, leaving us with:
        //
        //               r
        //     +---+---+---+---+---+---+
        //     | 0 | 1 | 1 | 2 | 3 | 3 |
        //     +---+---+---+---+---+---+
        //               w
        //
        // Comparing self[r] against self[w-1], this value is a duplicate,
        // so we increment `r` but leave everything else unchanged:
        //
        //                   r
        //     +---+---+---+---+---+---+
        //     | 0 | 1 | 1 | 2 | 3 | 3 |
        //     +---+---+---+---+---+---+
        //               w
        //
        // Comparing self[r] against self[w-1], this is not a duplicate,
        // so swap self[r] and self[w] and advance r and w:
        //
        //                       r
        //     +---+---+---+---+---+---+
        //     | 0 | 1 | 2 | 1 | 3 | 3 |
        //     +---+---+---+---+---+---+
        //                   w
        //
        // Not a duplicate, repeat:
        //
        //                           r
        //     +---+---+---+---+---+---+
        //     | 0 | 1 | 2 | 3 | 1 | 3 |
        //     +---+---+---+---+---+---+
        //                       w
        //
        // Duplicate, advance r. End of slice. Split at w.

        let len = this.len();
        if len <= 1 {
            return (this, &mut []);
        }

        let ptr = this.as_mut_ptr();
        let mut next_read: usize = 1;
        let mut next_write: usize = 1;

        // SAFETY: the `while` condition guarantees `next_read` and `next_write`
        // are less than `len`, thus are inside `self`. `prev_ptr_write` points to
        // one element before `ptr_write`, but `next_write` starts at 1, so
        // `prev_ptr_write` is never less than 0 and is inside the slice.
        // This fulfils the requirements for dereferencing `ptr_read`, `prev_ptr_write`
        // and `ptr_write`, and for using `ptr.add(next_read)`, `ptr.add(next_write - 1)`
        // and `prev_ptr_write.offset(1)`.
        //
        // `next_write` is also incremented at most once per loop at most meaning
        // no element is skipped when it may need to be swapped.
        //
        // `ptr_read` and `prev_ptr_write` never point to the same element. This
        // is required for `&mut *ptr_read`, `&mut *prev_ptr_write` to be safe.
        // The explanation is simply that `next_read >= next_write` is always true,
        // thus `next_read > next_write - 1` is too.
        unsafe {
            // Avoid bounds checks by using raw pointers.
            while next_read < len {
                let ptr_read = ptr.add(next_read);
                let prev_ptr_write = ptr.add(next_write - 1);
                if !same_bucket(&mut *ptr_read, &mut *prev_ptr_write) {
                    if next_read != next_write {
                        let ptr_write = prev_ptr_write.add(1);
                        core::ptr::swap(ptr_read, ptr_write);
                    }
                    next_write += 1;
                }
                next_read += 1;
            }
        }

        this.split_at_mut(next_write)
    }
}

#[cfg(feature = "alloc")]
pub use alloc_impl::*;
#[cfg(feature = "heapless")]
pub use heapless_impl::*;
