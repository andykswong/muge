//! Archetype helpers.

use super::{
    registry::{Ref, RefMut},
    Component, Components, Entities, Entity,
};
use crate::collections::Cons;
use core::any::Any;

/// Registry for archetypes.
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
pub trait Archetypes: Entities + Components + Sized {
    /// Registers an archetype.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::Cons;
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
    /// ```
    #[inline]
    fn register_archetype<E: Entity + Any, C: Cons>(&mut self)
    where
        (E, C): RegisterArchetype<Self>,
    {
        <(E, C)>::register(self);
    }

    /// Gets the mutable or immutable storages of an archetype.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{cons, Cons};
    /// # use muds::collections::MapMut;
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
}

impl<T: Entities + Components> Archetypes for T {}

/// Trait for registering an archetype.
pub trait RegisterArchetype<R: Entities + Components> {
    /// Registers the entity and components represented by self.
    fn register(registry: &mut R);
}

impl<R: Entities + Components, E: Entity + Any> RegisterArchetype<R> for (E, ()) {
    #[inline(always)]
    fn register(registry: &mut R) {
        registry.register_entity::<E>();
    }
}

impl<R: Entities + Components, E: Entity + Any, C: Component<E> + Any, Tail: Cons>
    RegisterArchetype<R> for (E, (C, Tail))
where
    (E, Tail): RegisterArchetype<R>,
{
    #[inline(always)]
    fn register(registry: &mut R) {
        <(E, Tail)>::register(registry);
        registry.register_component::<E, C>();
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
