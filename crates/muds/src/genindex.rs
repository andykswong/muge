//! Generational indices.

use crate::UnsignedNum;
use core::{
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use num::Bounded;

/// An index with generation that can be used as a weak reference to array values.
/// The generation part allows indices to be reused without suffering from [ABA problem](https://en.wikipedia.org/wiki/ABA_problem),
/// so that data can be safely stored in a packed array.
pub trait GenIndex: Copy + Debug + Default + Hash + PartialEq + PartialOrd {
    /// The type of index value.
    type Index: UnsignedNum;

    /// The type of generation value.
    type Generation: UnsignedNum;

    /// Returns the maximum generation value.
    fn max_generation() -> Self::Generation;

    /// Create a new `GenIndex` from its raw parts.
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self;

    /// Returns the index value of this `GenIndex`.
    fn index(&self) -> Self::Index;

    /// Returns the generation value of this `GenIndex`.
    fn generation(&self) -> Self::Generation;

    /// Returns a null value.
    #[inline]
    fn null() -> Self {
        Default::default()
    }

    /// Checks if the value represents null.
    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::null()
    }
}

// region: Index

/// A standard [GenIndex] with usize index and usize generation
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Index<I: UnsignedNum = usize, G: Bounded + UnsignedNum = usize>(I, G);

impl<I: UnsignedNum, G: Bounded + UnsignedNum> Default for Index<I, G> {
    fn default() -> Self {
        Self::from_raw_parts(I::zero(), G::zero())
    }
}

impl<I: UnsignedNum, G: Bounded + UnsignedNum> GenIndex for Index<I, G> {
    type Index = I;
    type Generation = G;

    /// Returns the maximum generation value.
    #[inline]
    fn max_generation() -> Self::Generation {
        G::max_value()
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self(index, generation)
    }

    #[inline]
    fn index(&self) -> Self::Index {
        self.0
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        self.1
    }
}

impl<I: UnsignedNum, G: Bounded + UnsignedNum> From<Index<I, G>> for (I, G) {
    #[inline]
    fn from(idx: Index<I, G>) -> Self {
        (idx.0, idx.1)
    }
}

impl<I: UnsignedNum, G: Bounded + UnsignedNum> From<(I, G)> for Index<I, G> {
    #[inline]
    fn from((index, generation): (I, G)) -> Self {
        Index::from_raw_parts(index, generation)
    }
}

// endregion: Index

// region: IndexF64

/// A [GenIndex] that is stored as f64, which 32bit index and 21bit generation.
/// Useful for interfacing with Javascript
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct IndexF64(f64);

/// Equals 2^21 - 1. f64 can safely store integer up to 2^53 - 1.
/// We used 32bits for the index, leaving 21bits for generation.
const MAX_SAFE_F64_GENERATION: u32 = (1 << 21) - 1;

impl GenIndex for IndexF64 {
    type Index = u32;
    type Generation = u32;

    #[inline]
    fn max_generation() -> Self::Generation {
        MAX_SAFE_F64_GENERATION
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self(index as f64 + (((generation & Self::max_generation()) as u64) << 32) as f64)
    }

    #[inline]
    fn index(&self) -> Self::Index {
        (self.0 as u64 & (u32::MAX as u64)) as u32
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        ((self.0 as u64) >> 32) as u32
    }
}

impl Hash for IndexF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0 as i64).hash(state);
    }
}

impl From<IndexF64> for (u32, u32) {
    #[inline]
    fn from(idx: IndexF64) -> Self {
        (idx.index(), idx.generation())
    }
}

impl From<(u32, u32)> for IndexF64 {
    #[inline]
    fn from((index, generation): (u32, u32)) -> Self {
        IndexF64::from_raw_parts(index, generation)
    }
}

// endregion: IndexF64

// region: IndexU64

/// A [GenIndex] that is stored as u64, which 32bit index and 32bit generation.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct IndexU64(u64);

impl GenIndex for IndexU64 {
    type Index = u32;
    type Generation = u32;

    #[inline]
    fn max_generation() -> Self::Generation {
        u32::MAX
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self(index as u64 + ((generation as u64) << 32))
    }

    #[inline]
    fn index(&self) -> Self::Index {
        (self.0 & (u32::MAX as u64)) as u32
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        (self.0 >> 32) as u32
    }
}

impl From<IndexU64> for (u32, u32) {
    #[inline]
    fn from(idx: IndexU64) -> Self {
        (idx.index(), idx.generation())
    }
}

impl From<(u32, u32)> for IndexU64 {
    #[inline]
    fn from((index, generation): (u32, u32)) -> Self {
        IndexU64::from_raw_parts(index, generation)
    }
}

// endregion: IndexU64

// region: TypedIndex

/// A [GenIndex] newtype.
#[derive(Eq, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct TypedIndex<T, I: GenIndex = IndexF64> {
    index: I,
    marker: PhantomData<*const T>,
}

impl<T, I: GenIndex> TypedIndex<T, I> {
    #[inline]
    pub fn from_index(index: I) -> Self {
        Self {
            index,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn to_index(&self) -> I {
        self.index
    }
}

impl<T, I: GenIndex> Clone for TypedIndex<T, I> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_index(self.index.clone())
    }
}

impl<T, I: GenIndex> Copy for TypedIndex<T, I> {}

impl<T, I: GenIndex> Default for TypedIndex<T, I> {
    #[inline]
    fn default() -> Self {
        Self::from_index(Default::default())
    }
}

impl<T, I: GenIndex> Debug for TypedIndex<T, I> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.index.fmt(f)
    }
}

impl<T, I: GenIndex> Hash for TypedIndex<T, I> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<T, I: GenIndex> PartialOrd for TypedIndex<T, I> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<T, I: GenIndex> PartialEq for TypedIndex<T, I> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl<T, I: GenIndex> GenIndex for TypedIndex<T, I> {
    type Index = I::Index;

    type Generation = I::Generation;

    #[inline]
    fn max_generation() -> Self::Generation {
        I::max_generation()
    }

    #[inline]
    fn from_raw_parts(index: Self::Index, generation: Self::Generation) -> Self {
        Self {
            index: I::from_raw_parts(index, generation),
            marker: PhantomData,
        }
    }

    #[inline]
    fn index(&self) -> Self::Index {
        self.index.index()
    }

    #[inline]
    fn generation(&self) -> Self::Generation {
        self.index.generation()
    }
}

impl<T, I: GenIndex> From<TypedIndex<T, I>> for (I::Index, I::Generation) {
    #[inline]
    fn from(idx: TypedIndex<T, I>) -> Self {
        (idx.index(), idx.generation())
    }
}

impl<T, I: GenIndex> From<(I::Index, I::Generation)> for TypedIndex<T, I> {
    #[inline]
    fn from((index, generation): (I::Index, I::Generation)) -> Self {
        TypedIndex::from_raw_parts(index, generation)
    }
}

// endregion: TypedIndex

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn test_indexf64_deserialize() {
        use crate::{GenIndex, IndexF64};
        use serde_json::{json, Value};

        let expected_index = IndexF64::from_raw_parts(123, 456);
        let json: Value = json!((456u64 << 32 | 123) as f64);

        let index: IndexF64 = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_indexf64_serialize() {
        use crate::{GenIndex, IndexF64};
        use serde_json::{json, Value};

        let index = IndexF64::from_raw_parts(123, 456);
        let expected_json: Value = json!((456u64 << 32 | 123) as f64);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_index_deserialize() {
        use crate::{GenIndex, Index};
        use alloc::vec;
        use serde_json::{json, Value};

        let expected_index = Index::from_raw_parts(123, 456);
        let json: Value = json!([123, 456]);

        let index: Index = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_index_serialize() {
        use crate::{GenIndex, Index};
        use alloc::vec;
        use serde_json::{json, Value};

        let index: Index = Index::from_raw_parts(123, 456);
        let expected_json: Value = json!([123, 456]);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_typedindex_deserialize() {
        use crate::{GenIndex, Index, TypedIndex};
        use alloc::vec;
        use serde_json::{json, Value};

        struct TestType;

        let expected_index = TypedIndex::<TestType, Index>::from_raw_parts(123, 456);
        let json: Value = json!([123, 456]);

        let index: TypedIndex<TestType, Index> = serde_json::from_value(json).unwrap();

        assert_eq!(index, expected_index);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_typedindex_serialize() {
        use crate::{GenIndex, Index, TypedIndex};
        use alloc::vec;
        use serde_json::{json, Value};

        struct TestType;

        let index = TypedIndex::<TestType, Index>::from_raw_parts(123, 456);
        let expected_json: Value = json!([123, 456]);

        let json: Value = serde_json::to_value(index).unwrap();

        assert_eq!(json, expected_json);
    }
}
