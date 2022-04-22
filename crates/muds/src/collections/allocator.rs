//! Generational index allocator.

use alloc::vec::Vec;
use num::{NumCast, ToPrimitive, Zero};

use crate::{GenIndex, IndexF64};

/// Allocator of generational indices.
#[derive(Clone, Debug)]
pub struct GenIndexAllocator<I: GenIndex = IndexF64> {
    indices: Vec<I>,
    free_list_head: I::Index,
    free_list_size: usize,
}

impl<I: GenIndex> GenIndexAllocator<I> {
    /// Constructs a new `GenIndexAllocator`.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::GenIndexAllocator;
    /// let allocator = <GenIndexAllocator>::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Constructs a new, empty `GenIndexAllocator` with the specified capacity.
    /// The allocator will be able to hold exactly `capacity` elements without reallocating.
    /// If `capacity` is 0, it will not allocate.
    ///
    /// # Panic
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::GenIndexAllocator;
    /// let allocator = <GenIndexAllocator>::with_capacity(10);
    /// assert_eq!(allocator.capacity(), 10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            indices: Vec::with_capacity(capacity),
            free_list_head: I::Index::zero(),
            free_list_size: 0,
        }
    }

    /// Returns the number of elements in the allocator, also referred to as its ‘length’.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// assert_eq!(allocator.len(), 0);
    /// allocator.create();
    /// assert_eq!(allocator.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.indices.len() - self.free_list_size
    }

    /// Returns `true` if the allocator contains no elements.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// assert!(allocator.is_empty());
    /// allocator.create();
    /// assert!(!allocator.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements the allocator can hold without reallocating.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexAllocator;
    /// let allocator = <GenIndexAllocator>::with_capacity(10);
    /// assert_eq!(allocator.capacity(), 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.indices.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the given allocator.
    /// The collection may reserve more space to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to self.len() + additional. Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// allocator.reserve(10);
    /// assert!(allocator.capacity() >= 10);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional);
    }

    /// Clears the allocator, removing all values.
    /// Note that this method has no effect on the allocated capacity of the allocator.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexAllocator;
    /// # let mut allocator = <GenIndexAllocator>::new();
    /// allocator.create();
    /// allocator.create();
    /// allocator.clear();
    /// assert!(allocator.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.free_list_head = I::Index::zero();
        self.free_list_size = self.indices.len();
        for (i, index) in self.indices.iter_mut().enumerate() {
            *index = I::from_raw_parts(
                NumCast::from(i + 1).unwrap_or(I::Index::zero()),
                index.generation(),
            );
        }
    }

    /// Creates and returns the next index, allocating more capacity if necessary.
    ///
    /// # Panics
    /// Panics if the capacity overflows.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// let i = &allocator.create();
    /// assert!(allocator.contains(i));
    /// ```
    pub fn create(&mut self) -> I {
        self.alloc_free();
        let free_index = self.free_list_head;
        let gen_index = &mut self.indices[free_index.to_usize().expect("index out of bounds")];
        self.free_list_head = gen_index.index();
        self.free_list_size -= 1;
        let mut gen = gen_index.generation();
        if gen < I::max_generation() {
            gen = gen + num::one();
        } else {
            // Avoid (0, 0) index (which represents null) in case of overflow
            gen = if free_index.is_zero() {
                num::one()
            } else {
                num::zero()
            }
        }
        *gen_index = I::from_raw_parts(free_index, gen);
        *gen_index
    }

    /// Removes index `i` from the allocator if exists.
    /// Returns a bool indicating whether the allocator originally contains the index.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// let i = &allocator.create();
    /// assert!(allocator.remove(i));
    /// assert!(!allocator.remove(i));
    /// ```
    pub fn remove(&mut self, i: &I) -> bool {
        if self.contains(i) {
            let next_free_index = i.index();
            if let Some(next_free_index_usize) = next_free_index.to_usize() {
                let next_free = &mut self.indices[next_free_index_usize];
                *next_free = I::from_raw_parts(self.free_list_head, next_free.generation());
                self.free_list_head = next_free_index;
                self.free_list_size += 1;
                return true;
            }
        }
        false
    }

    /// Returns true if the allocator contains the index `i` .
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// let i = &allocator.create();
    /// assert!(allocator.contains(i));
    /// allocator.remove(i);
    /// assert!(!allocator.contains(i));
    /// ```
    pub fn contains(&self, i: &I) -> bool {
        if let Some(idx) = i.index().to_usize() {
            if let Some(gen_index) = self.indices.get(idx) {
                if i == gen_index {
                    return true;
                }
            }
        }
        false
    }

    /// Given an index without a generation, get the `GenIndex` at that slot.
    ///
    /// # Examples
    /// ```
    /// # use muds::{GenIndex, collections::GenIndexAllocator};
    /// let mut allocator = <GenIndexAllocator>::new();
    /// let i = allocator.create();
    /// assert_eq!(i, *allocator.get(&i.index()).unwrap());
    /// ```
    pub fn get(&self, idx: &I::Index) -> Option<&I> {
        self.indices.get(idx.to_usize()?)
    }

    /// Retains only the indices specified by the predicate.
    /// In other words, removes all indices such that `f(index)` returns `false`.
    ///
    /// # Examples
    /// ```
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// let idx1 = &allocator.create();
    /// let idx2 = &allocator.create();
    /// allocator.retain(|idx| idx == idx1);
    /// assert!(allocator.contains(idx1));
    /// assert!(!allocator.contains(idx2));
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&I) -> bool) {
        for i in 0..self.indices.len() {
            {
                let gen_index = &self.indices[i];
                match gen_index.index().to_usize() {
                    Some(idx) if i == idx && !f(gen_index) => Some(*gen_index),
                    _ => None,
                }
            }
            .map(|ref gen_index| self.remove(gen_index));
        }
    }

    /// Returns an iterator over the allocator.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::GenIndexAllocator;
    /// let mut allocator = <GenIndexAllocator>::new();
    /// for i in 0..10 {
    ///     allocator.create();
    /// }
    ///
    /// for idx in &allocator {
    ///     println!("{:?}", idx);
    /// }
    pub fn iter(&self) -> core::slice::Iter<I> {
        self.indices.iter()
    }

    #[inline]
    fn alloc_free(&mut self) {
        if self.free_list_size > 0 {
            return;
        }
        self.free_list_head = NumCast::from(self.indices.len()).expect("index out of bounds");
        self.free_list_size = 1;
        self.indices.push(I::from_raw_parts(
            if self.free_list_head.is_zero() {
                num::one()
            } else {
                num::zero()
            },
            num::zero(),
        ));
    }
}

impl<I: GenIndex> Default for GenIndexAllocator<I> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, I: GenIndex> IntoIterator for &'a GenIndexAllocator<I> {
    type Item = &'a I;
    type IntoIter = core::slice::Iter<'a, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::GenIndexAllocator;
    use crate::GenIndex;
    use alloc::vec::Vec;
    use num::{ToPrimitive, Zero};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<I: GenIndex + Serialize> Serialize for GenIndexAllocator<I> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.indices.serialize(serializer)
        }
    }

    impl<'de, I: GenIndex + Deserialize<'de>> Deserialize<'de> for GenIndexAllocator<I> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let mut indices: Vec<I> = Deserialize::deserialize(deserializer)?;
            let mut free_list_head = num::zero();
            let mut free_list_size = 0;

            // Rebuild free list in asc order.
            for (i, gen_index) in indices.iter_mut().enumerate() {
                let index = gen_index.index();
                match index.to_usize() {
                    Some(idx) if i == idx => (),
                    _ => {
                        // Index not match => free index
                        let next_index = if free_list_size > 0 || !index.is_zero() {
                            free_list_head
                        } else {
                            num::one()
                        };
                        *gen_index = I::from_raw_parts(next_index, gen_index.generation());
                        free_list_head = index;
                        free_list_size += 1;
                    }
                }
            }

            Ok(GenIndexAllocator {
                indices,
                free_list_head,
                free_list_size,
            })
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::GenIndexAllocator;
    use crate::{GenIndex, Index};
    use alloc::vec;
    use serde_json::{json, Value};

    #[test]
    fn test_serialize() {
        let mut allocator = GenIndexAllocator::<Index>::with_capacity(3);
        let idx0 = allocator.create();
        allocator.create();
        allocator.remove(&idx0);
        allocator.create();

        let expected_json: Value = json!([[0, 2], [1, 1]]);

        let json: Value = serde_json::to_value(allocator).unwrap();

        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_deserialize() {
        let json: Value = json!([[0, 2], [1, 1], [2, 1], [100, 0]]);

        let allocator: GenIndexAllocator<Index> = serde_json::from_value(json).unwrap();

        assert_eq!(allocator.len(), 3);
        assert!(allocator.contains(&Index::from_raw_parts(0, 2)));
        assert!(allocator.contains(&Index::from_raw_parts(1, 1)));
        assert!(allocator.contains(&Index::from_raw_parts(2, 1)));
    }
}
