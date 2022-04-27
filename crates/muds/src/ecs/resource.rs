//! Resource locator traits.

use super::{
    registry::{Ref, RefMut},
    Component, Entity,
};
use core::any::Any;

/// Registry of resources.
pub trait Resources {
    /// Registers a resource type.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Resources};
    /// let mut registry = Registry::default();
    /// registry.register_resource(1u32);
    /// ```
    fn register_resource<R: Any>(&mut self, value: R);

    /// Returns if a resource type is registered.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Resources};
    /// let mut registry = Registry::default();
    /// registry.register_resource(1u32);
    /// assert!(registry.has_resource::<u32>());
    /// ```
    fn has_resource<R: Any>(&self) -> bool;

    /// Gets a resource.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Resources};
    /// let mut registry = Registry::default();
    /// registry.register_resource(1u32);
    /// assert_eq!(*registry.resource::<u32>(), 1u32);
    /// ```
    fn resource<'a, R: Any>(&'a self) -> Ref<'a, R>;

    /// Gets a resource mutably.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Resources};
    /// let mut registry = Registry::default();
    /// registry.register_resource(1u32);
    /// *registry.resource_mut::<u32>() = 2u32;
    /// assert_eq!(*registry.resource::<u32>(), 2u32);
    /// ```
    fn resource_mut<'a, R: Any>(&'a self) -> RefMut<'a, R>;
}

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
    fn components_mut<'a, E: Entity + Any, C: Component<E> + Any>(&'a self) -> RefMut<'a, C::Storage>;
}
