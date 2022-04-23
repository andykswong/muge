//! Registry type.

use super::resource::{Resource, ResourceLocator, Resources};
use crate::collections::AnyMap;
use core::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
};

/// A registry of resources.
#[derive(Debug, Default)]
pub struct Registry {
    resources: AnyMap,
}

impl Registry {
    /// Registers a resource type with given initial value.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::Registry;
    /// let mut registry = Registry::default();
    /// registry.register(1u32);
    /// ```
    pub fn register<R: Any>(&mut self, value: R) {
        if !self.has::<R>() {
            self.resources.insert::<RefCell<R>>(RefCell::new(value));
        }
    }

    /// Returns `true` if the map contains a value of type `R`.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::Registry;
    /// let mut registry = Registry::default();
    /// registry.register(1u32);
    /// assert!(registry.has::<u32>());
    /// ```
    #[inline]
    pub fn has<R: Any>(&self) -> bool {
        self.resources.contains::<RefCell<R>>()
    }

    /// Gets a resource.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::Registry;
    /// let mut registry = Registry::default();
    /// registry.register(1u32);
    /// assert_eq!(*registry.get::<u32>(), 1);
    /// ```
    pub fn get<'a, R: Any>(&'a self) -> Ref<'a, R> {
        self.resources
            .get::<RefCell<R>>()
            .expect("resource not registered")
            .borrow()
    }

    /// Gets a resource mutably.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::Registry;
    /// let mut registry = Registry::default();
    /// registry.register(1u32);
    /// *registry.get_mut::<u32>() = 2;
    /// assert_eq!(*registry.get::<u32>(), 2);
    /// ```
    pub fn get_mut<'a, R: Any>(&'a self) -> RefMut<'a, R> {
        self.resources
            .get::<RefCell<R>>()
            .expect("resource not registered")
            .borrow_mut()
    }
}

impl<'a, R: Any> ResourceLocator<'a, R> for Registry {
    type Ref = Ref<'a, R>;
    type RefMut = RefMut<'a, R>;

    #[inline]
    fn get(&'a self) -> Self::Ref {
        self.get::<R>()
    }

    #[inline]
    fn get_mut(&'a self) -> Self::RefMut {
        self.get_mut::<R>()
    }
}

impl Resources for Registry {
    #[inline]
    fn register_resource<R: Resource>(&mut self, value: R) {
        self.register(value)
    }

    #[inline]
    fn has_resource<R: Resource>(&self) -> bool {
        self.has::<R>()
    }
}
