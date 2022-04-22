//! Helper traits for joining with `Map`s.

use core::iter::FusedIterator;

use super::{Map, MapMut};

/// Iterator trait for joining with [Map]s.
pub trait MapJoin<'a, K: 'a, V>: Iterator<Item = (&'a K, V)> + Sized {
    /// Returns an iterator adaptor that wraps items as `Cons`, i.e. `(key, (value, ())`.
    /// Useful for chaining with multiple joins where the resulting item will be a valid `Cons`.
    #[inline(always)]
    fn cons(self) -> core::iter::Map<Self, fn((&'a K, V)) -> (&'a K, (V, ()))> {
        self.map(|(k, v)| (k, (v, ())))
    }

    /// Returns an iterator adaptor that inner joins this iterator with a `Map`.
    #[inline(always)]
    fn map_join<M>(self, rhs: &'a M) -> MapJoinIter<Self, &'a M>
    where
        M: Map<Key = K>,
    {
        MapJoinIter {
            iter: self,
            map: rhs,
        }
    }

    /// Returns an iterator adaptor that left joins this iterator with a `Map`.
    #[inline(always)]
    fn map_join_left<M>(self, rhs: &'a M) -> MapJoinLeftIter<Self, &'a M>
    where
        M: Map<Key = K>,
    {
        MapJoinLeftIter {
            iter: self,
            map: rhs,
        }
    }

    /// Returns an iterator adaptor that left exclusive joins this iterator with a `Map`.
    /// The returned iterator will yield only the elements with keys not in the RHS map.
    #[inline(always)]
    fn map_join_left_excl<M>(self, rhs: &'a M) -> MapJoinLeftExclIter<Self, &'a M>
    where
        M: Map<Key = K>,
    {
        MapJoinLeftExclIter {
            iter: self,
            map: rhs,
        }
    }

    /// Inner joins with a `MapMut`.
    ///
    /// # Safety
    /// Self must be a map iterator that never returns duplicate keys.
    /// Otherwise, this method may potentially hand out multiple mutable references to the same RHS value!
    #[inline(always)]
    unsafe fn map_join_mut<M>(self, rhs: &'a mut M) -> MapJoinIter<Self, &'a mut M>
    where
        M: MapMut<Key = K>,
    {
        MapJoinIter {
            iter: self,
            map: rhs,
        }
    }

    /// Left joins with a `MapMut`.
    ///
    /// # Safety
    /// Self must be a map iterator that never returns duplicate keys.
    /// Otherwise, this method may potentially hand out multiple mutable references to the same RHS value!
    #[inline(always)]
    unsafe fn map_join_left_mut<M>(self, rhs: &'a mut M) -> MapJoinLeftIter<Self, &'a mut M>
    where
        M: MapMut<Key = K>,
    {
        MapJoinLeftIter {
            iter: self,
            map: rhs,
        }
    }
}

impl<'a, T, K, V> MapJoin<'a, K, V> for T
where
    T: Iterator<Item = (&'a K, V)> + Sized,
    K: 'a,
{
}

/// Iterator adaptor that inner joins 2 maps.
#[derive(Debug)]
pub struct MapJoinIter<LHS: Iterator, RHS> {
    iter: LHS,
    map: RHS,
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinIter<LHS, &'a RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: Map<Key = K>,
{
    type Item = (&'a K, (&'a RHS::Value, V));

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key, lval)) = self.iter.next() {
            if let Some(rval) = self.map.get(key) {
                return Some((key, (rval, lval)));
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinIter<LHS, &'a mut RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: MapMut<Key = K>,
{
    type Item = (&'a K, (&'a mut RHS::Value, V));

    /// Advances the iterator and returns the next value.
    ///
    /// # Safety
    /// LHS must be a map iterator that does not return duplicate keys.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key, lval)) = self.iter.next() {
            if let Some(rval) = self.map.get_mut(key) {
                // Safety: there must be no duplicate key so that we do not hand out
                // multiple mutable references to the same value within RHS
                let rval = unsafe { &mut *(rval as *mut _) };
                return Some((key, (rval, lval)));
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<LHS, RHS> FusedIterator for MapJoinIter<LHS, RHS>
where
    Self: Iterator,
    LHS: FusedIterator,
{
}

/// Iterator adaptor that left joins 2 maps.
#[derive(Debug)]
pub struct MapJoinLeftIter<LHS: Iterator, RHS> {
    iter: LHS,
    map: RHS,
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinLeftIter<LHS, &'a RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: Map<Key = K>,
{
    type Item = (&'a K, (Option<&'a RHS::Value>, V));

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(key, lval)| (key, (self.map.get(key), lval)))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinLeftIter<LHS, &'a mut RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: MapMut<Key = K>,
{
    type Item = (&'a K, (Option<&'a mut RHS::Value>, V));

    /// Advances the iterator and returns the next value.
    ///
    /// # Safety
    /// LHS must be a map iterator that does not return duplicate keys.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, lval)| {
            let rval = self
                .map
                .get_mut(key)
                // Safety: there must be no duplicate key so that we do not hand out
                // multiple mutable references to the same value within RHS
                .map(|rval| unsafe { &mut *(rval as *mut _) });
            (key, (rval, lval))
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<LHS, RHS> FusedIterator for MapJoinLeftIter<LHS, RHS>
where
    Self: Iterator,
    LHS: FusedIterator,
{
}

/// Iterator adaptor that left exclusive joins 2 maps.
#[derive(Debug)]
pub struct MapJoinLeftExclIter<LHS: Iterator, RHS> {
    iter: LHS,
    map: RHS,
}

impl<'a, K: 'a, V, LHS, RHS> Iterator for MapJoinLeftExclIter<LHS, &'a RHS>
where
    LHS: Iterator<Item = (&'a K, V)>,
    RHS: Map<Key = K>,
{
    type Item = LHS::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key, val)) = self.iter.next() {
            if !self.map.contains_key(key) {
                return Some((key, val));
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<LHS, RHS> FusedIterator for MapJoinLeftExclIter<LHS, RHS>
where
    Self: Iterator,
    LHS: FusedIterator,
{
}
