//! Map backend by Vec of Option.

use super::{Map, MapMut};
use crate::UnsignedNum;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::{mem, ops};
use num::NumCast;

/// The `VecMap` is a type of associative array
/// that uses a [Vec] of [Option]s to map unsigned integer keys to elements.
#[derive(Clone, Debug)]
pub struct VecMap<V, I: UnsignedNum = usize> {
    items: Vec<Option<V>>,
    len: usize,
    phantom: PhantomData<I>,
}

impl<V, I: UnsignedNum> VecMap<V, I> {
    /// Constructs a new, empty `VecMap`.
    /// It will not allocate until elements are pushed onto it.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let map = VecMap::<()>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Constructs a new, empty `VecMap` with the specified capacity.
    /// It will be able to hold exactly capacity elements without reallocating.
    /// If capacity is 0, it will not allocate.
    ///
    /// # Panic
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let map = VecMap::<()>::with_capacity(10);
    /// assert_eq!(map.capacity(), 10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            len: 0,
            phantom: PhantomData,
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let map = VecMap::<()>::with_capacity(10);
    /// assert_eq!(map.capacity(), 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Returns the number of elements in the map, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<()>::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert(1, ());
    /// assert_eq!(map.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<i32>::new();
    /// let idx = 2;
    /// map.insert(idx, 123);
    ///
    /// assert_eq!(map.get(&idx), Some(&123));
    /// map.remove(&idx);
    /// assert!(map.get(&idx).is_none());
    /// ```
    #[inline]
    pub fn get(&self, i: &I) -> Option<&V> {
        match self.items.get(i.to_usize()?) {
            Some(Some(item)) => Some(item),
            _ => None,
        }
    }

    /// Returns a mutable reference to the value corresponding to the index `i` .
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<i32>::new();
    /// let idx = 1;
    /// map.insert(idx, 123);
    ///
    /// *map.get_mut(&idx).unwrap() += 1;
    /// assert_eq!(map.remove(&idx), Some(124));
    /// assert!(map.get_mut(&idx).is_none());
    /// ```
    #[inline]
    pub fn get_mut(&mut self, i: &I) -> Option<&mut V> {
        match self.items.get_mut(i.to_usize()?) {
            Some(Some(item)) => Some(item),
            _ => None,
        }
    }

    /// Clears the map, removing all values.
    /// Note that this method has no effect on the allocated capacity of the map.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<()>::new();
    /// map.insert(1, ());
    /// map.clear();
    /// assert!(map.len() == 0);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.items.clear();
        self.len = 0;
    }

    /// Inserts `value` into the map, allocating more capacity if necessary.
    /// The existing key-value in the map is returned.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<i32>::new();
    /// let idx = 1;
    /// assert!(map.insert(idx, 123).is_none());
    /// assert_eq!(map.insert(idx, 456).unwrap(), 123);
    /// assert!(map.insert(0, 123).is_none());
    /// assert_eq!(*map.get(&idx).unwrap(), 456);
    /// ```
    pub fn insert(&mut self, i: I, v: V) -> Option<V> {
        let index = i.to_usize()?;
        self.len += 1;
        match self.items.get_mut(index) {
            Some(Some(item)) => Some(mem::replace(item, v)),
            _ => {
                if index >= self.items.len() {
                    self.items.resize_with(index + 1, || None);
                }
                self.items[index] = Some(v);
                None
            }
        }
    }

    /// Removes and returns the element at index `i` from the map if exists.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<i32>::new();
    /// map.insert(1, 123);
    /// assert_eq!(map.remove(&1), Some(123));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove(&mut self, i: &I) -> Option<V> {
        let index = i.to_usize()?;
        match self.items.get_mut(index) {
            Some(Some(_)) => {
                self.len -= 1;
                self.items[index].take()
            }
            _ => None,
        }
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(index, &value)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<i32>::new();
    /// map.insert(1, 1);
    /// map.insert(0, 2);
    /// map.retain(|_, val| { if *val == 1 { *val = 3; true } else { false } });
    /// assert_eq!(*map.get(&1).unwrap(), 3);
    /// assert!(map.get(&0).is_none());
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&I, &mut V) -> bool) {
        for i in 0..self.items.len() {
            if let Some(item) = &mut self.items[i] {
                let index = NumCast::from(i).expect("index out of bounds");
                if !f(&index, item) {
                    self.remove(&index);
                }
            }
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the given map.
    /// The collection may reserve more space to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<()>::new();
    /// map.reserve(10);
    /// assert!(map.capacity() >= 10);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional);
    }

    /// Returns an iterator over the map.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<usize>::new();
    /// for i in 0..10 {
    ///     map.insert(i, i * i);
    /// }
    ///
    /// for (idx, value) in &map {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    pub fn iter(&self) -> iter::Iter<V, I> {
        iter::Iter {
            inner: self.items.iter().enumerate(),
            len: self.len,
            phantom: PhantomData,
        }
    }

    /// Returns an iterator that allows modifying each value over this map.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<usize>::new();
    /// for i in 0..10 {
    ///     map.insert(i, i * i);
    /// }
    ///
    /// for (_, value) in &mut map {
    ///     *value += 5;
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> iter::IterMut<V, I> {
        iter::IterMut {
            inner: self.items.iter_mut().enumerate(),
            len: self.len,
            phantom: PhantomData,
        }
    }
}

impl<V, I: UnsignedNum> Map for VecMap<V, I> {
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

impl<V, I: UnsignedNum> MapMut for VecMap<V, I> {
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

impl<V, I: UnsignedNum> Default for VecMap<V, I> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<V, I: UnsignedNum> ops::Index<I> for VecMap<V, I> {
    type Output = V;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.get(&index).expect("no entry found for index")
    }
}

impl<V, I: UnsignedNum> ops::IndexMut<I> for VecMap<V, I> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(&index).expect("no entry found for index")
    }
}

impl<V, I: UnsignedNum> IntoIterator for VecMap<V, I> {
    type Item = (I, V);
    type IntoIter = iter::IntoIter<V, I>;

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIter {
            inner: self.items.into_iter().enumerate(),
            len: self.len,
            phantom: PhantomData,
        }
    }
}

impl<'a, V, I: UnsignedNum> IntoIterator for &'a VecMap<V, I> {
    type Item = (I, &'a V);
    type IntoIter = iter::Iter<'a, V, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, I: UnsignedNum> IntoIterator for &'a mut VecMap<V, I> {
    type Item = (I, &'a mut V);
    type IntoIter = iter::IterMut<'a, V, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V, I: UnsignedNum> FromIterator<(I, V)> for VecMap<V, I> {
    fn from_iter<It: IntoIterator<Item = (I, V)>>(iter: It) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let cap = upper.unwrap_or(lower);
        let mut set = VecMap::with_capacity(cap);
        set.extend(iter);
        set
    }
}

impl<V, I: UnsignedNum> Extend<(I, V)> for VecMap<V, I> {
    fn extend<It: IntoIterator<Item = (I, V)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(i, t);
        }
    }
}

impl<'a, V, I> Extend<(I, &'a V)> for VecMap<V, I>
where
    I: UnsignedNum + 'a,
    V: Copy + 'a,
{
    fn extend<It: IntoIterator<Item = (I, &'a V)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(i, *t);
        }
    }
}

impl<'a, V, I> Extend<(&'a I, &'a V)> for VecMap<V, I>
where
    I: UnsignedNum + 'a,
    V: Copy + 'a,
{
    fn extend<It: IntoIterator<Item = (&'a I, &'a V)>>(&mut self, iter: It) {
        for (i, t) in iter {
            self.insert(*i, *t);
        }
    }
}

/// [VecMap] iterators.
pub mod iter {
    use alloc::vec;
    use core::iter::{Enumerate, FusedIterator};
    use core::marker::PhantomData;
    use core::slice;
    use num::NumCast;

    use crate::UnsignedNum;

    /// An immutable iterator over an arena.
    /// This struct is created by the `iter` method on `VecMap`.
    #[derive(Clone, Debug)]
    pub struct Iter<'a, T: 'a, I: UnsignedNum + 'a> {
        pub(super) inner: Enumerate<slice::Iter<'a, Option<T>>>,
        pub(super) len: usize,
        pub(super) phantom: PhantomData<I>,
    }

    impl<'a, T: 'a, I: UnsignedNum + 'a> Iter<'a, T, I> {
        fn next_back_or_forth(&mut self, forward: bool) -> Option<(I, &'a T)> {
            loop {
                match if forward {
                    self.inner.next()
                } else {
                    self.inner.next_back()
                } {
                    Some((i, Some(value))) => {
                        self.len -= 1;
                        return Some((NumCast::from(i).expect("index out of bounds"), value));
                    }
                    Some(_) => continue,
                    None => {
                        debug_assert_eq!(self.len, 0);
                        return None;
                    }
                }
            }
        }
    }

    /// An mutable iterator over an arena.
    /// This struct is created by the `iter_mut` method on `VecMap`.
    #[derive(Debug)]
    pub struct IterMut<'a, T: 'a, I: UnsignedNum + 'a> {
        pub(super) inner: Enumerate<slice::IterMut<'a, Option<T>>>,
        pub(super) len: usize,
        pub(super) phantom: PhantomData<I>,
    }

    impl<'a, T: 'a, I: UnsignedNum + 'a> IterMut<'a, T, I> {
        fn next_back_or_forth(&mut self, forward: bool) -> Option<(I, &'a mut T)> {
            loop {
                match if forward {
                    self.inner.next()
                } else {
                    self.inner.next_back()
                } {
                    Some((i, Some(value))) => {
                        self.len -= 1;
                        return Some((NumCast::from(i).expect("index out of bounds"), value));
                    }
                    Some(_) => continue,
                    None => {
                        debug_assert_eq!(self.len, 0);
                        return None;
                    }
                }
            }
        }
    }

    /// An iterator that moves out of a `VecMap`.
    /// This struct is created by the `into_iter` method on `VecMap` (provided by the IntoIterator trait).
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{VecMap};
    /// let mut map = VecMap::<usize>::new();
    /// for i in 0..10 {
    ///     map.insert(i, i * i);
    /// }
    ///
    /// for (idx, value) in map {
    ///     println!("{} is at index {:?}", value, idx);
    /// }
    /// ```
    #[derive(Debug)]
    pub struct IntoIter<T, I: UnsignedNum> {
        pub(super) inner: Enumerate<vec::IntoIter<Option<T>>>,
        pub(super) len: usize,
        pub(super) phantom: PhantomData<I>,
    }

    impl<T, I: UnsignedNum> IntoIter<T, I> {
        fn next_back_or_forth(&mut self, forward: bool) -> Option<(I, T)> {
            loop {
                match if forward {
                    self.inner.next()
                } else {
                    self.inner.next_back()
                } {
                    Some((i, Some(value))) => {
                        self.len -= 1;
                        return Some((NumCast::from(i).expect("index out of bounds"), value));
                    }
                    Some(_) => continue,
                    None => {
                        debug_assert_eq!(self.len, 0);
                        return None;
                    }
                }
            }
        }
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
                    self.next_back_or_forth(true)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len, Some(self.len))
                }
            }

            impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
                DoubleEndedIterator
            for $name $(< $( $lt ),+ >)?
            {
                fn next_back(&mut self) -> Option<Self::Item> {
                    self.next_back_or_forth(false)
                }
            }

            impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
                ExactSizeIterator
            for $name $(< $( $lt ),+ >)?
            {
                fn len(&self) -> usize {
                    self.len
                }
            }

            impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
                FusedIterator
            for $name $(< $( $lt ),+ >)?
            {}
        }
    }

    impl_iter_traits!(Iter<'a, T, I: UnsignedNum>, (I, &'a T));

    impl_iter_traits!(IterMut<'a, T, I: UnsignedNum>, (I, &'a mut T));

    impl_iter_traits!(IntoIter<T, I: UnsignedNum>, (I, T));
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::VecMap;
    use crate::UnsignedNum;
    use alloc::vec::Vec;
    use core::marker::PhantomData;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<V, I> Serialize for VecMap<V, I>
    where
        V: Serialize,
        I: UnsignedNum,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.items.serialize(serializer)
        }
    }

    impl<'de, V, I> Deserialize<'de> for VecMap<V, I>
    where
        V: Deserialize<'de>,
        I: UnsignedNum,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let items: Vec<Option<V>> = Deserialize::deserialize(deserializer)?;
            let mut len = items.len();
            for opt in &items {
                if opt.is_none() {
                    len -= 1;
                }
            }
            Ok(VecMap {
                items,
                len,
                phantom: PhantomData,
            })
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;
    use serde_json::{json, Value};

    use super::VecMap;

    #[test]
    fn test_serialize() {
        let mut set = VecMap::<&str>::with_capacity(3);
        set.insert(1, "a");
        set.insert(0, "b");
        set.insert(4, "c");

        let expected_json: Value = json!(["b", "a", null, null, "c"]);

        let json: Value = serde_json::to_value(set).unwrap();

        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_deserialize() {
        let json: Value = json!([null, "a", null, "c"]);

        let set: VecMap<String> = serde_json::from_value(json).unwrap();

        assert_eq!(set.len(), 2);
        assert_eq!(set[1], "a");
        assert_eq!(set[3], "c");
    }
}
