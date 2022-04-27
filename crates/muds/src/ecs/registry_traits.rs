use super::{
    registry::{Ref, RefMut},
    Component, Components, Entities, Entity, Registry, RegistryKey, Resources,
};
use core::any::Any;

impl Resources for Registry {
    #[inline]
    fn register_resource<R: Any>(&mut self, value: R) {
        self.register(RegistryKey::from_type::<R>(), value)
    }

    #[inline]
    fn has_resource<R: Any>(&self) -> bool {
        self.contains_key(&RegistryKey::from_type::<R>())
    }

    #[inline]
    fn resource<'a, R: Any>(&'a self) -> Ref<'a, R> {
        self.get::<R>(&RegistryKey::from_type::<R>())
            .expect("resource not registered")
    }

    #[inline]
    fn resource_mut<'a, R: Any>(&'a self) -> RefMut<'a, R> {
        self.get_mut::<R>(&RegistryKey::from_type::<R>())
            .expect("resource not registered")
    }
}

impl Entities for Registry {
    #[inline]
    fn register_entity<E: Entity + Any>(&mut self) {
        self.register(RegistryKey::from_type::<E>(), E::Storage::default());
    }

    #[inline]
    fn has_entity<E: Entity + Any>(&self) -> bool {
        self.contains_key(&RegistryKey::from_type::<E>())
    }

    #[inline]
    fn entities<'a, E: Entity + Any>(&'a self) -> Ref<'a, E::Storage> {
        self.get::<E::Storage>(&RegistryKey::from_type::<E>())
            .expect("entity not registered")
    }

    #[inline]
    fn entities_mut<'a, E: Entity + Any>(&'a self) -> RefMut<'a, E::Storage> {
        self.get_mut::<E::Storage>(&RegistryKey::from_type::<E>())
            .expect("entity not registered")
    }
}

impl Components for Registry {
    #[inline]
    fn register_component<E: Entity + Any, C: Component<E> + Any>(&mut self) {
        self.register(RegistryKey::from_type::<(E, C)>(), C::Storage::default());
    }

    #[inline]
    fn has_component<E: Entity + Any, C: Component<E> + Any>(&self) -> bool {
        self.contains_key(&RegistryKey::from_type::<(E, C)>())
    }

    #[inline]
    fn components<'a, E: Entity + Any, C: Component<E> + Any>(&'a self) -> Ref<'a, C::Storage> {
        self.get::<C::Storage>(&RegistryKey::from_type::<(E, C)>())
            .expect("component not registered")
    }

    #[inline]
    fn components_mut<'a, E: Entity + Any, C: Component<E> + Any>(
        &'a self,
    ) -> RefMut<'a, C::Storage> {
        self.get_mut::<C::Storage>(&RegistryKey::from_type::<(E, C)>())
            .expect("component not registered")
    }
}
