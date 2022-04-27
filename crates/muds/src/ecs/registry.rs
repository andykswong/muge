//! Registry of resources.

use alloc::boxed::Box;
use core::{
    any::{Any, TypeId},
    convert::TryFrom,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// A registry of resources.
#[derive(Debug)]
pub struct Registry {
    data: RegistryData,
}

impl Registry {
    /// Creates a new [Registry].
    #[inline]
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    /// Gets the underlying map.
    #[inline]
    pub fn data(&self) -> &RegistryData {
        &self.data
    }

    /// Gets the underlying map mutably.
    #[inline]
    pub fn data_mut(&mut self) -> &mut RegistryData {
        &mut self.data
    }

    /// Registers a resource with given key and initial value.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, RegistryKey};
    /// let mut registry = Registry::default();
    /// registry.register(RegistryKey::from_type::<u32>(), 1u32);
    /// ```
    pub fn register<R: Any>(&mut self, key: RegistryKey, value: R) {
        if !self.contains_key(&key) {
            self.data.insert(key, RefCell::new(Box::new(value)));
        }
    }

    /// Returns `true` if the registry contains given [RegistryKey].
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, RegistryKey};
    /// let mut registry = Registry::default();
    /// let key = RegistryKey::from_type::<u32>();
    /// registry.register(key, 1u32);
    /// assert!(registry.contains_key(&key));
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &RegistryKey) -> bool {
        self.data.contains_key(key)
    }

    /// Gets a resource of given [RegistryKey].
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, RegistryKey};
    /// let mut registry = Registry::default();
    /// let key = RegistryKey::from_type::<u32>();
    /// registry.register(key, 1u32);
    /// assert_eq!(*registry.get::<u32>(&key).unwrap(), 1);
    /// ```
    #[inline]
    pub fn get<'a, R: Any>(&'a self, key: &RegistryKey) -> Option<Ref<'a, R>> {
        self.data.get(key).map(RefCell::borrow).try_into().ok()
    }

    /// Gets a resource mutably.
    ///
    /// # Examples
    /// ```rust
    /// # use muds::ecs::{Registry, RegistryKey};
    /// let mut registry = Registry::default();
    /// let key = RegistryKey::from_type::<u32>();
    /// registry.register(key, 1u32);
    /// *registry.get_mut::<u32>(&key).unwrap() = 2;
    /// assert_eq!(*registry.get::<u32>(&key).unwrap(), 2);
    /// ```
    #[inline]
    pub fn get_mut<'a, R: Any>(&'a self, key: &RegistryKey) -> Option<RefMut<'a, R>> {
        self.data.get(key).map(RefCell::borrow_mut).try_into().ok()
    }
}

impl Default for Registry {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Registry data map type.
pub type RegistryData = BackingMap<RegistryKey, RefCell<Box<dyn Any>>>;

// TODO: Add a feature to enable the use of RwLock for multithreaded applications. 
/// Ref cell type.
type RefCell<T> = core::cell::RefCell<T>;

/// Registry data backing map type.
#[cfg(feature = "std")]
type BackingMap<K, V> = std::collections::HashMap<K, V>;
/// Registry data backing map type.
#[cfg(not(feature = "std"))]
type BackingMap<K, V> = alloc::collections::BTreeMap<K, V>;

/// Key of a [Registry].
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegistryKey {
    /// A key for a static type.
    Type(TypeId),
    /// A key for a dynamic type specified by a [u32] ID.
    Id(u32),
}

impl RegistryKey {
    #[inline]
    pub fn from_type<T: Any>() -> Self {
        RegistryKey::Type(TypeId::of::<T>())
    }

    #[inline]
    pub fn from_id(id: u32) -> Self {
        RegistryKey::Id(id)
    }
}

/// Wraps a borrowed reference to a value in a [Registry].
pub struct Ref<'a, T: Any> {
    guard: core::cell::Ref<'a, Box<dyn Any>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: Any> TryFrom<Option<core::cell::Ref<'a, Box<dyn Any>>>> for Ref<'a, T> {
    type Error = &'static str;

    #[inline]
    fn try_from(guard: Option<core::cell::Ref<'a, Box<dyn Any>>>) -> Result<Self, Self::Error> {
        guard
            .and_then(|guard| {
                if guard.deref().downcast_ref::<T>().is_some() {
                    Some(Self {
                        guard,
                        phantom: PhantomData,
                    })
                } else {
                    None
                }
            })
            .ok_or("resource not registered")
    }
}

impl<T: Any> Deref for Ref<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.guard
            .deref()
            .downcast_ref()
            .expect("resource not registered")
    }
}

/// Wraps a mutably borrowed reference to a value in a [Registry].
pub struct RefMut<'a, T> {
    guard: core::cell::RefMut<'a, Box<dyn Any>>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T: Any> TryFrom<Option<core::cell::RefMut<'a, Box<dyn Any>>>> for RefMut<'a, T> {
    type Error = &'static str;

    #[inline]
    fn try_from(guard: Option<core::cell::RefMut<'a, Box<dyn Any>>>) -> Result<Self, Self::Error> {
        guard
            .and_then(|guard| {
                if guard.deref().downcast_ref::<T>().is_some() {
                    Some(Self {
                        guard,
                        phantom: PhantomData,
                    })
                } else {
                    None
                }
            })
            .ok_or("resource not registered")
    }
}

impl<T: Any> Deref for RefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.guard
            .deref()
            .downcast_ref()
            .expect("resource not registered")
    }
}

impl<T: Any> DerefMut for RefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard
            .deref_mut()
            .downcast_mut()
            .expect("resource not registered")
    }
}
