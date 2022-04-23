//! Archetype helpers.

use core::marker::PhantomData;

use super::{resource::ResourceLocator, Component, Components, Entities, Entity, Resources};
use crate::collections::Cons;

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
/// let cons!(mut ent, mut pos, mut vel) = registry.storage_mut::<E, Cons!(&mut Pos, &mut Vel)>();
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
    fn register_archetype<E: Entity, C: Cons>(&mut self)
    where
        Archetype<E, C>: RegisterComponents<Self>,
    {
        self.register_entity::<E>();
        self.register_components::<E, C>();
    }

    /// Registers components for an entity type.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::Cons;
    /// # use muds::ecs::{Archetypes, Registry, Component, Entities, Entity, storage::{ArenaStorage, VecStorage}};
    /// struct E;
    /// struct Pos(u32, u32);
    /// struct Vel(u32, u32);
    /// impl Entity for E { type Storage = ArenaStorage<Self>; }
    /// impl Component<E> for Pos { type Storage = VecStorage<E, Self>; }
    /// impl Component<E> for Vel { type Storage = VecStorage<E, Self>; }
    ///
    /// let mut registry = Registry::default();
    /// registry.register_entity::<E>();
    /// registry.register_components::<E, Cons!(Pos, Vel)>();
    /// ```
    #[inline]
    fn register_components<E: Entity, C: Cons>(&mut self)
    where
        Archetype<E, C>: RegisterComponents<Self>,
    {
        Archetype::<E, C>::default().register(self);
    }

    /// Gets the storages of an archetype.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::{cons, Cons};
    /// # use muds::collections::Map;
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
    /// let cons!(e, p, v) = registry.storage::<E, Cons!(&Pos, &mut Vel)>();
    /// assert_eq!(e.len(), 0);
    /// assert_eq!(p.len(), 0);
    /// assert_eq!(v.len(), 0);
    /// ```
    #[inline]
    fn storage<'a, E: Entity, C: Cons>(
        &'a self,
    ) -> (
        <Self as ResourceLocator<'a, E::Storage>>::Ref,
        <Archetype<E, C> as ComponentStorages<'a, Self>>::Storage,
    )
    where
        Self: ResourceLocator<'a, E::Storage>,
        Archetype<E, C>: ComponentStorages<'a, Self>,
    {
        (
            self.entities::<E>(),
            Archetype::<E, C>::default().components(self),
        )
    }

    /// Gets the storages of an archetype with mutable entity data.
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
    /// let cons!(mut e, mut p, mut v) = registry.storage_mut::<E, Cons!(&mut Pos, &mut Vel)>();
    /// let eid = e.insert(E);
    /// p.insert(eid, Pos(1, 2));
    /// v.insert(eid, Vel(1, 2));
    /// ```
    #[inline]
    fn storage_mut<'a, E: Entity, C: Cons>(
        &'a self,
    ) -> (
        <Self as ResourceLocator<'a, E::Storage>>::RefMut,
        <Archetype<E, C> as ComponentStorages<'a, Self>>::Storage,
    )
    where
        Self: ResourceLocator<'a, E::Storage>,
        Archetype<E, C>: ComponentStorages<'a, Self>,
    {
        (
            self.entities_mut::<E>(),
            Archetype::<E, C>::default().components(self),
        )
    }
}

impl<T: Resources> Archetypes for T {}

/// Archetype (entity-components bundle) type token.
#[derive(Debug)]
pub struct Archetype<E, C: Cons>(PhantomData<(*const E, *const C)>);

impl<E: Entity, C: Cons> Default for Archetype<E, C> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Trait for registering components.
pub trait RegisterComponents<R: Components> {
    /// Registers the components represented by self.
    fn register(&self, registry: &mut R);
}

impl<R: Components, E: Entity> RegisterComponents<R> for Archetype<E, ()> {
    #[inline(always)]
    fn register(&self, _registry: &mut R) {}
}

impl<R: Components, E: Entity, C: Component<E>, Tail: Cons> RegisterComponents<R>
    for Archetype<E, (C, Tail)>
where
    Archetype<E, Tail>: RegisterComponents<R>,
{
    #[inline(always)]
    fn register(&self, registry: &mut R) {
        registry.register_component::<E, C>();
        Archetype::<E, Tail>::default().register(registry);
    }
}

/// Trait for getting component storages.
pub trait ComponentStorages<'a, R: Components> {
    /// Cons of storages type.
    type Storage: Cons;

    /// Gets the entity-component storage represented by self.
    fn components(&self, registry: &'a R) -> Self::Storage;
}

impl<'a, R: Components, E: Entity> ComponentStorages<'a, R> for Archetype<E, ()> {
    type Storage = ();

    #[inline(always)]
    fn components(&self, _registry: &'a R) -> Self::Storage {
        ()
    }
}

impl<'a, R: Components, E: Entity, C: Component<E> + 'static, Tail: Cons> ComponentStorages<'a, R>
    for Archetype<E, (&'a C, Tail)>
where
    R: ResourceLocator<'a, C::Storage>,
    Archetype<E, Tail>: ComponentStorages<'a, R>,
{
    type Storage = (
        <R as ResourceLocator<'a, C::Storage>>::Ref,
        <Archetype<E, Tail> as ComponentStorages<'a, R>>::Storage,
    );

    #[inline(always)]
    fn components(&self, registry: &'a R) -> Self::Storage {
        (
            registry.components::<E, C>(),
            Archetype::<E, Tail>::default().components(registry),
        )
    }
}

impl<'a, R: Components, E: Entity, C: Component<E> + Sized, Tail: Cons> ComponentStorages<'a, R>
    for Archetype<E, (&'a mut C, Tail)>
where
    R: ResourceLocator<'a, C::Storage>,
    Archetype<E, Tail>: ComponentStorages<'a, R>,
{
    type Storage = (
        <R as ResourceLocator<'a, C::Storage>>::RefMut,
        <Archetype<E, Tail> as ComponentStorages<'a, R>>::Storage,
    );

    #[inline(always)]
    fn components(&self, registry: &'a R) -> Self::Storage {
        (
            registry.components_mut::<E, C>(),
            Archetype::<E, Tail>::default().components(registry),
        )
    }
}
