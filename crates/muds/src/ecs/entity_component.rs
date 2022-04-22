//! Entity and component types.

use crate::{
    collections::{Arena, IterableMapMut, MapMut},
    TypedIndex,
};

/// Gen index type for ECS.
#[cfg(feature = "index-u64")]
pub type GenIndexType = crate::IndexU64;
/// Gen index type for ECS.
#[cfg(not(feature = "index-u64"))]
pub type GenIndexType = crate::IndexF64;

/// Entity ID type.
pub type EntityId<E> = TypedIndex<E, GenIndexType>;

/// Entity type.
pub trait Entity: Sized {
    /// Entity storage type.
    type Storage: EntityStorage<Self>;
}

/// Entity storage trait type.
pub trait EntityStorage<E: Entity>:
    Default + Arena<Key = EntityId<E>, Value = E> + for<'a> IterableMapMut<'a> + 'static
{
}

/// Component type.
pub trait Component<E: Entity>: Sized {
    /// Component storage type.
    type Storage: ComponentStorage<E, Self>;
}

/// Entity component storage trait type.
pub trait ComponentStorage<E: Entity, C: Component<E>>:
    Default + MapMut<Key = EntityId<E>, Value = C> + for<'a> IterableMapMut<'a> + 'static
{
}
