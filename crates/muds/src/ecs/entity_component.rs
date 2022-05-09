//! Entity and component types.

use super::registry::{Ref, RefMut};
use crate::{
    collections::{Arena, IterableMapMut, MapMut},
    TypedIndex,
};
use core::any::Any;

/// Gen index type for ECS.
#[cfg(feature = "index-u64")]
pub type GenIndexType = crate::IndexU64;
/// Gen index type for ECS.
#[cfg(not(feature = "index-u64"))]
pub type GenIndexType = crate::IndexF64;

/// [Entity] ID type.
pub type EntityId<E> = TypedIndex<E, GenIndexType>;

/// Entity type.
pub trait Entity: Sized {
    /// Entity storage type.
    type Storage: EntityStorage<Self>;
}

/// [Entity] storage trait type.
pub trait EntityStorage<E: Entity>:
    Default + Arena<Key = EntityId<E>, Value = E> + for<'a> IterableMapMut<'a> + 'static
{
}

/// Component type.
pub trait Component<E: Entity>: Sized {
    /// Component storage type.
    type Storage: ComponentStorage<E, Self>;
}

/// [Component] storage trait type.
pub trait ComponentStorage<E: Entity, C: Component<E>>:
    Default + MapMut<Key = EntityId<E>, Value = C> + for<'a> IterableMapMut<'a> + 'static
{
}

/// Type alias for the storage of an [Entity].
pub type EntityStorageOf<E> = <E as Entity>::Storage;

/// Type alias for the storage of a [Component].
pub type ComponentStorageOf<E, C> = <C as Component<E>>::Storage;

/// Registry of entities.
pub trait Entities {
    /// Registers an entity type.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Entities, Entity, storage::ArenaStorage};
    /// struct Pos(u32, u32);
    /// impl Entity for Pos { type Storage = ArenaStorage<Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<Pos>();
    /// ```
    fn register_entity<E: Entity + Any>(&mut self);

    /// Returns if an entity type is registered.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Entities, Entity, storage::ArenaStorage};
    /// struct Pos(u32, u32);
    /// impl Entity for Pos { type Storage = ArenaStorage<Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<Pos>();
    /// assert!(registry.has_entity::<Pos>());
    /// ```
    fn has_entity<E: Entity + Any>(&self) -> bool;

    /// Gets an entity storage.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Entities, Entity, storage::ArenaStorage};
    /// struct Pos(u32, u32);
    /// impl Entity for Pos { type Storage = ArenaStorage<Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<Pos>();
    /// let e = registry.entities::<Pos>();
    /// assert_eq!(e.len(), 0);
    /// ```
    fn entities<'a, E: Entity + Any>(&'a self) -> Ref<'a, E::Storage>;

    /// Gets an entity storage mutably.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Entities, Entity, storage::ArenaStorage};
    /// struct Pos(u32, u32);
    /// impl Entity for Pos { type Storage = ArenaStorage<Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<Pos>();
    /// let mut e = registry.entities_mut::<Pos>();
    /// e.insert(Pos(1, 2));
    /// assert_eq!(e.len(), 1);
    /// ```
    fn entities_mut<'a, E: Entity + Any>(&'a self) -> RefMut<'a, E::Storage>;
}

/// Registry for components.
pub trait Components {
    /// Registers an optional component type for an entity type.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Component, Components, Entity, Entities, storage::{ArenaStorage, VecStorage}};
    /// struct E;
    /// struct Pos(u32, u32);
    /// impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<E>();
    /// registry.register_component::<E, Pos>();
    /// ```
    fn register_component<E: Entity + Any, C: Component<E> + Any>(&mut self);

    /// Returns if a component type is registered.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Component, Components, Entity, Entities, storage::{ArenaStorage, VecStorage}};
    /// struct E;
    /// struct Pos(u32, u32);
    /// impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<E>();
    /// registry.register_component::<E, Pos>();
    /// assert!(registry.has_component::<E, Pos>());
    /// ```
    fn has_component<E: Entity + Any, C: Component<E> + Any>(&self) -> bool;

    /// Gets a component storage.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::Map;
    /// # use muds::ecs::{Registry, Component, Components, Entity, Entities, storage::{ArenaStorage, VecStorage}};
    /// struct E;
    /// struct Pos(u32, u32);
    /// impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<E>();
    /// registry.register_component::<E, Pos>();
    /// let c = registry.components::<E, Pos>();
    /// assert_eq!(c.len(), 0);
    /// ```
    fn components<'a, E: Entity + Any, C: Component<E> + Any>(&'a self) -> Ref<'a, C::Storage>;

    /// Gets a component storage mutably.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::collections::{Map, MapMut};
    /// # use muds::ecs::{Registry, Component, Components, Entity, Entities, storage::{ArenaStorage, VecStorage}};
    /// struct E;
    /// struct Pos(u32, u32);
    /// impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<E>();
    /// registry.register_component::<E, Pos>();
    /// let mut e = registry.entities_mut::<E>();
    /// let mut c = registry.components_mut::<E, Pos>();
    ///
    /// c.insert(e.insert(E), Pos(1, 2));
    /// assert_eq!(c.len(), 1);
    /// ```
    fn components_mut<'a, E: Entity + Any, C: Component<E> + Any>(
        &'a self,
    ) -> RefMut<'a, C::Storage>;
}
