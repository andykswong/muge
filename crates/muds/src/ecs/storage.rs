//! Entity and component storages.

use super::{Component, ComponentStorage, Entity, EntityId, EntityStorage};
use crate::collections::{GenIndexArena, GenIndexBTreeMap, GenIndexSparseSet, GenIndexVecMap};

/// Entity storage backed by a `GenIndexArena`.
pub type ArenaStorage<E> = GenIndexArena<E, EntityId<E>>;

/// Entity storage backed by a `GenIndexArena`.
impl<E: Entity + 'static> EntityStorage<E> for ArenaStorage<E> {}

/// Component storage backed by a `SparseSet`.
pub type SparseSetStorage<E, C> = GenIndexSparseSet<C, EntityId<E>>;

impl<E: Entity + 'static, C: Component<E> + 'static> ComponentStorage<E, C>
    for SparseSetStorage<E, C>
{
}

/// Component storage backed by a `VecMap`.
pub type VecStorage<E, C> = GenIndexVecMap<C, EntityId<E>>;

impl<E: Entity + 'static, C: Component<E> + 'static> ComponentStorage<E, C> for VecStorage<E, C> {}

/// Component storage backed by a `BTreeMap`.
pub type BTreeStorage<E, C> = GenIndexBTreeMap<C, EntityId<E>>;

impl<E: Entity + 'static, C: Component<E> + 'static> ComponentStorage<E, C> for BTreeStorage<E, C> {}

/// Component storage backed by a `HashMap`.
#[cfg(feature = "std")]
pub type HashMapStorage<E, C> = crate::collections::GenIndexHashMap<C, EntityId<E>>;

#[cfg(feature = "std")]
impl<E: Entity + 'static, C: Component<E> + 'static> ComponentStorage<E, C>
    for HashMapStorage<E, C>
{
}
