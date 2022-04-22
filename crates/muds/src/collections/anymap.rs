//! Map of any type to value.

use super::{IterableMap, MapMut};
use alloc::boxed::Box;
use core::any::{Any, TypeId};

#[cfg(feature = "std")]
type DefaultBackingMap = std::collections::HashMap<TypeId, Box<dyn Any>>;
#[cfg(not(feature = "std"))]
type DefaultBackingMap = alloc::collections::BTreeMap<TypeId, Box<dyn Any>>;

/// The `AnyMap` is an associative array that can store any item using the item type ID as key.
/// It stores data in a backing `Map`.
#[derive(Clone, Debug)]
pub struct AnyMap<M = DefaultBackingMap>
where
    M: MapMut<Key = TypeId, Value = Box<dyn Any>>,
{
    map: M,
}

impl<M> AnyMap<M>
where
    M: MapMut<Key = TypeId, Value = Box<dyn Any>>,
{
    /// Returns the number of elements in the map, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let map = <AnyMap>::new();
    /// assert_eq!(map.len(), 0);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let map = <AnyMap>::new();
    /// assert!(map.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the map contains a value of type `T`.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert::<i32>(123);
    /// assert!(map.contains::<i32>());
    /// ```
    #[inline]
    pub fn contains<T: 'static>(&self) -> bool {
        self.get::<T>().is_some()
    }

    /// Returns a reference to the value of type `T` if exists.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert::<i32>(123);
    /// assert_eq!(*map.get::<i32>().unwrap(), 123);
    /// assert!(map.get::<u32>().is_none());
    /// ```
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self.map.get(&TypeId::of::<T>()) {
            Some(boxed) => boxed.downcast_ref(),
            _ => None,
        }
    }

    /// Returns a mutable reference to the value of type `T` if exists.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert::<i32>(123);
    /// *map.get_mut::<i32>().unwrap() = 456;
    /// assert_eq!(*map.get::<i32>().unwrap(), 456);
    /// ```
    #[inline]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        match self.map.get_mut(&TypeId::of::<T>()) {
            Some(boxed) => boxed.downcast_mut(),
            _ => None,
        }
    }

    /// Inserts `value` of type T into the map. The existing value in the map is returned.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert::<i32>(123);
    /// assert!(map.contains::<i32>());
    /// assert_eq!(map.insert::<i32>(456).unwrap(), 123);
    /// ```
    pub fn insert<T: 'static>(&mut self, value: T) -> Option<T> {
        match self.map.insert(TypeId::of::<T>(), Box::new(value)) {
            Some(boxed) => Some(*boxed.downcast::<T>().ok()?),
            _ => None,
        }
    }

    /// Removes and returns the element of type `T` from the map if exists.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert::<i32>(123);
    /// assert_eq!(map.remove::<i32>().unwrap(), 123);
    /// ```
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        match self.map.remove(&TypeId::of::<T>()) {
            Some(boxed) => Some(*boxed.downcast::<T>().ok()?),
            _ => None,
        }
    }

    /// Clears the map, removing all values.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert::<i32>(123);
    /// map.clear();
    /// assert_eq!(map.len(), 0);
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Creates an iterator.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::AnyMap;
    /// let mut map = <AnyMap>::new();
    /// map.insert::<i32>(123);
    /// map.insert::<u32>(456);
    ///
    /// for (ty, value) in map.iter() {
    ///     println!("({:?}, {:?})", ty, value);
    /// }
    /// ```
    #[inline]
    pub fn iter<'a>(&'a self) -> <M as IterableMap<'a>>::Iter
    where
        M: IterableMap<'a>,
    {
        self.map.iter()
    }
}

impl<M> AnyMap<M>
where
    M: MapMut<Key = TypeId, Value = Box<dyn Any>> + Default,
{
    /// Constructs a new, empty `AnyMap`.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::AnyMap;
    /// let map = <AnyMap>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<M> Default for AnyMap<M>
where
    M: MapMut<Key = TypeId, Value = Box<dyn Any>> + Default,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
