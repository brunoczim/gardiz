#[cfg(test)]
mod test;

use crate::{coord::CoordPair, direc::Direction, map, map::Map};
use std::{borrow::Borrow, iter::FromIterator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set<T>
where
    T: Ord,
{
    inner: Map<T, ()>,
}

impl<T> Default for Set<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Set<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Set { inner: Map::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn contains<U>(&self, point: CoordPair<&U>) -> bool
    where
        T: Borrow<U>,
        U: Ord,
    {
        self.inner.contains(point)
    }

    pub fn neighbours<U>(
        &self,
        point: CoordPair<&U>,
        direction: Direction,
    ) -> Neighbours<T>
    where
        T: Borrow<U>,
        U: Ord,
    {
        Neighbours { inner: self.inner.neighbours(point, direction) }
    }

    pub fn first_neighbour<U>(
        &self,
        point: CoordPair<&U>,
        direction: Direction,
    ) -> Option<CoordPair<&T>>
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.inner.first_neighbour(point, direction)
    }

    pub fn last_neighbour<U>(
        &self,
        point: CoordPair<&U>,
        direction: Direction,
    ) -> Option<CoordPair<&T>>
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.inner.last_neighbour(point, direction)
    }

    pub fn insert(&mut self, point: CoordPair<T>) -> bool
    where
        T: Clone,
    {
        self.inner.insert(point, ()).is_none()
    }

    pub fn remove<U>(&mut self, point: CoordPair<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.inner.remove(point).is_some()
    }

    pub fn rows(&self) -> Rows<T> {
        Rows { inner: self.inner.rows() }
    }

    pub fn columns(&self) -> Columns<T> {
        Columns { inner: self.inner.columns() }
    }
}

impl<T> Extend<CoordPair<T>> for Set<T>
where
    T: Ord + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = CoordPair<T>>,
    {
        self.inner.extend(iter.into_iter().map(|key| (key, ())));
    }
}

impl<T> FromIterator<CoordPair<T>> for Set<T>
where
    T: Ord + Clone,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = CoordPair<T>>,
    {
        Self { inner: iter.into_iter().map(|key| (key, ())).collect() }
    }
}

#[derive(Debug)]
pub struct Neighbours<'set, T>
where
    T: Ord,
{
    inner: map::Neighbours<'set, T, ()>,
}

impl<'set, T> Iterator for Neighbours<'set, T>
where
    T: Ord,
{
    type Item = CoordPair<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'set, T> DoubleEndedIterator for Neighbours<'set, T>
where
    T: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(key, _)| key)
    }
}

#[derive(Debug)]
pub struct Rows<'set, T>
where
    T: Ord,
{
    inner: map::Rows<'set, T, ()>,
}

impl<'set, T> Iterator for Rows<'set, T>
where
    T: Ord,
{
    type Item = CoordPair<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'set, T> DoubleEndedIterator for Rows<'set, T>
where
    T: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(key, _)| key)
    }
}

#[derive(Debug)]
pub struct Columns<'set, T>
where
    T: Ord,
{
    inner: map::Columns<'set, T, ()>,
}

impl<'set, T> Iterator for Columns<'set, T>
where
    T: Ord,
{
    type Item = CoordPair<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'set, T> DoubleEndedIterator for Columns<'set, T>
where
    T: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(key, _)| key)
    }
}
