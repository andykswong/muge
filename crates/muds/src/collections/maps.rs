//! Map trait implementations.

use super::{IterableMap, IterableMapMut, Map, MapMut};
use alloc::collections::BTreeMap;

/// Implement `Map` trait for a compatible map type
macro_rules! impl_map {
    ( $name:ident < Key $( : $kclt:tt $(+ $kdlt:tt )* )? , Value $( : $vclt:tt $(+ $vdlt:tt )* )? >) => {
        impl<K $( : $kclt $(+ $kdlt )* )?, V $( : $vclt $(+ $vdlt )* )?> Map
        for $name <K, V> {
            type Key = K;
            type Value = V;

            #[inline]
            fn len(&self) -> usize {
                $name::len(self)
            }

            #[inline]
            fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
                $name::get(self, key)
            }
        }
    };
}

/// Implement `MapMut` trait for a compatible map type
macro_rules! impl_map_mut {
    ( $name:ident < Key $( : $kclt:tt $(+ $kdlt:tt )* )? , Value $( : $vclt:tt $(+ $vdlt:tt )* )? >) => {
        impl_map!($name<Key $( : $kclt $(+ $kdlt )* )?, Value $( : $vclt $(+ $vdlt )* )?>);

        impl<K $( : $kclt $(+ $kdlt )* )?, V $( : $vclt $(+ $vdlt )* )?> MapMut
        for $name <K, V> {
            #[inline]
            fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
                $name::get_mut(self, key)
            }

            #[inline]
            fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
                $name::insert(self, key, value)
            }

            #[inline]
            fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
                $name::remove(self, key)
            }

            #[inline]
            fn clear(&mut self) {
                $name::clear(self)
            }

            #[inline]
            fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool) {
                $name::retain(self, f)
            }
        }
    };
}

/// Implement `IterableMap` trait for a compatible map type
macro_rules! impl_iterable_map {
    ( $name:ident < Key $( : $kclt:tt $(+ $kdlt:tt )* )? , Value $( : $vclt:tt $(+ $vdlt:tt )* )? >) => {
        impl<'a, K $( : $kclt $(+ $kdlt )* )?, V $( : $vclt $(+ $vdlt )* )?> IterableMap<'a>
        for $name <K, V> where Self: 'a {
            type Iter = <&'a Self as IntoIterator>::IntoIter;

            #[inline]
            fn iter(&'a self) -> Self::Iter {
                IntoIterator::into_iter(self)
            }
        }
    };
}

/// Implement `IterableMapMut` trait for a compatible map type
macro_rules! impl_iterable_map_mut {
    ( $name:ident < Key $( : $kclt:tt $(+ $kdlt:tt )* )? , Value $( : $vclt:tt $(+ $vdlt:tt )* )? >) => {
        impl_iterable_map!($name<Key $( : $kclt $(+ $kdlt )* )?, Value $( : $vclt $(+ $vdlt )* )?>);

        impl<'a, K $( : $kclt $(+ $kdlt )* )?, V $( : $vclt $(+ $vdlt )* )?> IterableMapMut<'a>
        for $name <K, V> where Self: 'a {
            type IterMut = <&'a mut Self as IntoIterator>::IntoIter;

            #[inline]
            fn iter_mut(&'a mut self) -> Self::IterMut {
                IntoIterator::into_iter(self)
            }
        }
    };
}

impl_map_mut!(BTreeMap<Key: Ord, Value>);
impl_iterable_map_mut!(BTreeMap<Key: Ord, Value>);

#[cfg(feature = "std")]
mod hashmap {
    use super::{IterableMap, IterableMapMut, Map, MapMut};
    use core::hash::Hash;
    use std::collections::HashMap;

    impl_map_mut!(HashMap<Key: Eq + Hash, Value>);
    impl_iterable_map_mut!(HashMap<Key: Eq + Hash, Value>);
}
