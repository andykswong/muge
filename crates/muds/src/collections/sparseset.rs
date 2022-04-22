//! Sparse set.

use super::{IterableMap, IterableMapMut, Map, MapMut};
use crate::UnsignedNum;
use alloc::vec::Vec;
use core::{cmp, mem, ops};
use iter::{Drain, IntoIter, Iter, IterMut};

/// Represents a sparse index pointing to null
const NULL_INDEX: usize = usize::MAX;

/// The `SparseSet` is a type of associative array that uses unsigned integer as key.
/// It uses a dense and a sparse vector to map keys to elements.
#[derive(Clone, Debug)]
pub struct SparseSet<T, I: UnsignedNum = usize> {
    items: Vec<Entry<T, I>>,
    sparse: Vec<usize>,
}

/// `SparseSet` entry type
type Entry<T, I> = (I, T);

impl<T, I: UnsignedNum> SparseSet<T, I> {
    /// Constructs a new, empty `SparseSet`.
    /// It will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let set = SparseSet::<()>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Constructs a new, empty `SparseSet` with the specified capacity.
    /// It will be able to hold exactly capacity elements without reallocating.
    /// If capacity is 0, it will not allocate.
    ///
    /// # Panic
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let set = SparseSet::<()>::with_capacity(10);
    /// assert_eq!(set.capacity(), 10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut sparse = Vec::with_capacity(capacity);
        unsafe { sparse.set_len(capacity) }
        Self {
            items: Vec::with_capacity(capacity),
            sparse,
        }
    }

    /// Returns the number of elements the set can hold without reallocating.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let set = SparseSet::<()>::with_capacity(10);
    /// assert_eq!(set.capacity(), 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the given set.
    /// The collection may reserve more space to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<()>::new();
    /// set.reserve(10);
    /// assert!(set.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
        let min_sparse = self.items.len() + additional;
        if min_sparse > self.sparse.len() {
            self.reserve_sparse(min_sparse - self.sparse.len());
        }
    }

    /// Returns the number of elements in the set, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<()>::new();
    /// assert_eq!(set.len(), 0);
    /// set.insert(1, ());
    /// assert_eq!(set.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns a reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<i32>::new();
    /// set.insert(1, 123);
    ///
    /// assert_eq!(set.get(&1), Some(&123));
    /// set.remove(&1);
    /// assert!(set.get(&1).is_none());
    /// ```
    pub fn get(&self, i: &I) -> Option<&T> {
        if let Some(item_index) = self.get_item_index(i) {
            Some(&self.items[item_index].1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<i32>::new();
    /// set.insert(1, 123);
    ///
    /// *set.get_mut(&1).unwrap() += 1;
    /// assert_eq!(set.remove(&1), Some(124));
    /// assert!(set.get_mut(&1).is_none());
    /// ```
    pub fn get_mut(&mut self, i: &I) -> Option<&mut T> {
        if let Some(item_index) = self.get_item_index(i) {
            Some(&mut self.items[item_index].1)
        } else {
            None
        }
    }

    /// Clears the set, removing all values.
    /// Note that this method has no effect on the allocated capacity of the set.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<()>::new();
    /// set.insert(1, ());
    /// set.clear();
    /// assert!(set.len() == 0);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Inserts `value` into the set, allocating more capacity if necessary.
    /// The existing key-value in the set is returned.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<i32>::new();
    /// assert!(set.insert(1, 123).is_none());
    /// assert_eq!(set.insert(1, 456).unwrap(), 123);
    /// assert_eq!(*set.get(&1).unwrap(), 456);
    /// ```
    pub fn insert(&mut self, i: I, v: T) -> Option<T> {
        let sparse_index = i.to_usize()?;
        if sparse_index >= self.sparse.len() {
            self.reserve_sparse(sparse_index - self.sparse.len() + 1);
        }
        let item_index = self.sparse[sparse_index];

        match self.items.get_mut(item_index) {
            Some((index, value)) if i == *index => Some(mem::replace(value, v)),
            _ => {
                let new_item_index = self.items.len();
                self.items.push((i, v));
                self.sparse[sparse_index] = new_item_index;
                None
            }
        }
    }

    /// Removes and returns the element at index `i` from the set if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<i32>::new();
    /// set.insert(1, 123);
    /// set.insert(0, 456);
    ///
    /// assert_eq!(set.remove(&2), None);
    /// assert_eq!(set.remove(&1), Some(123));
    /// assert_eq!(set.remove(&1), None);
    /// ```
    pub fn remove(&mut self, i: &I) -> Option<T> {
        if let Some(item_index) = self.get_item_index(i) {
            let value = self.items.swap_remove(item_index).1;
            self.sparse[i.to_usize().unwrap()] = NULL_INDEX;
            if item_index < self.items.len() {
                let swapped_index = self.items[item_index].0.to_usize().unwrap();
                self.sparse[swapped_index] = item_index;
            }
            Some(value)
        } else {
            None
        }
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(index, value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<i32>::new();
    /// set.insert(1, 1);
    /// set.insert(0, 2);
    /// set.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(*set.get(&1).unwrap(), 3);
    /// assert!(set.get(&0).is_none());
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&I, &mut T) -> bool) {
        let mut i = 0;
        while i < self.items.len() {
            let (index, ref mut value) = self.items[i];
            if !f(&index, value) {
                // Item is swap-removed. Do not increment i here so that we can process the swapped item next
                self.remove(&index);
            } else {
                i += 1;
            }
        }
    }

    /// Returns an iterator over the set.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<usize>::new();
    /// for i in 0..10 {
    ///     set.insert(i, i * i);
    /// }
    ///
    /// for (idx, value) in &set {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    pub fn iter(&self) -> Iter<T, I> {
        Iter {
            inner: self.items.iter(),
        }
    }

    /// Returns an iterator that allows modifying each value over this set.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<usize>::new();
    /// for i in 0..10 {
    ///     set.insert(i, i * i);
    /// }
    ///
    /// for (_, value) in &mut set {
    ///     *value += 5;
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T, I> {
        IterMut {
            inner: self.items.iter_mut(),
        }
    }

    /// Creates a draining iterator that removes and yields all items in the set.
    /// When the iterator is dropped, all elements in the set are removed,
    /// even if the iterator was not fully consumed.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<i32>::new();
    /// set.insert(2, 1);
    /// set.insert(3, 2);
    /// assert!(set.get(&2).is_some());
    /// assert!(set.get(&3).is_some());
    ///
    /// {
    ///     let mut iter = set.drain();
    ///     let (idx, value) = iter.next().unwrap();
    ///     assert!(idx == 2 && value == 1);
    ///     let (idx, value) = iter.next().unwrap();
    ///     assert!(idx == 3 && value == 2);
    /// }
    ///
    /// assert!(set.get(&2).is_none());
    /// assert!(set.get(&3).is_none());
    /// assert!(set.len() == 0);
    /// ```
    pub fn drain(&mut self) -> Drain<T, I> {
        Drain {
            inner: self.items.drain(..),
        }
    }

    /// Sorts the set data with a comparator function.
    /// This sort is stable (i.e., does not reorder equal elements) and O(n * log(n)) worst-case.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<i32>::new();
    /// let (idx1, idx2, idx3) = (0, 5, 1);
    /// set.insert(idx1, 2);
    /// set.insert(idx2, 3);
    /// set.insert(idx3, 1);
    /// set.sort_by(|(_, v1), (_, v2)| { v1.cmp(v2) });
    ///
    /// let mut iter = set.iter();
    /// let (_, value) = iter.next().unwrap();
    /// assert_eq!(*value, 1);
    /// let (_, value) = iter.next().unwrap();
    /// assert_eq!(*value, 2);
    /// let (_, value) = iter.next().unwrap();
    /// assert_eq!(*value, 3);
    /// assert_eq!(set[idx1], 2);
    /// assert_eq!(set[idx2], 3);
    /// assert_eq!(set[idx3], 1);
    ///
    /// ```
    pub fn sort_by(&mut self, compare: impl FnMut(&Entry<T, I>, &Entry<T, I>) -> cmp::Ordering) {
        self.items.sort_by(compare);

        // Fix sparse array
        let mut item_index = 0;
        for (i, _) in &self.items {
            if let Some(sparse_index) = i.to_usize() {
                self.sparse[sparse_index] = item_index;
            }
            item_index += 1;
        }
    }

    fn get_item_index(&self, i: &I) -> Option<usize> {
        let sparse_index = i.to_usize()?;
        let item_index = *self.sparse.get(sparse_index)?;
        match self.items.get(item_index) {
            Some((index, _)) if *i == *index => Some(item_index),
            _ => None,
        }
    }

    #[inline]
    fn reserve_sparse(&mut self, additional: usize) {
        self.sparse.reserve(additional);
        unsafe { self.sparse.set_len(self.sparse.capacity()) }
    }
}

impl<V, I: UnsignedNum> Map for SparseSet<V, I> {
    type Key = I;
    type Value = V;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn get(&self, i: &Self::Key) -> Option<&Self::Value> {
        self.get(i)
    }
}

impl<V, I: UnsignedNum> MapMut for SparseSet<V, I> {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn get_mut(&mut self, i: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(i)
    }

    #[inline]
    fn insert(&mut self, i: Self::Key, v: Self::Value) -> Option<Self::Value> {
        self.insert(i, v)
    }

    #[inline]
    fn remove(&mut self, i: &Self::Key) -> Option<Self::Value> {
        self.remove(i)
    }

    #[inline]
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
        self.retain(f)
    }
}

impl<T, I: UnsignedNum> Default for SparseSet<T, I> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, I: UnsignedNum> ops::Index<I> for SparseSet<T, I> {
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.get(&index).expect("no entry found for index")
    }
}

impl<T, I: UnsignedNum> ops::IndexMut<I> for SparseSet<T, I> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(&index).expect("no entry found for index")
    }
}

impl<'a, T, I: UnsignedNum> IterableMap<'a> for SparseSet<T, I>
where
    Self: 'a,
{
    type Iter = Iter<'a, T, I>;

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'a, T, I: UnsignedNum> IterableMapMut<'a> for SparseSet<T, I>
where
    Self: 'a,
{
    type IterMut = IterMut<'a, T, I>;

    #[inline]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.into_iter()
    }
}

impl<T, I: UnsignedNum> IntoIterator for SparseSet<T, I> {
    type Item = (I, T);
    type IntoIter = IntoIter<T, I>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.items.into_iter(),
        }
    }
}

impl<'a, T, I: UnsignedNum> IntoIterator for &'a SparseSet<T, I> {
    type Item = (&'a I, &'a T);
    type IntoIter = Iter<'a, T, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, I: UnsignedNum> IntoIterator for &'a mut SparseSet<T, I> {
    type Item = (&'a I, &'a mut T);
    type IntoIter = IterMut<'a, T, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, I: UnsignedNum> FromIterator<(I, T)> for SparseSet<T, I> {
    fn from_iter<It: IntoIterator<Item = (I, T)>>(iter: It) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let cap = upper.unwrap_or(lower);
        let cap = cmp::max(cap, 1);
        let mut set = SparseSet::with_capacity(cap);
        set.extend(iter);
        set
    }
}

impl<T, I: UnsignedNum> Extend<(I, T)> for SparseSet<T, I> {
    fn extend<It: IntoIterator<Item = (I, T)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(i, t);
        }
    }
}

impl<'a, T, I> Extend<(&'a I, &'a T)> for SparseSet<T, I>
where
    I: UnsignedNum + 'a,
    T: Copy + 'a,
{
    fn extend<It: IntoIterator<Item = (&'a I, &'a T)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(*i, *t);
        }
    }
}

/// [SparseSet] iterators.
pub mod iter {
    use alloc::vec;
    use core::iter::FusedIterator;
    use core::slice;

    use super::Entry;
    use crate::UnsignedNum;

    /// An immutable iterator over an arena.
    /// This struct is created by the `iter` method on `SparseSet`.
    #[derive(Clone, Debug)]
    pub struct Iter<'a, T, I: UnsignedNum> {
        pub(super) inner: slice::Iter<'a, Entry<T, I>>,
    }

    /// An mutable iterator over an arena.
    /// This struct is created by the `iter_mut` method on `SparseSet`.
    #[derive(Debug)]
    pub struct IterMut<'a, T, I: UnsignedNum> {
        pub(super) inner: slice::IterMut<'a, Entry<T, I>>,
    }

    /// An iterator that moves out of a `SparseSet`.
    /// This struct is created by the `into_iter` method on `SparseSet` (provided by the IntoIterator trait).
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{SparseSet};
    /// let mut set = SparseSet::<usize>::new();
    /// for i in 0..10 {
    ///     set.insert(i, i * i);
    /// }
    ///
    /// for (idx, value) in set {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    /// ```
    #[derive(Debug)]
    pub struct IntoIter<T, I: UnsignedNum> {
        pub(super) inner: vec::IntoIter<Entry<T, I>>,
    }

    /// A draining iterator for an arena.
    /// This struct is created by the `drain` method on `SparseSet`.
    #[derive(Debug)]
    pub struct Drain<'a, T, I: UnsignedNum> {
        pub(super) inner: vec::Drain<'a, Entry<T, I>>,
    }

    /// Implement all Iterator traits for an arena iterator.
    macro_rules! impl_iter_traits {
        ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?, $item:ty ) => {
            impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
                Iterator
            for $name $(< $( $lt ),+ >)?
            {
                type Item = $item;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.inner.next() {
                        Some((index, value)) => Some((index, value)),
                        None => None,
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.inner.len(), Some(self.inner.len()))
                }
            }

            impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
                DoubleEndedIterator
            for $name $(< $( $lt ),+ >)?
            {
                fn next_back(&mut self) -> Option<Self::Item> {
                    match self.inner.next_back() {
                        Some((index, value)) => Some((index, value)),
                        None => None,
                    }
                }
            }

            impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
                ExactSizeIterator
            for $name $(< $( $lt ),+ >)?
            {
                fn len(&self) -> usize {
                    self.inner.len()
                }
            }

            impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
                FusedIterator
            for $name $(< $( $lt ),+ >)?
            {}
        }
    }

    impl_iter_traits!(Iter<'a, T, I: UnsignedNum>, (&'a I, &'a T));

    impl_iter_traits!(IterMut<'a, T, I: UnsignedNum>, (&'a I, &'a mut T));

    impl_iter_traits!(IntoIter<T, I: UnsignedNum>, (I, T));

    impl_iter_traits!(Drain<'a, T, I: UnsignedNum>, (I, T));
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::{Entry, SparseSet};
    use crate::UnsignedNum;
    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T, I> Serialize for SparseSet<T, I>
    where
        T: Serialize,
        I: UnsignedNum + Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.items.serialize(serializer)
        }
    }

    impl<'de, T, I> Deserialize<'de> for SparseSet<T, I>
    where
        T: Deserialize<'de>,
        I: Deserialize<'de> + UnsignedNum,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let items: Vec<Entry<T, I>> = Deserialize::deserialize(deserializer)?;
            let mut result = SparseSet::new();
            result.extend(items);
            Ok(result)
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;
    use serde_json::{json, Value};

    use super::SparseSet;

    #[test]
    fn test_serialize() {
        let mut set = SparseSet::<&str>::with_capacity(3);
        set.insert(1, "a");
        set.insert(0, "b");
        set.insert(4, "c");

        let expected_json: Value = json!([[1, "a"], [0, "b"], [4, "c"]]);

        let json: Value = serde_json::to_value(set).unwrap();

        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_deserialize() {
        let json: Value = json!([[1, "a"], [3, "c"]]);

        let set: SparseSet<String> = serde_json::from_value(json).unwrap();

        assert_eq!(set.len(), 2);
        assert_eq!(set[1], "a");
        assert_eq!(set[3], "c");
    }
}
