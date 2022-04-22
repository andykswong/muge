//! Generational index maps.

use super::{GenIndexMap, SparseSet, VecMap};
use crate::{GenIndex, IndexF64};
use alloc::collections::BTreeMap;

/// A `GenIndexMap` backed by a `SparseSet`
pub type GenIndexSparseSet<T, I = IndexF64> =
    GenIndexMap<T, I, SparseSet<(I, T), <I as GenIndex>::Index>>;

/// A `GenIndexMap` backed by a `VecMap`
pub type GenIndexVecMap<T, I = IndexF64> =
    GenIndexMap<T, I, VecMap<(I, T), <I as GenIndex>::Index>>;

/// A `GenIndexMap` backed by a `BTreeMap`
pub type GenIndexBTreeMap<T, I = IndexF64> =
    GenIndexMap<T, I, BTreeMap<<I as GenIndex>::Index, (I, T)>>;

#[cfg(feature = "std")]
pub type GenIndexHashMap<T, I = IndexF64> =
    GenIndexMap<T, I, std::collections::HashMap<<I as GenIndex>::Index, (I, T)>>;

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn test_genindex_btreemap_serialize() {
        use super::GenIndexBTreeMap;
        use crate::{GenIndex, Index};
        use alloc::collections::BTreeMap;
        use serde_json::Value;

        let mut map = GenIndexBTreeMap::default();
        map.insert(Index::from_raw_parts(1usize, 2usize), "a");
        map.insert(Index::from_raw_parts(0, 3), "b");
        map.insert(Index::from_raw_parts(4, 5), "c");

        let mut btree = BTreeMap::default();
        btree.insert(1, (Index::from_raw_parts(1usize, 2usize), "a"));
        btree.insert(0, (Index::from_raw_parts(0, 3), "b"));
        btree.insert(4, (Index::from_raw_parts(4, 5), "c"));

        let expected_json: Value = serde_json::to_value(btree).unwrap();
        let json: Value = serde_json::to_value(map).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_genindex_btreemap_deserialize() {
        use super::GenIndexBTreeMap;
        use crate::{GenIndex, Index};
        use alloc::collections::BTreeMap;
        use alloc::string::String;
        use serde_json::Value;

        let mut btree = BTreeMap::default();
        btree.insert(1usize, (Index::from_raw_parts(1usize, 2usize), "a"));
        btree.insert(3, (Index::from_raw_parts(3, 4), "c"));

        let json: Value = serde_json::to_value(btree).unwrap();

        let map: GenIndexBTreeMap<String, Index> = serde_json::from_value(json).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map[Index::from_raw_parts(1, 2)], "a");
        assert_eq!(map[Index::from_raw_parts(3, 4)], "c");
    }

    #[cfg(all(feature = "serde", feature = "std"))]
    #[test]
    fn test_genindex_hashmap_serialize() {
        use super::GenIndexHashMap;
        use crate::{GenIndex, Index};
        use serde_json::Value;
        use std::collections::HashMap;

        let mut map = GenIndexHashMap::default();
        map.insert(Index::from_raw_parts(1usize, 2usize), "a");
        map.insert(Index::from_raw_parts(0, 3), "b");
        map.insert(Index::from_raw_parts(4, 5), "c");

        let mut btree = HashMap::<usize, (Index, &str)>::default();
        btree.insert(1, (Index::from_raw_parts(1usize, 2usize), "a"));
        btree.insert(0, (Index::from_raw_parts(0, 3), "b"));
        btree.insert(4, (Index::from_raw_parts(4, 5), "c"));

        let expected_json: Value = serde_json::to_value(btree).unwrap();
        let json: Value = serde_json::to_value(map).unwrap();

        assert_eq!(json, expected_json);
    }

    #[cfg(all(feature = "serde", feature = "std"))]
    #[test]
    fn test_genindex_hashmap_deserialize() {
        use super::GenIndexHashMap;
        use crate::{GenIndex, Index};
        use alloc::string::String;
        use serde_json::Value;
        use std::collections::HashMap;

        let mut btree = HashMap::<usize, (Index, &str)>::default();
        btree.insert(1usize, (Index::from_raw_parts(1usize, 2usize), "a"));
        btree.insert(3, (Index::from_raw_parts(3, 4), "c"));

        let json: Value = serde_json::to_value(btree).unwrap();

        let map: GenIndexHashMap<String, Index> = serde_json::from_value(json).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(map[Index::from_raw_parts(1, 2)], "a");
        assert_eq!(map[Index::from_raw_parts(3, 4)], "c");
    }
}
