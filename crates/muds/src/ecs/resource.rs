//! Resource locator traits.

use super::{Component, Entity};
use core::{
    any::Any,
    ops::{Deref, DerefMut},
};

/// Resource marker trait.
pub trait Resource: Any {}

impl<T: Any> Resource for T {}

/// Locator of a resource type.
pub trait ResourceLocator<'a, R: Resource> {
    /// Resource reference type.
    type Ref: Deref<Target = R> + 'a;

    /// Mutable resource reference type.
    type RefMut: DerefMut<Target = R> + 'a;

    /// Registers a resource.
    fn register(&mut self, value: R);

    /// Returns `true` if resource type `R` is registered.
    fn has(&self) -> bool;

    /// Gets resource of type `R`.
    fn get(&'a self) -> Self::Ref;

    /// Gets resource of type `R` mutably.
    fn get_mut(&'a self) -> Self::RefMut;
}

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
    #[inline]
    fn register_resource<R: Resource>(&mut self, value: R)
    where
        for<'a> Self: ResourceLocator<'a, R>,
    {
        self.register(value)
    }

    /// Returns if a resource type is registered.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Resources};
    /// let mut registry = Registry::default();
    /// registry.register_resource(1u32);
    /// assert!(registry.has_resource::<u32>());
    /// ```
    #[inline]
    fn has_resource<R: Resource>(&self) -> bool
    where
        for<'a> Self: ResourceLocator<'a, R>,
    {
        ResourceLocator::<R>::has(self)
    }

    /// Gets a resource.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, Resources};
    /// let mut registry = Registry::default();
    /// registry.register_resource(1u32);
    /// assert_eq!(*registry.resource::<u32>(), 1u32);
    /// ```
    #[inline]
    fn resource<'a, R: Resource>(&'a self) -> <Self as ResourceLocator<'a, R>>::Ref
    where
        Self: ResourceLocator<'a, R>,
    {
        ResourceLocator::<R>::get(self)
    }

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
    #[inline]
    fn resource_mut<'a, R: Resource>(&'a self) -> <Self as ResourceLocator<'a, R>>::RefMut
    where
        Self: ResourceLocator<'a, R>,
    {
        ResourceLocator::<R>::get_mut(self)
    }
}

/// Registry of entities.
pub trait Entities: Resources {
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
    #[inline]
    fn register_entity<E: Entity>(&mut self)
    where
        for<'a> Self: ResourceLocator<'a, E::Storage>,
    {
        self.register(E::Storage::default())
    }

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
    #[inline]
    fn has_entity<E: Entity>(&self) -> bool
    where
        for<'a> Self: ResourceLocator<'a, E::Storage>,
    {
        ResourceLocator::<E::Storage>::has(self)
    }

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
    #[inline]
    fn entities<'a, E: Entity>(&'a self) -> <Self as ResourceLocator<'a, E::Storage>>::Ref
    where
        Self: ResourceLocator<'a, E::Storage>,
    {
        ResourceLocator::<E::Storage>::get(self)
    }

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
    #[inline]
    fn entities_mut<'a, E: Entity>(&'a self) -> <Self as ResourceLocator<'a, E::Storage>>::RefMut
    where
        Self: ResourceLocator<'a, E::Storage>,
    {
        ResourceLocator::<E::Storage>::get_mut(self)
    }
}

impl<T: Resources> Entities for T {}

/// Registry for components.
pub trait Components: Resources {
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
    #[inline]
    fn register_component<E: Entity, C: Component<E>>(&mut self)
    where
        for<'a> Self: ResourceLocator<'a, C::Storage>,
    {
        self.register(Default::default())
    }

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
    #[inline]
    fn has_component<E: Entity, C: Component<E>>(&self) -> bool
    where
        for<'a> Self: ResourceLocator<'a, C::Storage>,
    {
        ResourceLocator::<C::Storage>::has(self)
    }

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
    #[inline]
    fn components<'a, E: Entity, C: Component<E>>(
        &'a self,
    ) -> <Self as ResourceLocator<'a, C::Storage>>::Ref
    where
        Self: ResourceLocator<'a, C::Storage>,
    {
        ResourceLocator::<C::Storage>::get(self)
    }

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
    #[inline]
    fn components_mut<'a, E: Entity, C: Component<E>>(
        &'a self,
    ) -> <Self as ResourceLocator<'a, C::Storage>>::RefMut
    where
        Self: ResourceLocator<'a, C::Storage>,
    {
        ResourceLocator::<C::Storage>::get_mut(self)
    }
}

impl<T: Resources> Components for T {}
