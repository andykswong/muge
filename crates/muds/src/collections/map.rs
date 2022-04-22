//! Collection traits.

/// An immutable associative array.
pub trait Map {
    /// Key type
    type Key;

    /// Value type
    type Value;

    /// Returns a reference to the value corresponding to the `key` if exists.
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;

    /// Returns the number of elements in the map, also referred to as its ‘length’.
    fn len(&self) -> usize;

    /// Returns `true` if the map contains a value for the `key`.
    #[inline]
    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get(key).is_some()
    }

    /// Returns `true` if the map contains no elements.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A mutable associative array.
pub trait MapMut: Map {
    /// Clears the map, removing all values.
    fn clear(&mut self);

    /// Returns a mutable reference to the value corresponding to the `key` if exists.
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Inserts `value` into the map. The existing value in the map is returned.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    /// Removes and returns the element at `key` from the map if exists.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(&index, &mut value)` returns `false`.
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool);
}

/// An iterable map type.
pub trait IterableMap<'a>: Map + 'a {
    /// Which kind of iterator are we turning this into?
    type Iter: Iterator<Item = (&'a Self::Key, &'a Self::Value)>;

    /// Creates an iterator.
    fn iter(&'a self) -> Self::Iter;
}

/// A mutably iterable map type.
pub trait IterableMapMut<'a>: Map + IterableMap<'a> {
    /// Which kind of mutable iterator are we turning this into?
    type IterMut: Iterator<Item = (&'a Self::Key, &'a mut Self::Value)>;

    /// Creates a mutable iterator.
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

/// An arena allocator type.
pub trait Arena: Map {
    /// Clears the arena, removing all values.
    fn clear(&mut self);

    /// Returns a mutable reference to the value corresponding to the `key` if exists.
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Inserts `value` into the arena. The element's assigned key in the arena is returned.
    fn insert(&mut self, value: Self::Value) -> Self::Key;

    /// Removes and returns the element at `key` from the arena if exists.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    /// In other words, removes all elements such that `f(&index, &mut value)` returns `false`.
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool);
}

/// Implement MapMut for all Arena for compatibility
impl<A: Arena> MapMut for A {
    #[inline]
    fn clear(&mut self) {
        self.clear();
    }

    #[inline]
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    /// Set value of given key. The existing value in the map is returned.
    #[inline]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.get_mut(&key).map(|v| core::mem::replace(v, value))
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
