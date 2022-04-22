//! Map with generational index as key.

use super::{IterableMap, IterableMapMut, Map, MapMut};
use crate::GenIndex;
use core::marker::PhantomData;
use core::ops;

/// The `GenIndexMap` is a type of associative array that uses `GenIndex` as key.
/// It stores data in a backing `Map`.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>>,
{
    map: M,
}

/// `GenIndexMap` entry type
type Entry<T, I> = (I, T);

impl<T, I, M> GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>> + Default,
{
    /// Constructs a new, empty `GenIndexMap`.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{Index, collections::{GenIndexMap, VecMap}};
    /// let map = GenIndexMap::<(), Index, VecMap<(Index, ())>>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<T, I, M> GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>>,
{
    /// Returns a reference to the value corresponding to the `key` if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// let idx = Index::from_raw_parts(2, 0);
    /// map.insert(idx, 123);
    ///
    /// assert_eq!(map.get(&idx), Some(&123));
    /// map.remove(&idx);
    /// assert!(map.get(&idx).is_none());
    /// ```
    #[inline]
    pub fn get(&self, key: &I) -> Option<&T> {
        match self.map.get(&key.index()) {
            Some((i, value)) if i == key => Some(value),
            _ => None,
        }
    }

    /// Returns the number of elements in the map, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert(Index::from_raw_parts(1, 0), 123);
    /// assert_eq!(map.len(), 1);
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Gets the backing map.
    #[inline]
    pub fn map(&self) -> &M {
        &self.map
    }
}

impl<T, I, M> GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
{
    /// Inserts `value` into the map. The existing key-value in the map is returned.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// let idx = Index::from_raw_parts(1, 0);
    /// let idx2 = Index::from_raw_parts(1, 1);
    /// let idx3 = Index::from_raw_parts(0, 1);
    /// assert!(map.insert(idx, 123).is_none());
    /// assert_eq!(map.insert(idx2, 456).unwrap(), (idx, 123));
    /// assert!(map.insert(idx3, 123).is_none());
    /// assert_eq!(*map.get(&idx2).unwrap(), 456);
    /// ```
    #[inline]
    pub fn insert(&mut self, key: I, value: T) -> Option<Entry<T, I>> {
        self.map.insert(key.index(), (key, value))
    }

    /// Clears the map, removing all values.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// map.insert(Index::from_raw_parts(1, 0), 123);
    /// map.clear();
    /// assert!(map.len() == 0);
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Returns a mutable reference to the value corresponding to the `key` if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// let idx = Index::from_raw_parts(1, 0);
    /// map.insert(idx, 123);
    ///
    /// *map.get_mut(&idx).unwrap() += 1;
    /// assert_eq!(map.remove(&idx), Some(124));
    /// assert!(map.get_mut(&idx).is_none());
    #[inline]
    pub fn get_mut(&mut self, key: &I) -> Option<&mut T> {
        match self.map.get_mut(&key.index()) {
            Some((i, value)) if i == key => Some(value),
            _ => None,
        }
    }

    /// Removes and returns the element at `key` from the map if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// let idx = Index::from_raw_parts(1, 0);
    /// map.insert(idx, 123);
    /// assert_eq!(map.remove(&idx), Some(123));
    /// assert_eq!(map.remove(&idx), None);
    /// ```
    #[inline]
    pub fn remove(&mut self, key: &I) -> Option<T> {
        if self.contains_key(key) {
            if let Some((_, value)) = self.map.remove(&key.index()) {
                return Some(value);
            }
        }
        None
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(&index, &mut value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// let idx1 = Index::from_raw_parts(1, 0);
    /// let idx2 = Index::from_raw_parts(0, 2);
    /// map.insert(idx1, 1);
    /// map.insert(idx2, 2);
    /// map.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(*map.get(&idx1).unwrap(), 3);
    /// assert!(map.get(&idx2).is_none());
    /// ```
    #[inline]
    pub fn retain(&mut self, mut f: impl FnMut(&I, &mut T) -> bool) {
        self.map.retain(|_, (i, t)| f(i, t))
    }

    /// Gets the backing map mutably.
    #[inline]
    pub fn map_mut(&mut self) -> &mut M {
        &mut self.map
    }
}

impl<T, I, M> Map for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>>,
{
    type Key = I;
    type Value = T;

    #[inline]
    fn get(&self, key: &I) -> Option<&T> {
        self.get(key)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, I, M> MapMut for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
{
    /// Inserts `value` into the map. The existing value in the map is returned.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<i32, Index, VecMap<(Index, i32)>>::new();
    /// let idx = Index::from_raw_parts(1, 0);
    /// let idx2 = Index::from_raw_parts(1, 1);
    /// let idx3 = Index::from_raw_parts(0, 1);
    /// assert!(MapMut::insert(&mut map, idx, 123).is_none());
    /// assert_eq!(MapMut::insert(&mut map, idx2, 456).unwrap(), 123);
    /// assert!(MapMut::insert(&mut map, idx3, 123).is_none());
    /// assert_eq!(*map.get(&idx2).unwrap(), 456);
    /// ```
    #[inline]
    fn insert(&mut self, key: I, value: T) -> Option<T> {
        if let Some((_, value)) = self.map.insert(key.index(), (key, value)) {
            Some(value)
        } else {
            None
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.clear()
    }

    #[inline]
    fn get_mut(&mut self, key: &I) -> Option<&mut T> {
        self.get_mut(key)
    }

    #[inline]
    fn remove(&mut self, key: &I) -> Option<T> {
        self.remove(key)
    }

    #[inline]
    fn retain(&mut self, f: impl FnMut(&I, &mut T) -> bool) {
        self.retain(f)
    }
}

impl<T, I, M> Default for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>> + Default,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, I, M> ops::Index<I> for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>>,
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.get(&index).expect("no entry found for index")
    }
}

impl<T, I, M> ops::IndexMut<I> for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(&index).expect("no entry found for index")
    }
}

impl<'a, T, I, M, Idx> IterableMap<'a> for GenIndexMap<T, I, M>
where
    Self: 'a,
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>>,
    &'a M: IntoIterator<Item = (Idx, &'a Entry<T, I>)>,
{
    type Iter = iter::Iter<'a, T, I, M>;

    /// Creates an iterator.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, IterableMap, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<usize, Index, VecMap<(Index, usize)>>::new();
    /// for i in 0..10 {
    ///     map.insert(<Index>::from_raw_parts(i, 1), i * i);
    /// }
    ///
    /// for (idx, value) in map.iter() {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    /// ```
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'a, T, I, M, Idx> IterableMapMut<'a> for GenIndexMap<T, I, M>
where
    Self: 'a,
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
    &'a M: IntoIterator<Item = (Idx, &'a Entry<T, I>)>,
    &'a mut M: IntoIterator<Item = (Idx, &'a mut Entry<T, I>)>,
{
    type IterMut = iter::IterMut<'a, T, I, M>;

    /// Creates a mutable iterator.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, IterableMapMut, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<usize, Index, VecMap<(Index, usize)>>::new();
    /// for i in 0..10 {
    ///     map.insert(<Index>::from_raw_parts(i, 1), i * i);
    /// }
    ///
    /// for (_, value) in map.iter_mut() {
    ///     *value += 5;
    /// }
    /// ```
    #[inline]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.into_iter()
    }
}

impl<'a, T, I, M, Idx> IntoIterator for &'a GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>>,
    &'a M: IntoIterator<Item = (Idx, &'a Entry<T, I>)>,
{
    type Item = (&'a I, &'a T);
    type IntoIter = iter::Iter<'a, T, I, M>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: (&self.map).into_iter(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T, I, M, Idx> IntoIterator for &'a mut GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
    &'a mut M: IntoIterator<Item = (Idx, &'a mut Entry<T, I>)>,
{
    type Item = (&'a I, &'a mut T);
    type IntoIter = iter::IterMut<'a, T, I, M>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: (&mut self.map).into_iter(),
            phantom: PhantomData,
        }
    }
}

impl<T, I, M> IntoIterator for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: Map<Key = I::Index, Value = Entry<T, I>> + IntoIterator<Item = (I::Index, Entry<T, I>)>,
{
    type Item = (I, T);
    type IntoIter = iter::IntoIter<T, I, M>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: self.map.into_iter(),
            phantom: PhantomData,
        }
    }
}

impl<T, I, M> FromIterator<(I, T)> for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>> + Default,
{
    fn from_iter<It: IntoIterator<Item = (I, T)>>(iter: It) -> Self {
        let mut map: Self = Default::default();
        map.extend(iter.into_iter());
        map
    }
}

impl<T, I, M> Extend<(I, T)> for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
{
    fn extend<It: IntoIterator<Item = (I, T)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(i, t);
        }
    }
}

impl<'a, T, I, M> Extend<(I, &'a T)> for GenIndexMap<T, I, M>
where
    I: GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
    T: 'a + Copy,
{
    fn extend<It: IntoIterator<Item = (I, &'a T)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(i, *t);
        }
    }
}

impl<'a, T, I, M> Extend<(&'a I, &'a T)> for GenIndexMap<T, I, M>
where
    I: 'a + GenIndex,
    M: MapMut<Key = I::Index, Value = Entry<T, I>>,
    T: 'a + Copy,
{
    fn extend<It: IntoIterator<Item = (&'a I, &'a T)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(*i, *t);
        }
    }
}

/// [GenIndexMap] iterators.
pub mod iter {
    use super::{Entry, Map, MapMut};
    use crate::GenIndex;
    use core::iter::FusedIterator;
    use core::marker::PhantomData;

    /// An immutable iterator over a `GenIndexMap`.
    /// This struct is created by the `into_iter` method on `GenIndexMap` (provided by the IntoIterator trait).
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<usize, Index, VecMap<(Index, usize)>>::new();
    /// for i in 0..10 {
    ///     map.insert(<Index>::from_raw_parts(i, 1), i * i);
    /// }
    ///
    /// for (idx, value) in &map {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    /// ```
    pub struct Iter<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: Map<Key = I::Index, Value = Entry<T, I>>,
        &'a M: IntoIterator,
    {
        pub(super) inner: <&'a M as IntoIterator>::IntoIter,
        pub(super) phantom: PhantomData<(&'a I, &'a T)>,
    }

    impl<'a, T, I, M, Idx> Iterator for Iter<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: Map<Key = I::Index, Value = Entry<T, I>>,
        &'a M: IntoIterator<Item = (Idx, &'a Entry<T, I>)>,
    {
        type Item = (&'a I, &'a T);

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|(_, (key, value))| (key, value))
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }

    impl<'a, T, I, M, Idx> DoubleEndedIterator for Iter<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: Map<Key = I::Index, Value = Entry<T, I>>,
        &'a M: IntoIterator<Item = (Idx, &'a Entry<T, I>)>,
        <&'a M as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.inner.next_back().map(|(_, (key, value))| (key, value))
        }
    }

    impl<'a, T, I, M, Idx> ExactSizeIterator for Iter<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: Map<Key = I::Index, Value = Entry<T, I>>,
        &'a M: IntoIterator<Item = (Idx, &'a Entry<T, I>)>,
        <&'a M as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        #[inline]
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    impl<'a, T, I, M, Idx> FusedIterator for Iter<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: Map<Key = I::Index, Value = Entry<T, I>>,
        &'a M: IntoIterator<Item = (Idx, &'a Entry<T, I>)>,
        <&'a M as IntoIterator>::IntoIter: FusedIterator,
    {
    }

    /// An mutable iterator over a `GenIndexMap`.
    /// This struct is created by the `into_iter` method on `GenIndexMap` (provided by the IntoIterator trait).
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<usize, Index, VecMap<(Index, usize)>>::new();
    /// for i in 0..10 {
    ///     map.insert(<Index>::from_raw_parts(i, 1), i * i);
    /// }
    ///
    /// for (_, value) in &mut map {
    ///     *value += 5;
    /// }
    /// ```
    pub struct IterMut<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: Map<Key = I::Index, Value = Entry<T, I>>,
        &'a mut M: IntoIterator,
    {
        pub(super) inner: <&'a mut M as IntoIterator>::IntoIter,
        pub(super) phantom: PhantomData<(&'a I, &'a mut T)>,
    }

    impl<'a, T, I, M, Idx> Iterator for IterMut<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: MapMut<Key = I::Index, Value = Entry<T, I>>,
        &'a mut M: IntoIterator<Item = (Idx, &'a mut Entry<T, I>)>,
    {
        type Item = (&'a I, &'a mut T);

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|(_, (ref key, value))| (key, value))
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }

    impl<'a, T, I, M, Idx> DoubleEndedIterator for IterMut<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: MapMut<Key = I::Index, Value = Entry<T, I>>,
        &'a mut M: IntoIterator<Item = (Idx, &'a mut Entry<T, I>)>,
        <&'a mut M as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.inner
                .next_back()
                .map(|(_, (ref key, value))| (key, value))
        }
    }

    impl<'a, T, I, M, Idx> ExactSizeIterator for IterMut<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: MapMut<Key = I::Index, Value = Entry<T, I>>,
        &'a mut M: IntoIterator<Item = (Idx, &'a mut Entry<T, I>)>,
        <&'a mut M as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        #[inline]
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    impl<'a, T, I, M, Idx> FusedIterator for IterMut<'a, T, I, M>
    where
        I: GenIndex + 'a,
        T: 'a,
        M: MapMut<Key = I::Index, Value = Entry<T, I>>,
        &'a mut M: IntoIterator<Item = (Idx, &'a mut Entry<T, I>)>,
        <&'a mut M as IntoIterator>::IntoIter: FusedIterator,
    {
    }

    /// An iterator that moves out of a `GenIndexMap`.
    /// This struct is created by the `into_iter` method on `GenIndexMap` (provided by the IntoIterator trait).
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{GenIndex, Index, collections::{GenIndexMap, Map, MapMut, VecMap}};
    /// let mut map = GenIndexMap::<usize, Index, VecMap<(Index, usize)>>::new();
    /// for i in 0..10 {
    ///     map.insert(<Index>::from_raw_parts(i, 1), i * i);
    /// }
    ///
    /// for (idx, value) in map {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    /// ```
    pub struct IntoIter<T, I, M>
    where
        I: GenIndex,
        M: Map<Key = I::Index, Value = Entry<T, I>> + IntoIterator,
    {
        pub(super) inner: M::IntoIter,
        pub(super) phantom: PhantomData<(I, T)>,
    }

    impl<T, I, M> Iterator for IntoIter<T, I, M>
    where
        I: GenIndex,
        M: Map<Key = I::Index, Value = Entry<T, I>> + IntoIterator<Item = (I::Index, Entry<T, I>)>,
    {
        type Item = (I, T);

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|(_, (key, value))| (key, value))
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }

    impl<T, I, M> DoubleEndedIterator for IntoIter<T, I, M>
    where
        I: GenIndex,
        M: Map<Key = I::Index, Value = Entry<T, I>> + IntoIterator<Item = (I::Index, Entry<T, I>)>,
        M::IntoIter: DoubleEndedIterator,
    {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.inner.next_back().map(|(_, (key, value))| (key, value))
        }
    }

    impl<T, I, M> ExactSizeIterator for IntoIter<T, I, M>
    where
        I: GenIndex,
        M: Map<Key = I::Index, Value = Entry<T, I>> + IntoIterator<Item = (I::Index, Entry<T, I>)>,
        M::IntoIter: ExactSizeIterator,
    {
        #[inline]
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    impl<T, I, M> FusedIterator for IntoIter<T, I, M>
    where
        I: GenIndex,
        M: Map<Key = I::Index, Value = Entry<T, I>> + IntoIterator<Item = (I::Index, Entry<T, I>)>,
        M::IntoIter: FusedIterator,
    {
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{Entry, GenIndexMap, Map};
    use crate::GenIndex;

    impl<T, I, M> Serialize for GenIndexMap<T, I, M>
    where
        I: GenIndex,
        M: Map<Key = I::Index, Value = Entry<T, I>> + Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.map.serialize(serializer)
        }
    }

    impl<'de, T, I, M> Deserialize<'de> for GenIndexMap<T, I, M>
    where
        T: Deserialize<'de>,
        I: GenIndex,
        M: Map<Key = I::Index, Value = Entry<T, I>> + Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let map: M = Deserialize::deserialize(deserializer)?;
            Ok(Self { map })
        }
    }
}
