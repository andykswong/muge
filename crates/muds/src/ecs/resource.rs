//! Resource registry types.

use super::registry::{Ref, RefMut};
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
