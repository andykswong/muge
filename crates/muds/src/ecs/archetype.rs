//! Archetype helpers.

use super::{
    registry::{Ref, RefMut},
    Component, Components, Entities, Entity, EntityId, Resources,
};
use crate::collections::Cons;
use alloc::vec::Vec;
use core::any::Any;

/// Registry for archetypes, which represent a bundle of an [Entity] and its [Component]s.
///
/// # Examples
/// ```rust
/// # use muds::{cons, Cons};
/// # use muds::collections::{IterableMapMut, MapMut, MapJoin};
/// # use muds::ecs::{Archetypes, Registry, Component, Entity, storage::{ArenaStorage, VecStorage}};
/// struct E;
/// struct Pos(u32, u32);
/// struct Vel(u32, u32);
/// impl Entity for E { type Storage = ArenaStorage<Self>; }
/// impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
/// impl Component<E> for Vel { type Storage = VecStorage<E, Self>; }
///
/// let mut registry = Registry::default();
/// registry.register_archetype::<E, Cons!(Pos, Vel)>();
/// let cons!(mut ent, mut pos, mut vel) = registry.storage::<&mut E, Cons!(&mut Pos, &mut Vel)>();
/// for i in 0..10 {
///     let eid = ent.insert(E);
///     pos.insert(eid, Pos(i * 2, i * 2 + 1));
///     vel.insert(eid, Vel(i, i + 1));
/// }
///
/// for cons!(_id, v, p) in pos.iter_mut().cons().map_join(&*vel) {
///     p.0 += v.0;
///     p.1 += v.1;
/// }
/// ```
pub trait Archetypes: Entities + Components + Resources + Sized {
    /// Registers an [Entity] and its [Component]s.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::Cons;
    /// # use muds::ecs::{Archetypes, Registry, Component, Entity, storage::{ArenaStorage, VecStorage}};
    /// # struct E;
    /// # struct Pos(u32, u32);
    /// # struct Vel(u32, u32);
    /// # impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// # impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    /// # impl Component<E> for Vel { type Storage = VecStorage<E, Self>; }
    /// let mut registry = Registry::default();
    /// registry.register_archetype::<E, Cons!(Pos, Vel)>();
    /// ```
    #[inline]
    fn register_archetype<E: Entity + Any, C: Cons>(&mut self)
    where
        (E, C): RegisterArchetype<Self>,
    {
        <(E, C)>::register(self);
    }

    /// Gets the mutable or immutable storages of an [Entity] and its [Component]s.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{cons, Cons};
    /// # use muds::collections::MapMut;
    /// # use muds::ecs::{Archetypes, Registry, Component, Entity, storage::{ArenaStorage, VecStorage}};
    /// # struct E;
    /// # struct Pos(u32, u32);
    /// # struct Vel(u32, u32);
    /// # impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// # impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    /// # impl Component<E> for Vel { type Storage = VecStorage<E, Self>; }
    /// let mut registry = Registry::default();
    /// registry.register_archetype::<E, Cons!(Pos, Vel)>();
    /// let cons!(mut e, mut p, mut v) = registry.storage::<&mut E, Cons!(&mut Pos, &mut Vel)>();
    /// let eid = e.insert(E);
    /// p.insert(eid, Pos(1, 2));
    /// v.insert(eid, Vel(1, 2));
    /// ```
    #[inline]
    fn storage<'a, E, C: Cons>(&'a self) -> <(E, C) as ArchetypeStorage<'a, Self>>::Storage
    where
        (E, C): ArchetypeStorage<'a, Self>,
    {
        <(E, C)>::storage(self)
    }

    /// Inserts an [Entity] and its [Component]s.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{cons, Cons};
    /// # use muds::ecs::{Archetypes, Registry, Component, Entity, storage::{ArenaStorage, VecStorage}};
    /// # struct E;
    /// # struct Pos(u32, u32);
    /// # struct Vel(u32, u32);
    /// # impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// # impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    /// # impl Component<E> for Vel { type Storage = VecStorage<E, Self>; }
    /// let mut registry = Registry::default();
    /// registry.register_archetype::<E, Cons!(Pos, Vel)>();
    /// let eid = registry.insert_archetype(E, cons!(Pos(1, 2), Vel(1, 2)));
    /// ```
    #[inline]
    fn insert_archetype<E: Entity + Any, C: Cons>(&self, entity: E, components: C) -> EntityId<E>
    where
        Self: Any,
        C: InsertComponents<Self, E>,
    {
        self.resource::<Archetype<Self, E>>()
            .insert(self, entity, components)
    }

    /// Removes an [Entity] and all of its [Component]s by its [EntityId].
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{cons, Cons};
    /// # use muds::collections::{Map, MapMut};
    /// # use muds::ecs::{Archetypes, Registry, Component, Entity, storage::{ArenaStorage, VecStorage}};
    /// # struct E;
    /// # struct Pos(u32, u32);
    /// # struct Vel(u32, u32);
    /// # impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// # impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    /// # impl Component<E> for Vel { type Storage = VecStorage<E, Self>; }
    /// let mut registry = Registry::default();
    /// registry.register_archetype::<E, Cons!(Pos, Vel)>();
    /// let eid = registry.insert_archetype(E, cons!(Pos(1, 2), Vel(1, 2)));
    /// registry.remove_archetype(&eid);
    /// {
    ///   let cons!(e, p, v) = registry.storage::<&E, Cons!(&Pos, &Vel)>();
    ///   assert!(e.is_empty());
    ///   assert!(p.is_empty());
    ///   assert!(v.is_empty());
    /// }
    /// ```
    #[inline]
    fn remove_archetype<E: Entity + Any>(&self, key: &EntityId<E>)
    where
        Self: Any,
    {
        self.resource::<Archetype<Self, E>>().remove(self, key)
    }
}

impl<T: Entities + Components + Resources> Archetypes for T {}

/// An entity archetype.
pub struct Archetype<R: Entities + Components, E: Entity> {
    drop: Vec<fn(&R, &EntityId<E>)>,
}

impl<R: Entities + Components, E: Entity + Any> Archetype<R, E> {
    /// Creates a new [Archetype].
    #[inline]
    pub fn new() -> Self {
        Self {
            drop: Default::default(),
        }
    }

    /// Registers a [Component] to this [Archetype].
    pub fn register_component<C: Component<E> + Any>(&mut self) {
        use crate::collections::MapMut;

        self.drop.push(|r, e| {
            r.components_mut::<E, C>().remove(e);
        });
    }

    /// Inserts an entity and components to registry.
    pub fn insert<C: Cons>(&self, registry: &R, entity: E, components: C) -> EntityId<E>
    where
        C: InsertComponents<R, E>,
    {
        use crate::collections::Arena;

        let key = registry.entities_mut::<E>().insert(entity);
        components.insert(registry, key);
        key
    }

    /// Removes an entity and its components from registry.
    pub fn remove(&self, registry: &R, key: &EntityId<E>) {
        use crate::collections::Arena;

        for drop in &self.drop {
            drop(registry, key);
        }
        registry.entities_mut::<E>().remove(key);
    }
}

/// Trait for registering an archetype.
pub trait RegisterArchetype<R: Entities + Components + Resources> {
    /// Registers the entity and components represented by self.
    fn register(registry: &mut R);
}

impl<R: Entities + Components + Resources + Any, E: Entity + Any> RegisterArchetype<R> for (E, ()) {
    #[inline(always)]
    fn register(registry: &mut R) {
        registry.register_entity::<E>();
        registry.register_resource(Archetype::<R, E>::new());
    }
}

impl<
        R: Entities + Components + Resources + Any,
        E: Entity + Any,
        C: Component<E> + Any,
        Tail: Cons,
    > RegisterArchetype<R> for (E, (C, Tail))
where
    (E, Tail): RegisterArchetype<R>,
{
    #[inline(always)]
    fn register(registry: &mut R) {
        <(E, Tail)>::register(registry);
        registry.register_component::<E, C>();
        registry
            .resource_mut::<Archetype<R, E>>()
            .register_component::<C>();
    }
}

/// Trait for getting archetype storage.
pub trait ArchetypeStorage<'a, R: Entities + Components> {
    /// Cons of storage types.
    type Storage: Cons;

    /// Gets the storage represented by self.
    fn storage(registry: &'a R) -> Self::Storage;
}

impl<'a, R: Entities + Components, E: Entity + Any, C: Cons> ArchetypeStorage<'a, R> for (&'a E, C)
where
    (E, C): ComponentsStorage<'a, R>,
{
    type Storage = (
        Ref<'a, E::Storage>,
        <(E, C) as ComponentsStorage<'a, R>>::Storage,
    );

    #[inline(always)]
    fn storage(registry: &'a R) -> Self::Storage {
        (registry.entities::<E>(), <(E, C)>::components(registry))
    }
}

impl<'a, R: Entities + Components, E: Entity + Any, C: Cons> ArchetypeStorage<'a, R>
    for (&'a mut E, C)
where
    (E, C): ComponentsStorage<'a, R>,
{
    type Storage = (
        RefMut<'a, E::Storage>,
        <(E, C) as ComponentsStorage<'a, R>>::Storage,
    );

    #[inline(always)]
    fn storage(registry: &'a R) -> Self::Storage {
        (registry.entities_mut::<E>(), <(E, C)>::components(registry))
    }
}

/// Trait for getting archetype component storages.
pub trait ComponentsStorage<'a, R: Components> {
    /// Cons of storage types.
    type Storage: Cons;

    /// Gets the storage represented by self.
    fn components(registry: &'a R) -> Self::Storage;
}

impl<'a, R: Components, E: Entity + Any> ComponentsStorage<'a, R> for (E, ()) {
    type Storage = ();

    #[inline(always)]
    fn components(_registry: &'a R) -> Self::Storage {
        ()
    }
}

impl<'a, R: Components, E: Entity + Any, C: Component<E> + Any, Tail: Cons> ComponentsStorage<'a, R>
    for (E, (&'a C, Tail))
where
    (E, Tail): ComponentsStorage<'a, R>,
{
    type Storage = (
        Ref<'a, C::Storage>,
        <(E, Tail) as ComponentsStorage<'a, R>>::Storage,
    );

    #[inline(always)]
    fn components(registry: &'a R) -> Self::Storage {
        (
            registry.components::<E, C>(),
            <(E, Tail)>::components(registry),
        )
    }
}

impl<'a, R: Components, E: Entity + Any, C: Component<E> + Any, Tail: Cons> ComponentsStorage<'a, R>
    for (E, (&'a mut C, Tail))
where
    (E, Tail): ComponentsStorage<'a, R>,
{
    type Storage = (
        RefMut<'a, C::Storage>,
        <(E, Tail) as ComponentsStorage<'a, R>>::Storage,
    );

    #[inline(always)]
    fn components(registry: &'a R) -> Self::Storage {
        (
            registry.components_mut::<E, C>(),
            <(E, Tail)>::components(registry),
        )
    }
}

/// Trait for inserting components to [Entity].
pub trait InsertComponents<R: Components, E: Entity>: Sized {
    /// Inserts components held by self to given [EntityId].
    #[inline(always)]
    fn insert(self, _registry: &R, _key: EntityId<E>) {}
}

impl<'a, R: Components, E: Entity> InsertComponents<R, E> for () {}

impl<'a, R: Components, E: Entity + Any, C: Component<E> + Any, Tail: Cons> InsertComponents<R, E>
    for (C, Tail)
where
    Tail: InsertComponents<R, E>,
{
    #[inline(always)]
    fn insert(self, registry: &R, key: EntityId<E>) {
        use crate::collections::MapMut;

        registry.components_mut::<E, C>().insert(key, self.0);
        self.1.insert(registry, key);
    }
}
