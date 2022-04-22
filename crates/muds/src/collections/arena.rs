//! Generational index arena.
use super::{Arena, GenIndexAllocator, IterableMap, IterableMapMut, Map, VecMap};
use crate::{GenIndex, IndexF64};
use core::ops;

/// The `GenIndexArena` holds elements that are referenced to by [GenIndex].
/// It conveniently combines [GenIndexAllocator] and [VecMap] into a single container.
#[derive(Clone, Debug)]
pub struct GenIndexArena<T, I: GenIndex = IndexF64> {
    indices: GenIndexAllocator<I>,
    items: VecMap<T, I::Index>,
}

impl<T, I: GenIndex> GenIndexArena<T, I> {
    /// Constructs a new, empty [GenIndexArena].
    /// The arena will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let arena = GenIndexArena::<()>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Constructs a new, empty `GenIndexArena` with the specified capacity.
    /// The arena will be able to hold exactly capacity elements without reallocating.
    /// If capacity is 0, the arena will not allocate.
    ///
    /// # Panic
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let arena = GenIndexArena::<()>::with_capacity(10);
    /// assert_eq!(arena.capacity(), 10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            indices: GenIndexAllocator::with_capacity(capacity),
            items: VecMap::with_capacity(capacity),
        }
    }

    /// Returns the number of elements in the arena, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<()>::new();
    /// assert_eq!(arena.len(), 0);
    /// arena.insert(());
    /// assert_eq!(arena.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// Returns `true` if the arena contains no elements.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<()>::new();
    /// assert!(arena.is_empty());
    /// arena.insert(());
    /// assert!(!arena.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements the arena can hold without reallocating.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let arena = GenIndexArena::<()>::with_capacity(10);
    /// assert_eq!(arena.capacity(), 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.indices.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the given arena.
    /// The collection may reserve more space to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<()>::new();
    /// arena.reserve(10);
    /// assert!(arena.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional);
        self.items.reserve(additional);
    }

    /// Clears the arena, removing all values.
    /// Note that this method has no effect on the allocated capacity of the arena.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// # let mut arena = GenIndexArena::<()>::new();
    /// arena.insert(());
    /// arena.insert(());
    /// arena.clear();
    /// assert!(arena.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.indices.clear();
        self.items.clear();
    }

    /// Inserts `value` into the arena, allocating more capacity if necessary.
    /// The `value`'s assigned index in the arena is returned.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<i32>::new();
    /// let idx = arena.insert(123);
    /// assert_eq!(arena[idx], 123);
    /// ```
    pub fn insert(&mut self, value: T) -> I {
        let i = self.indices.create();
        self.items.insert(i.index(), value);
        return i;
    }

    /// Removes and returns the element at `key` from the arena if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<i32>::new();
    /// let i = &arena.insert(123);
    /// assert_eq!(arena.remove(i), Some(123));
    /// assert_eq!(arena.remove(i), None);
    /// ```
    pub fn remove(&mut self, key: &I) -> Option<T> {
        if self.indices.remove(key) {
            self.items.remove(&key.index())
        } else {
            None
        }
    }

    /// Returns a reference to the value at `key`.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<i32>::new();
    /// let idx = &arena.insert(123);
    ///
    /// assert_eq!(arena.get(idx), Some(&123));
    /// arena.remove(idx);
    /// assert!(arena.get(idx).is_none());
    /// ```
    #[inline]
    pub fn get(&self, key: &I) -> Option<&T> {
        self.items.get(&key.index())
    }

    /// Returns a mutable reference to the value at `key`.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<i32>::new();
    /// let idx = &arena.insert(123);
    ///
    /// *arena.get_mut(idx).unwrap() += 1;
    /// assert_eq!(arena.remove(idx), Some(124));
    /// assert!(arena.get_mut(idx).is_none());
    /// ```
    #[inline]
    pub fn get_mut(&mut self, key: &I) -> Option<&mut T> {
        self.items.get_mut(&key.index())
    }

    /// Returns true if the arena contains a value at `key`.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<i32>::new();
    /// let idx = &arena.insert(123);
    /// assert!(arena.contains_key(idx));
    /// arena.remove(idx);
    /// assert!(!arena.contains_key(idx));
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &I) -> bool {
        self.get(key).is_some()
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(key, &value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use muds::{GenIndex, Index, collections::GenIndexArena};
    /// let mut arena = GenIndexArena::<i32>::new();
    /// let idx1 = &arena.insert(1);
    /// let idx2 = &arena.insert(2);
    /// arena.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(*arena.get(idx1).unwrap(), 3);
    /// assert!(arena.get(idx2).is_none());
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&I, &mut T) -> bool) {
        self.items.retain(|i, t| {
            let mut retain = false;
            match self.indices.get(i) {
                Some(idx) if !f(idx, t) => Some(*idx),
                Some(_) => {
                    retain = true;
                    None
                }
                _ => None,
            }
            .map(|ref idx| self.indices.remove(idx));
            retain
        })
    }

    /// Returns an iterator over the arena.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<i32>::new();
    /// for i in 0..10 {
    ///     arena.insert(i);
    /// }
    ///
    /// for (idx, value) in &arena {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> iter::Iter<T, I> {
        iter::Iter {
            inner: self.items.iter(),
            indices: &self.indices,
        }
    }

    /// Returns an iterator that allows modifying each value over this arena.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::GenIndexArena;
    /// let mut arena = GenIndexArena::<i32>::new();
    /// for i in 0..10 {
    ///     arena.insert(i);
    /// }
    ///
    /// for (_, value) in &mut arena {
    ///     *value += 5;
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> iter::IterMut<T, I> {
        iter::IterMut {
            inner: self.items.iter_mut(),
            indices: &self.indices,
        }
    }
}

impl<T, I: GenIndex> Default for GenIndexArena<T, I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, I: GenIndex> ops::Index<I> for GenIndexArena<T, I> {
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.get(&index).expect("no entry found for index")
    }
}

impl<T, I: GenIndex> ops::IndexMut<I> for GenIndexArena<T, I> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(&index).expect("no entry found for index")
    }
}

impl<T, I: GenIndex> Map for GenIndexArena<T, I> {
    type Key = I;
    type Value = T;

    #[inline]
    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.get(key)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, I: GenIndex> Arena for GenIndexArena<T, I> {
    #[inline]
    fn clear(&mut self) {
        self.clear()
    }

    #[inline]
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    #[inline]
    fn insert(&mut self, value: Self::Value) -> Self::Key {
        self.insert(value)
    }

    #[inline]
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        self.remove(key)
    }

    #[inline]
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
        self.retain(f)
    }
}

impl<'a, T, I: GenIndex> IterableMap<'a> for GenIndexArena<T, I>
where
    Self: 'a,
{
    type Iter = iter::Iter<'a, T, I>;

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T, I: GenIndex> IterableMapMut<'a> for GenIndexArena<T, I>
where
    Self: 'a,
{
    type IterMut = iter::IterMut<'a, T, I>;

    #[inline]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.iter_mut()
    }
}

impl<T, I: GenIndex> IntoIterator for GenIndexArena<T, I> {
    type Item = (I, T);
    type IntoIter = iter::IntoIter<T, I>;

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIter {
            inner: self.items.into_iter(),
            indices: self.indices,
        }
    }
}

impl<'a, T, I: GenIndex> IntoIterator for &'a GenIndexArena<T, I> {
    type Item = (&'a I, &'a T);
    type IntoIter = iter::Iter<'a, T, I>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, I: GenIndex> IntoIterator for &'a mut GenIndexArena<T, I> {
    type Item = (&'a I, &'a mut T);
    type IntoIter = iter::IterMut<'a, T, I>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, I: GenIndex> FromIterator<T> for GenIndexArena<T, I> {
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let cap = upper.unwrap_or(lower);
        let mut arena = GenIndexArena::with_capacity(cap);
        arena.extend(iter);
        arena
    }
}

impl<T, I: GenIndex> Extend<T> for GenIndexArena<T, I> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        for t in iter {
            self.insert(t);
        }
    }
}

impl<'a, T, I> Extend<&'a T> for GenIndexArena<T, I>
where
    I: GenIndex,
    T: 'a + Copy,
{
    fn extend<It: IntoIterator<Item = &'a T>>(&mut self, iter: It) {
        for t in iter {
            self.insert(*t);
        }
    }
}

pub mod iter {
    use super::{GenIndexAllocator, VecMap};
    use crate::GenIndex;
    use core::iter::FusedIterator;

    /// An immutable iterator over a `GenIndexArena`.
    /// This struct is created by the `into_iter` method on `GenIndexArena` (provided by the IntoIterator trait).
    #[derive(Clone, Debug)]
    pub struct Iter<'a, T: 'a, I: GenIndex + 'a> {
        pub(super) inner: <&'a VecMap<T, I::Index> as IntoIterator>::IntoIter,
        pub(super) indices: &'a GenIndexAllocator<I>,
    }

    impl<'a, T: 'a, I: GenIndex + 'a> Iterator for Iter<'a, T, I> {
        type Item = (&'a I, &'a T);

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                Some((ref i, value)) => {
                    Some((self.indices.get(i).expect("index out of bounds"), value))
                }
                None => None,
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a> DoubleEndedIterator for Iter<'a, T, I> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self.inner.next_back() {
                Some((ref i, value)) => {
                    Some((self.indices.get(i).expect("index out of bounds"), value))
                }
                None => None,
            }
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a> ExactSizeIterator for Iter<'a, T, I> {
        #[inline]
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a> FusedIterator for Iter<'a, T, I> {}

    /// An mutable iterator over a `GenIndexArena`.
    /// This struct is created by the `into_iter` method on `GenIndexArena` (provided by the IntoIterator trait).
    #[derive(Debug)]
    pub struct IterMut<'a, T: 'a, I: GenIndex + 'a> {
        pub(super) inner: <&'a mut VecMap<T, I::Index> as IntoIterator>::IntoIter,
        pub(super) indices: &'a GenIndexAllocator<I>,
    }

    impl<'a, T: 'a, I: GenIndex + 'a> Iterator for IterMut<'a, T, I> {
        type Item = (&'a I, &'a mut T);

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                Some((ref i, value)) => {
                    Some((self.indices.get(i).expect("index out of bounds"), value))
                }
                None => None,
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a> DoubleEndedIterator for IterMut<'a, T, I> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self.inner.next_back() {
                Some((ref i, value)) => {
                    Some((self.indices.get(i).expect("index out of bounds"), value))
                }
                None => None,
            }
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a> ExactSizeIterator for IterMut<'a, T, I> {
        #[inline]
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    impl<'a, T: 'a, I: GenIndex + 'a> FusedIterator for IterMut<'a, T, I> {}

    /// An iterator that moves out of a `GenIndexArena`.
    /// This struct is created by the `into_iter` method on `GenIndexArena` (provided by the IntoIterator trait).
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{GenIndexArena};
    /// let mut arena = GenIndexArena::<usize>::new();
    /// for i in 0..10 {
    ///     arena.insert(i * i);
    /// }
    ///
    /// for (idx, value) in arena {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    /// ```
    #[derive(Debug)]
    pub struct IntoIter<T, I: GenIndex> {
        pub(super) inner: <VecMap<T, I::Index> as IntoIterator>::IntoIter,
        pub(super) indices: GenIndexAllocator<I>,
    }

    impl<T, I: GenIndex> Iterator for IntoIter<T, I> {
        type Item = (I, T);

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                Some((ref i, value)) => {
                    Some((*self.indices.get(i).expect("index out of bounds"), value))
                }
                None => None,
            }
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }

    impl<T, I: GenIndex> DoubleEndedIterator for IntoIter<T, I> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self.inner.next_back() {
                Some((ref i, value)) => {
                    Some((*self.indices.get(i).expect("index out of bounds"), value))
                }
                None => None,
            }
        }
    }

    impl<T, I: GenIndex> ExactSizeIterator for IntoIter<T, I> {
        #[inline]
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    impl<T, I: GenIndex> FusedIterator for IntoIter<T, I> {}
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::GenIndexArena;
    use crate::GenIndex;

    impl<T, I> Serialize for GenIndexArena<T, I>
    where
        T: Serialize,
        I: GenIndex + Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            (&self.indices, &self.items).serialize(serializer)
        }
    }

    impl<'de, T, I> Deserialize<'de> for GenIndexArena<T, I>
    where
        T: Deserialize<'de>,
        I: GenIndex + Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let (indices, items) = Deserialize::deserialize(deserializer)?;
            Ok(GenIndexArena { indices, items })
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        use super::GenIndexArena;
        use crate::Index;
        use alloc::vec;
        use serde_json::{json, Value};

        let mut arena = GenIndexArena::<&str, Index>::with_capacity(3);
        let idx1 = arena.insert("a");
        arena.insert("b");
        arena.insert("c");
        arena.remove(&idx1);
        arena.insert("d");

        let expected_json: Value = json!([[[0, 2], [1, 1], [2, 1]], ["d", "b", "c"]]);

        let json: Value = serde_json::to_value(arena).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        use super::GenIndexArena;
        use crate::{GenIndex, Index};
        use alloc::string::String;
        use alloc::vec;
        use serde_json::{json, Value};

        let json: Value = json!([[[0, 2], [1, 3], [5, 3], [3, 4]], ["d", "b", null, "c"]]);

        let arena: GenIndexArena<String, Index> = serde_json::from_value(json).unwrap();

        assert_eq!(arena.len(), 3);
        assert_eq!(arena[Index::from_raw_parts(1, 3)], "b");
        assert_eq!(arena[Index::from_raw_parts(3, 4)], "c");
        assert_eq!(arena[Index::from_raw_parts(0, 2)], "d");
    }
}
