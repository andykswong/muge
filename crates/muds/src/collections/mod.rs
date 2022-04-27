//! Collection library.

mod genindexmaps;
mod map;
mod maps;

pub use genindexmaps::*;
pub use maps::*;

pub mod allocator;
pub mod arena;
pub mod genindexmap;
pub mod join;
pub mod sparseset;
pub mod tuple;
pub mod vecmap;

pub use allocator::GenIndexAllocator;
pub use arena::GenIndexArena;
pub use genindexmap::GenIndexMap;
pub use sparseset::SparseSet;
pub use vecmap::VecMap;

/// All helper traits.
pub mod traits {
    pub use super::map::*;
    pub use super::join::MapJoin;
    pub use super::tuple::Cons;
}

pub use traits::*;
