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

    /// Converts an `&str` to a TextContainer. If non ASCII characters are passed in, they will be
    /// converted to the extended ASCII charset the display supports[^source]. If the character does
    /// not have an equivalent extended ASCII character, it will be replaced with a '?'
    ///
    /// [^source]: [HD44780U data sheet](https://cdn.sparkfun.com/assets/9/5/f/7/b/HD44780.pdf)
    fn from_str(s: &str) -> Result<Self, StorageError> {
        let mut new = Self::new();
        new.push_ascii_iter(s.chars().map(|c| char_to_ext_ascii_byte(c, b'?')))?;
        Ok(new)
    }
    /// Similar to [`TextContainer::from_str`], but allows you to set a custom replacement character.
    fn from_utf8_replace(s: &str, replacement: u8) -> Result<Self, StorageError> {
        let mut new = Self::new();
        new.push_ascii_iter(s.chars().map(|c| char_to_ext_ascii_byte(c, replacement)))?;
        Ok(new)
    }
}

fn char_to_ext_ascii_byte(ch: char, replacement: u8) -> u8 {
    if ch.is_ascii() {
        ch as u8
    } else {
        // https://cdn.sparkfun.com/assets/9/5/f/7/b/HD44780.pdf
        // page 17 (japanese version), table 4
        match ch {
            '→' => 0b01111110,
            '←' => 0b01111111,
            '。' => 0b10100001,
            '「' => 0b10100010,
            '」' => 0b10100011,
            '、' => 0b10100100,
            '・' => 0b10100101,
            'ヲ' => 0b10100110,
            'ァ' => 0b10100111,
            'ィ' => 0b10101000,
            'ゥ' => 0b10101001,
            'ェ' => 0b10101010,
            'ォ' => 0b10101011,
            'ャ' => 0b10101100,
            'ュ' => 0b10101101,
            'ョ' => 0b10101110,
            'ッ' => 0b10101111,
            'ー' => 0b10110000,
            'ア' => 0b10110001,
            'イ' => 0b10110010,
            'ウ' => 0b10110011,
            'エ' => 0b10110100,
            'オ' => 0b10110101,
            'カ' => 0b10110110,
            'キ' => 0b10110111,
            'ク' => 0b10111000,
            'ケ' => 0b10111001,
            'コ' => 0b10111010,
            'サ' => 0b10111011,
            'シ' => 0b10111100,
            'ス' => 0b10111101,
            'セ' => 0b10111110,
            'ソ' => 0b10111111,
            'タ' => 0b11000000,
            'チ' => 0b11000001,
            'ツ' => 0b11000010,
            'テ' => 0b11000011,
            'ト' => 0b11000100,
            'ナ' => 0b11000101,
            'ニ' => 0b11000110,
            'ヌ' => 0b11000111,
            'ネ' => 0b11001000,
            'ノ' => 0b11001001,
            'ハ' => 0b11001010,
            'ヒ' => 0b11001011,
            'フ' => 0b11001100,
            'ヘ' => 0b11001101,
            'ホ' => 0b11001110,
            'マ' => 0b11001111,
            'ミ' => 0b11010000,
            'ム' => 0b11010001,
            'メ' => 0b11010010,
            'モ' => 0b11010011,
            'ヤ' => 0b11010100,
            'ユ' => 0b11010101,
            'ヨ' => 0b11010110,
            'ラ' => 0b11010111,
            'リ' => 0b11011000,
            'ル' => 0b11011001,
            'レ' => 0b11011010,
            'ロ' => 0b11011011,
            'ワ' => 0b11011100,
            'ン' => 0b11011101,
            '゛' => 0b11011110,
            '゜' => 0b11011111,
            'α' => 0b11100000,
            'ä' => 0b11100001,
            'β' => 0b11100010,
            'ε' => 0b11100011,
            'μ' => 0b11100100,
            'σ' => 0b11100101,
            'ρ' => 0b11100110,
            // 'g' => 0b11100111, // long g
            '√' => 0b11101000,
            // 'j' => 0b11101001, // long j
            '※' => 0b11101010,
            '¢' => 0b11101011,
            'Ł' => 0b11101100,
            'ñ' => 0b11101101,
            'ö' => 0b11101110,
            // 'p' => 0b11101111, // long p
            // 'q' => 0b11110000, // long q
            'θ' => 0b11110001,
            '∞' => 0b11110010,
            'Ω' => 0b11110011,
            'ü' => 0b11110100,
            'Σ' => 0b11110101,
            'π' => 0b11110110,
            // 'x' => 0b11110111, // x-bar, needs 2 chars
            // 'y' => 0b11111000, // y with overline, also needs 2 chars
            '千' => 0b11111001,
            '万' => 0b11111010,
            '円' => 0b11111011,
            '÷' => 0b11111100,
            '■' => 0b11111111, // filled block

            _ => replacement,
        }
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
